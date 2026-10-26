fn nearest(a: i32, s: i32, g: fn(i32) -> i32) -> i32 {
    let mut n = s;
    while g(n) < a {
        n += 1;
    }
    g(n) - a
}

fn read_nums(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn solve(input: &str, s: i32, g: fn(i32) -> i32) -> String {
    let nums = read_nums(input);
    let res: i32 = nums.iter().map(|&n| nearest(n, s, g)).sum();
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
