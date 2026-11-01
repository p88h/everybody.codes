use std::ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not};

// A w x h grid of bits stored row-major, each row padded to a whole number of u64 words. 
// The padding is sized internally from `max_shift` = largest allowed horizontal shift
// Every operation clears the spare bits again. Derived grids inherit the shape.
#[derive(Clone)]
pub struct BitGrid {
    w: usize,
    h: usize,
    max_shift: usize,
    nw: usize,
    words: Vec<u64>,
}

impl BitGrid {
    pub fn new(w: usize, h: usize, max_shift: usize) -> Self {
        let nw = (w + max_shift + 1 + 63) / 64;
        BitGrid { w, h, max_shift, nw, words: vec![0; nw * h] }
    }

    // Sets the bit of every cell of `rows` (one byte slice per row) for which `pred` holds.
    pub fn from_rows<'a>(
        w: usize,
        max_shift: usize,
        rows: impl ExactSizeIterator<Item = &'a [u8]>,
        pred: impl Fn(u8) -> bool,
    ) -> Self {
        let mut g = BitGrid::new(w, rows.len(), max_shift);
        for (i, row) in rows.enumerate() {
            for (k, chunk) in row.chunks(64).enumerate() {
                g.words[i * g.nw + k] = chunk.iter().enumerate().fold(0, |m, (b, &c)| m | (pred(c) as u64) << b);
            }
        }
        g
    }

    // Builds a grid of the same shape from a per-word function of the word index.
    pub fn from_fn(&self, f: impl Fn(usize) -> u64) -> Self {
        self.with_words((0..self.words.len()).map(f).collect())
    }

    pub fn count(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn words(&self) -> &[u64] {
        &self.words
    }

    // Calls `f(row, col)` for every set bit, in row-major order.
    pub fn for_each_set(&self, mut f: impl FnMut(usize, usize)) {
        for (i, row) in self.words.chunks(self.nw).enumerate() {
            for (k, &word) in row.iter().enumerate() {
                let mut m = word;
                while m != 0 {
                    f(i, k * 64 + m.trailing_zeros() as usize);
                    m &= m - 1;
                }
            }
        }
    }

    // out[row][col] = self[row][col - k], zero filled; |k| must not exceed `max_shift`
    pub fn shift_h(&self, k: i32) -> Self {
        let m = k.unsigned_abs() as usize;
        assert!(m <= self.max_shift, "shift {k} exceeds max_shift {}", self.max_shift);
        let (q, r) = ((m / 64) as isize, (m % 64) as u32);
        let n = self.words.len() as isize;
        let get = |j: isize| if (0..n).contains(&j) { self.words[j as usize] } else { 0 };
        self.with_words(
            (0..n)
                .map(|i| {
                    if k >= 0 {
                        (get(i - q) << r) | if r > 0 { get(i - q - 1) >> (64 - r) } else { 0 }
                    } else {
                        (get(i + q) >> r) | if r > 0 { get(i + q + 1) << (64 - r) } else { 0 }
                    }
                })
                .collect(),
        )
    }

    // out[row][col] = self[row - r][col], zero filled
    pub fn shift_v(&self, r: isize) -> Self {
        let n = self.words.len() as isize;
        let off = r * self.nw as isize;
        self.with_words((0..n).map(|i| if (0..n).contains(&(i - off)) { self.words[(i - off) as usize] } else { 0 }).collect())
    }

    // The cell itself and its 8 neighbors.
    pub fn dilate(&self) -> Self {
        let row = &(self | &self.shift_h(1)) | &self.shift_h(-1);
        &(&row | &row.shift_v(1)) | &row.shift_v(-1)
    }

    // Same shape with new contents; clears any bits that landed outside the width.
    fn with_words(&self, words: Vec<u64>) -> Self {
        let mut g = BitGrid { w: self.w, h: self.h, max_shift: self.max_shift, nw: self.nw, words };
        let (full, used) = (g.w / 64, g.w % 64);
        let last = if used == 0 { 0 } else { (1u64 << used) - 1 };
        for row in g.words.chunks_mut(g.nw) {
            row[full] &= last;
            row[full + 1..].fill(0);
        }
        g
    }

    fn zip(&self, o: &Self, f: impl Fn(u64, u64) -> u64) -> Self {
        self.with_words(self.words.iter().zip(&o.words).map(|(&a, &b)| f(a, b)).collect())
    }
}

impl BitAnd for &BitGrid {
    type Output = BitGrid;
    fn bitand(self, o: &BitGrid) -> BitGrid {
        self.zip(o, |a, b| a & b)
    }
}

impl BitOr for &BitGrid {
    type Output = BitGrid;
    fn bitor(self, o: &BitGrid) -> BitGrid {
        self.zip(o, |a, b| a | b)
    }
}

impl BitXor for &BitGrid {
    type Output = BitGrid;
    fn bitxor(self, o: &BitGrid) -> BitGrid {
        self.zip(o, |a, b| a ^ b)
    }
}

impl Not for &BitGrid {
    type Output = BitGrid;
    fn not(self) -> BitGrid {
        self.with_words(self.words.iter().map(|w| !w).collect())
    }
}

impl BitOrAssign<&BitGrid> for BitGrid {
    fn bitor_assign(&mut self, o: &BitGrid) {
        for (a, &b) in self.words.iter_mut().zip(&o.words) {
            *a |= b;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Cells = Vec<Vec<bool>>;

    // xorshift, so the tests need no extra dependency
    fn random_cells(w: usize, h: usize, seed: u64) -> Cells {
        let mut x = seed | 1;
        (0..h)
            .map(|_| {
                (0..w)
                    .map(|_| {
                        x ^= x << 13;
                        x ^= x >> 7;
                        x ^= x << 17;
                        x & 3 != 0
                    })
                    .collect()
            })
            .collect()
    }

    fn make(cells: &Cells, max_shift: usize) -> BitGrid {
        let rows: Vec<Vec<u8>> = cells.iter().map(|r| r.iter().map(|&b| b as u8).collect()).collect();
        BitGrid::from_rows(cells[0].len(), max_shift, rows.iter().map(|r| r.as_slice()), |c| c == 1)
    }

    fn cells_of(g: &BitGrid) -> Cells {
        let mut out = vec![vec![false; g.w]; g.h];
        g.for_each_set(|i, j| out[i][j] = true);
        out
    }

    fn at(c: &Cells, i: isize, j: isize) -> bool {
        i >= 0 && j >= 0 && (i as usize) < c.len() && (j as usize) < c[0].len() && c[i as usize][j as usize]
    }

    fn assert_clean(g: &BitGrid) {
        let total: usize = cells_of(g).iter().flatten().filter(|&&b| b).count();
        assert_eq!(g.count(), total, "spare bits must stay zero");
    }

    const WIDTHS: [usize; 8] = [1, 5, 62, 63, 64, 65, 127, 130];

    #[test]
    fn roundtrip_and_count() {
        for (n, &w) in WIDTHS.iter().enumerate() {
            let c = random_cells(w, 7, 17 + n as u64);
            let g = make(&c, 2);
            assert_eq!(cells_of(&g), c);
            assert_eq!(g.count(), c.iter().flatten().filter(|&&b| b).count());
        }
    }

    #[test]
    fn for_each_set_is_row_major() {
        let g = make(&random_cells(70, 5, 3), 1);
        let mut seen = Vec::new();
        g.for_each_set(|i, j| seen.push((i, j)));
        assert!(seen.windows(2).all(|p| p[0] < p[1]));
    }

    #[test]
    fn shift_h_matches_naive() {
        for (n, &w) in WIDTHS.iter().enumerate() {
            let c = random_cells(w, 6, 100 + n as u64);
            let g = make(&c, 70);
            for k in [-70i32, -65, -64, -63, -2, -1, 0, 1, 2, 63, 64, 65, 70] {
                let s = g.shift_h(k);
                assert_clean(&s);
                let want: Cells = (0..6).map(|i| (0..w).map(|j| at(&c, i, j as isize - k as isize)).collect()).collect();
                assert_eq!(cells_of(&s), want, "w={w} k={k}");
            }
        }
    }

    #[test]
    fn shift_v_matches_naive() {
        let c = random_cells(100, 9, 5);
        let g = make(&c, 1);
        for r in [-10isize, -3, -1, 0, 1, 2, 8, 9, 12] {
            let s = g.shift_v(r);
            assert_clean(&s);
            let want: Cells = (0..9).map(|i| (0..100).map(|j| at(&c, i - r, j)).collect()).collect();
            assert_eq!(cells_of(&s), want, "r={r}");
        }
    }

    #[test]
    #[should_panic(expected = "exceeds max_shift")]
    fn shift_beyond_max_panics() {
        make(&random_cells(10, 2, 1), 2).shift_h(3);
    }

    #[test]
    fn row_shifts_do_not_leak_between_rows() {
        // a full row of ones, shifted right, must not bleed into the next row
        let c: Cells = vec![vec![true; 64], vec![false; 64]];
        let s = make(&c, 2).shift_h(2);
        assert_eq!(cells_of(&s)[1], vec![false; 64]);
        assert_eq!(s.count(), 62);
    }

    #[test]
    fn bit_operators_match_naive() {
        for (n, &w) in WIDTHS.iter().enumerate() {
            let (a, b) = (random_cells(w, 4, 40 + n as u64), random_cells(w, 4, 90 + n as u64));
            let (ga, gb) = (make(&a, 2), make(&b, 2));
            let check = |g: BitGrid, f: fn(bool, bool) -> bool| {
                assert_clean(&g);
                let want: Cells = (0..4).map(|i| (0..w).map(|j| f(a[i][j], b[i][j])).collect()).collect();
                assert_eq!(cells_of(&g), want, "w={w}");
            };
            check(&ga & &gb, |x, y| x & y);
            check(&ga | &gb, |x, y| x | y);
            check(&ga ^ &gb, |x, y| x ^ y);
            check(!&ga, |x, _| !x);
            let mut acc = ga.clone();
            acc |= &gb;
            check(acc, |x, y| x | y);
        }
    }

    #[test]
    fn not_keeps_spare_bits_clear() {
        let g = BitGrid::new(65, 3, 2);
        assert_eq!((!&g).count(), 65 * 3);
    }

    #[test]
    fn dilate_matches_naive() {
        for (n, &w) in WIDTHS.iter().enumerate() {
            let c = random_cells(w, 6, 7 + n as u64).iter().map(|r| r.iter().map(|&b| !b).collect()).collect::<Cells>();
            let d = make(&c, 1).dilate();
            assert_clean(&d);
            let want: Cells = (0..6)
                .map(|i| {
                    (0..w)
                        .map(|j| (-1..=1).any(|di| (-1..=1).any(|dj| at(&c, i + di, j as isize + dj))))
                        .collect()
                })
                .collect();
            assert_eq!(cells_of(&d), want, "w={w}");
        }
    }

    #[test]
    fn from_fn_clears_outside_width() {
        let g = BitGrid::new(10, 2, 2);
        assert_eq!(g.from_fn(|_| u64::MAX).count(), 20);
    }
}
