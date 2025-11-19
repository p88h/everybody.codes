use hungarian::minimize;
use itertools::Itertools;

fn toss(grid: &Vec<&[u8]>, dir: &[u8], slot: usize) -> i32 {
    let mut col = slot * 2;
    let mut row = 0;
    let mut cur = 0;
    while row < grid.len() {
        if grid[row][col] == b'.' {
            row += 1;
        } else if col == 0 || (dir[cur] == b'R' && col < grid[row].len() - 1) {
            col += 1;
            cur += 1;
        } else {
            col -= 1;
            cur += 1;
        }
    }
    let score = ((col as i32 / 2 + 1) * 2 - (slot as i32 + 1)).max(0);
    // println!("{:?} {} {} {}", dir.iter().map(|c| *c as char).collect::<String>(), slot + 1, col / 2 + 1, score);
    score
}

fn parse_input(input: &str) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let (b, d) = input.split("\n\n").collect_tuple().unwrap();
    let grid = b.lines().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();
    let dirs = d.lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    (grid, dirs)
}

pub fn part1(input: &str) -> String {
    let (grid, dirs) = parse_input(input);
    dirs.iter().enumerate().map(|(i, dir)| toss(&grid, dir, i)).sum::<i32>().to_string()
}

pub fn part2(input: &str) -> String {
    let (grid, dirs) = parse_input(input);
    let mut total = 0;
    for (_, dir) in dirs.iter().enumerate() {
        total += (0..(grid[0].len() + 1) / 2).map(|pos| toss(&grid, dir, pos)).max().unwrap();
    }
    total.to_string()
}

pub fn part3(input: &str) -> String {
    let (grid, dirs) = parse_input(input);
    let mut costs: Vec<i32> = vec![];
    for (_, dir) in dirs.iter().enumerate() {
        costs.append(&mut (0..(grid[0].len() + 1) / 2).map(|pos| toss(&grid, dir, pos)).collect::<Vec<i32>>());
    }
    let mins = minimize(&costs.as_slice(), dirs.len(), costs.len() / dirs.len());
    let sum1 = mins.iter().enumerate().map(|(i, &pos)| toss(&grid, dirs[i], pos.unwrap())).sum::<i32>();
    // now negate all costs
    costs = costs.iter().map(|c| 1000 - c).collect::<Vec<i32>>();
    let maxs = minimize(&costs.as_slice(), dirs.len(), costs.len() / dirs.len());
    let sum2 = maxs.iter().enumerate().map(|(i, &pos)| toss(&grid, dirs[i], pos.unwrap())).sum::<i32>();
    format!("{} {}", sum1, sum2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL
LRLLLLLLRLLL
RRLLLRLLRLRR
RLLLLLRLLLRL";
        assert_eq!(part1(input), "26");
    }

    #[test]
    fn test_part2() {
        let input = "\
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
..*.*.*.*...*.*...*.*.*..
.*...*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.......*.
*.*.*.*.*.*.*.*.*.*...*..
.*.*.*.*.*.*.*.*.....*.*.
*.*...*.*.*.*.*.*.*.*....
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*...*.*.
*.*.*.*.*.*.*.*.*...*.*.*
.*.*.*.*.*.*.*.*.....*.*.
*.*.*.*.*.*.*.*...*...*.*
.*.*.*.*.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*.*.*.*.*
.*...*.*.*.*...*.*.*...*.
*.*.*.*.*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.*.*.*.*.

RRRLLRRRLLRLRRLLLRLR
RRRRRRRRRRLRRRRRLLRR
LLLLLLLLRLRRLLRRLRLL
RRRLLRRRLLRLLRLLLRRL
RLRLLLRRLRRRLRRLRRRL
LLLLLLLLRLLRRLLRLLLL
LRLLRRLRLLLLLLLRLRRL
LRLLRRLLLRRRRRLRRLRR
LRLLRRLRLLRLRRLLLRLL
RLLRRRRLRLRLRLRLLRRL";
        assert_eq!(part2(input), "115");
    }

    #[test]
    fn test_part3_small() {
        let input = "\
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*...*..
.*.*.*.*.*...*.*.
*.*.....*...*.*.*
.*.*.*.*.*.*.*.*.
*...*...*.*.*.*.*
.*.*.*.*.*.*.*.*.
*.*.*...*.*.*.*.*
.*...*...*.*.*.*.
*.*.*.*.*.*.*.*.*
.*.*.*.*.*.*.*.*.

RRRLRLRRRRRL
LLLLRLRRRRRR
RLLLLLRLRLRL
LRLLLRRRLRLR
LLRLLRLLLRRL
LRLRLLLRRRRL";
        assert_eq!(part3(input), "13 43");
    }
}
