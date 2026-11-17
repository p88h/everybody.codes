use rayon::prelude::*;
use std::ops::Range;

// The number of parallel pieces to split the input into.
fn pieces(len: usize) -> usize {
    if len < 1 << 16 { 1 } else { rayon::current_num_threads() * 4 }
}

/// Splits `s[from..to]`, which starts with a gap, into gaps and the length of the c block after
/// each one. Also returns where the tail after the last block starts (`to` if there is none).
fn split_range(s: &[u8], c: u8, from: usize, to: usize) -> (Vec<Range<usize>>, Vec<u64>, usize) {
    let (mut gaps, mut blocks) = (Vec::new(), Vec::new());
    let mut j = from;
    loop {
        let gap_start = j;
        j += s[j..to].iter().take_while(|&&b| b != c).count();
        if j == to {
            return (gaps, blocks, gap_start);
        }
        let block_start = j;
        j += s[j..to].iter().take_while(|&&b| b == c).count();
        gaps.push(gap_start..block_start);
        blocks.push((j - block_start) as u64);
    }
}

/// Runs split_range in parallel over `s[from..]`; applying cuts in safe places.
fn split(s: &[u8], c: u8, from: usize) -> (Vec<Range<usize>>, Vec<u64>, usize) {
    let pieces = pieces(s.len() - from);
    let mut cuts = vec![from];
    for p in 1..pieces {
        let mut cut = (from + (s.len() - from) * p / pieces).max(cuts[cuts.len() - 1] + 1);
        while cut < s.len() && !(s[cut - 1] == c && s[cut] != c) {
            cut += 1;
        }
        if cut >= s.len() {
            break;
        }
        cuts.push(cut);
    }
    cuts.push(s.len());

    let parts: Vec<_> = cuts.par_windows(2).map(|w| split_range(s, c, w[0], w[1])).collect();
    let tail_start = parts.last().unwrap().2;
    let gaps = parts.par_iter().flat_map_iter(|p| p.0.iter().cloned()).collect();
    let blocks = parts.par_iter().flat_map_iter(|p| p.1.iter().copied()).collect();
    (gaps, blocks, tail_start)
}

fn group(mut s: Vec<u8>) -> (u64, u64) {
    let (mut flips, mut flipped) = (0, 0);
    // s holds only what isn't grouped yet; its first run starts the next group
    while !s.is_empty() {
        let c = s[0];
        let n = s.iter().take_while(|&&b| b == c).count();
        let (gaps, blocks, tail_start) = split(&s, c, n);
        let m = gaps.len();
        flips += m as u64;

        // Flip t moves one gap in front of the tail (from the front, reversed, when t is even;
        // else from the back) and one c block into the group (from the other end). Every flip
        // reverses all gaps and blocks still left, so each counts once per flip up to and
        // including the one that moves it: flip t adds (t + 1) times what it moves.
        // Chunks of flips run in parallel, each laying out its gaps last moved first.
        let chunk = m.div_ceil(pieces(tail_start - n)).max(1);
        let parts: Vec<(Vec<u8>, u64)> = (0..m)
            .into_par_iter()
            .step_by(chunk)
            .map(|t0| {
                let (mut part, mut part_flipped) = (Vec::new(), 0);
                for t in (t0..(t0 + chunk).min(m)).rev() {
                    let (gap, block) = if t.is_multiple_of(2) {
                        part.extend(s[gaps[t / 2].clone()].iter().rev());
                        (&gaps[t / 2], blocks[m - 1 - t / 2])
                    } else {
                        part.extend_from_slice(&s[gaps[m - 1 - t / 2].clone()]);
                        (&gaps[m - 1 - t / 2], blocks[t / 2])
                    };
                    part_flipped += (t as u64 + 1) * (gap.len() as u64 + block);
                }
                (part, part_flipped)
            })
            .collect();

        // the c blocks join the group; what's left is the parts, last chunk first, then the tail
        let mut rest = Vec::with_capacity(s.len());
        for (part, part_flipped) in parts.iter().rev() {
            rest.extend_from_slice(part);
            flipped += part_flipped;
        }
        rest.extend_from_slice(&s[tail_start..]);
        s = rest;
    }
    (flips, flipped)
}

pub fn part1(input: &str) -> String {
    group(input.trim().as_bytes().to_vec()).0.to_string()
}

pub fn part2(input: &str) -> String {
    group(input.trim().repeat(10).into_bytes()).1.to_string()
}

pub fn part3(input: &str) -> String {
    group(input.trim().repeat(100).into_bytes()).1.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRGGBBGRRGGBBG";
        assert_eq!(part1(input), "3");
        let input2 = "BBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGGBBRGGRRGBBRGGBRGBBRRBRRRBGGRRRBGBGG";
        assert_eq!(part1(input2), "26");
    }

    #[test]
    fn test_part2() {
        let input = "RRYGYBYG";
        assert_eq!(part2(input), "1053");
    }

    #[test]
    fn test_part3() {
        let input = "RRYGGYBBYG";
        assert_eq!(part3(input), "135154");
    }
}
