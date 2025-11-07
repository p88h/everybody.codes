pub fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let words = lines.next().unwrap().split(",").collect::<Vec<&str>>();
    lines.next();
    let dirs = lines.next().unwrap().split(",");
    let mut pos = 0;
    for dir in dirs {
        let ofs = dir[1..].parse::<i32>().unwrap() % words.len() as i32;
        match dir.chars().next().unwrap() {
            'L' => pos -= ofs,
            'R' => pos += ofs,
            _ => {}
        }
        if pos < 0 {
            pos = 0;
        } else if pos >= words.len() as i32 {
            pos = words.len() as i32 - 1;
        }
    }
    words[pos as usize].to_string()
}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let words = lines.next().unwrap().split(",").collect::<Vec<&str>>();
    lines.next();
    let dirs = lines.next().unwrap().split(",");
    let mut pos = 0;
    for dir in dirs {
        let ofs = dir[1..].parse::<i32>().unwrap() % words.len() as i32;
        match dir.chars().next().unwrap() {
            'L' => pos -= ofs,
            'R' => pos += ofs,
            _ => {}
        }
        if pos < 0 {
            pos += words.len() as i32;
        } else if pos >= words.len() as i32 {
            pos -= words.len() as i32;
        }
    }
    words[pos as usize].to_string()
}

pub fn part3(input: &str) -> String {
    let mut lines = input.lines();
    let mut words = lines.next().unwrap().split(",").collect::<Vec<&str>>();
    lines.next();
    let dirs = lines.next().unwrap().split(",");
    for dir in dirs {
        let ofs = dir[1..].parse::<usize>().unwrap() % words.len();
        let mut pos = 0;
        match dir.chars().next().unwrap() {
            'L' => pos = (words.len() - ofs) % words.len(),
            'R' => pos = ofs,
            _ => {}
        }
        words.swap(0, pos);
    }
    words[0].to_string()
}
