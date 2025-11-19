struct Node {
    value: u64,
    label: String,
    left: usize,
    right: usize,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    fn insert(&mut self, index: usize, into: usize) {
        if self.nodes[index].value < self.nodes[into].value {
            if self.nodes[into].left == 0 {
                self.nodes[into].left = index;
            } else {
                self.insert(index, self.nodes[into].left);
            }
        } else {
            if self.nodes[into].right == 0 {
                self.nodes[into].right = index;
            } else {
                self.insert(index, self.nodes[into].right);
            }
        }
    }

    fn add_node(&mut self, value: u64, label: String) {
        let index = self.nodes.len();
        self.nodes.push(Node { value, label, left: 0, right: 0 });
        if index > 1 {
            self.insert(index, index % 2);
        }
    }

    fn count_levels(&self, index: usize, level: usize, counts: &mut Vec<usize>) {
        let node = &self.nodes[index];
        counts[level] += 1;
        if node.left != 0 {
            self.count_levels(node.left, level + 1, counts);
        }
        if node.right != 0 {
            self.count_levels(node.right, level + 1, counts);
        }
    }

    fn largest_level(&self, subtree: usize) -> usize {
        let mut level_count = vec![0; self.nodes.len()];
        self.count_levels(subtree, 0, &mut level_count);
        level_count.iter().enumerate().rev().max_by_key(|&(_, count)| count).unwrap().0
    }

    fn visit_level(&self, index: usize, current: usize, level: usize, labels: &mut Vec<String>) {
        let node = &self.nodes[index];
        if current == level {
            labels.push(node.label.clone());
            return;
        }
        if node.left != 0 {
            self.visit_level(node.left, current + 1, level, labels);
        }
        if node.right != 0 {
            self.visit_level(node.right, current + 1, level, labels);
        }
    }

    fn get_level_labels(&self, level: usize, subtree: usize) -> String {
        let mut labels = Vec::new();
        self.visit_level(subtree, 0, level, &mut labels);
        labels.join("")
    }

    fn _display_tree(&self, subtree: usize, indent: usize) {
        let node = &self.nodes[subtree];
        for _ in 0..indent {
            print!("  ");
        }
        println!("{}-{}", node.value, node.label);
        if node.left != 0 {
            self._display_tree(node.left, indent + 1);
        }
        if node.right != 0 {
            self._display_tree(node.right, indent + 1);
        }
    }
}

fn parse_line(line: &str, tree: &mut Tree, super_swap: bool) {
    let mut parts = line.split(' ');
    let cmd = parts.next().unwrap();
    if cmd == "ADD" {
        let _id = parts.next().unwrap()[3..].parse::<u64>().unwrap();
        let ls = parts.next().unwrap();
        let mut left = ls[6..ls.len() - 1].split(',');
        let rs = parts.next().unwrap();
        let mut right = rs[7..rs.len() - 1].split(',');
        let lv = left.next().unwrap().parse::<u64>().unwrap();
        let rv = right.next().unwrap().parse::<u64>().unwrap();
        tree.add_node(lv, left.next().unwrap().to_string());
        tree.add_node(rv, right.next().unwrap().to_string());
    }
    if cmd == "SWAP" {
        let idx = parts.next().unwrap().parse::<usize>().unwrap() * 2 - 1;
        let (tmp_value, tmp_label) = (tree.nodes[idx - 1].value, tree.nodes[idx - 1].label.clone());
        let (tmp_left, tmp_right) = (tree.nodes[idx - 1].left, tree.nodes[idx - 1].right);
        tree.nodes[idx - 1].value = tree.nodes[idx].value;
        tree.nodes[idx - 1].label = tree.nodes[idx].label.clone();
        if super_swap {
            tree.nodes[idx - 1].left = tree.nodes[idx].left;
            tree.nodes[idx - 1].right = tree.nodes[idx].right;
            tree.nodes[idx].left = tmp_left;
            tree.nodes[idx].right = tmp_right;
        }
        tree.nodes[idx].value = tmp_value;
        tree.nodes[idx].label = tmp_label;
    }
}

fn run_task(input: &str, super_swap: bool) -> String {
    let mut tree = Tree::new();
    for line in input.lines() {
        parse_line(line, &mut tree, super_swap);
    }
    let level1 = tree.largest_level(0);
    let level2 = tree.largest_level(1);
    let labels1 = tree.get_level_labels(level1, 0);
    let labels2 = tree.get_level_labels(level2, 1);
    format!("{labels1}{labels2}")
}

pub fn part1(input: &str) -> String {
    run_task(input, false)
}

pub fn part2(input: &str) -> String {
    run_task(input, false)
}

pub fn part3(input: &str) -> String {
    run_task(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
        assert_eq!(part1(input), "CFGNLK");
    }

    #[test]
    fn test_part2() {
        let input = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]";
        assert_eq!(part2(input), "MGFLNK");
    }

    #[test]
    fn test_part3() {
        let input = "ADD id=1 left=[10,A] right=[30,H]
ADD id=2 left=[15,D] right=[25,I]
ADD id=3 left=[12,F] right=[31,J]
ADD id=4 left=[5,B] right=[27,L]
ADD id=5 left=[3,C] right=[28,M]
SWAP 1
SWAP 5
ADD id=6 left=[20,G] right=[32,K]
ADD id=7 left=[4,E] right=[21,N]
SWAP 2
SWAP 5";
        assert_eq!(part3(input), "DJCGL");
    }
}
