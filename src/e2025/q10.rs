use std::collections::HashMap;

fn find_dragon(grid: &mut Vec<Vec<u8>>) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'D' {
                grid[r][c] = b'.';
                return (r,c)
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
    let ofs = vec![(-1,-2), (-2,-1), (-2,1), (-1,2), (1,2), (2,1), (2,-1), (1,-2)];
    let mut sheep = 0;
    for _ in 0..4 {
        let mut new_pos = vec![];
        for &(r,c) in pos.iter() {
            for &(dr,dc) in ofs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
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
    let ofs = vec![(-1,-2), (-2,-1), (-2,1), (-1,2), (1,2), (2,1), (2,-1), (1,-2)];
    let mut sheep = 0;
    for _ in 0..20 {
        let mut new_pos = std::collections::HashSet::new(); 
        // move the dragon
        for &(r,c) in pos.iter() {
            for &(dr,dc) in ofs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] == b'S' {
                        sheep += 1;
                        grid[nr as usize][nc as usize] = b'.';
                    }
                    new_pos.insert((nr as usize, nc as usize));
                }
            }
        }
        // now move all the (alive) sheep
        for r in (0..grid.len()).rev() {
            for c in 0..grid[0].len() {
                if grid[r][c] == b'S' || grid[r][c] == b'H' {
                    grid[r][c] -= 37;       // S becomes ., H becomes #                    
                    if r < grid.len() - 1 {
                        if new_pos.contains(&(r + 1 ,c)) && grid[r+1][c] != b'#' {
                            sheep += 1;         // eaten by dragon  
                        } else {
                            grid[r+1][c] += 37; // . becomes S, # becomes H
                        }
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

// map start position to all end positions
fn precompute_dragon_moves(grid: &mut Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut moves = vec![vec![]; 64];
    let ofs = vec![(-1,-2), (-2,-1), (-2,1), (-1,2), (1,2), (2,1), (2,-1), (1,-2)];
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let idx = r * 8 + c;
            for &(dr,dc) in ofs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    let nidx = nr as usize * 8 + nc as usize;
                    moves[idx].push(nidx);
                }
            }
        }
    }
    moves
}

fn explore(grid: &mut Vec<Vec<u8>>, spos: u32, di: usize, moves: &Vec<Vec<usize>>, scnt: usize, 
    cache: &mut HashMap<u32, usize>) -> usize {
    let mut tot = 0;
    let cache_key = (spos & 0xFFFFFF) | ((di as u32) << 24);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    if scnt == 0 {
        return 1;
    }
    // try to move each of the sheep if possible
    for i in 0..grid[0].len() {
        let r = ((spos >> (i * 3)) & 0x7) as usize;
        if r != 0x7 {
            if r == grid.len() - 1 || grid[r + 1][i] == b'@' {
                // this sheep is safe
                continue;
            }
            // move down if possible
            let npos = if (r * 8 + i + 8) != di || grid[r+1][i] == b'#' { spos + (1 << (i * 3)) } else { spos };
            if npos == spos && scnt > 1 {
                continue;
            }
            // try all dragon moves from current dragon position
            for ndi in moves[di].iter() {
                let nr = ndi / 8;
                let nc = ndi % 8;                    
                if npos >> (nc * 3) & 7 == nr as u32 && grid[nr][nc] == b'.' {
                    // eat the sheep
                    tot += explore(grid, npos | (0x7 << (nc * 3)), *ndi, moves, scnt - 1, cache);
                } else {
                    tot += explore(grid, npos, *ndi, moves, scnt, cache);
                }
            }
        }
    }
    cache.insert(cache_key, tot);
    tot
}

pub fn part3(input: &str) -> String {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let dpos = find_dragon(&mut grid);
    let mut spos = !0u32;
    let mut scnt = 0;
    for i in 0..grid[0].len() {
        if grid[0][i] == b'S' {
            spos &= !(0x7 << (i * 3));
            scnt += 1;
            grid[0][i] = b'.';
        }
        for j in (0..grid.len()).rev() {
            // hideout at the bottom is basically safe zone
            if grid[j][i] == b'#' && (j == grid.len() - 1 || grid[j+1][i] == b'@') {
                grid[j][i] = b'@';
            }
        }
    }
    let moves = precompute_dragon_moves(&mut grid);
    let mut cache= HashMap::new();
    explore(&mut grid, spos, dpos.0 * 8 + dpos.1, &moves, scnt, &mut cache).to_string()
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
