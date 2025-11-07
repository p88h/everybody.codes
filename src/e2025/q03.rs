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
    for i in 0..nums.len()-1 {
        if nums[i+1] != nums[i] {
            total += nums[i]
        }
    }
    total += nums[nums.len()-1];    
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let nums = sorted_input(input);
    let mut total = 0;
    let mut cnt = 0;
    for i in 0..nums.len()-1 {
        if nums[i+1] != nums[i] {
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
    for i in 0..nums.len()-1 {
        if nums[i+1] != nums[i] {
            longest = longest.max(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }    
    longest += 1;
    longest.to_string()
}
