use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::ops::Range;
use std::sync::LazyLock;

fn pieces(len: usize, min: usize) -> usize {
    if len < min { 1 } else { rayon::current_num_threads() }
}

/// Flattened representation of the gaps and blocks of a colour in a string
struct Split {
    from: usize,
    to: usize,
    edges: Vec<u32>,
}

impl Split {
    /// The number of (gap, block) pairs.
    fn len(&self) -> usize {
        self.edges.len() / 2
    }

    fn gap(&self, j: usize) -> Range<usize> {
        let start = if j == 0 { self.from } else { self.edges[2 * j - 1] as usize };
        start..self.edges[2 * j] as usize
    }

    fn block(&self, j: usize) -> u64 {
        (self.edges[2 * j + 1] - self.edges[2 * j]) as u64
    }

    /// What's left after the last block.
    fn tail(&self) -> Range<usize> {
        self.edges.last().map_or(self.from, |&e| e as usize)..self.to
    }
}

/// The block edges of `s[from..to]`
fn split_range(s: &[u8], c: u8, from: usize, to: usize) -> Vec<u32> {
    let mut edges = vec![0; to - from + 1];
    let (mut n, mut in_block) = (0, false);
    for (i, &b) in s[from..to].iter().enumerate() {
        let is_c = b == c;
        edges[n] = (from + i) as u32;
        n += (is_c != in_block) as usize;
        in_block = is_c;
    }
    if in_block {
        edges[n] = to as u32;
        n += 1;
    }
    edges.truncate(n);
    edges
}

/// Runs split_range in parallel over `s[from..]`
fn split(s: &[u8], c: u8, from: usize) -> Split {
    let pieces = pieces(s.len() - from, 1 << 16);
    if pieces == 1 {
        return Split { from, to: s.len(), edges: split_range(s, c, from, s.len()) };
    }
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

    let parts: Vec<Vec<u32>> = cuts.par_windows(2).map(|w| split_range(s, c, w[0], w[1])).collect();
    Split { from, to: s.len(), edges: parts.concat() }
}

fn group(mut s: Vec<u8>) -> (u64, u64) {
    let (mut flips, mut flipped) = (0, 0);
    // s holds only what isn't grouped yet; its first run starts the next group
    while !s.is_empty() {
        let c = s[0];
        let n = s.iter().take_while(|&&b| b == c).count();
        let sp = split(&s, c, n);
        let m = sp.len();
        flips += m as u64;

        // Flip t moves one gap in front of the tail (from the front, reversed, when t is even;
        // else from the back) and one c block into the group (from the other end). Every flip
        // reverses all gaps and blocks still left, so each counts once per flip up to and
        // including the one that moves it: flip t adds (t + 1) times what it moves.
        // Chunks of flips run in parallel, each laying out its gaps last moved first.
        let chunk = m.div_ceil(pieces(s.len() - n, 1 << 16)).max(1);
        let parts: Vec<(Vec<u8>, u64)> = (0..m)
            .into_par_iter()
            .step_by(chunk)
            .map(|t0| {
                let (mut part, mut part_flipped) = (Vec::new(), 0);
                for t in (t0..(t0 + chunk).min(m)).rev() {
                    let (gap, block) = if t.is_multiple_of(2) {
                        part.extend(s[sp.gap(t / 2)].iter().rev());
                        (sp.gap(t / 2), sp.block(m - 1 - t / 2))
                    } else {
                        part.extend_from_slice(&s[sp.gap(m - 1 - t / 2)]);
                        (sp.gap(m - 1 - t / 2), sp.block(t / 2))
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
        rest.extend_from_slice(&s[sp.tail()]);
        s = rest;
    }
    (flips, flipped)
}

/// `unit` repeated `reps` times, made of `colours` distinct colours.
struct Repeated {
    unit: Vec<u8>,
    reps: usize,
    colours: usize,
}

impl Repeated {
    fn new(unit: Vec<u8>, reps: usize) -> Self {
        let mut seen = [false; 256];
        unit.iter().for_each(|&b| seen[b as usize] = true);
        let colours = seen.iter().filter(|&&s| s).count();
        Repeated { unit, reps, colours }
    }
}

/// Lays out gaps 0..k as rev(gap k-1), gap 0, rev(gap k-2), gap 1, ..., rev(gap 0), gap k-1.
fn zigzag<'a>(k: usize, gap: impl Fn(usize) -> &'a [u8] + Sync) -> Vec<u8> {
    let chunk = k.div_ceil(pieces(k, 1 << 12)).max(1);
    let parts: Vec<Vec<u8>> = (0..k)
        .into_par_iter()
        .step_by(chunk)
        .map(|i0| {
            let mut part = Vec::new();
            for i in i0..(i0 + chunk).min(k) {
                part.extend(gap(k - 1 - i).iter().rev());
                part.extend_from_slice(gap(i));
            }
            part
        })
        .collect();
    parts.concat()
}

/// One colour of `group` on a repeated string: returns `(flips, flipped)` and what's left, again
/// as a repeated string. Returns `None` when what's left can't be written that way.
fn group_step(r: &Repeated) -> Option<((u64, u64), Repeated)> {
    // what's left repeats with half the reps, so they must be even, unless nothing is left
    if r.reps % 2 == 1 && r.colours > 2 {
        return None;
    }
    let (n, len) = (r.reps as u64, r.unit.len() as u64);
    let c = r.unit[0];
    let a = r.unit.iter().take_while(|&&b| b == c).count();

    if r.unit[r.unit.len() - 1] == c {
        // The first flip reverses everything after the leading run: unit = c^a v becomes
        // c^a rev(v), which ends in a gap, as v starts with one.
        let mut unit = r.unit[..a].to_vec();
        unit.extend(r.unit[a..].iter().rev());
        let ((flips, flipped), rest) = group_step(&Repeated { unit, ..*r })?;
        return Some(((flips + 1, flipped + len * n - a as u64), rest));
    }

    // unit = X0 A0 X1 A1 ... X(k-1) A(k-1), with c runs Xi (X0 = c^a) and gaps Ai
    let sp = split(&r.unit, c, a);
    let k = sp.len() + 1;
    let x = |i: usize| if i == 0 { a as u64 } else { sp.block(i - 1) };
    let gap = |i: usize| &r.unit[if i < sp.len() { sp.gap(i) } else { sp.tail() }];

    // All copies but the leading X0 are blocks to flip. The flipped elements add up to a closed
    // form with even reps; with odd reps, which only happens for the last colour, each pair of
    // c run and gap from the other end also counts once per flip before the one that moves it.
    let flips = n * k as u64 - 1;
    let mut flipped = n / 2 * len * (k as u64 * (n + n % 2) - 1);
    if n % 2 == 1 {
        flipped += (0..k)
            .into_par_iter()
            .with_min_len(1 << 10)
            .map(|i| (x(i) + gap(k - 1 - i).len() as u64) * (2 * i).min(2 * (k - i) - 1) as u64)
            .sum::<u64>();
    }

    // with one colour left, it's all one group and nothing needs laying out
    if r.colours <= 2 {
        return Some(((flips, flipped), Repeated { unit: Vec::new(), reps: 0, colours: 1 }));
    }
    let unit = zigzag(k, gap);
    Some(((flips, flipped), Repeated { unit, reps: r.reps / 2, colours: r.colours - 1 }))
}

/// `group` on a repeated string, finishing on the plain string once there are too few repeats.
fn group_repeated(r: Repeated) -> (u64, u64) {
    if pieces(r.unit.len(), 1 << 16) == 1 {
        return group_repeated_in_pool(r);
    }
    // The steps only work on one unit, so their parallel pieces are small:
    // more threads would not improve performance significantly or would worsen it.
    static POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPoolBuilder::new().num_threads(4).build().unwrap());
    POOL.install(|| group_repeated_in_pool(r))
}

fn group_repeated_in_pool(mut r: Repeated) -> (u64, u64) {
    let (mut flips, mut flipped) = (0, 0);
    while r.colours > 1 {
        let Some(((f, fl), next)) = group_step(&r) else {
            let (f, fl) = group(r.unit.repeat(r.reps));
            return (flips + f, flipped + fl);
        };
        (flips, flipped) = (flips + f, flipped + fl);
        r = next;
    }
    (flips, flipped)
}

pub fn part1(input: &str) -> String {
    group(input.trim().as_bytes().to_vec()).0.to_string()
}

pub fn part2(input: &str) -> String {
    let unit = input.trim().as_bytes().to_vec();
    group_repeated(Repeated::new(unit, 10)).1.to_string()
}

pub fn part3(input: &str) -> String {
    let unit = input.trim().as_bytes().to_vec();
    group_repeated(Repeated::new(unit, 100)).1.to_string()
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
