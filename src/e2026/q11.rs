use crate::algo::apsp::DistMatrix;
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::LazyLock;

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

// Each task takes only a few µs, so the global pool's 18 threads lose more to waking up and
// waiting on the efficiency cores than they gain; these sizes measured fastest on a 6P + 12E M-series.
static PART2_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPoolBuilder::new().num_threads(4).build().unwrap());
static PART3_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPoolBuilder::new().num_threads(8).build().unwrap());

pub fn part2(input: &str) -> String {
    let (adj, ring_distance) = parse(input, 3);
    let n = adj.len();
    let dist = DistMatrix::from_bitsets(&adj);
    let pairs: Vec<(usize, usize)> = (0..n).flat_map(|a| (a + 1..n).map(move |b| (a, b))).collect();
    let best = PART2_POOL.install(|| pairs.par_iter().with_min_len(16).map(|&edge| dist.total_with([edge])).min().unwrap());
    (ring_distance - best).to_string()
}

// all ways to pair up 8 slots into 4 disjoint pairs: 7 * 5 * 3 * 1
const PAIRINGS: [[(usize, usize); 4]; 105] = {
    const fn fill(out: &mut [[(usize, usize); 4]; 105], count: &mut usize, used: u8, mut pairs: [(usize, usize); 4], depth: usize) {
        if depth == 4 {
            out[*count] = pairs;
            *count += 1;
            return;
        }
        // always pair the lowest free slot, so each matching is generated once
        let mut first = 0;
        while used >> first & 1 == 1 {
            first += 1;
        }
        let mut partner = first + 1;
        while partner < 8 {
            if used >> partner & 1 == 0 {
                pairs[depth] = (first, partner);
                fill(out, count, used | 1 << first | 1 << partner, pairs, depth + 1);
            }
            partner += 1;
        }
    }
    let mut out = [[(0, 0); 4]; 105];
    let mut count = 0;
    fill(&mut out, &mut count, 0, [(0, 0); 4], 0);
    assert!(count == 105);
    out
};

// same as part 2, but adds 4 edges that pair up 8 evenly spaced portals
pub fn part3(input: &str) -> String {
    let (adj, ring_distance) = parse(input, 3);
    let n = adj.len();
    let step = n / 8;
    let dist = DistMatrix::from_bitsets(&adj);
    // consecutive pairings share their first two pairs, so add those to the matrix once per three
    let groups = PAIRINGS.len() / 3;
    let best = PART3_POOL.install(|| {
        (0..step * groups)
            .into_par_iter()
            .with_min_len(2)
            .map_init(
                || dist.clone(),
                |work, i| {
                    let start = i / groups;
                    let edge = |(a, b): (usize, usize)| (start + a * step, start + b * step);
                    let group = &PAIRINGS[i % groups * 3..][..3];
                    work.clone_from(&dist);
                    for (a, b) in [edge(group[0][0]), edge(group[0][1])] {
                        work.add_edge(a, b);
                    }
                    group.iter().map(|pairs| work.total_with([edge(pairs[2]), edge(pairs[3])])).min().unwrap()
                },
            )
            .min()
            .unwrap()
    });
    (ring_distance - best).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairings_are_distinct_perfect_matchings() {
        let mut seen = std::collections::HashSet::new();
        for pairs in PAIRINGS {
            let slots = pairs.iter().fold(0u8, |mask, &(a, b)| mask | 1 << a | 1 << b);
            assert_eq!(slots, 255);
            assert!(pairs.iter().all(|&(a, b)| a < b));
            assert!(seen.insert(pairs));
        }
    }

    #[test]
    fn test_pairings_come_in_threes_sharing_two_pairs() {
        for group in PAIRINGS.chunks(3) {
            assert!(group.iter().all(|pairs| pairs[..2] == group[0][..2]));
        }
    }

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
