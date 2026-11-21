use rayon::prelude::*;

/// number of matchstics needed for every digit and +-= chars
fn matchsticks(digit: u8) -> usize {
    match digit {
        b'0' | b'6' | b'9' | b'A' => 6,
        b'1' | b'+' | b'=' => 2,
        b'2' | b'3' | b'5' => 5,
        b'4' => 4,
        b'7' => 3,
        b'8' => 7,
        b'-' => 1,
        _ => 0,
    }
}

/// Evaluate expression in place and check equality of both sides
fn is_correct(line: &[u8], base: u8) -> bool {
    let (mut total, mut num, mut sign, mut side) = (0, 0, 1, 1);
    for &b in line {
        match b {
            b'0'..=b'9' => num = num * base as i64 + (b - b'0') as i64,
            b'A'..=b'Z' => num = num * base as i64 + (b - b'A' + 10) as i64,
            b'+' | b'-' | b'=' => {
                total += sign * num;
                num = 0;
                if b == b'=' {
                    side = -1;
                }
                sign = side * if b == b'-' { -1 } else { 1 };
            }
            _ => {}
        }
    }
    total + sign * num == 0
}

/// What a symbol becomes when one of its sticks is taken away.
fn take(s: u8) -> &'static [u8] {
    match s {
        b'6' => b"5",
        b'7' => b"1",
        b'8' => b"069A",
        b'9' => b"35",
        b'+' => b"-",
        _ => b"",
    }
}

/// What a symbol becomes when a stick is added to it.
fn put(s: u8) -> &'static [u8] {
    match s {
        b'0' | b'6' | b'9' | b'A' => b"8",
        b'1' => b"7",
        b'3' => b"9",
        b'5' => b"69",
        b'-' => b"+",
        _ => b"",
    }
}

/// What a symbol becomes when one of its own sticks is moved.
fn flip(s: u8) -> &'static [u8] {
    match s {
        b'0' => b"69A",
        b'2' => b"3",
        b'3' => b"25",
        b'5' => b"3",
        b'6' => b"09A",
        b'9' => b"06A",
        b'A' => b"069",
        _ => b"",
    }
}

/// Whether moving exactly one matchstick makes the equation correct (base 11).
fn fixable(line: &str) -> bool {
    let mut eq = line.as_bytes().to_vec();
    for i in 0..eq.len() {
        let old_i = eq[i];
        // Move a stick within symbol i.
        for &a in flip(old_i) {
            eq[i] = a;
            if is_correct(&eq, 11) {
                return true;
            }
        }
        // Take a stick from symbol i and put it on another symbol j.
        for &a in take(old_i) {
            eq[i] = a;
            for j in (0..eq.len()).filter(|&j| j != i) {
                let old_j = eq[j];
                for &b in put(old_j) {
                    eq[j] = b;
                    if is_correct(&eq, 11) {
                        return true;
                    }
                }
                eq[j] = old_j;
            }
        }
        eq[i] = old_i;
    }
    false
}

/// `n` written in `base` (digits above 9 use `A`, `B`, ...).
fn to_base(mut n: usize, base: usize) -> String {
    let mut digits = Vec::new();
    loop {
        let d = (n % base) as u8;
        digits.push(if d < 10 { b'0' + d } else { b'A' + d - 10 });
        n /= base;
        if n == 0 {
            break;
        }
    }
    digits.reverse();
    String::from_utf8(digits).unwrap()
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| is_correct(line.as_bytes(), 10))
        .flat_map(str::bytes)
        .map(matchsticks)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let total = input
        .lines()
        .filter(|line| is_correct(line.as_bytes(), 11))
        .flat_map(str::bytes)
        .map(matchsticks)
        .sum::<usize>();
    to_base(total, 11)
}

pub fn part3(input: &str) -> String {
    // A move keeps the stick count, so it doesn't matter whether we count the original or the fixed line.
    let total = input
        .par_lines()
        .filter(|line| is_correct(line.as_bytes(), 11) || fixable(line))
        .map(|line| line.bytes().map(matchsticks).sum::<usize>())
        .sum::<usize>();
    to_base(total, 11)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
5 - 4 = 9 - 8
2 + 1 = 3 + 0
6 + 7 = 4 + 9
1 + 2 = 3 + 4";
        assert_eq!(part1(input), "75");
    }

    #[test]
    fn test_part2() {
        let input = "\
10 + 2A = 13 + 27
98 - 54 = 76 - 32
10 + 12 = 30 - 9
20 - 10 = 5 + 5";
        assert_eq!(part2(input), "101");
    }

    #[test]
    fn test_part3() {
        let input = "\
4469 - 5934 = 1631 - 2AA7
8471 - 2276 = 4510 + 2736
A767 - 6104 = 1825 + 2538
9961 + 1140 = 5963 + 2974
9188 - 406A = 3598 + 1533
96A6 - 1606 = 4676 - 3524
9114 - 15A2 = 5258 + 2371
7023 - 2209 = 1433 + 3436
921A - 2696 = 2A91 + 3688
8492 - 2055 = 4186 + 3260
1234 + 4567 = 8901 + 2345";
        assert_eq!(part3(input), "693");
    }

}
