fn parse(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut values = line.split(' ').map(|kv| kv.split_once('=').unwrap().1.parse().unwrap());
            let (x, y) = (values.next().unwrap(), values.next().unwrap());
            (x, y, values.next().unwrap_or(1))
        })
        .collect()
}

/// sweep from top to bottom, adding edges on left/right, accumulate filled area
fn water(mut towers: Vec<(u64, u64, u64)>) -> u64 {
    towers.sort_unstable_by_key(|&(_, y, _)| std::cmp::Reverse(y));
    let (mut left, mut right) = (u64::MAX, 0);
    let below = towers.iter().skip(1).map(|&(_, y, _)| y).chain([0]);
    towers.iter().zip(below).fold(0, |total, (&(x, y, w), below)| {
        (left, right) = (left.min(x), right.max(x + w));
        total + (right - left) * (y - below)
    })
}

pub fn part1(input: &str) -> String {
    water(parse(input)).to_string()
}

pub fn part2(input: &str) -> String {
    water(parse(input)).to_string()
}

pub fn part3(input: &str) -> String {
    water(parse(input)).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
x=0 y=5
x=5 y=7
x=6 y=8
x=7 y=1
x=8 y=3
x=10 y=6
x=15 y=3
x=18 y=4";
        assert_eq!(part1(input), "96");
    }

    #[test]
    fn test_part2() {
        let input = "\
x=0 y=5 width=1
x=1 y=4 width=3
x=5 y=3 width=2
x=7 y=9 width=1
x=8 y=8 width=1
x=9 y=9 width=3
x=12 y=3 width=1
x=16 y=7 width=1
x=19 y=3 width=5
x=25 y=5 width=1
x=29 y=1 width=2
x=31 y=6 width=1";
        assert_eq!(part2(input), "205");
    }
}
