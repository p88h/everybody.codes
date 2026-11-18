use super::q14::for_each_adjacent;
use std::collections::VecDeque;

/// Parse the input into a grid with a 1-cell border of `~`.
fn parse(input: &str) -> (Vec<u8>, usize, usize, usize) {
    let rows: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let (h, w) = (rows.len(), rows[0].len());
    let stride = w + 2;
    let mut cells = vec![b'~'; stride * (h + 2)];
    for (r, row) in rows.iter().enumerate() {
        cells[(r + 1) * stride + 1..][..w].copy_from_slice(row);
    }
    (cells, stride, w, h)
}

/// Neighbour indices of `a`; out-of-range ones wrap to huge values and fail a `get`.
fn neighbours<const N: usize>(a: usize, stride: usize) -> [usize; N] {
    const STEPS: [(isize, isize); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    std::array::from_fn(|i| a.wrapping_add_signed(STEPS[i].0 * stride as isize + STEPS[i].1))
}

/// generic multi-start BFS with N directions over a grid
fn bfs<const N: usize>(map: &mut [u32], cells: &[u8], stride: usize, starts: &[usize], first: u32, step: u32, open: u8) {
    starts.iter().for_each(|&s| map[s] = first);
    let mut queue = VecDeque::from_iter(starts.iter().copied());
    while let Some(a) = queue.pop_front() {
        for b in neighbours::<N>(a, stride) {
            if map.get(b) == Some(&0) && cells[b] == open {
                map[b] = map[a] + step;
                queue.push_back(b);
            }
        }
    }
}

const LAND: u32 = 1 << 20;

/// `LAND | distance` over the '#' tiles reachable from the top-left.
fn land(cells: &[u8], stride: usize) -> Vec<u32> {
    let mut map = vec![0; cells.len()];
    bfs::<4>(&mut map, cells, stride, &[stride + 1], LAND, 1, b'#');
    map
}

/// bfs computes distance from top-left to bottom-right
pub fn part1(input: &str) -> String {
    let (cells, stride, w, h) = parse(input);
    (land(&cells, stride)[h * stride + w] & !LAND).to_string()
}

/// bfs computes visited cells, then draw borders around the tiles
pub fn part2(input: &str) -> String {
    let (cells, stride, _, _) = parse(input);
    let land = land(&cells, stride);
    let mut perimeter = 0;
    for_each_adjacent(&cells, stride, |a, b, _| perimeter += ((land[a] ^ land[b]) & LAND != 0) as usize);
    perimeter.to_string()
}

/// bfs from bottom-left and top-right to find 'holes', then draw borders around them
pub fn part3(input: &str) -> String {
    let (cells, stride, w, h) = parse(input);
    let mut map = vec![0; cells.len()];
    let at = |r: usize, c: usize| r * stride + c;
    const LEFT_BOTTOM: u32 = 1 << 20;
    const TOP_RIGHT: u32 = 2 << 20;
    // every frame cell, corners included, gets a side; otherwise a corner reads as land
    let left_bottom: Vec<usize> = (0..=h + 1).map(|r| at(r, 0)).chain((1..=w).map(|c| at(h + 1, c))).collect();
    let top_right: Vec<usize> = (0..=h + 1).map(|r| at(r, w + 1)).chain((1..=w).map(|c| at(0, c))).collect();
    for (side, starts) in [(LEFT_BOTTOM, left_bottom), (TOP_RIGHT, top_right)] {
        bfs::<8>(&mut map, &cells, stride, &starts, side, 0, b'.');
    }
    // bucket every pair by the XOR of its markers: land/LEFT_BOTTOM lands in [1], land/TOP_RIGHT
    // in [2]; same-kind pairs go to [0] frame wraparound to [3]
    let mut edges = [0; 4];
    for_each_adjacent(&cells, stride, |a, b, _| edges[((map[a] ^ map[b]) >> 20) as usize] += 1);
    edges[1].min(edges[2]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
####.######
#.#.##..#..
##.##.##..#
#.#.###.###
#.###.###.#
###.##.#.##";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "21");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "94");
    }

        #[test]
    fn test_part3() {
        assert_eq!(part3(INPUT), "31");
    }

}
