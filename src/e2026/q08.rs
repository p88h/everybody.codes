// Cells are stored mod 12: digits 1-9 become 0-8, 'a'-'c' become 9-11, and a growing
// plant wraps from 11 back to 0.
const GROWN: u8 = 9;
const CYCLE: u8 = 12;

// Hexagon stored in axial coordinates (r = input row, j = (col + row - middle) / 2), sliced into
// rows f = r + j. Every ray direction then moves to row f +- 1 (or f +- 2 for (+-1, +-1)), and within
// a row the lane g = r - f / 2 + middle / 2 shifts by at most one, so a whole step is a sweep over the
// rows with one 16-lane vector each. Lanes outside the hexagon hold PAD: it acts like a fully grown
// plant for the light (never energized, keeps the shadow at or below zero) but never changes.
const LANES: usize = 16;
const PAD: u8 = u8::MAX;
type Row = [u8; LANES];

#[derive(Clone)]
struct Hex {
    rows: Vec<Row>,
}

impl Hex {
    fn parse(input: &str) -> Hex {
        let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
        let middle = lines.len() / 2;
        let mut rows = vec![[PAD; LANES]; 4 * middle + 1];
        for (r, line) in lines.iter().enumerate() {
            for (col, &c) in line.iter().enumerate() {
                if c == b'.' {
                    continue;
                }
                let j = (col + r - middle) / 2;
                let f = r + j;
                let g = r + middle / 2 - f / 2;
                assert!(g < LANES, "hexagon too large for {LANES} lanes");
                rows[f][g] = c - b'1';
            }
        }
        Hex { rows }
    }

    fn grow(&mut self, (dr, dc): (isize, isize)) -> (usize, usize) {
        let mut counts = Counts::default();
        let rows = &mut self.rows;
        match (dr, dc) {
            (1, 1) => sweep_diagonal(rows.iter_mut().enumerate(), &mut counts),
            (-1, -1) => sweep_diagonal(rows.iter_mut().rev().enumerate(), &mut counts),
            // Within a row f, lanes shift by `EVEN` or `ODD` when moving on, depending on f's parity.
            (0, 2) => sweep::<0, -1>(rows.iter_mut().enumerate(), &mut counts),
            (1, -1) => sweep::<1, 0>(rows.iter_mut().enumerate(), &mut counts),
            (0, -2) => sweep::<1, 0>(rows.iter_mut().enumerate().rev(), &mut counts),
            (-1, 1) => sweep::<0, -1>(rows.iter_mut().enumerate().rev(), &mut counts),
            _ => unreachable!(),
        }
        counts.totals()
    }
}

#[derive(Default)]
struct Counts {
    energy: Row,
    grown: Row,
}

impl Counts {
    fn totals(&self) -> (usize, usize) {
        let sum = |a: &Row| a.iter().map(|&x| x as usize).sum::<usize>();
        (sum(&self.energy), sum(&self.grown))
    }
}

// (+-1, +-1) moves two rows per cell and keeps the lane, so rows of each parity form their own chain.
fn sweep_diagonal<'a>(rows: impl Iterator<Item = (usize, &'a mut Row)>, counts: &mut Counts) {
    let mut u = [[0; LANES]; 2];
    for (i, row) in rows {
        update_row(row, (i / 2) as u8, &mut u[i & 1], counts);
    }
}

// Rays that move one row per cell. `rows` yields (row index f, row) in sweep order; the position
// along the rays is the number of rows swept so far.
fn sweep<'a, const EVEN: isize, const ODD: isize>(
    rows: impl Iterator<Item = (usize, &'a mut Row)>,
    counts: &mut Counts,
) {
    let mut u = [0; LANES];
    for (k, (f, row)) in rows.enumerate() {
        update_row(row, k as u8, &mut u, counts);
        u = if f & 1 == 0 { shift_lanes::<EVEN>(u) } else { shift_lanes::<ODD>(u) };
    }
}

#[inline(always)]
fn shift_lanes<const BY: isize>(u: Row) -> Row {
    let mut out = [0; LANES];
    match BY {
        1 => out[1..].copy_from_slice(&u[..LANES - 1]),
        -1 => out[..LANES - 1].copy_from_slice(&u[1..]),
        _ => out = u,
    }
    out
}

// u is the running maximum of x = v + 1 + k over the ray so far (see grow above); k is the index of
// this row along the rays.
#[inline(always)]
fn update_row(row: &mut Row, k: u8, u: &mut Row, counts: &mut Counts) {
    for l in 0..LANES {
        let v = row[l];
        let young = v < GROWN;
        let x = if young { v + 1 + k } else { 0 };
        let energized = x > u[l];
        u[l] = u[l].max(x);
        let advance = energized | (v.wrapping_sub(GROWN) < CYCLE - GROWN);
        let next = v + advance as u8;
        row[l] = if next == CYCLE { 0 } else { next };
        counts.energy[l] += energized as u8;
        counts.grown[l] += (energized & (v == GROWN - 1)) as u8;
    }
}

pub fn part1(input: &str) -> String {
    Hex::parse(input).grow((1, 1)).0.to_string()
}

pub fn part2(input: &str) -> String {
    let original = Hex::parse(input);
    [(1, 1), (1, -1), (0, -2), (-1, 1), (-1, -1), (0, 2)]
        .iter()
        .map(|&direction| original.clone().grow(direction).0)
        .sum::<usize>()
        .to_string()
}

pub fn part3(input: &str) -> String {
    let mut hex = Hex::parse(input);
    let dirs = [(1, 1), (1, -1), (0, -2), (-1, -1), (-1, 1), (0, 2)];
    let mut all_grown = 0;
    let mut all_energy = 0;
    let mut steps = 0;

    while all_grown < 100_000 {
        let (energy, grown) = hex.grow(dirs[steps % dirs.len()]);
        all_energy += energy;
        all_grown += grown;
        steps += 1;
    }
    all_energy.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....1.9.6.1.7....
...9.2.3.3.8.1...
..7.8.3.5.1.4.3..
.4.6.7.4.1.1.8.9.
4.5.3.6.5.8.9.4.9
.9.5.1.5.6.2.8.9.
..7.7.6.4.6.2.4..
...9.3.8.3.8.4...
....1.6.6.2.8....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "32");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "187");
    }

    #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT), "899889");
    }
}
