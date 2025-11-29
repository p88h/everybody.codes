fn expand_merge_gaps(cur: &mut Vec<(i32, i32, i32)>, next: &Vec<(i32, i32, i32)>) {
    let dx = next[0].0 - cur[0].0;
    // expand all ranges in cur by dx up and down
    for (_, y1, y2) in cur.iter_mut() {
        *y1 = (*y1 - dx).max(0);
        *y2 += dx;
    }
    // merge overlapping ranges in cur
    let mut merged = vec![];
    for (x, y1, y2) in cur.iter() {
        if let Some((_, _, my2)) = merged.last_mut() {
            if *y1 <= *my2 {
                *my2 = (*my2).max(*y2);
                continue;
            }
        }
        merged.push((*x, *y1, *y2));
    }
    // intersect merged with next into cur
    cur.clear();
    let mut mi = 0;
    for (_, y1, y2) in merged.iter() {
        // skip until we reach possible overlap
        while mi < next.len() && next[mi].2 < *y1 {
            mi += 1;
        }
        if mi >= next.len() {
            break;
        }
        // next[mi] may overlap with y1..y2. Put overlapping part into cur
        let ny1 = next[mi].1.max(*y1);
        let ny2 = next[mi].2.min(*y2);
        if ny1 <= ny2 {
            cur.push((next[mi].0, ny1, ny2));
        } 
        // the rest of next[mi] may overlap with further ranges, so continue checking
    }
}

pub fn part1(input: &str) -> String {
    let gaps: Vec<(i32, i32, i32)> = input.lines().map(|line| {
        let nums = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        (nums[0], nums[1], nums[2]+nums[1]-1)
    }).collect();
    // reachable ranges at each gap x, start from 0-0 at x=0
    let mut next = vec![(0, 0, 0)];
    let mut cur = vec![(0, 0, 0)];
    let mut cx = 0;
    for (gx, gy1, gy2) in gaps.iter() {
        if cx != *gx {
            expand_merge_gaps(&mut cur, &next);
            next.clear();
            cx = *gx;
        }
        // add the gap range to next
        next.push((*gx, *gy1, *gy2));
    }
    expand_merge_gaps(&mut cur, &next);
    let miny = cur.iter().map(|(_, y1, _)| *y1).min().unwrap();
    ((cur[0].0 + miny + 1) / 2).to_string()
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