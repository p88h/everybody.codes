pub fn part1(input: &str) -> String {
    let gears = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    // convert 2025 rotations into teeth count
    let d = 2025 * gears[0];
    // convert to full rotations of the last gear
    let l = d / gears[gears.len() - 1];
    l.to_string()
}

pub fn part2(input: &str) -> String {
    let gears = input
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect::<Vec<i64>>();
    // convert 10000000000000 rotations into teeth count
    let d = 10000000000000 * gears[gears.len() - 1];
    // convert to full rotations of the first gear
    let mut l = d / gears[0];
    if d % gears[0] != 0 {
        l += 1;
    }
    l.to_string()
}

pub fn part3(input: &str) -> String {
    let mut lines = input.lines();
    let first = lines
        .next()
        .and_then(|line| line.parse::<i64>().ok())
        .unwrap_or(0);
    let mut t = first * 100;
    for line in lines {
        if line.contains('|') {
            let parts = line
                .split('|')
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<i64>>();
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("128\n64\n32\n16\n8"), "32400");
        assert_eq!(part1("102\n75\n50\n35\n13"), "15888");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("128\n64\n32\n16\n8"), "625000000000");
        assert_eq!(part2("102\n75\n50\n35\n13"), "1274509803922");
    }
    #[test]
    fn test_part3() {
        assert_eq!(part3("5\n5|10\n10|20\n5"), "400");
        assert_eq!(part3("5\n7|21\n18|36\n27|27\n10|50\n10|50\n11"), "6818");
    }
}
