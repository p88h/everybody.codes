pub fn sorted_input(input: &str) -> Vec<i32> {
    let mut nums: Vec<i32> = input
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    nums.sort();
    nums
}

pub fn part1(input: &str) -> String {
    let nums = sorted_input(input);
    let mut total = 0;
    for i in 0..nums.len() - 1 {
        if nums[i + 1] != nums[i] {
            total += nums[i]
        }
    }
    total += nums[nums.len() - 1];
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let nums = sorted_input(input);
    let mut total = 0;
    let mut cnt = 0;
    for i in 0..nums.len() - 1 {
        if nums[i + 1] != nums[i] {
            total += nums[i];
            cnt += 1;
            if cnt == 20 {
                break;
            }
        }
    }
    total.to_string()
}

pub fn part3(input: &str) -> String {
    let nums = sorted_input(input);
    let mut longest = 0;
    let mut cnt = 0;
    for i in 0..nums.len() - 1 {
        if nums[i + 1] != nums[i] {
            longest = longest.max(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }
    longest += 1;
    longest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "10,5,1,10,3,8,5,2,2";
        assert_eq!(part1(input), "29");
    }
    #[test]
    fn test_part2() {
        let input = "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";
        assert_eq!(part2(input), "781");
    }
    #[test]
    fn test_part3() {
        let input = "4,51,13,64,57,51,82,57,16,88,89,48,32,49,49,2,84,65,49,43,9,13,2,3,75,72,63,48,61,14,40,77";
        assert_eq!(part3(input), "3");
    }
}
