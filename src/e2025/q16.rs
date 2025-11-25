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

fn inverse_spell(spell: &Vec<u64>) -> u64 {
    // inverse of part1 but for 202520252025000 blocks
    // if spell is [a,b,c], then the answer is x/a + x/b + x/c = 202520252025
    // so 202520252025 / x = (1/a + 1/b + 1/c)
    // the input numbers are small-ish, so we use a similar size denominator.
    // Integer math requires big integers here, and is not precise anyway, so we use f64
    let denom = 1000.0_f64;
    let numer = spell.iter().map(|x| denom / (*x as f64)).sum::<f64>();
    // now invert    
    let res1 = denom * 202520252025000u64 as f64;
    (res1 / numer) as u64
}

pub fn part3(input: &str) -> String {
    let spell = find_spell(input);
    // now increase until we find the right one, this needs just a few steps
    let mut result = inverse_spell(&spell);
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
