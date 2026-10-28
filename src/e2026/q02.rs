// numbers are single digits, so nodes are indices 0..10; returns (bigger, smaller)
fn parse(tok: &str) -> (usize, usize) {
    let b = tok.as_bytes();
    let (x, y) = ((b[0] - b'0') as usize, (b[2] - b'0') as usize);
    if b[1] == b'<' { (y, x) } else { (x, y) }
}

pub fn part1(input: &str) -> String {
    let mut total = 0u64;
    for line in input.lines() {
        // how many numbers are bigger than each one
        let mut above = [0usize; 10];
        let mut max = 0;
        for tok in line.split_ascii_whitespace() {
            let (big, small) = parse(tok);
            above[small] += 1;
            max = max.max(big).max(small);
        }
        let mut order = [0u64; 10];
        for i in 1..=max {
            order[above[i]] = i as u64;
        }
        total += order[..max].iter().fold(0, |acc, &d| acc * 10 + d);
    }
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let mut total = 0u64;
    for line in input.lines() {
        let mut above = [0usize; 10];
        let mut count = [0usize; 10];
        let mut max = 0;
        for tok in line.split_ascii_whitespace() {
            let (big, small) = parse(tok);
            above[small] += 1;
            count[big] += 1;
            count[small] += 1;
            max = max.max(big).max(small);
        }
        // find duplicates in above by value
        let mut seen = 0u16;
        let mut dupe_value = 0;
        for i in 1..=max {
            if seen & (1 << above[i]) != 0 {
                dupe_value = above[i];
            }
            seen |= 1 << above[i];
        }
        // there is one comparison missing, so count will be lower for two numbers.
        // above will be lower just for one number, so we can determine which is bigger
        // since the numbers in above need to be unique.
        let min_count = *count[1..=max].iter().min().unwrap();
        for i in 1..=max {
            if count[i] == min_count && above[i] == dupe_value {
                // need to adjust so above is unique
                above[i] += 1;
            }
        }
        // now the values are the positions, biggest first
        let mut order = [0u64; 10];
        for i in 1..=max {
            order[above[i]] = i as u64;
        }
        total += order[..max].iter().fold(0, |acc, &d| acc * 10 + d);
    }
    total.to_string()
}

pub fn part3(input: &str) -> String {
    let mut total = 0u64;
    for line in input.lines() {
        let mut adj = [0u16; 10]; // adj[big] = mask of smaller nodes
        let mut indeg = [0u8; 10];
        let mut max = 0;
        for tok in line.split_ascii_whitespace() {
            let (big, small) = parse(tok);
            max = max.max(big).max(small);
            // edge from the bigger to the smaller, so the sort yields biggest first
            if adj[big] & (1 << small) == 0 {
                adj[big] |= 1 << small;
                indeg[small] += 1;
            }
        }
        let mut stack = [0usize; 10];
        let mut sp = 0;
        // numbers are contiguous 1..=max
        for i in 1..=max {
            if indeg[i] == 0 {
                stack[sp] = i;
                sp += 1;
            }
        }
        // Kaaaaaaaahn!
        let mut acc = 0u64;
        while sp > 0 {
            sp -= 1;
            let node = stack[sp];
            acc = acc * 10 + node as u64;
            let mut m = adj[node];
            while m != 0 {
                let s = m.trailing_zeros() as usize;
                m &= m - 1;
                indeg[s] -= 1;
                if indeg[s] == 0 {
                    stack[sp] = s;
                    sp += 1;
                }
            }
        }
        total += acc;
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
4>1 2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 2<4 3>2 3<4 1>2 3>5 1<3 4>5";
        assert_eq!(part1(input), "122157");
    }

    #[test]
    fn test_part2() {
        let input = "\
2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5
2>1 3>2 1<5 4>5 2<5 3>1 4>3 2<4 5<3
5>1 5>2 4>1 3>2 3<4 1>2 3>5 1<3 4>5";
        assert_eq!(part2(input), "122157");
    }

    #[test]
    fn test_part3() {
        let input = "\
3>1 1>2 1<5 4<2 3>5
2>1 2<5 3>1 4>3 2<4 5<3
5>1 3<4 1>2 3>5";
        assert_eq!(part3(input), "122157");
    }

}
