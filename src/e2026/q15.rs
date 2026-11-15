use crate::algo::bellman_ford::bellman_ford;
use std::collections::VecDeque;

struct Arc {
    from: usize,
    to: usize,
    below: bool,
    len: usize,
    weight: usize,
}

/// Walks every corridor hanging off the spindle and returns the arcs plus the S..E distance.
fn parse(input: &str) -> (Vec<Arc>, usize) {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let row = grid.iter().position(|line| line.contains(&b'S')).unwrap();
    let start = grid[row].iter().position(|&c| c == b'S').unwrap();
    let end = grid[row].iter().position(|&c| c == b'E').unwrap();
    let mut arcs = Vec::new();
    // right feet of arcs already walked from their left foot, [above, below]
    let mut done = vec![[false; 2]; end];
    for x in start + 1..end {
        for up in [-1, 1] {
            let (mut pos, mut dir) = ((row as isize, x as isize), (up, 0));
            if grid[(row as isize + up) as usize][x] == b'#' || done[x][(up == 1) as usize] {
                continue;
            }
            let (mut len, mut weight) = (0, 0);
            loop {
                if grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize] == b'#' {
                    dir = if dir.1 == 0 { (0, 1) } else { (-up, 0) };
                }
                pos = (pos.0 + dir.0, pos.1 + dir.1);
                len += 1;
                if pos.0 == row as isize {
                    break;
                }
                weight += (grid[pos.0 as usize][pos.1 as usize] == b'@') as usize;
            }
            let to = pos.1 as usize;
            arcs.push(Arc { from: x - start, to: to - start, below: up == 1, len, weight });
            done[to][(up == 1) as usize] = true;
        }
    }
    (arcs, end - start)
}

// Group non-empty arcs, then solve each recursively.
pub fn part1(input: &str) -> String {
    let (arcs, width) = parse(input);
    // parse yields arcs ordered by `from`
    let arcs: Vec<&Arc> = arcs.iter().filter(|a| a.weight > 0).collect();
    let mut total = width;
    let mut i = 0;
    while i < arcs.len() {
        let (start, end) = (arcs[i].from, arcs[i].to);
        let j = i + arcs[i..].partition_point(|a| a.from < end);
        total += tour(&arcs[i..j], 0, start, end) - (end - start);
        i = j;
    }
    total.to_string()
}

fn tour(group: &[&Arc], taken: usize, start: usize, end: usize) -> usize {
    if taken == (1 << group.len()) - 1 {
        return end - start;
    }
    let mut best = usize::MAX;
    for (i, a) in group.iter().enumerate() {
        if taken & 1 << i != 0 {
            continue;
        }
        for (on, off) in [(a.from, a.to), (a.to, a.from)] {
            let cost = start.abs_diff(on) + a.len + tour(group, taken | 1 << i, off, end);
            best = best.min(cost);
        }
    }
    best
}

pub fn part2(input: &str) -> String {
    laps(&graph(input), 2026, false).to_string()
}

/// Nodes are arcs taken one way or the other, plus gates; 
/// `moves[node]` lists (next node, steps, weight).
struct Graph {
    moves: Vec<Vec<(usize, usize, usize)>>,
    start: usize,
    end: usize,
    /// S, the gates between groups, and E; moves out of the nodes from one gate up to the next only
    /// lead as far as that next gate
    gates: Vec<usize>,
}

/// Build a semi-dense graph of moves between arcs and gates.
fn graph(input: &str) -> Graph {
    let (arcs, width) = parse(input);
    let mut nodes = vec![(0, 0, 2, 0, 0)];
    let mut moves = Vec::new();
    let mut gates = vec![0];
    let mut i = 0;
    while i < arcs.len() {
        let end = arcs[i].to;
        let j = i + arcs[i..].partition_point(|a| a.from < end);
        let first = nodes.len();
        for a in &arcs[i..j] {
            nodes.push((a.from, a.to, a.below as usize, a.len, a.weight));
            nodes.push((a.to, a.from, a.below as usize, a.len, a.weight));
        }
        // the gate after the last group is E
        let gate = if j == arcs.len() { width } else { end };
        nodes.push((gate, gate, 2, 0, 0));
        gates.push(nodes.len() - 1);

        // the gate before this group and the group's arcs lead to the group's arcs and the gate
        for &(_, at, side, _, _) in &nodes[first - 1..nodes.len() - 1] {
            let mut next = Vec::new();
            for (k, &(on, _, next_side, steps, weight)) in nodes.iter().enumerate().skip(first) {
                // don't turn back
                if on != at || next_side != side {
                    next.push((k, at.abs_diff(on) + steps, weight));
                }
            }
            moves.push(next);
        }
        i = j;
    }
    moves.push(Vec::new());
    Graph { moves, start: 0, end: nodes.len() - 1, gates }
}

pub fn part3(input: &str) -> String {
    laps(&graph(input), 202620262026, true).to_string()
}

fn laps(g: &Graph, target: u64, exact: bool) -> u64 {
    let n = g.moves.len();
    let ((l, k), pot) = densest_loop(g);
    // with the potentials folded in, every move costs >= 0 and moves on the densest loops 0
    let cost = |s: usize, &(t, len, weight): &(usize, usize, usize)| {
        pot[s] + len as i64 * k - weight as i64 * l - pot[t]
    };

    // nodes on a densest loop: back to themselves along zero-cost moves
    let on_loop: Vec<bool> = (0..n)
        .map(|v| {
            let (mut seen, mut stack) = (vec![false; n], vec![v]);
            while let Some(u) = stack.pop() {
                for m in g.moves[u].iter().filter(|m| cost(u, m) == 0) {
                    if !seen[m.0] {
                        seen[m.0] = true;
                        stack.push(m.0);
                    }
                }
            }
            seen[v]
        })
        .collect();

    // cheapest costs over (node, weight mod k, loop touched yet).
    let period = k as usize;
    let id = |s: usize, r: usize, touched: bool| (s * period + r) * 2 + touched as usize;
    let mut dist = vec![i64::MAX; n * period * 2];
    let mut queued = vec![false; n * period * 2];
    let first = id(g.start, 0, on_loop[g.start]);
    dist[first] = 0;
    let mut queue = VecDeque::from([first]);
    while let Some(x) = queue.pop_front() {
        queued[x] = false;
        let (s, r, touched) = (x / 2 / period, x / 2 % period, x & 1 == 1);
        for m in &g.moves[s] {
            let y = id(m.0, (r + m.2) % period, touched || on_loop[m.0]);
            let d = dist[x] + cost(s, m);
            if d < dist[y] {
                dist[y] = d;
                if !queued[y] {
                    queued[y] = true;
                    queue.push_back(y);
                }
            }
        }
    }

    // the least weight >= target in each residue class, or just target itself
    let period = period as u64;
    let mut best = u64::MAX;
    for r in 0..period {
        let d = dist[id(g.end, r as usize, true)];
        let weight = target + (r + period - target % period) % period;
        if d == i64::MAX || exact && weight != target {
            continue;
        }
        // undo the potentials to get the walk's own cost, then add the laps
        let d = d - pot[g.start] + pot[g.end];
        best = best.min(((d + weight as i64 * l) / k) as u64);
    }
    best
}

/// The loop with the most weight per step as (steps, weight), plus potentials under which every move
/// costs `steps * loop_weight - weight * loop_steps` >= 0 (adjusted)
fn densest_loop(g: &Graph) -> ((i64, i64), Vec<i64>) {
    let mut pot = vec![0; g.moves.len()];
    let moves = move |s: usize| g.moves[s].as_slice();
    // against a loop of l steps per k weight, a move costs steps * k - weight * l
    let cost = |l: i64, k: i64| move |&(t, steps, weight): &(usize, usize, usize)| {
        (t, steps as i64 * k - weight as i64 * l)
    };
    // start from simple loops, expand within groups
    let (mut l, mut k) = (g.moves.iter().flatten().map(|m| m.1 as i64).sum::<i64>() + 1, 1);
    for gate in g.gates.windows(2) {
        while let Err(denser) = bellman_ford(gate[0]..gate[1], &mut pot, moves, cost(l, k)) {
            (l, k) = denser.iter().fold((0, 0), |(l, k), m| (l + m.1 as i64, k + m.2 as i64));
        }
    }
    // final potentials
    pot.fill(0);
    for gate in g.gates.windows(2) {
        bellman_ford(gate[0]..gate[1], &mut pot, moves, cost(l, k)).unwrap();
    }
    ((l, k), pot)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
#####################################################
#.@.........#................#.....................##
#.#########@#.##############.#.###################.##
#.#.......#.#.##..........##.#.###..@..@......@###.##
#.#.#####.#.#.##.########.##.#.###@###########.###.##
#.#.#@@@#.#.#.##.##....##.##@#.###.###.....###.###.##
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###.##
S...................................................E
#.#.#.#.#.#.#.##.##.##.##.##.#.###.###.###.###.###@##
#.#.#...#.#.#.##@##.@..##.##.#.###.###@....###.###.##
#.#.#####.#.#.##.########.##.#.###.###########.###.##
#.#.......#.#.##..@.......##.#.###.............###.##
#.#########.#.##############.#.###################.##
#...........#................#................@....##
#####################################################";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "184");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "5448");
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT), "540320698784");
    }


}
