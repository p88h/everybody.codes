use rayon::prelude::*;

// adjacency is represented as a bitmask in a u128 for each node
fn add_edge(adj: &mut [u128], a: usize, b: usize) {
    adj[a] |= 1 << b;
    adj[b] |= 1 << a;
}

// each node is linked to the next `reach` nodes around the ring
fn ring(n: usize, reach: usize) -> Vec<u128> {
    let mut adj = vec![0; n];
    for i in 0..n {
        for d in 1..=reach {
            let j = (i + d) % n;
            add_edge(&mut adj, i, j);
        }
    }
    adj
}

// sum of shortest distances from `start` to all nodes using BFS
fn bitbfs(adj: &[u128], start: usize) -> usize {
    let mut seen = 1u128 << start;
    let mut frontier = seen;
    let mut total = 0;
    let mut depth = 0;
    while frontier != 0 {
        depth += 1;
        let mut next = 0;
        while frontier != 0 {
            next |= adj[frontier.trailing_zeros() as usize];
            frontier &= frontier - 1;
        }
        next &= !seen;
        seen |= next;
        total += depth * next.count_ones() as usize;
        frontier = next;
    }
    total
}

// ring of nodes plus the listed edges
fn parse(input: &str, reach: usize) -> (Vec<u128>, usize) {
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().trim_start_matches("portals:").trim().parse().unwrap();
    let mut adj = ring(n, reach);
    let ring_distance = n * bitbfs(&ring(n, reach), 0);
    for line in lines {
        if let Some((a, b)) = line.split_once(' ') {
            add_edge(&mut adj, a.parse::<usize>().unwrap() - 1, b.parse::<usize>().unwrap() - 1);
        }
    }
    (adj, ring_distance)
}

// sum of shortest distances over all ordered pairs of nodes
fn total_distance(adj: &[u128]) -> usize {
    (0..adj.len()).map(|start| bitbfs(adj, start)).sum()
}

// how much the extra edges shorten the total distance over all pairs compared to the plain ring
pub fn part1(input: &str) -> String {
    let (adj, ring_distance) = parse(input, 1);
    (ring_distance - total_distance(&adj)).to_string()
}

pub fn part2(input: &str) -> String {
    let (adj, ring_distance) = parse(input, 3);
    let n = adj.len();
    let best = (0..n)
        .into_par_iter()
        .map_init(
            || adj.clone(),
            |work, a| {
                (a + 1..n)
                    .map(|b| {
                        work.copy_from_slice(&adj);
                        add_edge(work, a, b);
                        total_distance(work)
                    })
                    .min()
                    .unwrap_or(usize::MAX)
            },
        )
        .min()
        .unwrap()
        .min(total_distance(&adj));
    (ring_distance - best).to_string()
}

fn pairings(slots: &[usize], pairs: &mut Vec<(usize, usize)>, out: &mut Vec<Vec<(usize, usize)>>) {
    let Some((&first, rest)) = slots.split_first() else {
        out.push(pairs.clone());
        return;
    };
    for (i, &partner) in rest.iter().enumerate() {
        let mut remaining = rest.to_vec();
        remaining.remove(i);
        pairs.push((first, partner));
        pairings(&remaining, pairs, out);
        pairs.pop();
    }
}

// same as part 2, but adds 4 edges that pair up 8 evenly spaced portals
pub fn part3(input: &str) -> String {
    let (adj, ring_distance) = parse(input, 3);
    let n = adj.len();
    let step = n / 8;
    let mut offsets = Vec::new();
    pairings(&[0, 1, 2, 3, 4, 5, 6, 7], &mut Vec::new(), &mut offsets);
    // each candidate is only a few microseconds, so batch them to keep rayon's scheduling overhead down
    let best = (0..step * offsets.len())
        .into_par_iter()
        .map_init(
            || adj.clone(),
            |work, i| {
                let start = i / offsets.len();
                work.copy_from_slice(&adj);
                for &(a, b) in &offsets[i % offsets.len()] {
                    add_edge(work, start + a * step, start + b * step);
                }
                total_distance(work)
            },
        )
        .min()
        .unwrap();
    (ring_distance - best).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
portals: 10

5 10
2 7";
        assert_eq!(part1(input), "60");
    }

        #[test]
    fn test_part2() {
        let input = "\
portals: 10

5 10
2 7";
        assert_eq!(part2(input), "6");
    }

    #[test]
    fn test_part3_32() {
        let input = "\
portals: 32

1 10
20 30
15 29
13 10";
        assert_eq!(part3(input), "854");
    }

        #[test]
    fn test_part3_64() {
        let input = "\
portals: 64

1 40
3 35
5 42
7 50";
        assert_eq!(part3(input), "9634");
    }

}
