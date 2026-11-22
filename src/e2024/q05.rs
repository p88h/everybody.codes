use crate::algo::fxhash::FxHashMap;

/// Columns are stored reversed (front of the column is the last element), so taking the clapper
/// is a `pop` and the insertion is a single short memmove.
fn parse(input: &str) -> Vec<Vec<u16>> {
    let mut cols: Vec<Vec<u16>> = Vec::new();
    for line in input.lines() {
        for (i, num) in line.split(' ').filter_map(|s| s.parse::<u16>().ok()).enumerate() {
            if cols.len() <= i {
                cols.push(Vec::new());
            }
            cols[i].push(num);
        }
    }
    for col in cols.iter_mut() {
        col.reverse();
    }
    cols
}

/// One round: the clapper from column `*col` dances into the next column, and `*col` advances.
#[inline(always)]
fn dance(cols: &mut [Vec<u16>], col: &mut usize) {
    let first = cols[*col].pop().unwrap();
    *col += 1;
    if *col == cols.len() {
        *col = 0;
    }
    let dst = &mut cols[*col];
    let len = dst.len();
    // The clapper walks down one side and back up the other; skip the division in the
    // common case where it doesn't go round more than once.
    let mut ofs = first as usize - 1;
    if ofs >= 2 * len {
        ofs %= 2 * len;
    }
    let pos = if ofs < len { ofs } else { 2 * len - ofs };
    dst.insert(len - pos, first);
}

fn shout(cols: &[Vec<u16>], digits: u64) -> u64 {
    cols.iter().fold(0, |acc, col| acc * digits + *col.last().unwrap() as u64)
}

pub fn part1(input: &str) -> String {
    let mut cols = parse(input);
    let mut col = 0;
    for _ in 0..10 {
        dance(&mut cols, &mut col);
    }
    cols.iter().map(|col| col.last().unwrap().to_string()).collect::<Vec<String>>().join("")
}

pub fn part2(input: &str) -> String {
    let mut cols = parse(input);
    let mut col = 0;
    let mut counters: FxHashMap<u64, u32> = FxHashMap::default();
    // Simulate a batch of rounds, then count their shouts: keeping the serial simulation and the
    // cache-missing hash map lookups in separate loops lets both run faster.
    let mut batch = [0u64; 1024];
    let mut round = 0;
    loop {
        for code in batch.iter_mut() {
            dance(&mut cols, &mut col);
            *code = shout(&cols, 100);
        }
        for &code in &batch {
            round += 1;
            let count = counters.entry(code).or_insert(0);
            *count += 1;
            if *count == 2024 {
                return (round * code).to_string();
            }
        }
    }
}

pub fn part3(input: &str) -> String {
    let mut cols = parse(input);
    let mut col = 0;
    let mut max_code = 0;
    let mut wait = 1000;
    while wait > 0 {
        dance(&mut cols, &mut col);
        let code = shout(&cols, 10000);
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
