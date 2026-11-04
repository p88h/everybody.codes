use rayon::prelude::*;

// Starts at the node with the most connections; the limit is high enough that nothing explodes.
fn solve_single(input: &str, strength: usize) -> String {
    let (energies, graph) = parse(input);
    let mut start = 0;
    for (i, conns) in graph.iter().enumerate() {
        if conns.len() > graph[start].len() {
            start = i;
        }
    }
    simulate(start, strength, &energies, &graph, i64::MAX).to_string()
}

pub fn part1(input: &str) -> String {
    solve_single(input, 5)
}

pub fn part2(input: &str) -> String {
    solve_single(input, 10)
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<usize>>) {
    let cnt = input.lines().count() + 1;
    let mut energies = vec![0; cnt];
    let mut graph = vec![vec![]; cnt];
    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(3, ", ").collect();
        let id = parts[0].split_once('=').unwrap().1.parse::<usize>().unwrap();
        energies[id] = parts[1].split_once('=').unwrap().1.parse::<i64>().unwrap();
        for conn in parts[2].split_once('=').unwrap().1.split(',') {
            graph[id].push(conn.trim().parse::<usize>().unwrap());
        }
    }
    (energies, graph)
}

// Simulates a pulse started at `start`; returns the total energy of nodes that did not explode.
fn simulate(start: usize, strength: usize, base: &[i64], graph: &[Vec<usize>], limit: i64) -> i64 {
    let cnt = base.len();
    let mut energies = base.to_vec();
    let mut exploded = vec![false; cnt];
    // pulses[x][n] = number of pulses of strength x currently arriving at node n
    let mut pulses = vec![vec![0i64; cnt]; strength + 1];
    let mut pulse_cnt = vec![0i64; strength + 1];
    pulses[strength][start] = 1;
    pulse_cnt[strength] = 1;
    while pulse_cnt.iter().any(|&c| c > 0) {
        for x in 1..=strength {
            if pulse_cnt[x] == 0 {
                continue;
            }
            for n in 0..cnt {
                let p = pulses[x][n];
                if p == 0 {
                    continue;
                }
                energies[n] += p * x as i64;
                if x > 1 {
                    for &c in &graph[n] {
                        if !exploded[c] {
                            pulses[x - 1][c] += p;
                            pulse_cnt[x - 1] += p;
                        }
                    }
                }
            }
            pulses[x].fill(0);
            pulse_cnt[x] = 0;
        }
        // nodes over the limit explode after the whole step has been accumulated
        for n in 0..cnt {
            if exploded[n] || energies[n] < limit {
                continue;
            }
            exploded[n] = true;
            // drop everything queued for this node, including explosion pulses from earlier nodes
            for x in 1..=strength {
                pulse_cnt[x] -= pulses[x][n];
                pulses[x][n] = 0;
            }
            for &c in &graph[n] {
                if !exploded[c] {
                    pulses[strength][c] += 1;
                    pulse_cnt[strength] += 1;
                }
            }
        }
    }
    (0..cnt).filter(|&n| !exploded[n]).map(|n| energies[n]).sum()
}

fn solve3(input: &str, limit: i64) -> i64 {
    let (energies, graph) = parse(input);
    (1..energies.len())
        .into_par_iter()
        .map(|start| simulate(start, 10, &energies, &graph, limit))
        .max()
        .unwrap()
}

pub fn part3(input: &str) -> String {
    solve3(input, 1_000_000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5";
        assert_eq!(part1(input), "102");
    }

    #[test]
    fn test_part2() {
        let input = "\
id=1, energy=5, connections=2,3,5,7
id=2, energy=9, connections=1,3,4,5,6,7
id=3, energy=3, connections=1,2,4,6,7
id=4, energy=8, connections=1,2,7
id=5, energy=1, connections=2,3,4,6
id=6, energy=7, connections=1,2,3,4
id=7, energy=6, connections=1,2,4,6";
        assert_eq!(part2(input), "1214633");
    }

    #[test]
    fn test_part3() {
        let input = "\
id=1, energy=5, connections=2,5,7
id=2, energy=9, connections=6,7
id=3, energy=3, connections=4
id=4, energy=8, connections=3
id=5, energy=1, connections=6,7
id=6, energy=7, connections=1,7
id=7, energy=6, connections=5";
        assert_eq!(solve3(input, 100).to_string(), "187");
    }
}
