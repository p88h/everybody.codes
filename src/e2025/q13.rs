pub fn part1(input: &str) -> String {
    let nums = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut dial = vec![1; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        if i % 2 == 0 {
            dial[1 + i / 2] = num;
        } else {
            dial[nums.len() - i / 2] = num;
        }
    }
    dial[2025 % dial.len()].to_string()
}

fn rangefnder(input: &str, mut index: i64) -> i64 {
    let ranges = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let sum = 1 + ranges.iter().map(|(s, e)| e - s + 1).sum::<i64>();
    index %= sum;
    if index > 0 {
        let mut arr = [index - 1, sum - index - 1];
        let mut ofs = 0;
        // look both ways at the same time
        for (start, end) in ranges.iter() {
            let len = end - start + 1;
            if arr[ofs] < len {
                return start + arr[ofs];
            }
            arr[ofs] -= len;
            ofs ^= 1;
        }
    }
    // handle 0 case
    1
}

pub fn part2(input: &str) -> String {
    rangefnder(input, 20252025).to_string()
}

pub fn part3(input: &str) -> String {
    rangefnder(input, 202520252025).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("72\n58\n47\n61\n67"), "67");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("10-15\n12-13\n20-21\n19-23\n30-37"), "30");
    }
}
