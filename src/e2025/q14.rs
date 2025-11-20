fn life_round(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = grid.clone();
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    for r in 0..rows {
        for c in 0..cols {
            let mut live_neighbors = 0;
            for (dr, dc) in directions.iter() {
                let nr = r + dr;
                let nc = c + dc;
                if nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr as usize][nc as usize] {
                    live_neighbors += 1;
                }
            }
            if grid[r as usize][c as usize] {
                new_grid[r as usize][c as usize] = live_neighbors % 2 == 1;
            } else {
                new_grid[r as usize][c as usize] = live_neighbors % 2 == 0;
            }
        }
    }
    new_grid
}

pub fn part1(input: &str) -> String {
    let mut life_grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut tot = 0;
    for round in 0..10 {
        life_grid = life_round(&life_grid);
        let live_count = life_grid.iter().flatten().filter(|&&b| b).count();        
        tot += live_count;
    }    
    tot.to_string()
}

pub fn part2(input: &str) -> String {
    let mut life_grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut tot = 0;
    for round in 0..2025 {
        life_grid = life_round(&life_grid);
        let live_count = life_grid.iter().flatten().filter(|&&b| b).count();
        tot += live_count;
    }    
    tot.to_string()
}

pub fn part3(input: &str) -> String {
    let small_grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let mut life_grid = vec![vec![false; 34]; 34];
    let mut total_matched = 0;
    let mut last_match = 0;
    let mut matches = Vec::new();
    for round in 0..1000000000  {
        life_grid = life_round(&life_grid);
        let live_count = life_grid.iter().flatten().filter(|&&b| b).count();
        // check if center 8x8 grid matches small_grid
        let mut match_found = true;
        for r in 0..8 {
            for c in 0..8 {
                if life_grid[r + 13][c + 13] != small_grid[r][c] {
                    match_found = false;
                    break;
                }
            }
            if !match_found {
                break;
            }
        }
        if match_found {
            total_matched += live_count;
            matches.push((round - last_match, live_count));
            last_match = round;
            for i in 0..matches.len() - 1 {
                if matches[i] == matches[matches.len() - 1] {
                    // sum of all cycle deltas
                    let cycle_len = matches[i+1..].iter().map(|x| x.0).sum::<usize>();
                    let cycle_sum = matches[i+1..].iter().map(|x| x.1).sum::<usize>();
                    let cycles = (1000000000 - round - 1) / cycle_len;
                    total_matched += cycles * cycle_sum;
                    let mut remaining = (1000000000 - round - 1) % cycle_len;
                    for j in i..matches.len() - 1 {
                        if remaining < matches[j].0 {
                            break;
                        }
                        total_matched += matches[j].1;
                        remaining -= matches[j].0;
                    }
                    return total_matched.to_string();
                }
            }
            
        }
    }
    total_matched.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
.#.##.
##..#.
..##.#
.#.##.
.###..
###.##";
        assert_eq!(part1(input), "200");
    }
}