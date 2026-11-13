use crate::algo::union_find::UnionFind;

/// Parse just really figures out the stride and pads the input with 'frame' newlines.
fn parse(input: &str) -> (Vec<u8>, usize) {
    let stride = input.find('\n').unwrap_or(input.len()) + 1;
    let mut cells = input.as_bytes().to_vec();
    // pad with '\n' up to whole rows, then add the frame row
    cells.resize((cells.len().div_ceil(stride) + 1) * stride, b'\n');
    (cells, stride)
}

// Match each cell horizontally and vertically with its neighbor 
fn for_each_adjacent(cells: &[u8], stride: usize, mut f: impl FnMut(usize, usize, bool)) {
    let len = cells.len();
    for step in [1, stride] {
        for a in 0..len {
            let b = (a + step) % len;
            f(a, b, cells[a] == cells[b]);
        }
    }
}

pub fn part1(input: &str) -> String {
    let (cells, stride) = parse(input);
    let mut count = 0;
    for_each_adjacent(&cells, stride, |_, _, same| count += (!same) as usize);
    count.to_string()
}

/// A shape-graph edge: ((shape a, shape b), border length).
type Edge = ((usize, usize), usize);

/// Shape count and the shape graph's edges, sorted by length.
fn shape_graph(input: &str) -> (usize, Vec<Edge>) {
    let (cells, stride) = parse(input);
    let mut uf = UnionFind::new(cells.len());
    let mut nodes = cells.len(); // every cell starts as its own shape; each merge removes one
    let mut borders = Vec::new();
    for_each_adjacent(&cells, stride, |a, b, same| {
        if !same {
            borders.push((a, b));
        } else if uf.union(a, b) {
            nodes -= 1;
        }
    });

    // number the shapes by their union-find roots as they turn up, and likewise give each pair of
    // shapes an edge id the first time they meet; every border unit then adds 1 to its edge
    let mut node = vec![usize::MAX; cells.len()];
    // frame is node 0
    node[uf.find(cells.len() - 1)] = 0;
    let mut next = 1;
    let mut edge_id = vec![usize::MAX; nodes * nodes];
    let mut edges: Vec<Edge> = Vec::new();
    for (a, b) in borders {
        let [a, b] = [a, b].map(|cell| {
            let root = uf.find(cell);
            if node[root] == usize::MAX {
                node[root] = next;
                next += 1;
            }
            node[root]
        });
        let (a, b) = (a.min(b), a.max(b));
        let id = &mut edge_id[a * nodes + b];
        if *id == usize::MAX {
            *id = edges.len();
            edges.push(((a, b), 0));
        }
        edges[*id].1 += 1;
    }

    edges.sort_unstable_by_key(|&(_, len)| len);
    (nodes - 1, edges)
}

fn kruskal<'a>(uf: &'a mut UnionFind, edges: &'a [Edge], parts: usize) -> impl Iterator<Item = Edge> + 'a {
    edges.iter().copied().filter(|&((a, b), _)| uf.union(a, b)).take(parts - 1)
}

pub fn part2(input: &str) -> String {
    let (shapes, edges) = shape_graph(input);
    let mut uf = UnionFind::new(shapes + 1);
    kruskal(&mut uf, &edges, shapes + 1).map(|(_, len)| len).sum::<usize>().to_string()
}

pub fn part3(input: &str) -> String {
    let (shapes, edges) = shape_graph(input);

    // run the initial MST, the savings part only needs to reduce that
    let tree: Vec<Edge> = kruskal(&mut UnionFind::new(shapes + 1), &edges, shapes + 1).collect();
    let mut neighbours = vec![Vec::new(); shapes + 1];
    for &((a, b), _) in &edges {
        neighbours[a].push(b);
        neighbours[b].push(a);
    }

    // total length still to pay with `starters` joined to all their neighbours for free
    let mut uf = UnionFind::new(shapes + 1);
    let mut cost = |starters: &[usize]| {
        uf.reset();
        let mut parts = shapes + 1;
        for &s in starters {
            for &n in &neighbours[s] {
                parts -= uf.union(s, n) as usize;
            }
        }
        kruskal(&mut uf, &tree, parts).map(|(_, len)| len).sum::<usize>()
    };

    // Compute savings potential for each starter node
    let full: usize = tree.iter().map(|&(_, len)| len).sum();
    let mut by_saving: Vec<(usize, usize)> = (1..=shapes).map(|s| (full - cost(&[s]), s)).collect();
    by_saving.sort_unstable_by(|x, y| y.cmp(x));
    let mut best = usize::MAX;

    // Evaluate in order, stop if the remaining savings can't beat the best so far
    for (i, &(save_a, a)) in by_saving.iter().enumerate() {
        for &(save_b, b) in &by_saving[i + 1..] {
            if full.saturating_sub(save_a + save_b) >= best {
                break;
            }
            best = best.min(cost(&[a, b]));
        }
    }
    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
AAAEEEFFFF
AADDDDDGFF
BACDDIGGFF
BABBIIIGGH
BBBBIIIIHH";
        assert_eq!(part1(input), "67");
    }

    #[test]
    fn test_part2_small() {
        let input = "\
AAAEEEFFFF
AADDDDDGFF
BACDDIGGFF
BABBIIIGGH
BBBBIIIIHH";
        assert_eq!(part2(input), "12");
    }

        #[test]
    fn test_part2_big() {
        let input = "\
IIIIIILOHHHHHHHHGGGGGQSRRRRKKK
IIIIIILOOHHHHHHGGXGGGQSRRRRKKK
PPPPILLLOOOODDHBGGGGGQSSRRRKKK
PPZPLLLLFFOODDBBBBGGQQQSSRKKKK
PPPPLLLLFFFONDDDBBMGGQQSSKKJJJ
PPPCCCCCCFFONNDDMMMGQQQSSSKJJJ
PCCCCCCCCFFNNDDDDMAAQQQSSSKKJJ
PPCCECCCFFNNNNDADMAATQTTSSSJJJ
PPEEEEECCFFNNAAAMMATTTTTSTSSSJ
PPEEEEEEEFFNNNAAAAAATTTTTTSSJJ";
        assert_eq!(part2(input), "63");
    }

    #[test]
    fn test_part3() {
        let input = "\
IIIIIILOHHHHHHHHGGGGGQSRRRRKKK
IIIIIILOOHHHHHHGGXGGGQSRRRRKKK
PPPPILLLOOOODDHBGGGGGQSSRRRKKK
PPZPLLLLFFOODDBBBBGGQQQSSRKKKK
PPPPLLLLFFFONDDDBBMGGQQSSKKJJJ
PPPCCCCCCFFONNDDMMMGQQQSSSKJJJ
PCCCCCCCCFFNNDDDDMAAQQQSSSKKJJ
PPCCECCCFFNNNNDADMAATQTTSSSJJJ
PPEEEEECCFFNNAAAMMATTTTTSTSSSJ
PPEEEEEEEFFNNNAAAAAATTTTTTSSJJ";
        assert_eq!(part3(input), "21");
    }


}
