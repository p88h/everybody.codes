use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

pub fn part1(input: &str) -> String {
    let nums = input.split(',').filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let mut tot = 0;
    for n in &nums {
        tot += 90 / n;
    }
    tot.to_string()
}

pub fn part2(input: &str) -> String {
    let mut nums = input.split(',').filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let mut tot = 1;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            tot *= i + 1;
            let mut k = i;
            while k < nums.len() {
                nums[k] -= 1;
                k += i + 1;
            }
        }
    }
    tot.to_string()
}

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b == BigUint::from(0u32) { a } else { gcd(b.clone(), a % b) }
}

fn lcm(v: Vec<u128>) -> BigUint {
    v.iter().fold(1.to_biguint().unwrap(), |acc, x| {
        &acc * x / gcd(acc, x.to_biguint().unwrap())
    })
}

pub fn part3(input: &str) -> String {
    let mut nums = input.split(',').filter_map(|s| s.parse::<i128>().ok()).collect::<Vec<i128>>();
    let mut spell = vec![];
    for i in 0..nums.len() {
        if nums[i] != 0 {
            spell.push((i + 1) as u128);
            let mut k = i;
            while k < nums.len() {
                nums[k] -= 1;
                k += i + 1;
            }
        }
    }
    // inverse of part1 but for 202520252025000 blocks
    // if spell is [a,b,c], then the answer is x/a + x/b + x/c = 202520252025
    // so 202520252025 / x = (1/a + 1/b + 1/c)
    let denom = lcm(spell.clone());
    let numer = spell.iter().map(|x| &denom / x).fold(BigUint::from(0u32), |acc, x| acc + x);
    // now invert
    let res1 = denom * 202520252025000u128.to_biguint().unwrap();
    // now increase until we find the right one
    let mut result = (res1 / numer).to_u128().unwrap();
    loop {
        let mut tot = 0;
        for n in &spell {
            tot += (result + 1) / n;
        }
        if tot > 202520252025000u128 {
            break;
        }
        result += 1;
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "1,2,3,5,9";
        assert_eq!(part1(input), "193");
    }
    #[test]
    fn test_part2() {
        let input = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2";
        assert_eq!(part2(input), "270");
    }

    #[test]
    fn test_part3() {
        let input = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2";
        assert_eq!(part3(input), "94439495762954");
    }
}
