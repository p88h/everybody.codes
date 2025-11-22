use std::collections::{HashMap, VecDeque};

pub fn part1(input: &str) -> String {
    let mut cols: Vec<VecDeque<i64>> = Vec::new();
    for line in input.lines() {
        let nums = line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
        for (i, &num) in nums.iter().enumerate() {
            if cols.len() <= i {
                cols.push(VecDeque::new());
            }
            cols[i].push_back(num);
        }
    }
    for round in 0..10 {
        let col = round % cols.len();
        let first = cols[col].pop_front().unwrap();
        let dst = (col + 1) % cols.len();
        let ofs = (first - 1) as usize % (cols[dst].len() * 2);
        let dst_len = cols[dst].len();
        if ofs < dst_len {
            cols[dst].insert(ofs, first);
        } else {
            cols[dst].insert(dst_len - (ofs - dst_len), first);
        }
    }
    cols.iter().map(|col| col.front().unwrap().to_string()).collect::<Vec<String>>().join("")
}

pub fn part2(input: &str) -> String {
    let mut cols: Vec<VecDeque<i64>> = Vec::new();
    for line in input.lines() {
        let nums = line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
        for (i, &num) in nums.iter().enumerate() {
            if cols.len() <= i {
                cols.push(VecDeque::new());
            }
            cols[i].push_back(num);
        }
    }
    let mut counters = HashMap::new();
    let mut round = 0;
    loop {
        let col = round % cols.len();
        round += 1;
        let first = cols[col].pop_front().unwrap();
        let dst = (col + 1) % cols.len();
        let ofs = (first - 1) as usize % (cols[dst].len() * 2);
        let dst_len = cols[dst].len();
        if ofs < dst_len {
            cols[dst].insert(ofs, first);
        } else {
            cols[dst].insert(dst_len - (ofs - dst_len), first);
        }
        let code = cols.iter().fold(0i64, |acc, col| acc * 100 + col.front().unwrap());
        match counters.get(&code) {
            None => {
                counters.insert(code, 1);
            }
            Some(&count) => {
                if count == 2023 {
                    return (round * code as usize).to_string();
                }
                counters.insert(code, count + 1);
            }
        }
    }
}

pub fn part3(input: &str) -> String {
    let mut cols: Vec<VecDeque<i64>> = Vec::new();
    for line in input.lines() {
        let nums = line.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect::<Vec<i64>>();
        for (i, &num) in nums.iter().enumerate() {
            if cols.len() <= i {
                cols.push(VecDeque::new());
            }
            cols[i].push_back(num);
        }
    }
    let mut round = 0;
    let mut max_code = 0;
    let mut wait = 1000;
    while wait > 0 {
        let col = round % cols.len();
        round += 1;
        let first = cols[col].pop_front().unwrap();
        let dst = (col + 1) % cols.len();
        let ofs = (first - 1) as usize % (cols[dst].len() * 2);
        let dst_len = cols[dst].len();
        if ofs < dst_len {
            cols[dst].insert(ofs, first);
        } else {
            cols[dst].insert(dst_len - (ofs - dst_len), first);
        }
        let code = cols.iter().fold(0i64, |acc, col| acc * 10000 + col.front().unwrap());
        if code > max_code {
            max_code = code;
            wait = 1000;
        }
        wait -= 1;
    }
    max_code.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
2 3 4 5
3 4 5 2
4 5 2 3
5 2 3 4";
        assert_eq!(part1(input), "2323");
    }
}
