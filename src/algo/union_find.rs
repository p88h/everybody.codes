pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind { parent: (0..size).collect(), rank: vec![0; size] }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
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

    pub fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn max_set(&mut self) -> usize {
        let n = self.parent.len();
        let mut counts = vec![0usize; n];
        for i in 0..n {
            let root = self.find(i);
            counts[root] += 1;
        }
        counts.iter().enumerate().max_by_key(|(_, count)| *count).map(|(id, _)| id).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let uf = UnionFind::new(5);
        assert_eq!(uf.parent.len(), 5);
        assert_eq!(uf.rank.len(), 5);
    }

    #[test]
    fn test_find_self() {
        let mut uf = UnionFind::new(5);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(3), 3);
    }

    #[test]
    fn test_union_simple() {
        let mut uf = UnionFind::new(5);
        assert!(uf.union(0, 1));
        assert!(uf.connected(0, 1));
    }

    #[test]
    fn test_union_already_connected() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        assert!(!uf.union(0, 1));
    }

    #[test]
    fn test_connected_transitivity() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        assert!(uf.connected(0, 2));
    }

    #[test]
    fn test_not_connected() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        assert!(!uf.connected(0, 2));
    }

    #[test]
    fn test_max_set_single() {
        let mut uf = UnionFind::new(5);
        let max_root = uf.max_set();
        assert!(max_root < 5);
    }

    #[test]
    fn test_max_set_with_unions() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(0, 2);
        uf.union(3, 4);
        let max_root = uf.max_set();
        assert!(uf.connected(0, max_root) || uf.connected(3, max_root));
    }

    #[test]
    fn test_multiple_components() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);
        assert!(uf.connected(0, 1));
        assert!(uf.connected(2, 3));
        assert!(uf.connected(4, 5));
        assert!(!uf.connected(0, 2));
        assert!(!uf.connected(2, 4));
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);
        let root = uf.find(3);
        assert_eq!(uf.find(0), root);
    }
}
