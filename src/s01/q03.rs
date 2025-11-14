use crate::algo::math::*;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|s| s[2..].parse::<i32>().unwrap()).collect()
    }).collect()
}

pub fn part1(input: &str) -> String {
    let mut result = 0;
    for v in parse_input(input) {
        let (x, y) = (v[0] - 1, v[1] - 1);
        let z = x + y + 1;
        let dx = (x + 100) % z + 1;
        let dy = (y + z * 100 - 100) % z + 1;
        result += dx + 100 * dy;
    }
    result.to_string()
}

pub fn part2(input: &str) -> String {
    let mut remainders: Vec<i64> = Vec::new();
    let mut moduli: Vec<i64> = Vec::new();
    for v in parse_input(input) {
        let (x, y) = (v[0] - 1, v[1] - 1);
        let z = x + y + 1;
        moduli.push(z as i64);
        remainders.push(y as i64);
    }
    compute_crt(&remainders, &moduli).to_string()
}

pub fn part3(input: &str) -> String {
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("x=1 y=2\nx=2 y=3\nx=3 y=4\nx=4 y=4"), "1310");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("x=12 y=2\nx=8 y=4\nx=7 y=1\nx=1 y=5\nx=1 y=3"), "14");
    }
}
