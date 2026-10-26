pub fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();    
    let mut grid: std::collections::HashMap<(i32, i32), u8> = std::collections::HashMap::new();
    let mut sx = 0;
    let mut sy = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                sx = x as i32;
                sy = y as i32;
            } else if c != '.' {
                grid.insert((x as i32, y as i32), c as u8);
            }
        }
    }
    // move directions: up, right, down, left
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut di = 0;
    let mut steps = 1;
    loop {
        let nx = sx + dir[di].0;
        let ny = sy + dir[di].1;
        if let Some(&c) = grid.get(&(nx, ny)) {
            if c == b'#' {
                break;
            }
         } else {
            grid.insert((sx, sy), b'+');
            sx = nx;
            sy = ny;
            steps += 1;
        }
        di = (di + 1) % 4;
    }
    steps.to_string()
}

pub fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();    
    let mut grid: std::collections::HashMap<(i32, i32), u8> = std::collections::HashMap::new();
    let mut sx = 0;
    let mut sy = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                sx = x as i32;
                sy = y as i32;
            } else if c != '.' {
                grid.insert((x as i32, y as i32), c as u8);
            }
        }
    }
    // move directions: up, right, down, left
    let dir = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut di = 0;
    let mut cnt = 0;
    let mut stack = Vec::<(i32, i32, usize)>::new();
    let mut idle = 0;
    grid.insert((sx, sy), b'+');
    loop {
        let nx = sx + dir[di].0;
        let ny = sy + dir[di].1;
        if !grid.contains_key(&(nx, ny)) {
            idle = 0;
            stack.push((sx, sy, di));
            sx = nx;
            sy = ny;
            grid.insert((sx, sy), b'+');
            // check if any neighbor is a '#' 
            for (dx, dy) in dir.iter() {
                let nnx = sx + dx;
                let nny = sy + dy;
                if let Some(&nc) = grid.get(&(nnx, nny)) {
                    if nc == b'#' {
                        cnt += 1;
                    } 
                }
            }
            if cnt == 3 {
                break;
            }
            // sleep 1 second
            // std::thread::sleep(std::time::Duration::from_millis(1000));
        } else {
            idle += 1;
            // pop back to last position
            if idle == 4 {
                grid.insert((sx, sy), b'*');
                if let Some((px, py, pd)) = stack.pop() {
                    sx = px;
                    sy = py;
                    di = pd;
                    idle = 0;
                } else {
                    break;
                }
            }
        }
        di = (di + 1) % 4;
    }
    /*
        // clear terminal
        print!("\x1B[2J\x1B[1;1H");
        // print current position, direction, and steps
        println!("Position: ({}, {}), Direction: {}, Steps: {}, Marks: {}", sx, sy, di, stack.len(), cnt);
        // display the grid
        for y in 0..lines.len() as i32 {
            for x in 0..lines[0].len() as i32 {
                if sx == x && sy == y {
                    print!("@");
                } else if let Some(&c) = grid.get(&(x, y)) {
                    print!("{}", c as char);
                } else {
                    print!(".");
                }
            }
            println!();
        }
     */
    stack.len().to_string()
}

pub fn part3(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
.......
.......
.......
.#.@...
.......
.......
.......";
        assert_eq!(part1(input), "12");
    }

    #[test]
    fn test_part2() {
        let input = "\
.......
.......
.......
.#.@...
.......
.......
.......";
        assert_eq!(part2(input), "47");
    }
}
