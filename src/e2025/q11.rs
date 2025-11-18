use core::num;

pub fn part1(input: &str) -> String {
    let mut nums = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut round = 0;
    let mut moved = true;
    while moved && round < 10 {
        moved = false;
        for i in 0..nums.len()-1 {
            if nums[i] > nums[i+1] {
                nums[i] -= 1;
                nums[i+1] += 1;
                moved = true;
            }
        }
        if moved {  
            round += 1;
        }
    }
    moved = true;
    while moved && round < 10 {
        moved = false;
        for i in 0..nums.len()-1 {
            if nums[i] < nums[i+1] {
                nums[i+1] -= 1;
                nums[i] += 1;
                moved = true;
            }
        }
        if moved {  
            round += 1;
        }
    }
    let checksum = nums.iter().enumerate().map(|(i, &n)| (i as i64 + 1) * n).sum::<i64>();
    checksum.to_string()
}

struct Segment {
    cnt: usize,
    height: i64,
}

pub fn part2(input: &str) -> String {
    let nums = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut done : Vec<Segment> = vec![];
    let mut start = 0;
    while start < nums.len() -1 {
        let mut end = start;
        let mut sum = nums[start];
        while end < nums.len()-1 && nums[end] >= nums[end + 1] {
            end += 1;
            sum += nums[end];
        }
        // average height of the new segment
        let mut len = end - start + 1;
        while done.len() > 0 && done.last().unwrap().height >= sum / len as i64 {
            // need to merge with previous segment
            let last = done.pop().unwrap();
            sum += last.height * last.cnt as i64;
            len += last.cnt;
        }
        let rem = sum as usize % len;
        let cnt_left = len - rem;
        let height = sum / len as i64;
        // we want all segments to be of exactly equal height for later processing
        done.push(Segment { cnt: cnt_left, height });
        if rem > 0 {
            done.push(Segment { cnt: rem, height: height + 1 });
        }
        start = end + 1;
    }
    // compute target height based on segmentation
    let nums2  = done.iter().flat_map(|s| vec![s.height; s.cnt]).collect::<Vec<i64>>();    
    let mut max_gap = 0;
    let mut carry = 0;
    // shift ducks over to the new heights
    for i in 0..nums2.len() {
        let diff = nums[i] - nums2[i] + carry;
        max_gap = max_gap.max(diff);
        carry = diff;
    }
    // then shift left to the global average
    let global_avg = nums.iter().sum::<i64>() / nums.len() as i64;
    for i in 0..nums2.len() {
        if nums2[i] < global_avg {
            max_gap += global_avg - nums2[i];
        }
    }
    max_gap.to_string()
}

pub fn part3(input: &str) -> String {
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3() {
        assert_eq!(part3("9\n1\n1\n4\n9\n6"), "11");
        assert_eq!(part3("805\n706\n179\n48\n158\n150\n232\n885\n598\n524\n423"), "1579");
    }
    
}
