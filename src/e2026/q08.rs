fn start_edge(height: usize, width: usize, (dr, dc): (isize, isize)) -> Vec<(usize, usize)> {
    let middle = height / 2;
    let center_left = 0;
    let center_right = width as isize - 1;
    let top_left = middle as isize;
    let top_right = 3 * middle as isize;

    let (mut i, mut j, (first_dr, first_dc), (second_dr, second_dc)) = match (dr, dc) {
        (1, 1) => (middle as isize, center_left, (-1, 1), (0, 2)),
        (1, -1) => (middle as isize, center_right, (-1, -1), (0, -2)),
        (-1, 1) => (middle as isize, center_left, (1, 1), (0, 2)),
        (-1, -1) => (middle as isize, center_right, (1, -1), (0, -2)),
        (0, -2) => (0, top_right, (1, 1), (1, -1)),
        (0, 2) => (0, top_left, (1, -1), (1, 1)),
        _ => unreachable!(),
    };

    let mut starts = Vec::new();
    starts.push((i as usize, j as usize));
    for _ in 0..middle {
        i += first_dr;
        j += first_dc;
        starts.push((i as usize, j as usize));
    }
    for _ in 0..middle {
        i += second_dr;
        j += second_dc;
        starts.push((i as usize, j as usize));
    }
    starts
}

pub fn part1(input: &str) -> String {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let direction = (1, 1);
    let starts = start_edge(grid.len(), grid[0].len(), direction);
    grow(&mut grid, direction, &starts).0.to_string()
}

pub fn part2(input: &str) -> String {
    let original: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let directions = [(1, 1), (1, -1), (0, -2), (-1, 1), (-1, -1), (0, 2)];
    directions
        .iter()
        .map(|&direction| {
            let mut grid = original.clone();
            let starts = start_edge(grid.len(), grid[0].len(), direction);
            grow(&mut grid, direction, &starts).0
        })
        .sum::<usize>()
        .to_string()
}

fn grow(grid: &mut [Vec<u8>], (dr, dc): (isize, isize), starts: &[(usize, usize)]) -> (usize, usize) {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let mut energy = 0;
    let mut grown = 0;

    for &(start_i, start_j) in starts {
        let (mut i, mut j) = (start_i as isize, start_j as isize);
        let mut shadow = 0usize;

        while i >= 0 && i < height && j >= 0 && j < width {
            let cell = &mut grid[i as usize][j as usize];
            if cell.is_ascii_digit() {
                let digit = (*cell - b'0') as usize;
                if digit >= shadow {
                    energy += 1;
                    if digit == 9 {
                        *cell = b'a';
                        grown += 1;
                    } else {
                        *cell += 1;
                    }
                    shadow = digit + 1;
                }
            } else if matches!(*cell, b'a'..=b'c') {
                *cell = match *cell {
                    b'a' => b'b',
                    b'b' => b'c',
                    b'c' => b'1',
                    _ => unreachable!(),
                };
            } else if *cell == b'.' {
                break;
            }
            shadow = shadow.saturating_sub(1);
            i += dr;
            j += dc;
        }
    }
    (energy, grown)
}

pub fn part3(input: &str) -> String {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let dirs = [(1, 1), (1, -1), (0, -2), (-1, -1), (-1, 1), (0, 2)];
    let starts: Vec<Vec<(usize, usize)>> = dirs.iter().map(|&dir| start_edge(grid.len(), grid[0].len(), dir)).collect();
    let mut all_grown = 0;
    let mut all_energy = 0;
    let mut steps = 0;

    while all_grown < 100_000 {
        let idx = steps % dirs.len();
        let (energy, grown) = grow(&mut grid, dirs[idx], &starts[idx]);
        all_energy += energy;
        all_grown += grown;
        steps += 1;
    }
    all_energy.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "32");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "187");
    }

    #[test]
    fn test_part3_reaches_removal_threshold() {
        assert_eq!(part3(INPUT), "899889");
    }
}
