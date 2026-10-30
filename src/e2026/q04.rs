// Parse "id=.. quest=.. part=.. time=.." into (id, quest, part, time).
fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let nums: Vec<i32> = line
        .split_whitespace()
        .filter_map(|p| p.split_once('=').and_then(|(_, v)| v.parse().ok()))
        .collect();
    (nums[0], nums[1], nums[2], nums[3])
}

// Checksum of position * value + same-score skipping
fn checksum(items: &[(i32, i32)]) -> String {
    let sum: i32 = items
        .iter()
        .map(|&(key, value)| (1 + items.iter().filter(|&&(other, _)| other < key).count() as i32) * value)
        .sum();
    sum.to_string()
}

pub fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut total_by_id = std::collections::HashMap::new();
    for line in &lines {
        let (id, _, _, time) = parse_line(line);
        *total_by_id.entry(id).or_insert(0) += time;
    }
    let totals: Vec<(i32, i32)> = total_by_id.values().map(|&time| (time, time)).collect();
    checksum(&totals)
}

pub fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut total_by_id = std::collections::HashMap::new();
    let mut count_by_id = std::collections::HashMap::new();
    for line in &lines {
        let (id, _, _, time) = parse_line(line);
        *total_by_id.entry(id).or_insert(0) += time;
        *count_by_id.entry(id).or_insert(0) += 1;
    }
    let max_count = count_by_id.values().cloned().max().unwrap_or(0);
    // increase the total time for IDs that have less the maximum count
    for (&id, &count) in &count_by_id {
        *total_by_id.entry(id).or_insert(0) += 1000000 * (max_count - count) as i32;
    }
    // ignore the added penalty when computing the checksum
    let totals: Vec<(i32, i32)> = total_by_id.values().map(|&time| (time, time % 1000000)).collect();
    checksum(&totals)
}

pub fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut entries = Vec::new();
    for line in &lines {
        entries.push(parse_line(line));
    }
    let mut time_by_quest_and_id = std::collections::HashMap::new();
    let mut score_by_id = std::collections::HashMap::new();
    for part in 1..=3 {
        // accumulate time from previous parts for the same quest and id
        let mut part_entries: Vec<(i32, i32, i32, i32)> = entries
            .iter()
            .filter(|&&(_, _, p, _)| p == part)
            .map(|&(id, quest, p, time)| {
                let total = time_by_quest_and_id.entry((quest, id)).or_insert(0);
                *total += time;
                (id, quest, p, *total)
            })
            .collect();
        // sort by quest and then cumulative time
        part_entries.sort_by_key(|&(_, quest, _, time)| (quest, time));
        // keep best time for each quest, just 30 zeroes
        let mut best_times = vec![0; 30];
        let mut quest_score = vec![0; 30];
        let mut quest_done = vec![0; 30];
        for &(id, quest, _, time) in &part_entries {
            // if the time is higher than best, set the score to whatever remains
            if time > best_times[quest as usize] {
                best_times[quest as usize] = time;
                quest_score[quest as usize] = (50 * part - quest_done[quest as usize]).max(0);
            }
            quest_done[quest as usize] += 1;
            *score_by_id.entry(id).or_insert(0) += quest_score[quest as usize];
            // println!("id={} quest={} part={} time={} score={}", id, quest, part, time, quest_score[quest as usize]);
        }
    }
    let scores: Vec<(i32, i32)> = score_by_id.values().map(|&points| (-points, points)).collect();
    checksum(&scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
id=1 quest=1 part=1 time=187
id=3 quest=1 part=1 time=413
id=5 quest=1 part=3 time=744
id=2 quest=1 part=3 time=745
id=4 quest=1 part=2 time=558
id=1 quest=1 part=2 time=299
id=1 quest=1 part=3 time=362
id=4 quest=1 part=3 time=1268
id=4 quest=1 part=1 time=457
id=5 quest=1 part=1 time=257
id=2 quest=1 part=1 time=192
id=3 quest=1 part=3 time=1168
id=5 quest=1 part=2 time=346
id=3 quest=1 part=2 time=455
id=2 quest=1 part=2 time=562";
        assert_eq!(part1(input), "27598");
    }

    #[test]
    fn test_part2() {
        let input = "\
id=2 quest=1 part=1 time=100
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=140
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=240
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=175
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=160";
        assert_eq!(part2(input), "6470");
    }

    #[test]
    fn test_part3() {
        let input = "\
id=2 quest=1 part=1 time=100
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=140
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=240
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=175
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=160";
        assert_eq!(part3(input), "2998");
        let input = "\
id=2 quest=1 part=1 time=90
id=4 quest=2 part=1 time=150
id=1 quest=2 part=1 time=130
id=1 quest=1 part=2 time=180
id=2 quest=2 part=1 time=130
id=4 quest=1 part=1 time=110
id=1 quest=2 part=2 time=210
id=2 quest=2 part=2 time=220
id=1 quest=2 part=3 time=330
id=2 quest=1 part=2 time=170
id=3 quest=1 part=1 time=90
id=4 quest=1 part=2 time=170
id=3 quest=2 part=1 time=120
id=1 quest=1 part=1 time=120
id=3 quest=2 part=2 time=220
id=2 quest=2 part=3 time=350
id=3 quest=1 part=2 time=170";
        assert_eq!(part3(input), "3004");
    }
}
