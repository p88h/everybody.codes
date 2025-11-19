use std::collections::{HashSet};

fn bfs(grid: &Vec<Vec<u8>>, skip: &Vec<bool>, start: &Vec<(usize, usize)>) -> (Vec<bool>, usize) {
    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut cur = start.clone();
    let mut visited = skip.clone();
    for (x, y) in cur.iter() {
        visited[y * 256 + x] = true;
    }
    let mut cnt = 0;
    while cur.len() > 0 {
        let mut next = vec![];
        cnt += cur.len();
        for &(r, c) in cur.iter() {
            for &(dr, dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                    let nidx = (nc * 256 + nr) as usize;
                    if grid[nr as usize][nc as usize] <= grid[r][c] && !visited[nidx] {
                        visited[nidx] = true;
                        next.push((nr as usize, nc as usize));
                    }
                }
            }
        }
        cur = next;
    }
    (visited, cnt)
}

pub fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let visited = bfs(&grid, &vec![false; 256 * 256], &vec![(0, 0)]);
    visited.1.to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let visited = bfs(
        &grid,
        &vec![false; 256 * 256],
        &vec![(0, 0), (grid.len() - 1, grid[0].len() - 1)],
    );
    visited.1.to_string()
}

struct ClassNode {
    size: usize,
    children: Vec<usize>,
    has_parent: bool,
}

fn compute_class_graph(
    grid: &Vec<Vec<u8>>,
    start: &(usize, usize),
    cache: &mut Vec<Option<usize>>,
    graph: &mut Vec<ClassNode>,
) -> usize {
    if let Some(class) = cache[start.0 * 256 + start.1] {
        return class;
    }
    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut cur = vec![*start];
    let mut child_set: HashSet<usize> = HashSet::new();
    let class = graph.len();
    graph.push(ClassNode {
        size: 0,
        children: vec![],
        has_parent: false,
    });
    cache[start.0 * 256 + start.1] = Some(class);
    let mut cnt = 0;
    // compute the equivalence area
    while cur.len() > 0 {
        let mut next = vec![];
        for &(r, c) in cur.iter() {
            cnt += 1;
            for &(dr, dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                    let npos = (nr as usize, nc as usize);
                    // extend equivalence area
                    if grid[nr as usize][nc as usize] == grid[r][c] {
                        // add to queue unless already visited
                        if cache[npos.0 * 256 + npos.1].is_none() {
                            next.push(npos);
                            cache[npos.0 * 256 + npos.1] = Some(class);
                        }
                    } else if grid[nr as usize][nc as usize] < grid[r][c] {
                        // lower adjacent area - record its class
                        let child_class = compute_class_graph(grid, &npos, cache, graph);
                        graph[child_class].has_parent = true;
                        child_set.insert(child_class);
                    }
                }
            }
        }
        cur = next;
    }
    graph[class].children = child_set.into_iter().collect::<Vec<usize>>();
    graph[class].size = cnt;
    class
}

fn fast_bfs(start: usize, graph: &mut Vec<ClassNode>, explode: bool) -> usize {
    let mut cur = vec![start];
    let mut visited = vec![false; graph.len()];
    visited[start] = true;
    let mut cnt = 0;
    while cur.len() > 0 {
        let mut next = vec![];
        for &node in cur.iter() {
            cnt += graph[node].size;
            for &child in graph[node].children.iter() {
                if !visited[child] {
                    visited[child] = true;
                    next.push(child);
                }
            }
            if explode {
                graph[node].children = vec![];
                graph[node].size = 0;                
            }
        }
        cur = next;
    }
    cnt
}

pub fn part3(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let all_points = (0..grid.len())
        .flat_map(|r| (0..grid[0].len()).map(move |c| (r, c)))
        .collect::<Vec<(usize, usize)>>();
    let mut cache: Vec<Option<usize>> = vec![None; 256 * 256];
    let mut graph: Vec<ClassNode> = vec![];
    all_points.iter().for_each(|pos| {
        compute_class_graph(&grid, pos, &mut cache, &mut graph);
    });
    // select roots only for processing
    let roots = graph
        .iter()
        .enumerate()
        .filter(|(_, node)| !node.has_parent)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    let mut total = 0;
    for _ in 0..3 {
        let best = roots
            .iter()
            .map(|&root| (root, fast_bfs(root, &mut graph, false)))
            .max_by_key(|(_, cnt)| *cnt)
            .unwrap();
        total += best.1;
        // explode the best class
        fast_bfs(best.0, &mut graph, true);
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part3() {
        let input = "\
41951111131882511179
32112222211518122215
31223333322115122219
31234444432147511128
91223333322176121892
61112222211166431583
14661111166111111746
11111119142122222177
41222118881233333219
71222127839122222196
56111126279711111517";
        assert_eq!(part3(input), "136");
    }
}
