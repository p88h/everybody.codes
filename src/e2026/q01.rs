fn read_nums(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

// input is sorted, so resume the linear scan from the previous result
fn solve(input: &str, s: i64, g: fn(i64) -> i64) -> String {
    let mut n = s;
    let mut res: i64 = 0;
    for a in read_nums(input) {
        while g(n) < a {
            n += 1;
        }
        res += g(n) - a;
    }
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
