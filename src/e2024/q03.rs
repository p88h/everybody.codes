fn solve(input: &str, directions: Vec<(isize, isize)>) -> usize {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut pos = Vec::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'#' {
                grid[r][c] = b'1';
                pos.push((r, c));
            } else {
                grid[r][c] = b'0';
            }
        }
    }
    let mut total = 0;
    while pos.len() > 0 {
        total += pos.len();
        let mut npos = Vec::new();
        for &(r, c) in &pos {
            let mut min_height = grid[r][c];
            for (dx, dy) in &directions {
                let nr = r as isize + dx;
                let nc = c as isize + dy;
                if nr < grid.len() as isize && nc < grid[0].len() as isize && nr >= 0 && nc >= 0 {
                    min_height = min_height.min(grid[nr as usize][nc as usize]);
                } else {
                    min_height = 0;
                }
            }
            if min_height == grid[r][c] {
                grid[r][c] += 1;
                npos.push((r, c));
            }
        }
        pos = npos;
    }
    total
}

pub fn part1(input: &str) -> String {
    solve(input, vec![(0, 1), (1, 0), (0, -1), (-1, 0)]).to_string()
}

pub fn part2(input: &str) -> String {
    solve(input, vec![(0, 1), (1, 0), (0, -1), (-1, 0)]).to_string()
}

pub fn part3(input: &str) -> String {
    solve(input, vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
..........
..###.##..
...####...
..######..
..######..
...####...
..........";
        assert_eq!(part1(input), "35");
    }
}
