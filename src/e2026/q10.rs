use crate::algo::fxhash::FxHashSet;

// A ghost is (pos << 2) | dir, with dir 0..4 = > v < ^. Walls surround the grid.
struct State {
    next: Vec<u16>,   // ghost -> ghost after one move, ignoring other ghosts
    count: Vec<u8>,   // scratch: ghosts per cell
    ghosts: Vec<u16>, // only the ghosts still around
    lives: Vec<i64>,
    collisions: u64,
}

fn parse(input: &str, lives: i64) -> State {
    let w = input.lines().next().unwrap().len();
    let grid: Vec<u8> = input.lines().flat_map(|line| line.bytes()).collect();
    let mut ghosts = Vec::new();
    for (pos, &cell) in grid.iter().enumerate() {
        if let Some(dir) = b">v<^".iter().position(|&d| d == cell) {
            ghosts.push((pos << 2 | dir) as u16);
        }
    }
    // move ahead, or stay and turn clockwise when facing a wall
    let next = (0..grid.len() * 4)
        .map(|g| {
            let (pos, dir) = (g >> 2, g & 3);
            if grid[pos] == b'#' {
                return g as u16;
            }
            let ahead = [pos + 1, pos + w, pos - 1, pos - w][dir];
            if grid[ahead] == b'#' { (pos << 2 | (dir + 1) & 3) as u16 } else { (ahead << 2 | dir) as u16 }
        })
        .collect();
    let lives = vec![lives; ghosts.len()];
    State {
        next,
        count: vec![0; grid.len()],
        ghosts,
        lives,
        collisions: 0,
    }
}

// One round for all ghosts; ghosts that run out of lives are removed. Returns whether any did.
fn step(state: &mut State) -> bool {
    let State { next, count, ghosts, lives, collisions } = state;
    let mut hits = 0;
    for (g, life) in ghosts.iter_mut().zip(lives.iter_mut()) {
        let moved = next[*g as usize];
        let wall = (moved >> 2 == *g >> 2) as i64; // stayed in place: hit a wall
        *life -= wall;
        hits += wall;
        *g = moved;
    }
    for &g in ghosts.iter() {
        count[(g >> 2) as usize] += 1;
    }
    let mut died = false;
    for (g, life) in ghosts.iter_mut().zip(lives.iter_mut()) {
        let crash = (count[(*g >> 2) as usize] >= 2) as u16; // sharing a cell: turn around
        *g ^= crash << 1;
        *life -= crash as i64;
        hits += crash as i64;
        died |= *life <= 0;
    }
    for &g in ghosts.iter() {
        count[(g >> 2) as usize] = 0;
    }
    *collisions += hits as u64;
    if died {
        let mut k = 0;
        for i in 0..ghosts.len() {
            if lives[i] > 0 {
                (ghosts[k], lives[k]) = (ghosts[i], lives[i]);
                k += 1;
            }
        }
        ghosts.truncate(k);
        lives.truncate(k);
    }
    died
}

pub fn part1(input: &str) -> String {
    let mut state = parse(input, i64::MAX);
    for _round in 0..10 {
        step(&mut state);
    }
    state.collisions.to_string()
}

pub fn part2(input: &str) -> String {
    vanish(input, 10000)
}

pub fn part3(input: &str) -> String {
    vanish(input, 1_000_000_000_000)
}

// Ghost states since the last death, one row per round.
struct History {
    n: usize,
    ghosts: Vec<u16>,
}

impl History {
    fn reset(&mut self, state: &State) {
        self.n = state.ghosts.len();
        self.ghosts.clear();
        self.push(state);
    }
    fn push(&mut self, state: &State) {
        self.ghosts.extend_from_slice(&state.ghosts);
    }
    fn rows(&self) -> usize {
        self.ghosts.len() / self.n
    }
    fn ghost(&self, row: usize, i: usize) -> u16 {
        self.ghosts[row * self.n + i]
    }
    // hits ghost i took in the round that produced this row: staying put means it hit a wall
    // and turned right (and turned around too if it then collided); moving and turning around
    // means a collision
    fn hits(&self, row: usize, i: usize) -> i64 {
        let (a, b) = (self.ghost(row - 1, i), self.ghost(row, i));
        let turn = b.wrapping_sub(a) & 3;
        if a >> 2 == b >> 2 { 1 + (turn == 3) as i64 } else { (turn == 2) as i64 }
    }
}

// A group of ghosts that only collide with each other, repeating with `period`.
struct Group {
    members: Vec<usize>,
    period: usize,
    cells: Vec<u64>,           // bitset of cells visited during one period
    visits: Vec<(u16, usize)>, // (cell, rounds from now) during one period
}

fn find(parent: &mut [usize], x: usize) -> usize {
    let mut r = x;
    while parent[r] != r {
        r = parent[r];
    }
    parent[x] = r;
    r
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Scratch space for find_groups, indexed by cell.
struct Scratch {
    epoch: u32,
    stamp: Vec<u32>,
    owner: Vec<usize>,
}

// Split the ghosts into groups that collide only among themselves over the second half of the
// history, and find the period of each. Returns None if some group is not periodic yet, or if
// two groups would meet at some point in the future.
fn find_groups(hist: &History, scratch: &mut Scratch) -> Option<Vec<Group>> {
    let n = hist.n;
    let last = hist.rows() - 1;
    let window = last / 2;
    let mut parent: Vec<usize> = (0..n).collect();
    for row in last - window..=last {
        scratch.epoch += 1;
        for i in 0..n {
            let cell = (hist.ghost(row, i) >> 2) as usize;
            if scratch.stamp[cell] == scratch.epoch {
                let (a, b) = (find(&mut parent, i), find(&mut parent, scratch.owner[cell]));
                parent[a] = b;
            } else {
                scratch.stamp[cell] = scratch.epoch;
                scratch.owner[cell] = i;
            }
        }
    }
    let mut groups: Vec<Group> = Vec::new();
    let mut group_of = vec![usize::MAX; n];
    for i in 0..n {
        let root = find(&mut parent, i);
        if group_of[root] == usize::MAX {
            group_of[root] = groups.len();
            groups.push(Group {
                members: Vec::new(),
                period: 0,
                cells: Vec::new(),
                visits: Vec::new(),
            });
        }
        groups[group_of[root]].members.push(i);
    }
    let words = scratch.stamp.len().div_ceil(64);
    for g in groups.iter_mut() {
        // the group moves on its own over the window, so one repeated state means it cycles
        g.period = (1..=window).find(|&p| g.members.iter().all(|&i| hist.ghost(last, i) == hist.ghost(last - p, i)))?;
        g.cells = vec![0; words];
        for t in 1..=g.period {
            for &i in &g.members {
                let cell = hist.ghost(last - g.period + t, i) >> 2;
                g.cells[cell as usize / 64] |= 1 << (cell % 64);
                g.visits.push((cell, t));
            }
        }
    }
    // two groups meet at a cell iff they visit it at times that agree modulo gcd of their periods
    let mut seen: FxHashSet<(u16, usize)> = FxHashSet::default();
    for (a, ga) in groups.iter().enumerate() {
        for gb in &groups[a + 1..] {
            if ga.cells.iter().zip(&gb.cells).all(|(x, y)| x & y == 0) {
                continue;
            }
            let m = gcd(ga.period, gb.period);
            let shared = |c: u16| gb.cells[c as usize / 64] & ga.cells[c as usize / 64] & (1 << (c % 64)) != 0;
            seen.clear();
            seen.extend(ga.visits.iter().filter(|v| shared(v.0)).map(|&(c, t)| (c, t % m)));
            if gb.visits.iter().any(|&(c, t)| seen.contains(&(c, t % m))) {
                return None;
            }
        }
    }
    Some(groups)
}

const NEVER: i64 = i64::MAX;
const FIRST_CHECK: usize = 32;

// Simulate until every ghost is gone. Every so often (doubling intervals since the last death),
// split the ghosts into independent periodic groups; then each ghost's last round is known, ghosts
// alone in their group can be retired right away, and we can jump to the round before the next
// death that changes how some group moves.
fn vanish(input: &str, lives: i64) -> String {
    let mut state = parse(input, lives);
    let mut round: i64 = 0;
    let mut last_death: i64 = 0;
    let mut hist = History { n: 0, ghosts: Vec::new() };
    hist.reset(&state);
    let mut next_check = FIRST_CHECK;
    let cells = state.count.len();
    let mut scratch = Scratch { epoch: 0, stamp: vec![0; cells], owner: vec![0; cells] };
    while !state.ghosts.is_empty() {
        if hist.rows() > next_check {
            next_check *= 2;
            if let Some(groups) = find_groups(&hist, &mut scratch) {
                let last = hist.rows() - 1;
                let n = hist.n;
                let cycle_hits = |g: &Group, i: usize, upto: usize| -> i64 {
                    (1..=upto).map(|t| hist.hits(last - g.period + t, i)).sum()
                };
                // round at which each ghost loses its last life, if nothing changes
                let mut death = vec![NEVER; n];
                for g in &groups {
                    for &i in &g.members {
                        let per_cycle = cycle_hits(g, i, g.period);
                        if per_cycle == 0 {
                            continue;
                        }
                        let left = state.lives[i] - 1;
                        let (cycles, mut extra) = (left / per_cycle, left % per_cycle);
                        for t in 1..=g.period {
                            extra -= hist.hits(last - g.period + t, i);
                            if extra < 0 {
                                death[i] = round + cycles * g.period as i64 + t as i64;
                                break;
                            }
                        }
                    }
                }
                // a death in a group of several ghosts changes how the rest of it moves: simulate that
                let first_change = groups
                    .iter()
                    .filter(|g| g.members.len() > 1)
                    .flat_map(|g| g.members.iter().map(|&i| death[i]))
                    .min()
                    .unwrap_or(NEVER);
                if first_change == NEVER {
                    let end = death.iter().copied().max().unwrap();
                    assert!(end != NEVER, "some ghost never vanishes");
                    return end.max(last_death).to_string();
                }
                let target = first_change - 1;
                if target > round {
                    // fast-forward every group to the round before that death; lone ghosts that
                    // run out of lives on the way are done
                    let skip = target - round;
                    for g in &groups {
                        let (cycles, phase) = (skip / g.period as i64, (skip % g.period as i64) as usize);
                        for &i in &g.members {
                            state.lives[i] -= cycles * cycle_hits(g, i, g.period) + cycle_hits(g, i, phase);
                            state.ghosts[i] = hist.ghost(last - g.period + phase, i);
                            if death[i] <= target {
                                last_death = last_death.max(death[i]);
                            }
                        }
                    }
                    let (mut k, State { ghosts, lives, .. }) = (0, &mut state);
                    for i in 0..n {
                        if death[i] > target {
                            (ghosts[k], lives[k]) = (ghosts[i], lives[i]);
                            k += 1;
                        }
                    }
                    ghosts.truncate(k);
                    lives.truncate(k);
                    round = target;
                    hist.reset(&state);
                    next_check = FIRST_CHECK;
                    continue;
                }
            }
        }
        round += 1;
        if step(&mut state) {
            hist.reset(&state);
            next_check = FIRST_CHECK;
        } else {
            hist.push(&state);
        }
    }
    round.max(last_death).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########";
        assert_eq!(part1(input), "29");
    }

    #[test]
    fn test_part2() {
        let input = "\
###########
#####.#####
###.....###
##.......##
##.>>.<<.##
#v...>...v#
##...v...##
##.......##
###.....###
#####.#####
###########";
        assert_eq!(vanish(input, 10), "53");
    }

    #[test]
    fn test_part3() {
        let input = "\
#######################
###########.###########
#######.^>.^.<^.#######
#####..<.>.v^>>^^<#####
####..v<^v.>.>v..^.####
###.................###
###.................###
##.......<...<.<....>##
##...v....>....v.v<.^##
##........###........##
##v......#####.......##
#.<......#####......>.#
##.......#####.......##
##........###........##
##...................##
##...................##
###.................###
###.................###
####...............####
#####.............#####
#######....v....#######
###########.###########
#######################";
        assert_eq!(vanish(input, 1_000_000_000_000), "13314695352588");
    }
}
