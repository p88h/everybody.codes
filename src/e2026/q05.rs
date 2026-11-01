// Flat grid with a 1-cell border of zero bytes, so neighbor lookups never need bounds checks.
struct Grid {
    d: Vec<u8>,
    h: usize,
    w: usize,
    s: usize,
}

impl Grid {
    fn rows(&self) -> impl Iterator<Item = usize> {
        let (s, h) = (self.s, self.h);
        (1..=h).map(move |i| i * s)
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

// Marks `seeds` with `mark`, then repeatedly marks cells accepted by `can_mark` that touch at least 5
// cells which are either `precounted` in the original grid or already marked. Returns the number of marked cells.
fn grow(g: &mut Grid, seeds: Vec<usize>, precounted: u8, mark: u8, can_mark: impl Fn(u8) -> bool) -> usize {
    let offsets = g.offsets();
    let mut count = count_neighbors(g, precounted);
    // the flag records whether the cell was `precounted` before being marked
    let mut frontier: Vec<(usize, bool)> = Vec::with_capacity(seeds.len());
    for idx in seeds {
        frontier.push((idx, g.d[idx] == precounted));
        g.d[idx] = mark;
    }
    let mut next = 0;
    while next < frontier.len() {
        let (idx, was_precounted) = frontier[next];
        next += 1;
        for &o in &offsets {
            let n = (idx as isize + o) as usize;
            // cells that were precounted already contribute to their neighbors
            if !was_precounted {
                count[n] += 1;
            }
            // border cells can never reach 5 (at most 3 real neighbors)
            if can_mark(g.d[n]) && count[n] >= 5 {
                frontier.push((n, g.d[n] == precounted));
                g.d[n] = mark;
            }
        }
    }
    frontier.len()
}

fn make_land(g: &mut Grid) -> usize {
    let center = (g.h / 2 + 1) * g.s + g.w / 2 + 1;
    grow(g, vec![center], b'1', b'#', |c| c != b'#')
}

pub fn part1(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g).to_string()
}

fn make_sand(g: &mut Grid) -> Vec<usize> {
    for c in g.d.iter_mut() {
        if *c != b'#' && *c != 0 {
            *c = b'.';
        }
    }
    let water = count_neighbors(g, b'.');
    let mut sand = Vec::new();
    for row in g.rows() {
        for idx in row + 1..row + 1 + g.w {
            if g.d[idx] == b'#' && water[idx] > 0 {
                sand.push(idx);
            }
        }
    }
    sand
}

pub fn part2(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g);
    let sand = make_sand(&mut g);
    grow(&mut g, sand, b'=', b'=', |c| c == b'#').to_string()
}

fn plant_trees(g: &mut Grid) {
    let s = g.s;
    for i in 2..g.h.saturating_sub(2) {
        for j in 2..g.w.saturating_sub(2) {
            let idx = (i + 1) * s + j + 1;
            if g.d[idx] != b'#' {
                continue;
            }
            // 5x5 block without its four corners
            let ok = (-2isize..=2).all(|di| {
                (-2isize..=2).all(|dj| {
                    let n = (idx as isize + di * s as isize + dj) as usize;
                    (di.abs() == 2 && dj.abs() == 2) || matches!(g.d[n], b'#' | b'@')
                })
            });
            if ok {
                g.d[idx] = b'@';
            }
        }
    }
}

// One simultaneous growth step: land with at least 4 tree neighbors and no sand neighbors becomes a tree.
fn grow_forest(g: &mut Grid) {
    let tree_count = count_neighbors(g, b'@');
    let sand_count = count_neighbors(g, b'=');
    for idx in 0..g.d.len() {
        if g.d[idx] == b'#' && tree_count[idx] >= 4 && sand_count[idx] == 0 {
            g.d[idx] = b'@';
        }
    }
}

pub fn part3(input: &str) -> String {
    let mut g = build_grid(input);
    make_land(&mut g);
    let sand = make_sand(&mut g);
    grow(&mut g, sand, b'=', b'=', |c| c == b'#');
    plant_trees(&mut g);
    grow_forest(&mut g);
    grow_forest(&mut g);
    g.d.iter().filter(|&&c| c == b'@').count().to_string()
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
