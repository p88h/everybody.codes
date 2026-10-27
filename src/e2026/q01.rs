use crate::algo::binary_search::*;

fn nearest(a: i64, s: i64, g: fn(i64) -> i64) -> i64 {
    g(lower_bound(s, a, g)) - a
}

fn read_nums(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn solve(input: &str, s: i64, g: fn(i64) -> i64) -> String {
    let nums = read_nums(input);
    let res: i64 = nums.iter().map(|&n| nearest(n, s, g)).sum();
    res.to_string()
}

pub fn part1(input: &str) -> String {
    solve(input, 1, |x| x * x)
}

pub fn part2(input: &str) -> String {
    solve(input, 3, |x| (x * x) - (x - 2) * (x - 2))
}

pub fn part3(input: &str) -> String {
    solve(input, 1, |x| (x * 2 + 1) * (x * 2 + 1) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3,11,12,16,24,25";
        assert_eq!(part1(input), "11");
    }

    #[test]
    fn test_part2() {
        let input = "3,11,12,16,24,25";
        assert_eq!(part2(input), "9");
    }

    #[test]
    fn test_part3() {
        let input = "3,11,12,16,24,25";
        assert_eq!(part3(input), "61");
    }
}
