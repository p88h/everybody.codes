use rayon::prelude::*;
use std::ops::Range;

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

/// `unit` repeated `reps` times, followed by `tail`, made of `colours` distinct colours.
struct Repeated {
    unit: Vec<u8>,
    reps: usize,
    tail: Vec<u8>,
    colours: usize,
}

impl Repeated {
    fn new(unit: Vec<u8>, reps: usize, tail: Vec<u8>) -> Self {
        let mut seen = [false; 256];
        unit.iter().chain(&tail).for_each(|&b| seen[b as usize] = true);
        let colours = seen.iter().filter(|&&s| s).count();
        Repeated { unit, reps, tail, colours }
    }

    fn flatten(&self) -> Vec<u8> {
        let mut s = self.unit.repeat(self.reps);
        s.extend_from_slice(&self.tail);
        s
    }
}

/// Lays out the gaps moved by flips `ts` out of `m`
fn lay_out<'a>(ts: Range<usize>, m: usize, gap: impl Fn(usize) -> &'a [u8] + Sync) -> Vec<u8> {
    let chunk = ts.len().div_ceil(pieces(ts.len(), 1 << 12)).max(1);
    let mut parts: Vec<Vec<u8>> = ts
        .clone()
        .into_par_iter()
        .step_by(chunk)
        .map(|t0| {
            let mut part = Vec::new();
            for t in (t0..(t0 + chunk).min(ts.end)).rev() {
                if t % 2 == 0 {
                    part.extend(gap(t / 2).iter().rev());
                } else {
                    part.extend_from_slice(gap(m - 1 - t / 2));
                }
            }
            part
        })
        .collect();
    parts.reverse();
    parts.concat()
}

/// One colour of `group` on a repeated string: returns `(flips, flipped)` and the next step.
fn group_step(r: &Repeated) -> Option<((u64, u64), Repeated)> {
    // the unit starts with the run that is grouped next: unit = c^a v
    let c = r.unit[0];
    let a = r.unit.iter().take_while(|&&b| b == c).count();
    if a == r.unit.len() {
        let tail = [&[c], &r.tail[..]].concat();
        return Some(((0, 0), Repeated::new(Vec::new(), 0, tail)));
    }
    if r.reps < 2 {
        return None;
    }

    // Without the leading c^a, the rest is (v c^a)^(reps - 1) v tail. 
    // Each copy of w = v c^a has the pairs of v, except that the last block 
    // (which also holds the next copy's c^a). 
    // if v ends in a gap instead, that gap gets a block of just c^a.
    let vs = split(&r.unit, c, a);
    let v_end = vs.tail();
    let q = vs.len() + usize::from(!v_end.is_empty());
    let w_gap = |j: usize| if j < vs.len() { vs.gap(j) } else { v_end.clone() };
    let w_block = |j: usize| {
        let block = if j < vs.len() { vs.block(j) } else { 0 };
        if j == q - 1 { block + a as u64 } else { block }
    };

    // The final copy v tail starts with the same q - 1 pairs. 
    // Its last pair runs into the tail so only that stretch is copied.    
    let joint_end = match r.tail.iter().position(|&b| b == c) {
        Some(i) => i + r.tail[i..].iter().take_while(|&&b| b == c).count(),
        None => r.tail.len(),
    };
    let joint = [&r.unit[w_gap(q - 1).start..], &r.tail[..joint_end]].concat();
    let (js, ts) = (split(&joint, c, 0), split(&r.tail, c, joint_end));

    // gaps and blocks 0..periodic come from the copies of w, 
    // then the q - 1 + joint + tail pairs of the final copy
    let periodic = (r.reps - 1) * q;
    let (in_joint, in_tail) = (q - 1, q - 1 + js.len());
    let m = periodic + in_tail + ts.len();
    let p = m - periodic;
    let gap = |j: usize| {
        let k = if j < periodic { j % q } else { j - periodic };
        if j < periodic || k < in_joint {
            &r.unit[w_gap(k)]
        } else if k < in_tail {
            &joint[js.gap(k - in_joint)]
        } else {
            &r.tail[ts.gap(k - in_tail)]
        }
    };
    let block = |j: usize| {
        let k = if j < periodic { j % q } else { j - periodic };
        if j < periodic || k < in_joint {
            w_block(k)
        } else if k < in_tail {
            js.block(k - in_joint)
        } else {
            ts.block(k - in_tail)
        }
    };

    // As in `group`, flip t moves gap t/2 (reversed) and block m-1-t/2 when t is even, else gap
    // m-1-t/2 and block t/2. From flip 2p on, both come from copies of w as long as the middle
    // does, so flips t and t + 2q move the same sizes.
    if (m - 1) / 2 >= periodic || m < 2 * p + 2 * q {
        return None;
    }
    let size = |t: usize| {
        let (g, b) = if t % 2 == 0 { (t / 2, m - 1 - t / 2) } else { (m - 1 - t / 2, t / 2) };
        gap(g).len() as u64 + block(b)
    };

    // flip t adds (t + 1) times what it moves
    let mut flipped: u64 = (0..2 * p).into_par_iter().with_min_len(1 << 10).map(|t| (t as u64 + 1) * size(t)).sum();
    flipped += (2 * p..2 * p + 2 * q)
        .into_par_iter()
        .with_min_len(1 << 10)
        .map(|t| {
            // flips t, t + 2q, ... below m: Σ (t' + 1) is an arithmetic series
            let count = ((m - 1 - t) / (2 * q) + 1) as u64;
            size(t) * (count * (t as u64 + 1) + (2 * q) as u64 * count * (count - 1) / 2)
        })
        .sum::<u64>();

    // with one colour left, it's all one group and nothing needs laying out
    if r.colours <= 2 {
        let done = Repeated { unit: Vec::new(), reps: 0, tail: Vec::new(), colours: 1 };
        return Some(((m as u64, flipped), done));
    }

    // The last 2q flips form the new unit, which repeats for as many whole periods as there are
    // flips from 2p on; the remaining flips go to the tail.
    let reps = (m - 2 * p) / (2 * q);
    let mut tail = lay_out(0..m - reps * 2 * q, m, gap);
    tail.extend_from_slice(&joint[js.tail()]);
    tail.extend_from_slice(&r.tail[ts.tail()]);
    let unit = lay_out(m - 2 * q..m, m, gap);
    Some(((m as u64, flipped), Repeated { unit, reps, tail, colours: r.colours - 1 }))
}

/// `group` on a repeated string, finishing on the plain string once there are too few repeats.
fn group_repeated(r: Repeated) -> (u64, u64) {
    // Run on a rayon worker when there is parallel work
    if pieces(r.unit.len() + r.tail.len(), 1 << 16) == 1 {
        group_repeated_in_pool(r)
    } else {
        rayon::scope(|_| group_repeated_in_pool(r))
    }
}

fn group_repeated_in_pool(mut r: Repeated) -> (u64, u64) {
    let (mut flips, mut flipped) = (0, 0);
    while r.colours > 1 && r.reps > 0 && !r.unit.is_empty() {
        let Some(((f, fl), next)) = group_step(&r) else { break };
        (flips, flipped) = (flips + f, flipped + fl);
        r = next;
    }
    if r.colours <= 1 {
        return (flips, flipped);
    }
    let (f, fl) = group(r.flatten());
    (flips + f, flipped + fl)
}

pub fn part1(input: &str) -> String {
    group(input.trim().as_bytes().to_vec()).0.to_string()
}

pub fn part2(input: &str) -> String {
    let unit = input.trim().as_bytes().to_vec();
    group_repeated(Repeated::new(unit, 10, Vec::new())).1.to_string()
}

pub fn part3(input: &str) -> String {
    let unit = input.trim().as_bytes().to_vec();
    group_repeated(Repeated::new(unit, 100, Vec::new())).1.to_string()
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
