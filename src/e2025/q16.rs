use num_bigint::{BigUint, ToBigUint};
use num_traits::cast::ToPrimitive;

fn spell_power(spell: &Vec<u64>, val: u64) -> u64 {
    let mut tot = 0;
    for n in spell {
        tot += val / n;
    }
    tot
}

pub fn part1(input: &str) -> String {
    let nums = input.split(',').filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<u64>>();
    spell_power(&nums, 90).to_string()
}

fn find_spell(input: &str) -> Vec<u64> {
    let mut nums = input.split(',').filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<u64>>();
    let mut ret = vec![];
    for i in 0..nums.len() {
        if nums[i] != 0 {
            ret.push((i + 1) as u64);
            let mut k = i;
            while k < nums.len() {
                nums[k] -= 1;
                k += i + 1;
            }
        }
    }
    ret
}

pub fn part2(input: &str) -> String {
    find_spell(input).iter().product::<u64>().to_string()
}

fn gcd(a: BigUint, b: BigUint) -> BigUint {
    if b == BigUint::from(0u32) { a } else { gcd(b.clone(), a % b) }
}

fn lcm(v: Vec<u64>) -> BigUint {
    v.iter().fold(1.to_biguint().unwrap(), |acc, x| {
        &acc * x / gcd(acc, x.to_biguint().unwrap())
    })
}

pub fn part3(input: &str) -> String {
    let spell = find_spell(input);
    // inverse of part1 but for 202520252025000 blocks
    // if spell is [a,b,c], then the answer is x/a + x/b + x/c = 202520252025
    // so 202520252025 / x = (1/a + 1/b + 1/c)    
    let denom = lcm(spell.clone());
    let numer = spell.iter().map(|x| &denom / x).fold(BigUint::from(0u64), |acc, x| acc + x);
    // now invert    
    let res1 = denom * 202520252025000u64.to_biguint().unwrap();
    let mut result = (res1 / numer).to_u64().unwrap();
    // now increase until we find the right one, this needs just a few steps
    let mut step = 4;
    loop {
        if spell_power(&spell, result + step) > 202520252025000u64 {
            if step > 1 {
                step /= 2;
                continue;
            }
            break;
        } 
        result += step;
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
