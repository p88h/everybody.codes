fn parse_graph<'a>(input: &'a str, invert_edges: bool) -> std::collections::HashMap<&'a str, Vec<&'a str>> {
    let mut graph = std::collections::HashMap::new();
    for line in input.lines() {
        let (key, values) = line.split_once(':').unwrap();
        for value in values.split(',') {
            let (from, to) = if invert_edges { (value, key) } else { (key, value) };
            graph.entry(from).or_insert_with(Vec::new).push(to);
        }
    }
    graph
}

fn bfs<'a>(
    graph: &std::collections::HashMap<&'a str, Vec<&'a str>>,
    start: &'a str,
) -> std::collections::HashSet<&'a str> {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(node) {
            queue.extend(neighbors.iter().filter(|&&next| visited.insert(next)));
        }
    }
    visited
}

pub fn part1(input: &str) -> String {
    let graph = parse_graph(input, false);
    let visited = bfs(&graph, "NULL");
    (visited.len() - 1).to_string()
}

pub fn part2(input: &str) -> String {
    let graph = parse_graph(input, true);
    let visited = bfs(&graph, "NULL");
    (visited.len() - 1).to_string()
}

pub fn part3(input: &str) -> String {
    let graph = parse_graph(input, true);

    let mut reachable = std::collections::HashMap::new();
    for start in ["NULLv1", "NULLv2", "NULLv3"] {
        for node in bfs(&graph, start) {
            *reachable.entry(node).or_insert(0) += 1;
        }
    }
    reachable.values().filter(|&&count| count > 1).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS";
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn test_part2() {
        let input = "\
BRDS:FLWR,SLTT,YSTS
FLWR:SLTT,YSTS
SLTT:YSTS,MLKK,BTTR
NULL:BRDS,FLWR,SLTT
YSTS:BRDS,FLWR
MLKK:SPCS,NULL
HNYN:BRDS,FLWR,SLTT,YSTS";
        assert_eq!(part2(input), "6");
    }

    #[test]
    fn test_part3() {
        let input = "\
MEAL:BRDS,CAKE,DOUG
BRDS:DOUG,FLWR
CAKE:DOUG,FLWR,NULLv2,NULLv3
DOUG:FLWR,YSTS,SLTT,NULLv1
FLWR:BRDS,DOUG
YSTS:SPCS
SOUP:BRDS,CAKE,DOUG,FLWR";
        assert_eq!(part3(input), "3");
    }
}
