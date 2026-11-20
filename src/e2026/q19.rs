use itertools::Itertools;

/// Colour produced by overlaying two colours.
fn mix(c1: u8, c2: u8) -> u8 {
    match (c1.min(c2), c1.max(c2)) {
        (a, b) if a == b => a,
        (b'C', b'M') => b'B',
        (b'C', b'Y') => b'G',
        (b'M', b'Y') => b'R',
        _ => b'K',
    }
}

/// How often each (grid colour, pattern symbol) pair occurs in the two grids.
fn pair_counts(input: &str) -> Vec<((u8, u8), usize)> {
    const SYMBOLS: &[u8; 16] = b"?KYM1234?C??????";
    let grids: Vec<u8> = input.bytes().map(|c| (c % 29) & 15).collect();
    let split = (input.len() - 2) / 2;
    let (grid, pattern) = (&grids[..split], &grids[split + 2..][..split]);
    let mut counts = [0usize; 256];
    for (&s1, &s2) in grid.iter().zip(pattern) {
        counts[(s1 << 4 | s2) as usize] += 1;
    }
    // only indices 1..=9 are used
    (1..=9)
        .cartesian_product(1..=9)
        .filter(|&(i, j)| counts[i << 4 | j] > 0)
        .map(|(i, j)| ((SYMBOLS[i], SYMBOLS[j]), counts[i << 4 | j]))
        .collect()
}

/// Product of the three largest colour counts, with pattern digits 1..=4 read through `mapping`.
fn score(pairs: &[((u8, u8), usize)], mapping: &[u8]) -> usize {
    let mut counters = [0usize; 26];
    for &((c1, c2), n) in pairs {
        let c2 = if (b'1'..=b'4').contains(&c2) { mapping[(c2 - b'1') as usize] } else { c2 };
        counters[(mix(c1, c2) - b'A') as usize] += n;
    }
    counters.sort_unstable_by(|a, b| b.cmp(a));
    counters[0] * counters[1] * counters[2]
}

pub fn part1(input: &str) -> String {
    score(&pair_counts(input), &[]).to_string()
}

pub fn part2(input: &str) -> String {
    let pairs = pair_counts(input);
    b"CMYK"
        .iter()
        .copied()
        .permutations(4)
        .map(|mapping| score(&pairs, &mapping))
        .max()
        .unwrap()
        .to_string()
}

pub fn part3(input: &str) -> String {
    let k = input.bytes().filter(|&b| b == b'K').count();
    let s = input.bytes().filter(|b| b"CMY".contains(b)).count();
    let thirds: usize = (0..3).map(|i| (s + i) / 3).product();
    let halves = s / 2 * s.div_ceil(2);
    thirds.max(k * halves).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

CMYKC
CMYKC
CMYKC
CMYKC
CMYKC";
        assert_eq!(part1(input), "144");
    }

    #[test]
    fn test_part2() {
        let input = "\
CCCCC
MMMMM
YYYYY
KKKKK
CCMYY

14231
14231
14231
14231
14231";
        assert_eq!(part2(input), "180");
    }

    #[test]
    fn test_part3() {
        let input = "\
CCMM
YYKK";
        assert_eq!(part3(input), "18");
    }

    #[test]
    fn test_part3_med() {
        let input = "\
CCCMMM
CCMMYY
CCCYKK
CMMYYK";
        assert_eq!(part3(input), "343");
    }

        #[test]
    fn test_part3_large() {
        let input = "\
CCCCYMMM
CCMMMYYY
CCCYYYMM
MMCCCYYY
YYYMMMCC
CCYYMMKK
MMYYCCKK
YYYCCMMM";
        assert_eq!(part3(input), "8000");
    }
}
