use std::collections::VecDeque;

fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(',').collect();
        let growth = parts[1].split_once('=').unwrap().1.trim().parse().unwrap();
        let limit = parts[2].split_once('=').unwrap().1.trim().parse().unwrap();
        (growth, limit)
    })
}

fn plant_size_time(growth: u64, limit: u64) -> u64 {
    let mut branches = vec![(0, 1)];
    let mut count = 0;
    let mut steps = 0;
    while count < limit {
        steps += 1;
        let mut new_branches = Vec::new();
        for (size, num) in branches.iter() {
            let new_size = size + 1;
            if new_size >= growth {
                count += *num * 2;
                new_branches.push((new_size / 2, *num * 2));
                new_branches.push((1, *num));
            } else {
                new_branches.push((new_size, *num));
                count += *num;
            }
        }
        branches = new_branches;
    }
    steps
}

// Every split adds a leaf. The two halves split again after half + 1 seconds and the new
// branch after growth - 1 seconds. Splits are handled in time order, so each of these two
// delays yields a queue that is already sorted, and the next second is the earlier front.
// The first split is seeded in both queues (with no new branch), so neither is ever empty.
fn leaves_time(growth: u64, limit: u64) -> u64 {
    let mut halves = VecDeque::from([(growth, 1u64)]);
    let mut shoots = VecDeque::from([(growth, 0u64)]);
    let mut leaves = 1;
    let mut time = 1;
    while leaves < limit {
        time = halves[0].0.min(shoots[0].0);
        let mut num = 0;
        if halves[0].0 == time {
            num += halves.pop_front().unwrap().1;
        }
        if shoots[0].0 == time {
            num += shoots.pop_front().unwrap().1;
        }
        leaves += num;
        halves.push_back((time + growth / 2 + 1, num * 2));
        shoots.push_back((time + growth - 1, num));
    }
    time
}

pub fn part1(input: &str) -> String {
    parse(input).map(|(growth, limit)| plant_size_time(growth, limit)).sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    parse(input).map(|(growth, limit)| leaves_time(growth, limit)).sum::<u64>().to_string()
}

pub fn part3(input: &str) -> String {
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30";
        assert_eq!(part1(input), "27");
    }

    #[test]
    fn test_part2() {
        let input = "\
id=1, growth=3, limit=10
id=2, growth=5, limit=20
id=3, growth=7, limit=30";
        assert_eq!(part2(input), "42");
    }
}
