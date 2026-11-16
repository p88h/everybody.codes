use crate::algo::fxhash::FxHashMap;

fn find_dragon(grid: &mut Vec<Vec<u8>>) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'D' {
                grid[r][c] = b'.';
                return (r, c);
            }
        }
    }
    (grid.len(), grid[0].len())
}

pub fn part1(input: &str) -> String {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let dpos = find_dragon(&mut grid);
    let mut pos = vec![dpos];
    // knight moves
    let ofs = vec![(-1, -2), (-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2)];
    let mut sheep = 0;
    for _ in 0..4 {
        let mut new_pos = vec![];
        for &(r, c) in pos.iter() {
            for &(dr, dc) in ofs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] != b'X' {
                        if grid[nr as usize][nc as usize] == b'S' {
                            sheep += 1;
                        }
                        grid[nr as usize][nc as usize] = b'X';
                        new_pos.push((nr as usize, nc as usize));
                    }
                }
            }
        }
        if new_pos.len() == 0 {
            break;
        }
        pos.clear();
        pos.extend(new_pos);
    }
    sheep.to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let dpos = find_dragon(&mut grid);
    let mut pos = vec![dpos];
    // knight moves
    let ofs = vec![(-1, -2), (-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2)];
    let mut sheep = 0;
    let (h, w) = (grid.len(), grid[0].len());
    // dragon positions after this round: a list to iterate, plus a grid of marks for lookups
    let mut new_pos = vec![];
    let mut on = vec![false; h * w];
    for _ in 0..20 {
        for &(r, c) in new_pos.iter() {
            on[r * w + c] = false;
        }
        new_pos.clear();
        // move the dragon
        for &(r, c) in pos.iter() {
            for &(dr, dc) in ofs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < grid.len() as i32 && nc >= 0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] == b'S' {
                        sheep += 1;
                        grid[nr as usize][nc as usize] = b'.';
                    }
                    let i = nr as usize * w + nc as usize;
                    if !on[i] {
                        on[i] = true;
                        new_pos.push((nr as usize, nc as usize));
                    }
                }
            }
        }
        // now move all the (alive) sheep
        for r in (0..grid.len()).rev() {
            for c in 0..grid[0].len() {
                if grid[r][c] == b'S' || grid[r][c] == b'H' {
                    grid[r][c] -= 37; // S becomes ., H becomes #
                    if r < grid.len() - 1 {
                        if on[(r + 1) * w + c] && grid[r + 1][c] != b'#' {
                            sheep += 1; // eaten by dragon
                        } else {
                            grid[r + 1][c] += 37; // . becomes S, # becomes H
                        }
                    }
                }
            }
        }
        if new_pos.len() == 0 {
            break;
        }
        pos.clear();
        pos.extend_from_slice(&new_pos);
    }
    sheep.to_string()
}

// Board squares are indexed r * 8 + c, 
struct Board {
    moves: [u64; 64], // knight targets of each square, as a bitmask of squares
    hide: u64,        // hideout squares
    stop: [u32; 8],   // per column: a sheep on this row can only step to safety, which never helps
    width: usize,
}

fn explore(b: &Board, spos: u32, di: u32, scnt: u32, cache: &mut FxHashMap<u32, u64>) -> u64 {
    if scnt == 0 {
        return 1;
    }
    let key = spos | (di << 24);
    if let Some(&v) = cache.get(&key) {
        return v;
    }
    let mut tot = 0;
    for c in 0..b.width {
        let r = (spos >> (c * 3)) & 7;
        if r == 7 || r == b.stop[c] {
            continue;
        }
        // a sheep can't step onto the dragon outside a hideout; it passes only if it's the last one
        let below = (r + 1) * 8 + c as u32;
        let blocked = below == di && (b.hide >> below) & 1 == 0;
        if blocked && scnt > 1 {
            continue;
        }
        let npos = if blocked { spos } else { spos + (1 << (c * 3)) };
        let mut m = b.moves[di as usize];
        while m != 0 {
            let ndi = m.trailing_zeros();
            m &= m - 1;
            let (nr, nc) = (ndi / 8, ndi % 8);
            if ((npos >> (nc * 3)) & 7) == nr && (b.hide >> ndi) & 1 == 0 {
                tot += explore(b, npos | (7 << (nc * 3)), ndi, scnt - 1, cache);
            } else {
                tot += explore(b, npos, ndi, scnt, cache);
            }
        }
    }
    cache.insert(key, tot);
    tot
}

pub fn part3(input: &str) -> String {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let (dr, dc) = find_dragon(&mut grid);
    let (h, w) = (grid.len(), grid[0].len());
    assert!(w <= 8 && h <= 7, "grid {w}x{h} too large for the state encoding");
    let mut b = Board { moves: [0; 64], hide: 0, stop: [0; 8], width: w };
    let ofs = [(-1, -2), (-2, -1), (-2, 1), (-1, 2), (1, 2), (2, 1), (2, -1), (1, -2)];
    for r in 0..h {
        for c in 0..w {
            let idx = r * 8 + c;
            if grid[r][c] == b'#' {
                b.hide |= 1 << idx;
            }
            for &(or, oc) in ofs.iter() {
                let (nr, nc) = (r as i32 + or, c as i32 + oc);
                if nr >= 0 && nr < h as i32 && nc >= 0 && nc < w as i32 {
                    b.moves[idx] |= 1 << (nr as usize * 8 + nc as usize);
                }
            }
        }
    }
    let mut spos = (1u32 << (w * 3)) - 1; // every column starts as "no sheep"
    let mut scnt = 0;
    for c in 0..w {
        if grid[0][c] == b'S' {
            spos &= !(7 << (c * 3));
            scnt += 1;
        }
        // hideouts running down to the bottom edge are safe forever
        let mut s = h - 1;
        while s > 0 && grid[s][c] == b'#' {
            s -= 1;
        }
        b.stop[c] = s as u32;
    }
    let mut cache = FxHashMap::with_capacity_and_hasher(1 << 17, Default::default());
    explore(&b, spos, (dr * 8 + dc) as u32, scnt, &mut cache).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part3() {
        assert_eq!(part3("SSS\n..#\n#.#\n#D."), "15");
        assert_eq!(part3("SSS\n..#\n..#\n.##\n.D#"), "8");
        assert_eq!(part3("..S..\n.....\n..#..\n.....\n..D.."), "44");
        assert_eq!(part3(".SS.S\n#...#\n...#.\n##..#\n.####\n##D.#"), "4406");
        assert_eq!(part3("SSS.S\n.....\n#.#.#\n.#.#.\n#.D.#"), "13033988838");
    }
}
