use rayon::prelude::*;

struct Pair {
    val: u128,
    sum: u128,
}

impl std::ops::Add for Pair {
    type Output = Pair;
    fn add(self, other: Pair) -> Pair {
        Pair { val: self.val + other.val, sum: self.sum + other.sum }
    }
}

fn eni(base: u64, exp: u64, modulus: u64, tail: usize) -> Pair {
    let mut r = 1u64;
    let mut v: Vec<u64> = Vec::new();
    let mut p: usize = 0;
    let mut last = vec![0; modulus as usize];
    let mut skip_sum = 0u64;
    while p < exp as usize {
        p += 1;
        r = (r * base) % modulus;
        if tail > 0 && last[r as usize] != 0 && skip_sum == 0 {
            let cycle = p - last[r as usize];
            let skip = (exp as usize - p - tail) / cycle;
            skip_sum = v[p - cycle - 1..p - 1].iter().sum::<u64>() * (skip as u64) + 1;
            p += skip * cycle;
        }
        last[r as usize] = p;
        v.push(r);
    }
    let tot_sum = v.iter().sum::<u64>() + skip_sum - 1;
    let take_count = if tail > 0 { tail } else { v.len() };
    let vv = v.iter().rev().take(take_count);
    let z = vv.fold(0u64, |acc, &x| {
        let digits = if x == 0 { 10 } else { x.ilog10() + 1 };
        acc * 10u64.pow(digits) + x
    });
    Pair { val: z as u128, sum: tot_sum as u128 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eni() {
        assert_eq!(eni(4, 3, 11, 0).val, 954);
        assert_eq!(eni(6, 8, 14, 0).val, 86868686);
        assert_eq!(eni(8, 6, 16, 0).val, 8);
        assert_eq!(eni(8580, 219136221, 54, 5).val, 0);
        assert_eq!(eni(2, 7, 5, 1).sum, 19);
        assert_eq!(eni(3, 8, 16, 1).sum, 48);
        assert_eq!(eni(4, 3000, 110, 1).sum, 132000);
        assert_eq!(eni(8, 16000, 160, 1).sum, 1279880);
    }
}

fn compute(line: &str, tail: usize) -> Pair {
    let ps: Vec<u64> = line.split(' ').map(|s| s[2..].parse::<u64>().unwrap()).collect();
    let (a, b, c, x, y, z, m) = (ps[0], ps[1], ps[2], ps[3], ps[4], ps[5], ps[6]);
    eni(a, x, m, tail) + eni(b, y, m, tail) + eni(c, z, m, tail)
}

pub fn part1(input: &str) -> String {
    input.lines().map(|line| compute(line, 0).val).max().unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    input.lines().map(|line| compute(line, 5).val).max().unwrap().to_string()
}

pub fn part3(input: &str) -> String {
    input.par_lines().map(|line| compute(line, 1).sum).max().unwrap().to_string()
}
