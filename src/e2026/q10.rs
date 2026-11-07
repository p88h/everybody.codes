use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    // find all ghosts (positions of '>', '<', '^', 'v')
    let mut ghosts = Vec::new();
    let mut ghost_count = vec![vec![0; grid[0].len()]; grid.len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'>' || cell == b'<' || cell == b'^' || cell == b'v' {
                ghost_count[i][j] += 1;
                match cell {
                    b'>' => ghosts.push((i, j, 0)),
                    b'v' => ghosts.push((i, j, 1)),
                    b'<' => ghosts.push((i, j, 2)),
                    b'^' => ghosts.push((i, j, 3)),
                    _ => {}
                }
            }
        }
    }
    // keep track of count of ghosts per cell
    let mut collisions = 0;
    for _round in 0..10 {
        for (i, j, cell) in ghosts.iter_mut() {
            let (mut ni, mut nj) = (*i, *j);
            ghost_count[*i][*j] -= 1;
            match *cell {
                0 => nj += 1,
                1 => ni += 1,
                2 => nj -= 1,
                3 => ni -= 1,
                _ => {}
            }
            // if the new position is a wall, don't move but trun clockwise
            if grid[ni][nj] == b'#' {
                *cell = (*cell + 1) % 4;
                collisions += 1;
            } else {
                *i = ni;
                *j = nj;
            }
            ghost_count[*i][*j] += 1;
        }
        for (i, j, cell) in ghosts.iter_mut() {
            if ghost_count[*i][*j] > 1 {
                *cell = (*cell + 2) % 4;
                collisions += 1;
            }
        }
    }
    collisions.to_string()
}

pub fn part2(input: &str) -> String {
    vanish(input, 10000)
}

const CHECK_EVERY: i64 = 16;

fn vanish(input: &str, lives: i64) -> String {
    let w = input.lines().next().unwrap().len();
    let grid: Vec<u8> = input.lines().flat_map(|line| line.bytes()).collect();
    let mut ghosts: Vec<u16> = Vec::new();
    let mut ghost_count = vec![0; grid.len()];
    for (pos, &cell) in grid.iter().enumerate() {
        if let Some(dir) = b">v<^".iter().position(|&d| d == cell) {
            ghost_count[pos] += 1;
            ghosts.push((pos << 2 | dir) as u16);
        }
    }
    let mut ghost_lives = vec![lives; ghosts.len()];
    let mut alive = ghosts.len();
    let mut round: i64 = 0;
    // state (all ghosts) -> (round, lives at that time)
    let mut seen: HashMap<Vec<u16>, (i64, Vec<i64>)> = HashMap::new();
    while alive > 0 {
        // only sample every few rounds: sampled states still repeat, with a period that is a multiple of the cycle
        if round % CHECK_EVERY == 0 {
            let key = ghosts.clone();
            if let Some((r0, l0)) = seen.get(&key) {
                let len = round - r0;
                let mut k = i64::MAX;
                for (&before, &after) in l0.iter().zip(&ghost_lives) {
                    if before > after {
                        // how many times this ghost can go through the cycle
                        k = k.min((after - 1) / (before - after));
                    }
                }
                if k > 0 && k != i64::MAX {
                    round += k * len;
                    for (&before, after) in l0.iter().zip(ghost_lives.iter_mut()) {
                        *after -= k * (before - *after);
                    }
                    seen.clear();
                }
            }
            seen.insert(key, (round, ghost_lives.clone()));
        }
        let alive_before = alive;
        round += 1;
        let mut dead = Vec::new();
        for (g, life) in ghosts.iter_mut().zip(ghost_lives.iter_mut()) {
            if *g == 0 {
                continue;
            }
            let pos = (*g >> 2) as usize;
            let dir = *g & 3;
            ghost_count[pos] -= 1;
            let next = match dir {
                0 => pos + 1,
                1 => pos + w,
                2 => pos - 1,
                _ => pos - w,
            };
            if grid[next] == b'#' {
                *life -= 1;
                if *life <= 0 {
                    alive -= 1;
                    *g = 0;
                    continue;
                }
                *g = (*g & !3) | ((dir + 1) & 3); // turn
            } else {                
                *g = (next << 2) as u16 | dir; // move
            }
            ghost_count[(*g >> 2) as usize] += 1;
        }
        // check for ghost-ghost collisions
        for (g, life) in ghosts.iter_mut().zip(ghost_lives.iter_mut()) {
            if *g == 0 {
                continue;
            }
            let pos = (*g >> 2) as usize;
            if ghost_count[pos] > 1 {
                *g ^= 2; // flip
                *life -= 1;
                if *life <= 0 {
                    alive -= 1;
                    dead.push(pos);
                    *g = 0;
                }
            }
        }
        // remove after the check so all ghosts on a shared cell see the same count
        for pos in dead {
            ghost_count[pos] -= 1;
        }
        if alive != alive_before {
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
