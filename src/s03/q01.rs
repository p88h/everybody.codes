fn parse_bits(bits: &str) -> Vec<i32> {
    // bits is 6 chars like 'rrrrrr' or 'ggGgGG', lowercase is 0, uppercase is 1
    bits.split_whitespace()
        .map(|b| b.chars().fold(0, |acc, c| (acc << 1) | if c.is_uppercase() { 1 } else { 0 }))
        .collect()
}

fn parse_line(line: &str) -> (i32, Vec<i32>) {
    let (id, bits) = line.split_once(':').unwrap();
    let id: i32 = id.parse().unwrap();
    let values: Vec<i32> = parse_bits(bits);
    (id, values)
}

pub fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    // format is '<id>:<bits> <bits> <bits>'
    let mut total = 0;
    for line in lines {
        let (id, values) = parse_line(line);
        if values[1] > values[0] && values[1] > values[2] {
            total += id;
        }
        // now values has 8-bit per color
    }
    total.to_string()
}

pub fn part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    // format is '<id>:<bits> <bits> <bits>'
    let mut best = 0;
    let mut shiniest = 0;
    let mut darkest = 1024;
    for line in lines {
        let (id, values) = parse_line(line);
        let tint = values[0] + values[1] + values[2];
        if values[3] >= shiniest && tint < darkest {
            shiniest = values[3];
            darkest = tint;
            best = id;
        }
    }
    best.to_string()
}

pub fn part3(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut groups = vec![0; 6];
    let mut sums: Vec<i32> = vec![0; 6];
    for line in lines {
        let (id, values) = parse_line(line);
        let mut dominant_color = 0;
        if values[1] > values[0] {
            dominant_color = 1;
        }
        if values[2] > values[dominant_color] {
            dominant_color = 2;
        }
        let a = (dominant_color + 1) % 3;
        let b= (dominant_color + 2) % 3;
        if values[dominant_color] <= values[a] || values[dominant_color] <= values[b] {
            continue;
        }
        let group_id;
        if values[3] <= 30 {
            group_id = dominant_color * 2;
        } else if values[3] >= 33 {
            group_id = dominant_color * 2 + 1;
        } else {
            continue;
        }
        groups[group_id as usize] += 1;
        sums[group_id as usize] += id;
    }
    let best = groups.iter().enumerate().max_by(|(_, a), (_, b)| a.cmp(b)).map(|(i, _)| i);
    sums[best.unwrap() as usize].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
2456:rrrrrr ggGgGG bbbbBB
7689:rrRrrr ggGggg bbbBBB
3145:rrRrRr gggGgg bbbbBB
6710:rrrRRr ggGGGg bbBBbB";
        assert_eq!(part1(input), "9166");
    }

    #[test]
    fn test_part2() {
        let input = "\
2456:rrrrrr ggGgGG bbbbBB sSsSsS
7689:rrRrrr ggGggg bbbBBB ssSSss
3145:rrRrRr gggGgg bbbbBB sSsSsS
6710:rrrRRr ggGGGg bbBBbB ssSSss";
        assert_eq!(part2(input), "2456");
    }

    #[test]
    fn test_part3() {
        let input = "\
15437:rRrrRR gGGGGG BBBBBB sSSSSS
94682:RrRrrR gGGggG bBBBBB ssSSSs
56513:RRRrrr ggGGgG bbbBbb ssSsSS
76346:rRRrrR GGgggg bbbBBB ssssSs
87569:rrRRrR gGGGGg BbbbbB SssSss
44191:rrrrrr gGgGGG bBBbbB sSssSS
49176:rRRrRr GggggG BbBbbb sSSssS
85071:RRrrrr GgGGgg BBbbbb SSsSss
44303:rRRrrR gGggGg bBbBBB SsSSSs
94978:rrRrRR ggGggG BBbBBb SSSSSS
26325:rrRRrr gGGGgg BBbBbb SssssS
43463:rrrrRR gGgGgg bBBbBB sSssSs
15059:RRrrrR GGgggG bbBBbb sSSsSS
85004:RRRrrR GgGgGG bbbBBB sSssss
56121:RRrRrr gGgGgg BbbbBB sSsSSs
80219:rRRrRR GGGggg BBbbbb SssSSs";
        assert_eq!(part3(input), "292320");
    }
}
