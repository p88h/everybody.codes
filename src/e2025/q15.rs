use std::collections::{BTreeSet, HashMap, VecDeque};

fn bfs(
    grid: &mut Vec<Vec<u8>>,
    costfn: impl Fn((usize, usize), (usize, usize)) -> usize,
    start: (usize, usize),
    goal: (usize, usize),
) -> Option<usize> {
    let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((start, 0usize));
    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(dist);
        }
        for (dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid[0].len()
                && (ny as usize) < grid.len()
                && grid[ny as usize][nx as usize] == b' '
            {
                grid[ny as usize][nx as usize] = b'.';
                let cost = costfn((x, y), (nx as usize, ny as usize));
                queue.push_back(((nx as usize, ny as usize), dist + cost));
            }
        }
    }
    None
}

pub fn part1(input: &str) -> String {
    let directions = input
        .split(',')
        .map(|s| (s.as_bytes()[0], s[1..].parse::<usize>().unwrap()))
        .collect::<Vec<(u8, usize)>>();
    let mut pos = (32i32, 32i32);
    let mut dir = (0, 1); // facing north
    let mut grid = vec![vec![b' '; 64]; 64];
    grid[pos.1 as usize][pos.0 as usize] = b'S';
    for (turn, dist) in directions {
        dir = match turn {
            b'L' => (-dir.1, dir.0),
            b'R' => (dir.1, -dir.0),
            _ => dir,
        };
        for _ in 0..dist {
            pos.0 += dir.0;
            pos.1 += dir.1;
            grid[pos.1 as usize][pos.0 as usize] = b'#';
        }
    }
    grid[pos.1 as usize][pos.0 as usize] = b' ';
    let dist = bfs(&mut grid, |_, _| 1, (32, 32), (pos.0 as usize, pos.1 as usize));
    dist.unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let directions = input
        .split(',')
        .map(|s| (s.as_bytes()[0], s[1..].parse::<usize>().unwrap()))
        .collect::<Vec<(u8, usize)>>();
    let mut pos = (0, 0);
    let mut dir = (0, 1); // facing north
    let mut seq = vec![pos];
    for (turn, dist) in directions {
        dir = match turn {
            b'L' => (-dir.1, dir.0),
            b'R' => (dir.1, -dir.0),
            _ => dir,
        };
        pos.0 += dir.0 * dist as i32;
        pos.1 += dir.1 * dist as i32;
        seq.push(pos);
    }
    // insert all relevant points into sets
    let mut vx = BTreeSet::new();
    let mut vy = BTreeSet::new();
    for (x, y) in &seq {
        for d in -1..2 {
            vx.insert(x + d);
            vy.insert(y + d);
        }
    }
    // build mappings between coords abd compressed indexes
    let xpos = vx.iter().map(|x| *x).collect::<Vec<i32>>();
    let ypos = vy.iter().map(|y| *y).collect::<Vec<i32>>();
    let xmap = vx.iter().enumerate().map(|(i, &x)| (x, i)).collect::<HashMap<i32, usize>>();
    let ymap = vy.iter().enumerate().map(|(i, &y)| (y, i)).collect::<HashMap<i32, usize>>();
    let mut grid = vec![vec![b' '; xpos.len()]; ypos.len()];
    // do the whole thing again but now paint the grid in compressed space
    let mut pos = (xmap[&0], ymap[&0]);
    grid[pos.1][pos.0] = b'S';
    for i in 1..seq.len() {
        let npos = (xmap[&seq[i].0], ymap[&seq[i].1]);
        let dx = (npos.0 as isize - pos.0 as isize).signum();
        let dy = (npos.1 as isize - pos.1 as isize).signum();
        while pos != npos {
            pos.0 = (pos.0 as isize + dx as isize) as usize;
            pos.1 = (pos.1 as isize + dy as isize) as usize;
            grid[pos.1][pos.0] = b'#';
        }
        pos = npos;
    }
    grid[pos.1][pos.0] = b' ';
    // run BFS in compressed space, with decompression cost function
    let dist = bfs(
        &mut grid,
        |(sx, sy), (nx, ny)| {
            let real_dx = (xpos[nx] as isize - xpos[sx] as isize).abs() as usize;
            let real_dy = (ypos[ny] as isize - ypos[sy] as isize).abs() as usize;
            real_dx + real_dy
        },
        (xmap[&0], ymap[&0]),
        (pos.0, pos.1),
    );
    dist.unwrap().to_string()
}

pub fn part3(input: &str) -> String {
    part2(input)
}
