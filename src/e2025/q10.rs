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

fn explore(grid: &mut Vec<Vec<u8>>, mut spos: u32, cr: usize, cc: usize, scnt: usize, 
    cache: &mut HashMap<u32, usize>) -> usize {
    let ofs = vec![(-1,-2), (-2,-1), (-2,1), (-1,2), (1,2), (2,1), (2,-1), (1,-2)];
    // move one of the sheep if possible
    let mut tot = 0;
    let cache_key = (spos & 0xFFFFFF) | ((cr as u32) << 24) | ((cc as u32) << 28);
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    if scnt == 0 {
        return 1;
    }
    for i in 0..grid[0].len() {
        let cs = (spos >> (i * 3)) & 0x7;
        if cs != 0x7 {
            let r = cs as usize;
            let c = i;
            if r == grid.len() - 1 || grid[r + 1][c] == b'@' {
                // this sheep is safe
                continue;
            }
            if (r + 1, c) != (cr, cc) || grid[r+1][c] == b'#' {
                // move down
                spos += 1 << (i * 3);
                for &(dr,dc) in ofs.iter() {
                    let nr = cr as i32 + dr;
                    let nc = cc as i32 + dc;                    
                    if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                        if spos >> (nc * 3) & 7 == nr as u32 && grid[nr as usize][nc as usize] == b'.' {
                            spos |= 0x7 << (nc * 3); // eaten
                            tot += explore(grid, spos, nr as usize, nc as usize, scnt - 1, cache);
                            spos &= !(0x7 << (nc * 3)); // backtrack
                            spos |= (nr as u32) << (nc * 3);
                        } else {
                            tot += explore(grid, spos, nr as usize, nc as usize, scnt, cache);
                        }
                    }
                }
                spos -= 1 << (i * 3);
            } else if scnt == 1 {
                for &(dr,dc) in ofs.iter() {
                    let nr = cr as i32 + dr;
                    let nc = cc as i32 + dc;                    
                    if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                        if spos >> (nc * 3) & 0x7 == nr as u32 && grid[nr as usize][nc as usize] == b'.' {
                            spos |= 0x7 << (nc * 3); // eaten
                            tot += explore(grid, spos, nr as usize, nc as usize, scnt - 1, cache);
                            spos &= !(0x7 << (nc * 3)); // backtrack                                                    
                        } else {
                            tot += explore(grid, spos, nr as usize, nc as usize, scnt, cache);
                        }
                    }
                }
            }
        }
    }
    cache.insert(cache_key, tot);
    tot
}


pub fn part3(input: &str) -> String {
    let mut grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut spos = !0u32;
    let dpos = find_dragon(&mut grid);
    let mut scnt = 0;
    for i in 0..grid[0].len() {
        for j in (0..grid.len()).rev() {
            // hideout at the bottom is basically safe zone
            if grid[j][i] == b'#' && (j == grid.len() - 1 || grid[j+1][i] == b'@') {
                grid[j][i] = b'@';
            }
        }
        if grid[0][i] == b'S' {
            spos &= !(0x7 << (i * 3));
            scnt += 1;
            grid[0][i] = b'.';
        }
    }
    let mut cache= HashMap::new();
    explore(&mut grid, spos, dpos.0, dpos.1, scnt, &mut cache).to_string()
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
