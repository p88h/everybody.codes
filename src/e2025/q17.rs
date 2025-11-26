use std::collections::HashSet;

struct Grid {
    data: Vec<Vec<u16>>,
    start: (usize, usize),
    center: (usize, usize),
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut data = Vec::new();
        let mut start = (0, 0);
        let mut center = (0, 0);
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.bytes().enumerate() {
                match ch {
                    b'S' => {
                        start = (x, y);
                        row.push(0);
                    }
                    b'@' => {
                        center = (x, y);
                        row.push(0);
                    }
                    b'0'..=b'9' => row.push((ch - b'0') as u16),
                    _ => row.push(0),
                }
            }
            data.push(row);
        }
        let width = data[0].len();
        let height = data.len();
        Grid { data, start, center, width, height }
    }

    fn compute_circles(self: &mut Self, max_radius: usize) -> Vec<i32> {
        let (dx, dy) = self.center;
        let mut radius_sums = vec![0; max_radius + 1];        
        for y in dy.saturating_sub(max_radius)..=(dy + max_radius).min(self.height - 1) {
            let y_part = ((y as isize - dy as isize) * (y as isize - dy as isize)) as usize;
            for x in dx.saturating_sub(max_radius)..=(dx + max_radius).min(self.width - 1) {
                let x_part = ((x as isize - dx as isize) * (x as isize - dx as isize)) as usize;
                let dist_sq = x_part + y_part;
                
                // Find the minimum radius at which this pixel is included
                // A pixel at distance_squared d is included when radius^2 >= d
                // So minimum radius is ceil(sqrt(d))
                let dist = (dist_sq as f64).sqrt();
                let min_radius = dist.ceil() as usize;
                
                if min_radius <= max_radius {
                    radius_sums[min_radius] += self.data[y][x] as i32;
                    // record the radius in the high byte
                    // since we only do this up to max_radius, this reduces the search space to just 
                    // max-radius sized circle, rather than a square. 
                    // Actual inputs will work with this assumption, but the biggest test example will not
                    // (it crawls on the edges of the whole _grid_)
                    self.data[y][x] |= (min_radius as u16) << 8;
                }
            }
        }
        // draw an extra 0-line to the bottom to simplify search - paths will never cross this line
        for y in dy..self.height {
            self.data[y][dx] &= 0xFF;
        }
        // this is used in part 1/2        
        radius_sums
    }

    fn search(self: &Self, goal1: (usize, usize), goal2: (usize, usize), radius: usize) -> Vec<usize> {
        let mut queues: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 1000];
        let mut visited = HashSet::new();
        queues[0].push(self.start);
        visited.insert(self.start);
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
                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;
                    // no border checks - the radius grid will have a border
                    let npos = (nx, ny);            
                    // if the cell was claimed below current radius, skip
                    if self.data[ny][nx] >> 8 <= radius as u16 {
                        continue;
                    }
                    if !visited.contains(&npos) {
                        let new_cost = cost + (self.data[ny][nx] & 0xFF) as usize;
                        queues[new_cost].push(npos);
                        visited.insert(npos);
                    }
                }
            }
        }
        costs
    }

}

pub fn part1(input: &str) -> String {
    let mut grid = Grid::new(input);
    grid.compute_circles(10).iter().sum::<i32>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut grid = Grid::new(input);
    let radius_sums = grid.compute_circles(50);
    let (idx, max) = radius_sums.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
    (idx as i32 * max).to_string()
}

pub fn part3(input: &str) -> String {
    let mut grid = Grid::new(input);
    // this sacrifiices 4 pixels as border, any value between 1 and 8 seems safe
    // higher value->faster, but can perhaps fail for some inputs
    // at least 1 is needed to havee a border for search (alternatively, could also expand grid)
    grid.compute_circles(grid.width/2 - 4);
    // minimum radius given start position and destination position is 5
    let mut rad = 5;
    let (dx, dy) = grid.center;
    loop {
        // Create a fresh copy and apply painting up to current radius       
        let costs = grid.search((dx - 1, dy + rad + 1), (dx + 1, dy + rad + 1), rad);
        if costs.len() == 2 {
            let cost = costs[0] + costs[1] + (grid.data[dy + rad + 1][dx] & 0xFF) as usize;
            if cost > rad * 30 + 29 {
                while cost > rad * 30 + 29 {
                    rad += 1;
                }
                continue;
            }
            return (rad * cost).to_string();
        } else {
            // this should not happen, but just in case
            rad += 1;
        }
    }
}
