use rayon::prelude::*;

// Returns (number, weights); weights are already pre-computed with position-powers
fn parse(line: &str) -> (u64, Vec<u64>) {
    let (iso, rhs) = line.split_once(' ').unwrap();
    let iso: Vec<u64> = iso.split_once('-').unwrap().1.bytes().map(|b| u64::from(b - b'0')).collect();
    let weights = (0..rhs.len()).map(|k| iso[iso.len() - 1 - k % iso.len()].pow(k as u32)).collect();
    (rhs.parse().unwrap(), weights)
}

fn score(value: u64, weights: &[u64]) -> u64 {
    weights.iter().fold((0, value), |(sum, v), w| (sum + v % 10 * w, v / 10)).0
}

pub fn part1(input: &str) -> String {
    part2(input)
}

pub fn part2(input: &str) -> String {
    let total: u64 = input
        .lines()
        .map(|line| {
            let (number, weights) = parse(line);
            score(number, &weights)
        })
        .sum();
    total.to_string()
}

pub fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let total: u64 = lines
        .par_iter()
        .map(|line| {
            let (number, weights) = parse(line);
            let target = score(number, &weights);
            // only counts when no smaller number gives the same score
            if (0..number).any(|c| score(c, &weights) == target) { 0 } else { target }
        })
        .sum();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
ISO-3210 7654
ISO-8742 2468
ISO-1111 1111";
        assert_eq!(part1(input), "1478");
    }

    #[test]
    fn test_part2() {
        let input = "\
ISO-3210 7654
ISO-8742 2468
ISO-1 1111
ISO-13 777
ISO-34 140856
ISO-789 2465112
ISO-2341 945";
        assert_eq!(part2(input), "1161463");
    }

    #[test]
    fn test_part3() {
        // the first two are not valid
        let input = "\
ISO2-3 23
ISO2-61 149
ISO2-978 99
ISO2-5887 5923
ISO2-76988 123749
ISO2-435657 929977
ISO2-3949 870737";
        assert_eq!(part3(input), "50987");
    }
}
