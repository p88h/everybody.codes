#[derive(Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Segment {
    left: Option<i32>,
    right: Option<i32>,
    center: Option<i32>,
}

#[derive(Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Fishbone {
    scores: Vec<i64>,
    id: i32,
    segments: Vec<Segment>,
}

fn make_fishbone(input: &str) -> Fishbone {
    let pc = input.find(':').unwrap();
    let id = input[..pc].parse::<i32>().unwrap_or(0);
    let parts = input[pc + 1..].split(',').filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<i32>>();
    let mut fishbone = Fishbone { id, segments: vec![Segment::default()], scores: vec![0] };
    for &value in parts.iter() {
        for seg in fishbone.segments.iter_mut() {
            if seg.center.is_some() {
                if value < seg.center.unwrap() && seg.left.is_none() {
                    seg.left = Some(value);
                    break;
                }
                if value > seg.center.unwrap() && seg.right.is_none() {
                    seg.right = Some(value);
                    break;
                }
            } else {
                seg.center = Some(value);
            }
        }
        if fishbone.segments[fishbone.segments.len() - 1].center.is_some() {
            fishbone.segments.push(Segment::default());
        }
    }
    fishbone.segments.pop();
    let mut result = String::new();
    for seg in fishbone.segments.iter_mut() {
        let mut score = seg.center.unwrap();
        if let Some(l) = seg.left {
            score += l * 10
        }
        if let Some(r) = seg.right {
            score = score * 10 + r
        }
        // println!("{}-{}-{}\n  |", seg.left.unwrap_or(0), seg.center.unwrap(), seg.right.unwrap_or(0));
        result.push_str(&format!("{}", seg.center.unwrap()));
        fishbone.scores.push(score.into());
    }
    fishbone.scores[0] = result.parse::<i64>().unwrap();
    fishbone
}

pub fn part1(input: &str) -> String {
    let fishbone = make_fishbone(input);
    format!("{}", fishbone.scores[0])
}

pub fn part2(input: &str) -> String {
    let scores = input.lines().map(|line| make_fishbone(line).scores[0]).collect::<Vec<i64>>();
    let min = scores.iter().min().unwrap_or(&0);
    let max = scores.iter().max().unwrap_or(&0);
    format!("{}", max - min)
}

pub fn part3(input: &str) -> String {
    let mut bones = input.lines().map(|line| make_fishbone(line)).collect::<Vec<Fishbone>>();
    bones.sort();
    bones.reverse();
    let result = bones.iter().enumerate().map(|(i, bone)| bone.id * (i as i32 + 1)).sum::<i32>();
    format!("{:?}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "58:5,3,7,8,9,10,4,5,7,8,8";
        assert_eq!(part1(input), "581078");
    }

    #[test]
    fn test_part2() {
        let input = "1:2,4,1,1,8,2,7,9,8,6
2:7,9,9,3,8,3,8,8,6,8
3:4,7,6,9,1,8,3,7,2,2
4:6,4,2,1,7,4,5,5,5,8
5:2,9,3,8,3,9,5,2,1,4
6:2,4,9,6,7,4,1,7,6,8
7:2,3,7,6,2,2,4,1,4,2
8:5,1,5,6,8,3,1,8,3,9
9:5,7,7,3,7,2,3,8,6,7
10:4,1,9,3,8,5,4,3,5,5
";
        assert_eq!(part2(input), "77053");
    }

    #[test]
    fn test_part3() {
        let input = "1:7,1,9,1,6,9,8,3,7,2\n2:7,1,9,1,6,9,8,3,7,2";
        assert_eq!(part3(input), "4");
        let big_input = "1:7,1,9,1,6,9,8,3,7,2
2:6,1,9,2,9,8,8,4,3,1
3:7,1,9,1,6,9,8,3,8,3
4:6,1,9,2,8,8,8,4,3,1
5:7,1,9,1,6,9,8,3,7,3
6:6,1,9,2,8,8,8,4,3,5
7:3,7,2,2,7,4,4,6,3,1
8:3,7,2,2,7,4,4,6,3,7
9:3,7,2,2,7,4,1,6,3,7";
        assert_eq!(part3(big_input), "260");
    }
}
