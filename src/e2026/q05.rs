use crate::algo::bitplane::BitGrid;

// Flat grid with a 1-cell border of zero bytes, so neighbor lookups never need bounds checks.
struct Grid {
    d: Vec<u8>,
    h: usize,
    w: usize,
    s: usize,
}

impl Grid {
    fn rows(&self) -> impl ExactSizeIterator<Item = usize> {
        let (s, h) = (self.s, self.h);
        (1..h + 1).map(move |i| i * s)
    }

    fn row_slices(&self) -> impl ExactSizeIterator<Item = &[u8]> {
        self.rows().map(|r| &self.d[r + 1..r + 1 + self.w])
    }

    fn offsets(&self) -> [isize; 8] {
        let s = self.s as isize;
        [-s - 1, -s, -s + 1, -1, 1, s - 1, s, s + 1]
    }
}

fn build_grid(input: &str) -> Grid {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (h, w) = (lines.len(), lines[0].len());
    let s = w + 2;
    let mut d = vec![0u8; s * (h + 2)];
    for (i, line) in lines.iter().enumerate() {
        d[(i + 1) * s + 1..(i + 1) * s + 1 + w].copy_from_slice(line);
    }
    Grid { d, h, w, s }
}

fn count_neighbors(g: &Grid, what: u8) -> Vec<u8> {
    let s = g.s;
    let mut count = vec![0u8; g.d.len()];
    for (idx, &c) in g.d.iter().enumerate() {
        if c != what {
            continue;
        }
        // stamp the 3x3 block (the border guarantees it is in range), then remove the cell itself
        for base in [idx - s - 1, idx - 1, idx + s - 1] {
            for n in &mut count[base..base + 3] {
                *n += 1;
            }
        }
        count[idx] -= 1;
    }
    count
}

// plant looks two columns to either side
const MAX_SHIFT: usize = 2;
const ELIGIBLE: u8 = 0x80;

// Per-cell growth state in one byte: the count of `precounted` neighbors in the low bits, plus the
// ELIGIBLE flag for cells accepted by `can_mark`. Keeping both in one array keeps the hot loop cache-friendly.
fn init_state(g: &Grid, precounted: u8, can_mark: impl Fn(u8) -> bool) -> Vec<u8> {
    let mut state = count_neighbors(g, precounted);
    for (st, &c) in state.iter_mut().zip(&g.d) {
        // zero bytes are the border, which must never be marked
        if c != 0 && can_mark(c) {
            *st |= ELIGIBLE;
        }
    }
    state
}

// Marks `seeds` with `mark`, then repeatedly marks ELIGIBLE cells that touch at least 5 cells which are
// either `precounted` in the original grid or already marked. Returns the number of marked cells.
fn grow(g: &mut Grid, mut state: Vec<u8>, seeds: Vec<usize>, precounted: u8, mark: u8) -> usize {
    let offsets = g.offsets();
    // the flag records whether the cell was `precounted` before being marked
    let mut frontier: Vec<(usize, bool)> = Vec::with_capacity(g.d.len());
    for idx in seeds {
        frontier.push((idx, g.d[idx] == precounted));
        g.d[idx] = mark;
        state[idx] &= !ELIGIBLE;
    }
    let mut next = 0;
    while next < frontier.len() {
        let (idx, was_precounted) = frontier[next];
        next += 1;
        // cells that were precounted already contribute to their neighbors
        let inc = !was_precounted as u8;
        for &o in &offsets {
            let n = (idx as isize + o) as usize;
            state[n] += inc;
            if state[n] >= ELIGIBLE + 5 {
                state[n] &= !ELIGIBLE;
                frontier.push((n, g.d[n] == precounted));
                g.d[n] = mark;
            }
        }
    }
    frontier.len()
}

fn make_land(g: &mut Grid) -> usize {
    let center = (g.h / 2 + 1) * g.s + g.w / 2 + 1;
    let state = init_state(g, b'1', |c| c != b'#');
    grow(g, state, vec![center], b'1', b'#')
}

pub fn part1(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g).to_string()
}

// Land cells (plane `land`) whose 5x5 neighborhood, minus corners, is entirely land.
fn plant(land: &BitGrid) -> BitGrid {
    let h3 = &(land & &land.shift_h(1)) & &land.shift_h(-1);
    let h5 = &(&h3 & &land.shift_h(2)) & &land.shift_h(-2);
    let wide = &(&h5 & &h5.shift_v(1)) & &h5.shift_v(-1);
    let tall = &(&(&h3 & &h3.shift_v(1)) & &h3.shift_v(-1)) & &(&h3.shift_v(2) & &h3.shift_v(-2));
    &(land & &wide) & &tall
}

// One simultaneous growth step: land cells with at least 4 tree neighbors and no sand neighbors become trees.
fn forest_step(trees: &mut BitGrid, cand: &mut BitGrid, nosand: &BitGrid) {
    let (l, r) = (trees.shift_h(1), trees.shift_h(-1));
    // horizontal 3-sums as 2-bit numbers (s1 s0); the cell itself is never a tree for candidates
    let (t, lw, rw) = (trees.words(), l.words(), r.words());
    let s0 = trees.from_fn(|i| t[i] ^ lw[i] ^ rw[i]);
    let s1 = trees.from_fn(|i| (t[i] & lw[i]) | (t[i] & rw[i]) | (lw[i] & rw[i]));
    let (p0, n0) = (s0.shift_v(1), s0.shift_v(-1));
    let (p1, n1) = (s1.shift_v(1), s1.shift_v(-1));
    let (s0, s1, p0, n0, p1, n1) = (s0.words(), s1.words(), p0.words(), n0.words(), p1.words(), n1.words());
    // total = bit0 + 2 * (p1 + s1 + n1 + carry): at least 4 iff two of those weight-2 bits are set
    let ge2 = trees.from_fn(|i| {
        let carry = (p0[i] & s0[i]) | (p0[i] & n0[i]) | (s0[i] & n0[i]);
        let (a, b, c, d) = (p1[i], s1[i], n1[i], carry);
        (a & b) | (c & d) | ((a | b) & (c | d))
    });
    let new = &(&*cand & &ge2) & nosand;
    *trees |= &new;
    *cand = &*cand & &!&new;
}

fn make_sand(g: &Grid) -> Vec<usize> {
    let land = BitGrid::from_rows(g.w, MAX_SHIFT, g.row_slices(), |c| c == b'#');
    let water = BitGrid::from_rows(g.w, MAX_SHIFT, g.row_slices(), |c| c != b'#').dilate();
    let mut sand = Vec::new();
    (&land & &water).for_each_set(|i, j| sand.push((i + 1) * g.s + 1 + j));
    sand
}

fn grow_sand(g: &mut Grid, sand: Vec<usize>) -> usize {
    let state = init_state(g, b'=', |c| c == b'#');
    grow(g, state, sand, b'=', b'=')
}

pub fn part2(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g);
    let sand = make_sand(&g);
    grow_sand(&mut g, sand).to_string()
}

pub fn part3(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g);
    let sand = make_sand(&g);
    grow_sand(&mut g, sand);
    let land = BitGrid::from_rows(g.w, MAX_SHIFT, g.row_slices(), |c| c == b'#');
    let mut trees = plant(&land);
    let nosand = !&BitGrid::from_rows(g.w, MAX_SHIFT, g.row_slices(), |c| c == b'=').dilate();
    let mut cand = &land & &!&trees;
    forest_step(&mut trees, &mut cand, &nosand);
    forest_step(&mut trees, &mut cand, &nosand);
    trees.count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
010100100100001101000
111101001000001110000
101111011000101010011
001000101010000100000
000110100110011001100
000001110111111000000
101100010101011110000
001101100000001010011
001001001111010001001
100011011110011010101
001100101110010010000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), "137");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), "67");
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(EXAMPLE), "14");
    }
}
