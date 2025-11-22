pub fn part1(input: &str) -> String {
    input.as_bytes().iter().map(|&b| (((b - b'A') * 3) / 2) as usize).sum::<usize>().to_string()
}

fn solve_chunks(input: &str, chunk_size: usize) -> usize {
    input.as_bytes().chunks(chunk_size).map(|chunk| {
        let xcnt = chunk.iter().filter(|&&b| b == b'x').count();
        if xcnt < chunk_size {
            let extra = chunk_size - 1 - xcnt;
            let chunk_cost = chunk.iter().map(|&b| match b {
                b'A' => 0 + extra,
                b'B' => 1 + extra,
                b'C' => 3 + extra,
                b'D' => 5 + extra,
                b'x' => 0,
                _ => unreachable!(),
            }).sum::<usize>();
            chunk_cost
        } else {
            0
        }
    }).sum::<usize>()
}

pub fn part2(input: &str) -> String {
    solve_chunks(input, 2).to_string()
}

pub fn part3(input: &str) -> String {
    solve_chunks(input, 3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1("ABBAC"), "5");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2("AxBCDDCAxD"), "28");
    }
    #[test]
    fn test_part3() {
        let input = "xBxAAABCDxCC";
        assert_eq!(part3(input), "30");
    }
}
