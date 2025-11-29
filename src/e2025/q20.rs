use std::collections::VecDeque;

pub fn part1(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let width = grid[0].len();
    let height = grid.len();
    let mut paired = vec![vec![false; width]; height];
    let mut pairs = 0;
    for y in 0..height {
        let p = y % 2;
        for x in 0..width {
            // horizontal pair
            if x + 1 < width && grid[y][x] == b'T' && grid[y][x + 1] == b'T' {
                pairs += 1;
                paired[y][x] = true;
                paired[y][x + 1] = true;
            }
            // triangle has a wide side on every 2nd column
            if y + 1 < height && (x % 2) != p && grid[y][x] == b'T' && grid[y+1][x] == b'T' {
                pairs += 1;
                paired[y][x] = true;
                paired[y+1][x] = true;
            }
        }
    }
    pairs.to_string()
}

pub fn part2(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let width = grid[0].len();
    let height = grid.len();
    let (mut start, mut end) = ((0, 0), (0, 0));
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'S' {
                start = (x, y);
            } else if grid[y][x] == b'E' {
                end = (x, y);
            }
        }
    }
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    visited[start.1][start.0] = true;
    while !queue.is_empty() {
        let (x, y, dist) = queue.pop_front().unwrap();
        if (x, y) == end {
            return dist.to_string();
        }
        let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
        let p = y % 2;
        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < width && ny < height && !visited[ny][nx] && grid[ny][nx] > b'A' {
                if *dy > 0 && (x % 2) == p {
                    // cannot move down on this column
                    continue;
                }
                if *dy < 0 && (x % 2) != p {
                    // cannot move down on this column
                    continue;
                }
                visited[ny][nx] = true;
                queue.push_back((nx, ny, dist + 1));
            }
        }
    }
    "".to_string()
}


fn rotate_triangle_120(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    // grid contains triangle pointing down
    // after rotation the top edge becomes the right edge
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![b'.'; width]; height];
    for y in 0..height {        
        // the row spans from y to width-y, but first & last y cols are already rotated
        let mut e1 = vec![];
        for x in y*3..width-y*3 {
            e1.push((x,y));
            new_grid[y][x] = grid[y][x];
        }
        if e1.len() < 2 {
            break;
        }
        let mut e2 = vec![];
        // and last two cells are the right edge
        e2.push(e1.pop().unwrap());
        e2.push(e1.pop().unwrap());
        while e2.len() <= e1.len() + 1 {
            let j = e2.len() - 2;
            let (ex, ey) = e2[j];
            let (fx, fy) = e2[j+1];
            // walk diagonally down-left
            e2.push((ex - 1, ey + 1));
            e2.push((fx - 1, fy + 1));
        }
        e2.pop(); // last one is extra
        let mut e3 = vec![];
        // and these last two are actually left edge
        e3.push(e2.pop().unwrap());
        e3.push(e2.pop().unwrap());
        while e3.len() < e1.len() {
            let j = e3.len() - 2;
            let (ex, ey) = e3[j];
            let (fx, fy) = e3[j+1];
            // walk diagonally up-left
            e3.push((ex - 1, ey - 1));
            e3.push((fx - 1, fy - 1));
        }
        e3.pop(); // last one is extra
        // now rotate grid contents between e1,e2,e3
        for i in 0..e1.len() {
            let (x1, y1) = e1[i];
            let (x2, y2) = e2[i];
            let (x3, y3) = e3[i];
            (new_grid[y2][x2], new_grid[y3][x3], new_grid[y1][x1]) = (grid[y1][x1], grid[y2][x2], grid[y3][x3]);
        }
    }
    new_grid
}

pub fn part3(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let width = grid[0].len();
    let height = grid.len();
    let mut start = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'S' {
                start = (x, y);
            }
        }
    }
    let grid2 = rotate_triangle_120(&grid);
    let grid3 = rotate_triangle_120(&grid2);
    let grids = vec![grid, grid2, grid3];
    let mut visited = vec![vec![false; width * 3]; height];
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0, 0));
    visited[start.1][start.0 * 3] = true;
    while !queue.is_empty() {
        let (x, y, gidx, dist) = queue.pop_front().unwrap();
        if grids[gidx][y][x] == b'E' {
            return dist.to_string();
        }
        let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0),(0,0)];
        let p = y % 2;
        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let ng = (gidx + 1) % 3;            
            if nx < width && ny < height && !visited[ny][nx * 3 + ng] && grids[ng][ny][nx] > b'A' {
                if *dy > 0 && (x % 2) == p {
                    // cannot move down on this column
                    continue;
                }
                if *dy < 0 && (x % 2) != p {
                    // cannot move down on this column
                    continue;
                }
                visited[ny][nx * 3 + ng] = true;
                queue.push_back((nx, ny, ng, dist + 1));
            }
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
T#TTT###T##
.##TT#TT##.
..T###T#T..
...##TT#...
....T##....
.....#.....";
        assert_eq!(part1(input), "7");
    }

    #[test]
    fn test_part2() {
        let input = "\
TTTTTTTTTTTTTTTTT
.TTTT#T#T#TTTTTT.
..TT#TTTETT#TTT..
...TT#T#TTT#TT...
....TTT#T#TTT....
.....TTTTTT#.....
......TT#TT......
.......#TT.......
........S.........";
        assert_eq!(part2(input), "32");
    }

    #[test]
    fn test_part3() {
        let input = "\
T####T#TTT##T##T#T#
.T#####TTTT##TTT##.
..TTTT#T###TTTT#T..
...T#TTT#ETTTT##...
....#TT##T#T##T....
.....#TT####T#.....
......T#TT#T#......
.......T#TTT.......
........TT#........
.........S.........";
        assert_eq!(part3(input), "23");
    }

    #[test]
    fn test_rotate_triangle_120_5() {
        let mut grid = vec![
            b"12345".to_vec(),
            b".678.".to_vec(),
            b"..9..".to_vec(),
        ];
        let rotated = rotate_triangle_120(&mut grid);
        assert_eq!(rotated, vec![
            b"97621".to_vec(),
            b".843.".to_vec(),
            b"..5..".to_vec(),
        ]);
    }


    #[test]
    fn test_rotate_triangle_120_7() {
        let mut grid = vec![
            b"1234567".to_vec(),
            b".89ABC.".to_vec(),
            b"..DEF..".to_vec(),
            b"...G...".to_vec(),
        ];
        let rotated = rotate_triangle_120(&mut grid);
        assert_eq!(rotated, vec![
            b"GED9821".to_vec(),
            b".FBA43.".to_vec(),
            b"..C65..".to_vec(),
            b"...7...".to_vec(),
        ]);
    }

    #[test]
    fn test_rotate_triangle_120_9() {
        let mut grid = vec![
            b"123456789".to_vec(),
            b".ABcdeFG.".to_vec(),
            b"..HIjKL..".to_vec(),
            b"...MNO...".to_vec(),
            b"....P....".to_vec(),
        ];
        let rotated = rotate_triangle_120(&mut grid);
        assert_eq!(rotated, vec![
            b"PNMIHBA21".to_vec(),
            b".OKjdc43.".to_vec(),
            b"..LFe65..".to_vec(),
            b"...G87...".to_vec(),
            b"....9....".to_vec(),
        ]);
    }

}