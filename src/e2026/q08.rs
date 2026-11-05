// Cells are stored mod 12: digits 1-9 become 0-8, 'a'-'c' become 9-11, and a growing
// plant wraps from 11 back to 0. Cells outside the hexagon are never visited.
const GROWN: u8 = 9;
const CYCLE: u8 = 12;

fn parse_grid(input: &str) -> (Vec<u8>, usize, usize) {
    let rows: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let height = rows.len();
    let width = rows[0].len();
    let grid = rows
        .into_iter()
        .flatten()
        .map(|&c| match c {
            b'1'..=b'9' => c - b'1',
            b'a'..=b'c' => c - b'a' + GROWN,
            _ => 0,
        })
        .collect();
    (grid, height, width)
}

// A ray start as (cell index, number of cells in the hexagon along the ray).
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

    // The n-th start along the two edges always has a ray of 2 * middle + 1 - |n - middle| cells.
    let mut n = 0usize;
    let mut ray = |i: isize, j: isize| {
        let len = 2 * middle + 1 - n.abs_diff(middle);
        n += 1;
        (i as usize * width + j as usize, len)
    };

    let mut starts = Vec::new();
    starts.push(ray(i, j));
    for _ in 0..middle {
        i += first_dr;
        j += first_dc;
        starts.push(ray(i, j));
    }
    for _ in 0..middle {
        i += second_dr;
        j += second_dc;
        starts.push(ray(i, j));
    }
    starts
}

// Transition table indexed by shadow * CYCLE + cell. Each entry packs the new cell value (bits 0-3),
// the next shadow (bits 4-7), whether the cell was energized (bit 8) and whether it became fully
// grown (bit 9).
const TRANSITIONS: [u16; 10 * CYCLE as usize] = {
    let mut table = [0; 10 * CYCLE as usize];
    let mut shadow = 0;
    while shadow < 10 {
        let mut v = 0;
        while v < CYCLE {
            // Shadowed young plants don't grow; fully grown ones always advance.
            let energized = v < GROWN && v >= shadow;
            let next_shadow = if energized { v + 1 } else { shadow };
            let next_v = if energized || v >= GROWN { (v + 1) % CYCLE } else { v };
            let grown = energized && v == GROWN - 1;
            table[(shadow * CYCLE + v) as usize] = next_v as u16
                | (next_shadow.saturating_sub(1) as u16) << 4
                | (energized as u16) << 8
                | (grown as u16) << 9;
            v += 1;
        }
        shadow += 1;
    }
    table
};

fn grow(grid: &mut [u8], step: isize, starts: &[(usize, usize)]) -> (usize, usize) {
    let mut energy = 0;
    let mut grown = 0;

    for &(start, len) in starts {
        let mut pos = start as isize;
        // Lowest value that still receives energy; each cell passed lowers it by one.
        let mut shadow = 0;

        for _ in 0..len {
            let cell = &mut grid[pos as usize];
            let t = TRANSITIONS[shadow * CYCLE as usize + *cell as usize] as usize;
            *cell = (t & 15) as u8;
            shadow = (t >> 4) & 15;
            energy += (t >> 8) & 1;
            grown += t >> 9;
            pos += step;
        }
    }
    (energy, grown)
}

fn step((dr, dc): (isize, isize), width: usize) -> isize {
    dr * width as isize + dc
}

pub fn part1(input: &str) -> String {
    let (mut grid, height, width) = parse_grid(input);
    let direction = (1, 1);
    let starts = start_edge(height, width, direction);
    grow(&mut grid, step(direction, width), &starts).0.to_string()
}

pub fn part2(input: &str) -> String {
    let (original, height, width) = parse_grid(input);
    let directions = [(1, 1), (1, -1), (0, -2), (-1, 1), (-1, -1), (0, 2)];
    directions
        .iter()
        .map(|&direction| {
            let mut grid = original.clone();
            let starts = start_edge(height, width, direction);
            grow(&mut grid, step(direction, width), &starts).0
        })
        .sum::<usize>()
        .to_string()
}

pub fn part3(input: &str) -> String {
    let (mut grid, height, width) = parse_grid(input);
    let dirs = [(1, 1), (1, -1), (0, -2), (-1, -1), (-1, 1), (0, 2)];
    let starts: Vec<Vec<(usize, usize)>> = dirs.iter().map(|&dir| start_edge(height, width, dir)).collect();
    let steps_by_dir: Vec<isize> = dirs.iter().map(|&dir| step(dir, width)).collect();
    let mut all_grown = 0;
    let mut all_energy = 0;
    let mut steps = 0;

    while all_grown < 100_000 {
        let idx = steps % dirs.len();
        let (energy, grown) = grow(&mut grid, steps_by_dir[idx], &starts[idx]);
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
