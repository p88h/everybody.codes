//! All-pairs shortest distances of an undirected, unit-weight graph, with fast "what if these
//! edges were added" queries.
//!
//! Adding an edge a-b can only help a pair u, v through the paths u..a-b..v or u..b-a..v, so
//! d'(u, v) = min(d(u, v), d(u, a) + 1 + d(b, v), d(u, b) + 1 + d(a, v)). With the graph undirected,
//! the distances to a and b are rows a and b, so each update is one vectorised pass over the matrix.
//!
//! Distances are bytes, stored row by row in 16-byte chunks. Pairs 127 or more steps apart count as
//! unreachable and are stored as [`UNREACHABLE`]; that keeps every sum above within a byte
//! (at most 128 + 127), and such a sum is never below a stored distance, so it is never taken.
//!
//! Example: the edge that shortens a graph's total distance the most.
//! ```ignore
//! let dist = DistMatrix::from_neighbours(n, |u| adj[u].iter().copied());
//! let best = pairs.iter().map(|&(a, b)| dist.total_with([(a, b)])).min();
//! ```

pub const UNREACHABLE: u8 = 127;

const LANES: usize = 16;
type Chunk = [u8; LANES];

// rows copied out per query live on the stack up to this many chunks (1 KiB), on the heap beyond
const STACK_CHUNKS: usize = 64;

// keeps a row's sum per lane (at most 512 chunks * 126) within a u16
const MAX_NODES: usize = 512 * LANES;

pub struct DistMatrix {
    n: usize,
    // chunks per row; padding columns are 0 and so add nothing to any total
    width: usize,
    dist: Vec<Chunk>,
    // no pair is unreachable, so totals need not filter out UNREACHABLE
    connected: bool,
}

impl Clone for DistMatrix {
    fn clone(&self) -> Self {
        DistMatrix { dist: self.dist.clone(), ..*self }
    }

    // reuses the allocation, for resetting a work copy in a loop
    fn clone_from(&mut self, source: &Self) {
        (self.n, self.width, self.connected) = (source.n, source.width, source.connected);
        self.dist.clone_from(&source.dist);
    }
}

impl DistMatrix {
    fn empty(n: usize) -> Self {
        assert!(n <= MAX_NODES);
        let width = n.div_ceil(LANES);
        let mut dist = vec![[0; LANES]; n * width];
        for row in dist.chunks_exact_mut(width) {
            row.as_flattened_mut()[..n].fill(UNREACHABLE);
        }
        DistMatrix { n, width, dist, connected: false }
    }

    fn check_connected(&mut self) {
        self.connected = self.dist.iter().all(|c| !c.contains(&UNREACHABLE));
    }

    /// BFS from every node; `neighbours(u)` lists the nodes next to `u`, which must be symmetric.
    /// Up to 8192 nodes. O(n × (n + edges)).
    pub fn from_neighbours<I: IntoIterator<Item = usize>>(n: usize, neighbours: impl Fn(usize) -> I) -> Self {
        let mut m = Self::empty(n);
        let mut queue = Vec::with_capacity(n);
        for (start, row) in m.dist.chunks_exact_mut(m.width).enumerate() {
            let row = row.as_flattened_mut();
            row[start] = 0;
            queue.clear();
            queue.push(start);
            let mut head = 0;
            while let Some(&u) = queue.get(head) {
                head += 1;
                let next = row[u] + 1;
                if next == UNREACHABLE {
                    break;
                }
                for v in neighbours(u) {
                    if row[v] == UNREACHABLE {
                        row[v] = next;
                        queue.push(v);
                    }
                }
            }
        }
        m.check_connected();
        m
    }

    /// Like [`from_neighbours`](Self::from_neighbours), for up to 128 nodes given as neighbour
    /// bitmasks: bit v of `adj[u]` is set when u and v are joined. Much faster on dense graphs.
    pub fn from_bitsets(adj: &[u128]) -> Self {
        assert!(adj.len() <= 128);
        let mut m = Self::empty(adj.len());
        for (start, row) in m.dist.chunks_exact_mut(m.width).enumerate() {
            let row = row.as_flattened_mut();
            row[start] = 0;
            let mut seen = 1u128 << start;
            let mut frontier = seen;
            let mut depth = 0;
            while frontier != 0 && depth + 1 < UNREACHABLE {
                depth += 1;
                let mut next = 0;
                while frontier != 0 {
                    next |= adj[frontier.trailing_zeros() as usize];
                    frontier &= frontier - 1;
                }
                next &= !seen;
                seen |= next;
                frontier = next;
                while next != 0 {
                    row[next.trailing_zeros() as usize] = depth;
                    next &= next - 1;
                }
            }
        }
        m.check_connected();
        m
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Distance from `u` to `v`, or `None` if unreachable (or 127 or more steps away).
    pub fn get(&self, u: usize, v: usize) -> Option<u8> {
        let d = self.row(u).as_flattened()[v];
        (d != UNREACHABLE).then_some(d)
    }

    fn row(&self, u: usize) -> &[Chunk] {
        &self.dist[u * self.width..][..self.width]
    }

    /// Adds the edge a-b. O(n²).
    pub fn add_edge(&mut self, a: usize, b: usize) {
        let width = self.width;
        with_scratch(2 * width, |scratch| {
            let (ra, rb) = scratch.split_at_mut(width);
            ra.copy_from_slice(self.row(a));
            rb.copy_from_slice(self.row(b));
            for (u, row) in self.dist.chunks_exact_mut(width).enumerate() {
                apply(row, steps_via(ra, rb, u), ra, rb);
            }
        });
        if !self.connected {
            self.check_connected();
        }
    }

    /// Sum of the distances over all ordered pairs of nodes that reach each other.
    pub fn total(&self) -> usize {
        self.total_with([])
    }

    /// [`total`](Self::total) as it would be with `edges` added, without changing the matrix.
    /// O(K × n²), reading the matrix once.
    pub fn total_with<const K: usize>(&self, edges: [(usize, usize); K]) -> usize {
        if self.connected { self.total_inner::<K, false>(edges) } else { self.total_inner::<K, true>(edges) }
    }

    fn total_inner<const K: usize, const FILTER: bool>(&self, edges: [(usize, usize); K]) -> usize {
        let width = self.width;
        // per edge, its endpoints' rows as they are once the edges before it have been added
        with_scratch(2 * K * width, |ends| {
            for (j, &(a, b)) in edges.iter().enumerate() {
                let (done, rest) = ends.split_at_mut(2 * j * width);
                for (row, x) in rest.chunks_exact_mut(width).zip([a, b]) {
                    row.copy_from_slice(self.row(x));
                    for prev in done.chunks_exact(2 * width) {
                        let (ra, rb) = prev.split_at(width);
                        apply(row, steps_via(ra, rb, x), ra, rb);
                    }
                }
            }
            // slices of exactly `width` chunks, so indexing them by chunk needs no bounds checks
            let ra: [&[Chunk]; K] = std::array::from_fn(|k| &ends[2 * k * width..][..width]);
            let rb: [&[Chunk]; K] = std::array::from_fn(|k| &ends[(2 * k + 1) * width..][..width]);
            let mut lanes = [0u32; LANES];
            for (u, row) in self.dist.chunks_exact(width).enumerate() {
                let row = &row[..width];
                let s: [(u8, u8); K] = std::array::from_fn(|k| steps_via(ra[k], rb[k], u));
                let mut row_lanes = [0u16; LANES];
                for c in 0..width {
                    let mut d = row[c];
                    for k in 0..K {
                        d = shortcut(&d, s[k], &ra[k][c], &rb[k][c]);
                    }
                    for i in 0..LANES {
                        row_lanes[i] += if FILTER && d[i] == UNREACHABLE { 0 } else { d[i] as u16 };
                    }
                }
                for i in 0..LANES {
                    lanes[i] += row_lanes[i] as u32;
                }
            }
            lanes.iter().map(|&x| x as usize).sum()
        })
    }
}

// lowers the distances from u in `row` by the paths over a new edge a-b, given the steps from u to
// it and the distances from its ends `ra`, `rb`
#[inline(always)]
fn apply(row: &mut [Chunk], s: (u8, u8), ra: &[Chunk], rb: &[Chunk]) {
    for (d, (ca, cb)) in row.iter_mut().zip(ra.iter().zip(rb)) {
        *d = shortcut(d, s, ca, cb);
    }
}

// distances from u to the ends a and b of a new edge, plus the step across it
#[inline(always)]
fn steps_via(ra: &[Chunk], rb: &[Chunk], u: usize) -> (u8, u8) {
    (ra.as_flattened()[u] + 1, rb.as_flattened()[u] + 1)
}

#[inline(always)]
fn shortcut(d: &Chunk, (au, bu): (u8, u8), ra: &Chunk, rb: &Chunk) -> Chunk {
    std::array::from_fn(|i| d[i].min(au + rb[i]).min(bu + ra[i]))
}

fn with_scratch<R>(len: usize, f: impl FnOnce(&mut [Chunk]) -> R) -> R {
    if len <= STACK_CHUNKS {
        f(&mut [[0; LANES]; STACK_CHUNKS][..len])
    } else {
        f(&mut vec![[0; LANES]; len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // small deterministic pseudo-random graphs
    fn random_edges(n: usize, m: usize, seed: u64) -> Vec<(usize, usize)> {
        let mut x = seed;
        let mut next = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (x >> 33) as usize
        };
        (0..m).map(|_| (next() % n, next() % n)).collect()
    }

    fn naive(n: usize, edges: &[(usize, usize)]) -> DistMatrix {
        let mut adj = vec![vec![]; n];
        for &(a, b) in edges {
            adj[a].push(b);
            adj[b].push(a);
        }
        DistMatrix::from_neighbours(n, |u| adj[u].clone())
    }

    fn naive_total(n: usize, edges: &[(usize, usize)]) -> usize {
        let m = naive(n, edges);
        (0..n).flat_map(|u| (0..n).map(move |v| (u, v))).filter_map(|(u, v)| m.get(u, v)).map(|d| d as usize).sum()
    }

    fn assert_same(a: &DistMatrix, b: &DistMatrix) {
        assert_eq!(a.len(), b.len());
        for u in 0..a.len() {
            for v in 0..a.len() {
                assert_eq!(a.get(u, v), b.get(u, v), "{u} -> {v}");
            }
        }
    }

    #[test]
    fn test_path() {
        let edges: Vec<_> = (0..4).map(|i| (i, i + 1)).collect();
        let m = naive(5, &edges);
        assert_eq!(m.get(0, 4), Some(4));
        assert_eq!(m.get(3, 1), Some(2));
        assert_eq!(m.get(2, 2), Some(0));
        assert_eq!(m.total(), 2 * (4 + 3 * 2 + 2 * 3 + 4));
    }

    #[test]
    fn test_unreachable() {
        let mut m = naive(4, &[(0, 1), (2, 3)]);
        assert_eq!(m.get(0, 2), None);
        assert_eq!(m.total(), 4);
        assert_eq!(m.total_with([(1, 2)]), naive_total(4, &[(0, 1), (2, 3), (1, 2)]));
        m.add_edge(1, 2);
        assert_same(&m, &naive(4, &[(0, 1), (2, 3), (1, 2)]));
        assert_eq!(m.total_with([(0, 3)]), naive_total(4, &[(0, 1), (2, 3), (1, 2), (0, 3)]));
    }

    #[test]
    fn test_too_far_counts_as_unreachable() {
        let edges: Vec<_> = (0..299).map(|i| (i, i + 1)).collect();
        let m = naive(300, &edges);
        assert_eq!(m.get(0, 126), Some(126));
        assert_eq!(m.get(0, 127), None);
        assert_eq!(m.get(299, 0), None);
        // closing the ring brings far ends within reach
        let ring = [edges.clone(), vec![(0, 299)]].concat();
        let mut closed = m.clone();
        closed.add_edge(0, 299);
        assert_eq!(closed.get(0, 255), Some(45));
        assert_same(&closed, &naive(300, &ring));
        assert_eq!(m.total_with([(0, 299)]), naive_total(300, &ring));
    }

    #[test]
    fn test_bitsets_match_lists() {
        for n in [1, 16, 37, 128] {
            let edges = random_edges(n, 2 * n, n as u64);
            let mut adj = vec![0u128; n];
            for &(a, b) in &edges {
                adj[a] |= 1 << b;
                adj[b] |= 1 << a;
            }
            assert_same(&DistMatrix::from_bitsets(&adj), &naive(n, &edges));
        }
    }

    #[test]
    fn test_added_edges_match_rebuild() {
        // sparse (disconnected) and denser (connected) graphs, sizes on and off chunk boundaries
        for (n, m) in [(10, 8), (32, 64), (37, 40), (150, 200), (150, 600), (300, 280)] {
            let edges = random_edges(n, m, 7 * n as u64);
            let extra = random_edges(n, 3, n as u64);
            let base = naive(n, &edges);
            let all = [edges.clone(), extra.clone()].concat();
            assert_eq!(base.total(), naive_total(n, &edges));
            assert_eq!(base.total_with([extra[0]]), naive_total(n, &[edges.clone(), vec![extra[0]]].concat()));
            assert_eq!(base.total_with([extra[0], extra[1], extra[2]]), naive_total(n, &all));
            let mut grown = base.clone();
            for &(a, b) in &extra {
                grown.add_edge(a, b);
            }
            assert_same(&grown, &naive(n, &all));
        }
    }
}
