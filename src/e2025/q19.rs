pub fn part1(input: &str) -> String {
    let gaps: Vec<(i32, i32, i32)> = input.lines().map(|line| {
        let nums = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        (nums[0], nums[1], nums[2]+nums[1]-1)
    }).collect();
    // states are (x, y, cost). We start at 0 (floor) with cost 0
    let mut next = vec![(0, 0, 0)];
    let mut cur = vec![];
    let mut cx = 0;
    for (gx, gy1, gy2) in gaps.iter() {
        if cx != *gx {
            cur = next;
            next = vec![];
            cx = *gx;
        }
        for gy in *gy1..=*gy2 {
            let mut min_cost = i32::MAX;
            for (x, y, cost) in cur.iter() {
                let dx = gx - x;
                let xm = dx % 2;
                // we can only move diagonally at each x, so parity must match
                if (y + xm) % 2 != gy % 2 {
                    continue;
                }
                let dy = (gy - y).abs();
                // we cannot move by more than dx in y direction
                if dx < dy {
                    continue;
                }
                // downward moves are free, so we need to cover cost of moving up + half of horizontal moves
                let up_cost = if gy > *y { gy - *y } else { 0 };
                let horiz_cost = (dx - dy) / 2;
                let total_cost = cost + up_cost + horiz_cost;
                min_cost = min_cost.min(total_cost);                
            }
            if min_cost != i32::MAX {
                next.push((*gx, gy, min_cost));
            }
        }
    }
    cur.iter().map(|(_, _, cost)| *cost).min().unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    part1(input)
}

pub fn part3(input: &str) -> String {
    part1(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
7,7,2
12,0,4
15,5,3
24,1,6
28,5,5
40,8,2";
        assert_eq!(part1(input), "24");
    }
}