use std::collections::HashMap;

fn visit_tree(
    tree_map: &HashMap<String, Vec<String>>,
    node: &String,
    path: &mut Vec<String>,
    visitor: &mut dyn FnMut(&Vec<String>),
) {
    path.push(node.clone());
    visitor(path);
    if let Some(children) = tree_map.get(node) {
        for child in children {
            visit_tree(tree_map, child, path, visitor);
        }
    }
    path.pop();
}

fn build_tree_map(input: &str) -> HashMap<String, Vec<String>> {
    let tree_defs = input
        .lines()
        .map(|line| {
            let (label, links) = line.split_once(':').unwrap();
            let children = links.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
            (label.to_string(), children)
        })
        .collect::<Vec<(String, Vec<String>)>>();
    let mut label_counts = tree_defs.iter().cloned().fold(HashMap::new(), |mut acc, (_, children)| {
        for child in children {
            *acc.entry(child).or_insert(0) += 1;
        }
        acc
    });
    *label_counts.entry("RR".to_string()).or_insert(0) += 1; // root node
    tree_defs
        .into_iter()
        .filter(|(label, _)| label_counts[label] == 1)
        .collect::<HashMap<String, Vec<String>>>()
}

fn solve(input: &str, max_len: usize) -> String {
    let tree_map = build_tree_map(input);
    let mut fruit_counts = HashMap::new();
    visit_tree(&tree_map, &"RR".to_string(), &mut Vec::new(), &mut |path| {
        if *path.last().unwrap() == "@" {
            *fruit_counts.entry(path.len()).or_insert(0) += 1;
        }
    });
    let mut result = String::new();
    visit_tree(&tree_map, &"RR".to_string(), &mut Vec::new(), &mut |path| {
        if *path.last().unwrap() == "@" && fruit_counts[&path.len()] == 1 {
            result = path.iter().map(|s| s.chars().take(max_len).collect::<String>()).collect::<String>();
        }
    });
    result
}

pub fn part1(input: &str) -> String {
    solve(input, 999)
}

pub fn part2(input: &str) -> String {
    solve(input, 1)
}

pub fn part3(input: &str) -> String {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@";
        assert_eq!(part1(input), "RRB@");
    }


}
