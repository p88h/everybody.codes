pub fn part1(input: &str) -> String {
    let nums = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut total = 0;
    for i in 1..nums.len() {
        if (nums[i] - nums[i - 1]).abs() == 16 {
            total += 1;
        }
    }
    total.to_string()
}

fn read_segments(input: &str) -> ([[u32; 256]; 256], Vec<i32>) {
    let nums = input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap() - 1)
        .collect::<Vec<i32>>();
    let mut raw: [[u32; 256]; 256] = [[0u32; 256]; 256];
    // input is 1-indexed but we want 0-indexed for array access
    for i in 1..nums.len() {
        let a = nums[i].min(nums[i - 1]) as usize;
        let b = nums[i].max(nums[i - 1]) as usize;
        raw[a][b] += 1;
    }
    (raw, nums)
}

fn build_cache(raw: &[[u32; 256]; 256]) -> [[u32; 256]; 256] {
    let mut stage1 = [[0u32; 256]; 256];
    // compute stage1: number of segments ending at b with start >= a
    for b in 1..256 {
        stage1[b - 1][b] = raw[b - 1][b];
        for a in (0..b - 1).rev() {
            stage1[a][b] = raw[a][b] + stage1[a + 1][b];
        }
    }
    // compute stage2: number of segments starting at >= a and ending at <= b
    let mut stage2 = [[0u32; 256]; 256];
    for a in 0..256 {
        for b in (a + 1)..256 {
            stage2[a][b] = stage2[a][b - 1] + stage1[a][b];
        }
    }
    stage2
}

fn count_intesections(cache: &[[u32; 256]; 256], raw: &[[u32; 256]; 256], a: usize, b: usize) -> u32 {
    let intersect_left = cache[0][b - 1] - cache[0][a] - cache[a][b - 1];
    let intersect_right = cache[a + 1][255] - cache[b][255] - cache[a + 1][b];
    intersect_left + intersect_right + raw[a][b]
}

pub fn part2(input: &str) -> String {
    let (raw, nums) = read_segments(input);
    let cache = build_cache(&raw);
    let mut total = 0;
    for i in 1..nums.len() {
        let a = nums[i].min(nums[i - 1]) as usize;
        let b = nums[i].max(nums[i - 1]) as usize;
        total += count_intesections(&cache, &raw, a, b) - raw[a][b];
    }
    (total / 2).to_string()
}

pub fn part3(input: &str) -> String {
    let (raw, _nums) = read_segments(input);
    let cache = build_cache(&raw);
    let mut max = 0;
    for a in 0..255 {
        for b in a + 1..256 {
            let total = count_intesections(&cache, &raw, a, b);
            if total > max {
                max = total;
            }
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "1,5,2,6,8,4,1,7,3,5,7,8,2";
        assert_eq!(part2(input), "21");
    }

    #[test]
    fn test_part3() {
        let input = "1,5,2,6,8,4,1,7,3,6";
        assert_eq!(part3(input), "7");
    }

    #[test]
    fn test_build_cache_single_segment() {
        let mut raw = [[0u32; 256]; 256];
        raw[5][10] = 3;
        let cache = build_cache(&raw);
        assert_eq!(cache[4][10], 3);
        assert_eq!(cache[4][11], 3);
        assert_eq!(cache[0][255], 3);
        assert_eq!(cache[5][11], 3);
        assert_eq!(cache[6][10], 0);
        assert_eq!(cache[5][9], 0);
    }

    #[test]
    fn test_build_cache_multiple_segments() {
        let mut raw = [[0u32; 256]; 256];
        raw[5][10] = 2;
        raw[7][9] = 1;
        raw[15][20] = 3;
        let cache = build_cache(&raw);
        assert_eq!(cache[4][11], 3);
        assert_eq!(cache[4][21], 6);
        assert_eq!(cache[14][21], 3);
    }

    #[test]
    fn test_build_cache_overlapping_intervals() {
        let mut raw = [[0u32; 256]; 256];
        raw[10][20] = 1;
        raw[15][25] = 1;
        raw[12][18] = 1;
        let cache = build_cache(&raw);
        assert_eq!(cache[9][26], 3);
        assert_eq!(cache[11][19], 1);
        assert_eq!(cache[9][21], 2);
    }

    #[test]
    fn test_build_cache_boundary_cases() {
        let mut raw = [[0u32; 256]; 256];
        raw[1][2] = 5;
        raw[254][255] = 7;
        let cache = build_cache(&raw);
        assert_eq!(cache[0][2], 5);
        assert_eq!(cache[0][3], 5);
        assert_eq!(cache[1][2], 5);
        assert_eq!(cache[253][255], 7);
        assert_eq!(cache[0][255], 12);
    }

    #[test]
    fn test_build_cache_accumulation() {
        let mut raw = [[0u32; 256]; 256];
        for i in 1..10 {
            raw[i][i + 1] = 1;
        }
        let cache = build_cache(&raw);
        assert_eq!(cache[0][11], 9);
        assert_eq!(cache[5][11], 5);
    }

    #[test]
    fn test_build_cache_same_start_different_end() {
        let mut raw = [[0u32; 256]; 256];
        raw[10][15] = 2;
        raw[10][20] = 3;
        raw[10][25] = 1;
        let cache = build_cache(&raw);
        assert_eq!(cache[9][26], 6);
        assert_eq!(cache[9][16], 2);
    }
}
