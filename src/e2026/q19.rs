use itertools::Itertools;
use rayon::prelude::*;

fn mix(c1: u8, c2: u8) -> u8 {
    match (c1.min(c2), c1.max(c2)) {
        (a, b) if a == b => a,
        (b'C', b'M') => b'B',
        (b'C', b'Y') => b'G',
        (b'M', b'Y') => b'R',
        _ => b'K',
    }
}

pub fn solve(input: &str, mapping: &[u8]) -> usize {
    let split = input.find("\n\n").unwrap();
    let width = input.find('\n').unwrap();
    let height = (split + 1) / (width + 1);
    let grid1 = input[..split].as_bytes();
    let grid2 = input[split + 2..].as_bytes();
    let mut counters = [0usize; 26];
    for i in 0..height {
        for j in 0..width {
            let c1 = grid1[i * (width + 1) + j];
            let mut c2 = grid2[i * (width + 1) + j];
            if (b'1'..=b'4').contains(&c2) {
                c2 = mapping[(c2 - b'1') as usize];
            }
            counters[(mix(c1, c2) - b'A') as usize] += 1;
        }
    }
    counters.sort_unstable_by(|a, b| b.cmp(a));
    counters[0] * counters[1] * counters[2]
}

pub fn part1(input: &str) -> String {
    solve(input, b"").to_string()    
}

pub fn part2(input: &str) -> String {
    // try every assignment of digits 1234 to a distinct colour of CMYK
    b"CMYK"
        .iter()
        .copied()
        .permutations(4)
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|mapping| solve(input, &mapping))
        .max()
        .unwrap()
        .to_string()
}

/// Largest `x0 * x1 * x2` over whole numbers with `x(U) <= cap[U]` for every subset `U`.
/// Start with the tightest subset of colours (lowest cap per colour), 
/// allocate its share, and repeat for the remaining colours.
fn best_product(cap: &[usize; 8]) -> usize {
    let mut low = [0usize; 3];
    let mut high = [0usize; 3];
    let mut done = 0usize;
    while done != 0b111 {
        // (extra cells, colours) of a subset of the remaining colours
        let share = |mask: usize| (cap[mask | done] - cap[done], mask.count_ones() as usize);
        let tightest = (1..8usize)
            .filter(|mask| mask & done == 0)
            .min_by(|&m1, &m2| {
                let ((c1, n1), (c2, n2)) = (share(m1), share(m2));
                (c1 * n2).cmp(&(c2 * n1)).then(n2.cmp(&n1))
            })
            .unwrap();
        let (cells, colours) = share(tightest);
        for k in (0..3).filter(|k| tightest >> k & 1 == 1) {
            low[k] = cells / colours;
            high[k] = cells.div_ceil(colours);
        }
        done |= tightest;
    }
    (0..8usize)
        .map(|up| -> [usize; 3] { std::array::from_fn(|k| if up >> k & 1 == 1 { high[k] } else { low[k] }) })
        .filter(|x| (1..8usize).all(|mask| (0..3).filter(|k| mask >> k & 1 == 1).map(|k| x[k]).sum::<usize>() <= cap[mask]))
        .map(|x| x[0] * x[1] * x[2])
        .max()
        .unwrap()
}

pub fn part3(input: &str) -> String {
    let mut supply = [0usize; 26];
    for b in input.bytes().filter(u8::is_ascii_uppercase) {
        supply[(b - b'A') as usize] += 1;
    }
    // result colours each grid colour can turn into (K can only stay K)
    let sources: Vec<(usize, Vec<u8>)> = (b'A'..=b'Z')
        .filter(|&c| supply[(c - b'A') as usize] > 0)
        .map(|c| {
            let reach = b"CMY".iter().map(|&p| mix(c, p)).unique().collect();
            (supply[(c - b'A') as usize], reach)
        })
        .collect();
    let outputs: Vec<u8> = sources.iter().flat_map(|(_, r)| r.iter().copied()).unique().collect();

    let mut best = 0;
    for trio in outputs.iter().copied().combinations(3) {
        // cap[mask]: cells that can end up in the colours of `mask`. 
        let mut cap = [0usize; 8];
        for (mask, cap) in cap.iter_mut().enumerate() {
            *cap = sources
                .iter()
                .filter(|(_, reach)| (0..3).any(|k| mask >> k & 1 == 1 && reach.contains(&trio[k])))
                .map(|(n, _)| n)
                .sum();
        }
        best = best.max(best_product(&cap));
    }
    best.to_string()
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
