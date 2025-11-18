use rayon::prelude::*;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let dirs = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut cur = vec![(0,0)];
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0,0));
    while cur.len() > 0 {
        let mut next = vec![];
        for &(r,c) in cur.iter() {
            for &(dr,dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] <= grid[r][c] && !visited.contains(&(nr as usize, nc as usize)) {
                        if !visited.contains(&(nr as usize, nc as usize)) {
                            visited.insert((nr as usize, nc as usize));
                            next.push((nr as usize, nc as usize));
                        }
                    }
                }
            }
        }
        cur = next;
    }
    visited.len().to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let dirs = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut cur = vec![(0,0), (grid.len()-1,grid[0].len()-1)];
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    visited.extend(cur.iter());
    while cur.len() > 0 {
        let mut next = vec![];
        for &(r,c) in cur.iter() {
            for &(dr,dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] <= grid[r][c] && !visited.contains(&(nr as usize, nc as usize)) {
                        if !visited.contains(&(nr as usize, nc as usize)) {
                            visited.insert((nr as usize, nc as usize));
                            next.push((nr as usize, nc as usize));
                        }
                    }
                }
            }
        }
        cur = next;
    }
    visited.len().to_string()
}

fn try_explode(grid: &mut Vec<Vec<u8>>, exploded: &HashSet<(usize, usize)>, sr: usize, sc: usize) -> HashSet<(usize, usize)> {
    let dirs = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut cur = vec![(sr,sc)];
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    if exploded.contains(&(sr,sc)) {
        return visited;
    }    
    visited.insert((sr,sc));
    if grid[sr][sc] != b'9' {
        for &(dr,dc) in dirs.iter() {
            let nr = sr as i32 + dr;
            let nc = sc as i32 + dc;
            if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                if grid[nr as usize][nc as usize] >= grid[sr][sc]  && !exploded.contains(&(nr as usize, nc as usize)) {
                    return visited;
                }
            }
        }
    }
    while cur.len() > 0 {
        let mut next = vec![];
        for &(r,c) in cur.iter() {
            for &(dr,dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    if grid[nr as usize][nc as usize] <= grid[r][c] && 
                    !visited.contains(&(nr as usize, nc as usize)) &&
                    !exploded.contains(&(nr as usize, nc as usize)) {
                        if !visited.contains(&(nr as usize, nc as usize)) {
                            visited.insert((nr as usize, nc as usize));
                            next.push((nr as usize, nc as usize));
                        }
                    }
                }
            }
        }
        cur = next;
    }
    return visited;
}

pub fn part3(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut all_exploded: HashSet<(usize, usize)> = HashSet::new();
    for _ in 0..3 {        
        let best_explode = (0..input.lines().count()).into_par_iter().map(|r| {
            let mut row_best_explode = HashSet::new();
            for c in 0..input.lines().next().unwrap().len() {
                let exploded = try_explode(&mut grid.clone(), &all_exploded, r, c);
                if exploded.len() > row_best_explode.len() {
                    row_best_explode = exploded;                
                }
            }
            row_best_explode
            }).max_by_key(|exploded| exploded.len()).unwrap_or(HashSet::new());
        all_exploded.extend(best_explode.iter());
    }
    all_exploded.len().to_string()  
}
