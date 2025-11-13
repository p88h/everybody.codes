use rayon::prelude::*;

fn similarity(a: &[u8], b: &[u8], c: &[u8]) -> u32 {
    let (mut A, mut B) = (0u32, 0u32);
    for i in 0..a.len() {
        if a[i] == c[i] {
            A += 1;
        }
        if b[i] == c[i] {
            B += 1;
        }
    }
    A * B
}

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let a = lines.next().unwrap()[2..].as_bytes();
    let b = lines.next().unwrap()[2..].as_bytes();
    let c = lines.next().unwrap()[2..].as_bytes();
    similarity(a, b, c).to_string()
}

fn maybe_child(a: &[u8], b: &[u8], t: usize) -> bool {
    let mut same = 0;
    for i in 0..a.len() {
        if a[i] == b[i] {
            same += 1;
        }
    }
    same >= t
}

fn is_child(a: &[u8], b: &[u8], c: &[u8]) -> bool {
    let mut l = 0;
    let mut r = 0;
    for i in 0..a.len() {
        if a[i] != c[i] && b[i] != c[i] {
            return false;
        }
        if a[i] == c[i] {
            l += 1;
        }
        if b[i] == c[i] {
            r += 1;
        }
    }
    true
}

fn find_family(dna: &Vec<&[u8]>, i: usize) -> [usize; 3] {
    let mut maybe_parents = vec![];
    for j in 0..dna.len() {
        if i != j && maybe_child(dna[j], dna[i], 60) {
            maybe_parents.push(j);
        }
    }
    for a in 0..maybe_parents.len() {
        for b in a + 1..maybe_parents.len() {
            let j = maybe_parents[a];
            let k = maybe_parents[b];
            if i != j && i != k {
                if is_child(dna[j], dna[k], dna[i]) {
                    return [j, k, i];
                }
            }
        }
    }
    [i, i, i]
}

pub fn part2(input: &str) -> String {
    let dna = input
        .lines()
        .map(|line| line.split(':').nth(1).unwrap().as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut total = 0;
    for i in 0..dna.len() {
        let f = find_family(&dna, i);
        if f[0] != i {
            let s = similarity(dna[f[0]], dna[f[1]], dna[i]);
            total += s;
        }
    }
    total.to_string()
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn max_set(&mut self) -> usize {
        let n = self.parent.len();
        let mut counts = vec![0usize; n];
        for i in 0..n {
            let root = self.find(i);
            counts[root] += 1;
        }
        counts
            .iter()
            .enumerate()
            .max_by_key(|(_, count)| *count)
            .map(|(id, _)| id)
            .unwrap_or(0)
    }
}

pub fn part3(input: &str) -> String {
    let dna = input
        .lines()
        .map(|line| line.split(':').nth(1).unwrap().as_bytes())
        .collect::<Vec<&[u8]>>();
    let mut total = 0;
    let mut uf = UnionFind::new(dna.len());

    let families: Vec<[usize; 3]> = (0..dna.len())
        .into_par_iter()
        .map(|i| find_family(&dna, i))
        .collect();

    for family in families {
        uf.union(family[0], family[2]);
        uf.union(family[1], family[2]);
    }
    let max_set_id = uf.max_set();
    for i in 0..dna.len() {
        if uf.connected(i, max_set_id) {
            total += i + 1;
        }
    }
    total.to_string()
}
