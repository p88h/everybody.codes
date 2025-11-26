use std::collections::HashSet;

fn paint_circle(grid: &mut Vec<Vec<u8>>, center: (usize, usize), radius: usize) -> i32 {
    let (dx, dy) = center;
    let mut sum = 0;
    for y in dy - radius..=dy + radius {
        let y_part = (y - dy) * (y - dy);
        for x in dx - radius..=dx + radius {
            let x_part = (x - dx) * (x - dx);
            let dist = x_part + y_part;
            if dist <= radius * radius {
                if grid[y][x] >= b'0' && grid[y][x] <= b'9' {
                    sum += (grid[y][x] - b'0') as i32;
                }
                grid[y][x] = b'.';
            }
        }
    }
    sum
}

pub fn part1(input: &str) -> String {
    let mut char_grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    paint_circle(&mut char_grid, (15, 15), 10).to_string()
}

pub fn part2(input: &str) -> String {
    let mut char_grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let (mut max, mut idx) = (0, 0);
    for rad in 1..51 {
        let sum = paint_circle(&mut char_grid, (50, 50), rad);
        if sum > max {
            max = sum;
            idx = rad as i32;
        }
    }
    (idx * max).to_string()
}

fn bfscd(grid: &Vec<Vec<u8>>, start: (usize, usize), goal1: (usize, usize), goal2: (usize, usize)) -> Vec<usize> {
    let mut queues: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 1000];
    let mut visited = HashSet::new();
    queues[0].push(start);
    visited.insert(start);
    let mut costs = vec![];

    for cost in 0..queues.len() {
        while let Some((x, y)) = queues[cost].pop() {
            if (x, y) == goal1 || (x, y) == goal2 {
                costs.push(cost);
                if costs.len() == 2 {
                    return costs;
                }
            }
            let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 && (ny as usize) < grid.len() && (nx as usize) < grid[ny as usize].len() {
                    let npos = (nx as usize, ny as usize);
                    if !visited.contains(&npos) && grid[ny as usize][nx as usize] != b'.' {
                        let new_cost = cost + (grid[ny as usize][nx as usize] - b'0') as usize;
                        queues[new_cost].push(npos);
                        visited.insert(npos);
                    }
                }
            }
        }
    }
    costs
}

pub fn part3(input: &str) -> String {
    let mut char_grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let (sx, sy) = (75, 10);
    let (dx, dy) = (75, 75);
    assert!(char_grid[sy][sx] == b'S');
    assert!(char_grid[dy][dx] == b'@');
    char_grid[sy][sx] = b'.';
    for y in 75..char_grid.len() {
        char_grid[y][sx] = b'.';
    }
    // minimum radius given start position and destination position is 5
    let mut rad = 5;
    loop {
        paint_circle(&mut char_grid, (dx, dy), rad);
        let costs = bfscd(&char_grid, (sx, sy), (dx - 1, dy + rad + 1), (dx + 1, dy + rad + 1));
        if costs.len() == 2 {
            let cost = costs[0] + costs[1] + 9;
            if cost > rad * 30 + 29 {
                while cost > rad * 30 + 29 {
                    rad += 1;
                }
                continue;
            }
            return (rad * cost).to_string();
        } else {
            rad += 1;
        }
    }
}
