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

fn rangefnder(input: &str, index: i64) -> i64 {
    let ranges = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let mut left = Vec::new();
    let mut right = vec![(1i64, 1i64)];
    for (idx, (start, end)) in ranges.iter().enumerate() {
        if idx % 2 == 0 {
            right.push((*start, *end));
        } else {
            left.push((*end, *start));
        }
    }
    left.reverse();
    right.extend(left);
    let sum = right.iter().map(|(s, e)| if e > s { e - s + 1 } else { s - e + 1 }).sum::<i64>();
    let mut target = index % sum;
    for (start, end) in right.iter() {
        let len = if end > start { end - start + 1 } else { start - end + 1 };
        if target < len {
            if end >= start {
                return start + target;
            } else {
                return start - target;
            }
        } else {
            target -= len;
        }
    }
    0
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
