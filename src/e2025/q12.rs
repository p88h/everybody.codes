use rayon::prelude::*;

fn bfs(grid: &Vec<Vec<u8>>, skip: &Vec<bool>, start: &Vec<(usize, usize)>) -> (Vec<bool>, usize) {
    let dirs = vec![(0,1), (1,0), (0,-1), (-1,0)];
    let mut cur = start.clone();
    let mut visited = skip.clone();
    for (x,y) in cur.iter() {
        visited[y * 256 + x] = true;
    }
    let mut cnt = 0;
    while cur.len() > 0 {
        let mut next = vec![];
        cnt += cur.len();
        for &(r,c) in cur.iter() {
            for &(dr,dc) in dirs.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                    let nidx = (nc * 256 + nr) as usize;
                    if grid[nr as usize][nc as usize] <= grid[r][c] && !visited[nidx] {
                        visited[nidx] = true;
                        next.push((nr as usize, nc as usize));
                    }
                }
            }
        }
        cur = next;
    }
    (visited, cnt)
}

pub fn part1(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let visited = bfs(&grid, &vec![false; 256*256], &vec![(0,0)]);
    visited.1.to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let visited = bfs(&grid, &vec![false; 256*256], &vec![(0,0), (grid.len()-1,grid[0].len()-1)]);
    visited.1.to_string()
}

fn try_explode(grid: &mut Vec<Vec<u8>>, exploded: &Vec<bool>, sr: usize, sc: usize) -> (Vec<bool>, usize) {
    let dirs = vec![(0,1), (1,0), (0,-1), (-1,0)];
    if grid[sr][sc] == b'9' {
        // simpler check for 9s - only take the top-left ones
        if (sc > 0 && grid[sr][sc-1] == b'9') || (sr > 0 && grid[sr-1][sc] == b'9') {
            return (vec![], 0);
        }
    } else {
        // otherwise take local maximums only
        for &(dr,dc) in dirs.iter() {
            let nr = sr as i32 + dr;
            let nc = sc as i32 + dc;
            if nr >=0 && nr < grid.len() as i32 && nc >=0 && nc < grid[0].len() as i32 {
                if grid[nr as usize][nc as usize] >= grid[sr][sc] {
                    return (vec![], 0);
                }
            }
        }
    }
    bfs(&grid, exploded, &vec![(sr, sc)])
}

pub fn part3(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut previous: Vec<bool> = vec![false; 256*256];
    for _ in 0..3 {        
        let best = (0..grid.len()).into_par_iter().map(|r| {
            let mut row_best = vec![];
            let mut row_best_cnt = 0;
            for c in 0..grid[0].len() {
                let (exploded, cnt) = try_explode(&mut grid.clone(), &previous, r, c);
                if  cnt > row_best_cnt {
                    row_best_cnt = cnt;
                    row_best = exploded;
                }
            }
            (row_best, row_best_cnt)
            }).max_by_key(|(_, cnt)| *cnt).unwrap().0;
        previous = best
    }
    previous.iter().filter(|&&v| v).count().to_string()
}
