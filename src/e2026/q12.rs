pub fn part1(input: &str) -> String {
    part3(input)
}

pub fn part2(input: &str) -> String {
    part3(input)
}

pub fn part3(input: &str) -> String {
    let mut shapes: Vec<Shape> = Vec::new();
    let w = input.lines().next().unwrap().len();
    let mut starts = vec![(usize::MAX, 0); w + 1];
    let mut ends = vec![(usize::MAX, 0); w + 1];
    for (y, row) in input.lines().map(str::as_bytes).enumerate() {
        let mut x = 0;
        while let Some(skip) = row[x..].iter().position(|&c| c == b'#') {
            let start = x + skip;
            x = start + row[start..].iter().take_while(|&&c| c == b'#').count();
            // (a U open at the top would line up with a different shape at each end: doesn't occur)
            let (left, right) = (starts[start].0 == y, ends[x].0 == y);
            let id = if left || right {
                let id = if left { starts[start].1 } else { ends[x].1 };
                shapes[id].add_run(x - start, left, right);
                id
            } else {
                shapes.push(Shape { w: x - start, t: 1, ..Default::default() });
                shapes.len() - 1
            };
            starts[start] = (y + 1, id);
            ends[x] = (y + 1, id);
        }
    }
    let mut count = 0;
    for shape in &shapes {
        count += shape.lines();
    }
    count.to_string()
}

#[derive(Default)]
struct Shape {
    w: usize, // full width
    l: usize, // width of the bar left
    r: usize, // width of the bar right
    t: usize, // rows above the hole (all rows if there's no hole)
    b: usize, // rows below the hole
    h: usize, // runs beside the hole (one per side)
}

impl Shape {
    fn add_run(&mut self, width: usize, left: bool, right: bool) {
        if width > self.w {
            if left {
                self.l = self.w;
            } else {
                self.r = self.w;
            }
            (self.h, self.t, self.b, self.w) = (self.t, 0, 1, width);
        } else if width < self.w {
            match (left, right) {
                (true, false) => self.l = width,
                (false, true) => self.r = width,
                _ => {}
            }
            self.h += 1;
        } else if self.h == 0 {
            self.t += 1;
        } else {
            self.b += 1;
        }
    }

    fn lines(&self) -> usize {
        if self.h == 0 {
            return self.w.min(self.t);
        }
        let hw = self.w - self.l - self.r;
        let hh = self.h / ((self.l > 0) as usize + (self.r > 0) as usize);
        (self.t + self.b + hh.min(self.l) + hh.min(self.r)).min(self.l + self.r + hw.min(self.t) + hw.min(self.b))
    }
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
.##.......................###.
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
