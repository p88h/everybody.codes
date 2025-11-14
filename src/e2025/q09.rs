fn encode(line: &str) -> [u128; 4] {
    let b = line.split(':').nth(1).unwrap().as_bytes();
    let mut v = [0u128; 4];
    for i in 0..128 {
        let bc = match b[i] {
            b'A' => 1,
            b'C' => 2,
            b'G' => 4,
            b'T' => 8,
            _ => 0,
        };
        v[i / 32] |= bc << ((i % 32) * 4);
    }
    v
}

fn is_child(a: &[u128; 4], b: &[u128; 4], c: &[u128; 4]) -> bool {
    a.iter()
        .zip(b.iter())
        .zip(c.iter())
        .all(|((ai, bi), ci)| ((ai | bi) & ci) ^ ci == 0)
}

fn pair_score(a: &[u128; 4], b: &[u128; 4]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| (ai & bi).count_ones())
        .sum::<u32>()
}

fn similarity(a: &[u128; 4], b: &[u128; 4], c: &[u128; 4]) -> u32 {
    pair_score(a, c) * pair_score(b, c)
}

pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let a = encode(lines.next().unwrap());
    let b = encode(lines.next().unwrap());
    let c = encode(lines.next().unwrap());
    if is_child(&a, &b, &c) {
        similarity(&a, &b, &c).to_string()
    } else if is_child(&c, &b, &a) {
        similarity(&c, &b, &a).to_string()
    } else if is_child(&a, &c, &b) {
        similarity(&a, &c, &b).to_string()
    } else {
        "0".to_string()
    }
}

fn find_family(dna: &Vec<[u128; 4]>, i: usize) -> [usize; 3] {
    let mut maybe_parents = vec![];
    for j in 0..dna.len() {
        if i != j && pair_score(&dna[j], &dna[i]) > 60 {
            maybe_parents.push(j);
        }
    }
    for a in 0..maybe_parents.len() {
        for b in a + 1..maybe_parents.len() {
            let j = maybe_parents[a];
            let k = maybe_parents[b];
            if i != j && i != k {
                if is_child(&dna[j], &dna[k], &dna[i]) {
                    return [j, k, i];
                }
            }
        }
    }
    [i, i, i]
}

pub fn part2(input: &str) -> String {
    let dna = input.lines().map(encode).collect::<Vec<[u128; 4]>>();
    let mut total = 0;
    for i in 0..dna.len() {
        let f = find_family(&dna, i);
        if f[0] != i {
            let s = similarity(&dna[f[0]], &dna[f[1]], &dna[i]);
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
    let dna = input.lines().map(encode).collect::<Vec<[u128; 4]>>();
    let mut total = 0;
    let mut uf = UnionFind::new(dna.len());

    let families: Vec<[usize; 3]> = (0..dna.len()).map(|i| find_family(&dna, i)).collect();

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
