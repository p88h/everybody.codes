pub fn part1(input: &str) -> String {
    let nums = input.lines().filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let min = *nums.iter().min().unwrap_or(&0);
    let sum: i32 = nums.iter().sum();
    let result = sum - min * nums.len() as i32;
    result.to_string()
}

pub fn part2(input: &str) -> String {
    part1(input)
}

pub fn part3(input: &str) -> String {
    let nums = input.lines().filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
    let sum: i64 = nums.iter().sum();
    let avg = sum / nums.len() as i64;
    let mut best_cost = i64::MAX;
    let mut ofs = 0;
    for dir in [-1, 1] {
        loop {
            let candidate = avg + ofs * dir;
            let cost = nums.iter().map(|x| (x - candidate).abs()).sum::<i64>();
            // println!("+{ofs} candidate: {candidate}, cost: {cost}");
            if cost < best_cost {
                best_cost = cost;
                ofs += 1;
            } else {
                break;
            }
        }
    }
    best_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "3\n4\n7\n8";
        assert_eq!(part1(input), "10");
    }

    #[test]
    fn test_part3() {
        let input = "2\n4\n5\n6\n8";
        assert_eq!(part3(input), "8");
    }
}
