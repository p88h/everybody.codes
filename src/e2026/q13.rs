use crate::algo::math;

/// Number of times any two hands of one block meet while all their moves play out.
fn count_overlaps(block: &str) -> i64 {
    let lists = block
        .lines()
        .map(|line| line.split_whitespace().nth(1).unwrap().split(':').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let hands: Vec<usize> = lists[0].iter().map(|s| s.parse().unwrap()).collect();
    let n = hands.len();
    let lcm = hands.iter().fold(1, |acc, &h| math::lcm(acc, h)) as i64;
    let step: Vec<i64> = hands.iter().map(|&h| lcm / h as i64).collect();
    let directions: Vec<i64> = lists[1].iter().map(|s| if *s == "CW" { 1 } else { -1 }).collect();
    let mut current: Vec<i64> = lists[2].iter().enumerate().map(|(i, s)| s.parse::<i64>().unwrap() * step[i]).collect();
    let mut moves: Vec<i64> = lists[3].iter().map(|s| s.parse().unwrap()).collect();
    let mut overlaps = 0;
    let mut velocity: Vec<i64> = (0..n).map(|i| if moves[i] > 0 { directions[i] * step[i] } else { 0 }).collect();

    // jump to the next point in time when any hand runs out of moves
    while let Some(k) = moves.iter().copied().filter(|&m| m > 0).min() {
        for i in 0..n {
            for j in i + 1..n {
                let rel = velocity[i] - velocity[j];
                if rel == 0 {
                    continue;
                }
                // compute initial gap (distance in the direction of relative motion)
                let gap = if rel > 0 { current[j] - current[i] } else { current[i] - current[j] }.rem_euclid(lcm);
                let gap = if gap == 0 { lcm } else { gap };
                // gap is already direction-normalized (positive in the direction of relative motion)
                // so travel is absolute distance covered along the relative motion direction
                let travel = rel.abs() * k;
                if travel >= gap {
                    overlaps += 1 + (travel - gap) / lcm;
                }
            }
        }

        for i in 0..n {
            if moves[i] > 0 {
                current[i] = (current[i] + velocity[i] * k).rem_euclid(lcm);
                moves[i] -= k;
            }
            if moves[i] == 0 {
                velocity[i] = 0;
            }
        }
    }
    overlaps
}

pub fn part1(input: &str) -> String {
    count_overlaps(input).to_string()
}

pub fn part2(input: &str) -> String {
    // blocks are separated by empty lines
    input.split("\n\n").map(count_overlaps).sum::<i64>().to_string()
}

pub fn part3(input: &str) -> String {
    // same format as part 2, just far more moves; `count_overlaps` skips ahead in stretches
    part2(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4";
        assert_eq!(part1(input), "5");
        let input2 = "\
hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15";
        assert_eq!(part1(input2), "7");
        let input3 = "\
hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20";
        assert_eq!(part1(input3), "9");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
hands 6:8:12
direction CW:CCW:CW
current 2:4:8
move 9:7:4

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:15

hands 6:8:12
direction CW:CCW:CW
current 1:2:2
move 10:12:20"
            ),
            "21"
        );
    }
}
