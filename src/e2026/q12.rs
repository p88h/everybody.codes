use crate::algo::vertex_cover::min_vertex_cover;

pub fn part1(input: &str) -> String {
    // load the input into u8 grid
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.chars().map(|c| c as u8).collect()).collect();
    // look for '#'
    let mut count = 0;
    let width = grid[0].len();
    let height = grid.len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'#' {
                let mut rw = 1;
                let mut rh = 1;
                while x + rw < width && grid[y][x + rw] == b'#' {
                    rw += 1;
                }
                while y + rh < height && grid[y + rh][x] == b'#' {
                    rh += 1;
                }
                // zero out the rectangle to avoid double counting
                for dy in 0..rh {
                    for dx in 0..rw {
                        grid[y + dy][x + dx] = b'.';
                    }
                }
                // count the min size
                count += rw.min(rh);
            }
        }
    }
    count.to_string()
}

pub fn part2(input: &str) -> String {
    // part2 had a simplified formula but it wasn't faster than part3 solution
    part3(input)
}

pub fn part3(input: &str) -> String {
    let g: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (h, w) = (g.len(), g[0].len());
    let mut adj: Vec<Vec<usize>> = Vec::new();
    let mut vid = vec![usize::MAX; w];
    let mut nv = 0;
    for y in 0..h {
        for x in 0..w {
            if g[y][x] == b'#' {
                // start a new h-run unless continuing from the left
                if x == 0 || g[y][x - 1] != b'#' {
                    adj.push(Vec::new());
                }
                // start a new v-run unless continuing from above
                if y == 0 || g[y - 1][x] != b'#' {
                    vid[x] = nv;
                    nv += 1;
                }
                adj.last_mut().unwrap().push(vid[x]);
            }
        }
    }
    min_vertex_cover(&adj, nv).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
........#.
.##.......
.##..####.
.##..####.
.....####.
.###......";
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn test_part2() {
        let input = "\
........#.
.##.......
.##..####.
.#...#....
.....#....
.####.....";
        assert_eq!(part2(input), "6");
    }

    #[test]
    fn test_part2_big() {
        let input = "\
...............................
.####....#####..#######........
.####....#####..#######........
.####....#####..#######........
.........#####.......##........
.........########....##........
.........########....##........
...............................
.######....#.....#.....###.....
...........#...........###.....
...........#......########.....
...........#......########.....
...........#......########.....
..................########.....
...............................";
        assert_eq!(part2(input), "23");
    }

    #[test]
    fn test_part3_small() {
        let input = "\
........#.
.##.......
.##..####.
.#...#..#.
.....####.
.###......";
        assert_eq!(part3(input), "8");
    }

    #[test]
    fn test_part3_medium() {
        let input = "\
..............................
.##########....##.....#######.
.##########.##.##..#..#######.
.####..####.##.##.....#.#####.
.##########.##.#####..#.#####.
.##########.##........#######.
..............................
.##########..################.
.##########..#..............#.
.#######..#..#..............#.
.#######..#..################.
.##########..################.
..............................";
        assert_eq!(part3(input), "29");
    }

    #[test]
    fn test_part3_spiral() {
        let input = "\
..............................
.############################.
.##...................###.###.
.##..################.###.###.
.##..######.........#.###.###.
.##..################.###.###.
.##...................###.###.
.##..####################.###.
.##..####################.###.
.##.......................###.
.############################.
..............................";
        assert_eq!(part3(input), "16");
    }
}
