fn parse_bit_grid(input: &str) -> (Vec<u64>, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let grid = lines
        .iter()
        .map(|line| {
            let mut row: u64 = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1u64 << (63 - i);
                }
            }
            row
        })
        .collect();
    (grid, width)
}

fn life_round_bits(grid: &Vec<u64>, width: usize) -> Vec<u64> {
    let rows = grid.len();
    let mut new_grid = vec![0u64; rows];
    let mask = !0u64 << (64 - width);

    for r in 0..rows {
        let prev_row = if r > 0 { grid[r - 1] } else { 0 };
        let this_row = grid[r];
        let next_row = if r + 1 < rows { grid[r + 1] } else { 0 };

        // Compute new row: ~(diagonal_xor ^ this_row)
        new_grid[r] = !(prev_row << 1 ^ prev_row >> 1 ^ this_row ^ next_row << 1 ^ next_row >> 1) & mask;
    }

    new_grid
}

pub fn part1(input: &str) -> String {
    let (mut life_grid, width) = parse_bit_grid(input);
    let mut tot = 0;
    for _ in 0..10 {
        life_grid = life_round_bits(&life_grid, width);
        let live_count: u32 = life_grid.iter().map(|row| row.count_ones()).sum();
        tot += live_count;
    }
    tot.to_string()
}

pub fn part2(input: &str) -> String {
    let (mut life_grid, width) = parse_bit_grid(input);
    let mut tot = 0;
    for _ in 0..2025 {
        life_grid = life_round_bits(&life_grid, width);
        let live_count: u32 = life_grid.iter().map(|row| row.count_ones()).sum();
        tot += live_count;
    }
    tot.to_string()
}

pub fn part3(input: &str) -> String {
    let (small_grid, _) = parse_bit_grid(input);
    // Create empty 34x34 bit grid
    let mut life_grid = vec![0u64; 34];
    let width = 34;
    let mut total_matched = 0;
    let mut last_match = 0;
    let mut matches = Vec::new();

    for round in 0..1000000000 {
        life_grid = life_round_bits(&life_grid, width);
        let live_count: u32 = life_grid.iter().map(|row| row.count_ones()).sum();

        // Check if center 8x8 grid matches small_grid
        // The center 8x8 starts at row 13, col 13
        let mut match_found = true;
        for r in 0..8 {
            // Extract 8 bits from position 13 to 20 (inclusive) in the life_grid row
            let life_row_bits = (life_grid[r + 13] >> (64 - 13 - 8)) & 0xFF;
            let small_row_bits = (small_grid[r] >> (64 - 8)) & ((1u64 << 8) - 1);
            if life_row_bits != small_row_bits {
                match_found = false;
                break;
            }
        }

        if match_found {
            total_matched += live_count as usize;
            matches.push((round - last_match, live_count as usize));
            last_match = round;
            for i in 0..matches.len() - 1 {
                if matches[i] == matches[matches.len() - 1] {
                    // sum of all cycle deltas
                    let cycle_len = matches[i + 1..].iter().map(|x| x.0).sum::<usize>();
                    let cycle_sum = matches[i + 1..].iter().map(|x| x.1).sum::<usize>();
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
