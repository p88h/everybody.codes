pub fn part1(input: &str) -> String {
    let bolts = "RGB".as_bytes();
    let mut pos = 0;
    let mut used = 0;
    for baloon in input.as_bytes() {
        if baloon != &bolts[pos % 3] {
            pos += 1;
            used = 0;
        } else {
            used = 1;
        }
    }
    (pos + used).to_string()
}

fn carnival(input: &str, rep: usize) -> usize{
    let mut left = input.as_bytes().repeat(rep).to_vec();
    left.reserve(input.len() * rep);
    let right = input.as_bytes().repeat(rep).to_vec();    
    let bolts = "RGB".as_bytes();
    let mut cur = 0;
    let (mut lp, mut rp, mut ls, mut rs) = (0, 0, left.len(), right.len());
    while ls + rs > 0 {
        // uneven - pop one from the left side
        if ls != rs {
            lp += 1;
            ls -= 1;
        } else if left[lp] == bolts[cur % 3] {
            lp += 1;
            rp += 1;
            ls -= 1;
            rs -= 1;            
        } else {
            lp += 1;
            left.push(right[rp]);
            rp += 1;
            rs -= 1;
        }
        cur += 1;
    }
    cur
}

pub fn part2(input: &str) -> String {
    carnival(input, 50).to_string()
}

pub fn part3(input: &str) -> String {
    carnival(input, 50000).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("GRBGGGBBBRRRRRRRR"), "7");
        assert_eq!(part1("R"), "1");
        assert_eq!(part1("RRR"), "1");
        assert_eq!(part1("G"), "1");
        assert_eq!(part1("GGG"), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG"), "2955");
    }
}