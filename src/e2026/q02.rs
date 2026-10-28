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
    part3(input)
}

pub fn part3(input: &str) -> String {
    let mut total = 0u64;
    for line in input.lines() {
        let mut adj = [0usize; 10];
        let mut indeg = [0u32; 10];
        for tok in line.split_ascii_whitespace() {
            let (big, small) = parse(tok);
            adj[big] = adj[big] * 16 + small;
            indeg[small] += 1;
        }
        indeg[0] = 1; // sentinel
        let mut acc = 0u64;
        let mut top = indeg.iter().position(|&x| x == 0).unwrap();
        // Kaaaaaaaahn!
        while top > 0 {
            acc = acc * 10 + top as u64;
            let mut next = 0;
            while adj[top] != 0 {
                let child = adj[top] % 16;
                adj[top] /= 16;
                indeg[child] -= 1;
                if indeg[child] == 0 {
                    next = child;
                }
            }
            top = next;
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
