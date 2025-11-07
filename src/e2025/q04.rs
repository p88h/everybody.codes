pub fn part1(input: &str) -> String {
    let gears = input.lines().filter_map(|line| line.parse::<i32>().ok()).collect::<Vec<i32>>();
    // convert 2025 rotations into teeth count
    let d= 2025 * gears[0];
    // convert to full rotations of the last gear
    let l = d / gears[gears.len()-1];
    l.to_string()
}

pub fn part2(input: &str) -> String {
    let gears = input.lines().filter_map(|line| line.parse::<i64>().ok()).collect::<Vec<i64>>();
    // convert 10000000000000 rotations into teeth count
    let d= 10000000000000  * gears[gears.len()-1];
    // convert to full rotations of the first gear
    let mut l = d / gears[0];
    if d % gears[0] != 0 {
        l += 1;
    }
    l.to_string()
}

pub fn part3(input: &str) -> String {
    let mut lines = input.lines();
    let first = lines.next().and_then(|line| line.parse::<i64>().ok()).unwrap_or(0);
    let mut t = first * 100;
    for line in lines {
        if line.contains('|') {
            let parts = line.split('|').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
            if parts[0] > parts[1] {
                let d = parts[0] / parts[1];
                t /= d;
            } else if parts[1] > parts[0] {
                let d = parts[1] / parts[0];
                t *= d;
            }
        } else {
            let num = line.parse::<i64>().unwrap_or(1);
            t /= num;
        }
    }
    t.to_string()
}
