fn count_pairs(input: &str) -> Vec<usize> {
    let mut counts = vec![0; 26];
    let mut results = vec![0; 26];
    for c in input.chars() {
        if c.is_uppercase() {
            counts[c as usize - 'A' as usize] += 1;
        } else {
            let pos = c as usize - 'a' as usize;
            results[pos] += counts[pos];
        }
    }
    results
}

pub fn part1(input: &str) -> String {
    let results = count_pairs(input);
    format!("{}", results[0])
}

pub fn part2(input: &str) -> String {
    let results = count_pairs(input);
    format!("{}", results.iter().sum::<usize>())
}

pub fn part3(input: &str) -> String {
    let r: usize;
    let q: usize;
    let long_input: String;
    // ensure input is at least 1000 chars long
    if input.len() < 10 {
        long_input = input.repeat(1000);
        r = 1;
        q = 1;
    } else if input.len() < 100 {
        long_input = input.repeat(100);
        r = 8;
        q = 3;
    } else if input.len() < 1000 {
        long_input = input.repeat(10);
        r = 98;
        q = 3;
    } else {
        long_input = input.to_string();
        r = 998;
        q = 3;
    }
    let iiinput = long_input.as_bytes().repeat(q);
    let mut counts = vec![0; 26];
    for c in iiinput[0..1000].to_vec() {
        if c <= b'Z' {
            counts[(c - b'A') as usize] += 1;
        }
    }
    let mut results = vec![0; 26];
    for (i, c) in iiinput.iter().enumerate() {
        // add tent at position i+1000
        if i + 1000 < iiinput.len() {
            let d = iiinput[i + 1000];
            if d <= b'Z' {
                let idx = (d - b'A') as usize;
                counts[idx] += 1;
            }
        }
        if c >= &b'a' {
            let idx = (c - b'a') as usize;
            if i >= long_input.len() && i < iiinput.len() - long_input.len() {
                results[idx] += r * counts[idx];
            } else {
                results[idx] += counts[idx];
            }
        }
        // remove tent at position i-1000
        if i >= 1000 {
            let d = iiinput[i - 1000];
            if d <= b'Z' {
                let idx = (d - b'A') as usize;
                counts[idx] -= 1;
            }
        }
    }
    format!("{}", results.iter().sum::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "ABabACacBCbca";
        assert_eq!(part1(input), "5");
    }

    #[test]
    fn test_part2() {
        let input = "ABabACacBCbca";
        assert_eq!(part2(input), "11");
    }

    #[test]
    fn test_part3() {
        let input = "AABCBABCABCabcabcABCCBAACBCa";
        assert_eq!(part3(input), "3442321");
    }
}
