use std::collections::HashMap;

struct State {
    grid: Vec<u8>, // low bit: wall; bits 1-6: ghost count; high bit: collision marker
    w: usize,
    ghosts: Vec<u16>, // (pos << 2) | dir
    ghost_lives: Vec<i64>,
    alive: usize,
    collisions: u128,
}

const WALL: u8 = 1;
const GHOST: u8 = 2;
const MARK: u8 = 128;

fn parse(input: &str, lives: i64) -> State {
    let w = input.lines().next().unwrap().len();
    let mut grid = Vec::new();
    let mut ghosts = Vec::new();
    for cell in input.lines().flat_map(|line| line.bytes()) {
        let pos = grid.len();
        if let Some(dir) = b">v<^".iter().position(|&d| d == cell) {
            grid.push(GHOST);
            ghosts.push((pos << 2 | dir) as u16);
        } else {
            grid.push(if cell == b'#' { WALL } else { 0 });
        }
    }
    let alive = ghosts.len();
    let ghost_lives = vec![lives; alive];
    State { grid, w, ghosts, ghost_lives, alive, collisions: 0 }
}

fn hit(g: &mut u16, life: &mut i64, alive: &mut usize, collisions: &mut u128) {
    *collisions += 1;
    *life -= 1;
    if *life <= 0 {
        *alive -= 1;
        *g = 0;
    }
}

// one round of movement for all ghosts
fn step(state: &mut State) {
    let State { grid, w, ghosts, ghost_lives, alive, collisions } = state;
    let w = *w;
    for (gi, g) in ghosts.iter_mut().enumerate() {
        if *g == 0 {
            continue;
        }
        let pos = (*g >> 2) as usize;
        let dir = *g & 3;
        grid[pos] &= !MARK;
        grid[pos] -= GHOST;
        let next = match dir {
            0 => pos + 1,
            1 => pos + w,
            2 => pos - 1,
            _ => pos - w,
        };
        // if the new position is a wall, don't move but turn clockwise
        if grid[next] & WALL != 0 {
            *g = (*g & !3) | ((dir + 1) & 3); // turn
            hit(g, &mut ghost_lives[gi], alive, collisions);
        } else {
            *g = (next << 2) as u16 | dir; // move
        }
        if *g != 0 {
            let npos = (*g >> 2) as usize;
            grid[npos] &= !MARK;
            grid[npos] += GHOST;
        }
    }
    // check ghost-to-ghost collisions
    for (gi, g) in ghosts.iter_mut().enumerate() {
        if *g == 0 {
            continue;
        }
        let pos = (*g >> 2) as usize;
        if grid[pos] >= 2 * GHOST {
            *g ^= 2; // flip
            hit(g, &mut ghost_lives[gi], alive, collisions);
            if *g == 0 {
                // remove ghost but add marker
                grid[pos] = ((grid[pos] & !MARK) - GHOST) | MARK;
            }
        }
    }
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

const CHECK_EVERY: i64 = 16;

fn vanish(input: &str, lives: i64) -> String {
    let mut state: State = parse(input, lives);
    let mut round: i64 = 0;
    // state (all ghosts) -> (round, lives at that time)
    let mut seen: HashMap<Vec<u16>, (i64, Vec<i64>)> = HashMap::new();
    while state.alive > 0 {
        // only sample every few rounds: sampled states still repeat, with a period that is a multiple of the cycle
        if round % CHECK_EVERY == 0 {
            let key = state.ghosts.clone();
            if let Some((r0, l0)) = seen.get(&key) {
                let len = round - r0;
                let mut k = i64::MAX;
                for (&before, &after) in l0.iter().zip(&state.ghost_lives) {
                    if before > after {
                        // how many times this ghost can go through the cycle
                        k = k.min((after - 1) / (before - after));
                    }
                }
                if k > 0 && k != i64::MAX {
                    round += k * len;
                    for (&before, after) in l0.iter().zip(state.ghost_lives.iter_mut()) {
                        let skipped_hits = k * (before - *after);
                        *after -= skipped_hits;
                        state.collisions += skipped_hits as u128;
                    }
                    seen.clear();
                }
            }
            seen.insert(key, (round, state.ghost_lives.clone()));
        }
        let alive_before = state.alive;
        round += 1;
        step(&mut state);
        if state.alive != alive_before {
            seen.clear();
        }
    }
    round.to_string()
}

pub fn part3(input: &str) -> String {
    vanish(input, 1_000_000_000_000)
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
