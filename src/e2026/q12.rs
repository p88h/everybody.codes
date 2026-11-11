use crate::algo::vertex_cover::min_vertex_cover;

pub fn part1(input: &str) -> String {
    part2(input)
}

pub fn part2(input: &str) -> String {
    let w = input.lines().next().unwrap().len();
    let mut shapes: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut prev = vec![usize::MAX; w];
    let mut cur = vec![usize::MAX; w];
    for row in input.lines().map(str::as_bytes) {
        cur.fill(usize::MAX);
        let mut x = 0;
        while let Some(skip) = row[x..].iter().position(|&c| c == b'#') {
            let start = x + skip;
            x = start + row[start..].iter().take_while(|&&c| c == b'#').count();
            let mut id = prev[start].min(prev[x - 1]);
            if id == usize::MAX {
                id = shapes.len();
                shapes.push((usize::MAX, 0, 0, 0));
            }
            let width = x - start;
            let (min_w, max_w, num_full, rows) = &mut shapes[id];
            *min_w = (*min_w).min(width);
            if width > *max_w {
                *max_w = width;
                *num_full = 0;
            }
            if width == *max_w {
                *num_full += 1;
            }
            *rows += 1;
            cur[start..x].fill(id);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    let mut count = 0;
    for &(min_w, max_w, num_full, rows) in &shapes {
        count += max_w.min(rows).min(num_full + min_w);
    }
    count.to_string()
}

pub fn part3(input: &str) -> String {
    let g: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (h, w) = (g.len(), g[0].len());
    // flat adjacency: h-run u's v-runs are targets[starts[u]..starts[u + 1]]
    let mut starts: Vec<usize> = Vec::new();
    let mut targets: Vec<usize> = Vec::new();
    let mut vid = vec![usize::MAX; w];
    let mut nv = 0;
    for y in 0..h {
        for x in 0..w {
            if g[y][x] == b'#' {
                // start a new h-run unless continuing from the left
                if x == 0 || g[y][x - 1] != b'#' {
                    starts.push(targets.len());
                }
                // start a new v-run unless continuing from above
                if y == 0 || g[y - 1][x] != b'#' {
                    vid[x] = nv;
                    nv += 1;
                }
                targets.push(vid[x]);
            }
        }
    }
    let nh = starts.len();
    starts.push(targets.len());
    min_vertex_cover(nh, nv, |u| &targets[starts[u]..starts[u + 1]]).to_string()
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
