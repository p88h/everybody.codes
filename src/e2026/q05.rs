fn build_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>()
}

fn count_neighbors(grid: &Vec<Vec<u8>>, what: u8) -> Vec<Vec<i32>> {
    let mut neighbor_count = vec![vec![0; grid[0].len()]; grid.len()];
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for &(di, dj) in &directions {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && ni < grid.len() as isize && nj >= 0 && nj < grid[0].len() as isize {
                    if grid[ni as usize][nj as usize] == what {
                        neighbor_count[i][j] += 1;
                    }
                }
            }
        }
    }
    neighbor_count
}

// Marks `seeds` with `mark`, then repeatedly marks cells accepted by `can_mark` that touch at least 5
// cells which are either `precounted` in the original grid or already marked. Returns the number of marked cells.
fn grow(
    grid: &mut Vec<Vec<u8>>,
    seeds: Vec<(usize, usize)>,
    precounted: u8,
    mark: u8,
    can_mark: impl Fn(u8) -> bool,
) -> usize {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    let mut neighbor_count = count_neighbors(grid, precounted);
    let original = grid.clone();
    let mut frontier = seeds;
    for &(i, j) in &frontier {
        grid[i][j] = mark;
    }
    let mut next = 0;
    while next < frontier.len() {
        let (i, j) = frontier[next];
        next += 1;
        for &(di, dj) in &directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni >= 0 && ni < grid.len() as isize && nj >= 0 && nj < grid[0].len() as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                // cells that were precounted already contribute to their neighbors
                if original[i][j] != precounted {
                    neighbor_count[ni][nj] += 1;
                }
                if can_mark(grid[ni][nj]) && neighbor_count[ni][nj] >= 5 {
                    grid[ni][nj] = mark;
                    frontier.push((ni, nj));
                }
            }
        }
    }
    frontier.len()
}

fn make_land(grid: &mut Vec<Vec<u8>>) -> usize {
    let center = (grid.len() / 2, grid[0].len() / 2);
    grow(grid, vec![center], b'1', b'#', |c| c != b'#')
}

pub fn part1(input: &str) -> String {
    let mut grid = build_grid(input);
    make_land(&mut grid).to_string()
}

fn make_sand(grid: &mut Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if *cell != b'#' {
                *cell = b'.';
            }
        }
    }
    let water = count_neighbors(grid, b'.');
    let mut sand = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'#' && water[i][j] > 0 {
                sand.push((i, j));
            }
        }
    }
    sand
}

pub fn part2(input: &str) -> String {
    let mut grid = build_grid(input);
    make_land(&mut grid);
    let sand = make_sand(&mut grid);
    grow(&mut grid, sand, b'=', b'=', |c| c == b'#').to_string()
}

fn plant_trees(grid: &mut Vec<Vec<u8>>) {
    let (h, w) = (grid.len(), grid[0].len());
    for i in 2..h.saturating_sub(2) {
        for j in 2..w.saturating_sub(2) {
            if grid[i][j] == b'#' && (i - 2..=i + 2).all(|ni| {
                (j - 2..=j + 2).all(|nj| {
                    let corner = ni.abs_diff(i) == 2 && nj.abs_diff(j) == 2;
                    corner || matches!(grid[ni][nj], b'#' | b'@')
                })
            }) {
                grid[i][j] = b'@';
            }
        }
    }
}

// One simultaneous growth step: land with at least 4 tree neighbors and no sand neighbors becomes a tree.
fn grow_forest(grid: &mut Vec<Vec<u8>>) {
    let tree_count = count_neighbors(grid, b'@');
    let sand_count = count_neighbors(grid, b'=');
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'#' && tree_count[i][j] >= 4 && sand_count[i][j] == 0 {
                grid[i][j] = b'@';
            }
        }
    }
}

pub fn part3(input: &str) -> String {
    let mut grid = build_grid(input);
    make_land(&mut grid);
    let sand = make_sand(&mut grid);
    grow(&mut grid, sand, b'=', b'=', |c| c == b'#');
    plant_trees(&mut grid);
    grow_forest(&mut grid);
    grow_forest(&mut grid);
    grid.iter().flatten().filter(|&&c| c == b'@').count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), "137");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), "67");
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(EXAMPLE), "14");
    }
}
