use crate::algo::strings::*;

pub fn part1(input: &str) -> String {
    // println!("input: {}", input);
    let (header, text) = input.split_once("\n\n").unwrap();
    let words = header.split_once(':').unwrap().1.split(',').collect::<Vec<_>>();
    let matcher = MultiStringMatcher::new(words);
    matcher.find_all_matches(text.as_bytes()).len().to_string()
}

pub fn part2(input: &str) -> String {
    let (header, text) = input.split_once("\n\n").unwrap();
    let words = header.split_once(':').unwrap().1.split(',').collect::<Vec<_>>();
    let matcher = MultiStringMatcher::new(words);
    let mut matching_pos = vec![false; text.len()];
    for (cpos, clen) in matcher.find_all_matches(text.as_bytes()) {
        for i in 0..clen {
            matching_pos[cpos + i] = true;
        }
    }
    // now reverse the text and do the same
    let rev_text: Vec<u8> = text.as_bytes().iter().copied().rev().collect();
    let rev_matches = matcher.find_all_matches(&rev_text);
    for (cpos, clen) in rev_matches {
        let start = text.len() - cpos - clen;
        for i in 0..clen {
            matching_pos[start + i] = true;
        }
    }
    matching_pos.iter().filter(|&&b| b).count().to_string()
}

fn rotate_clockwise<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut rotated = vec![vec![]; ncols];
    for c in 0..ncols {
        for r in (0..nrows).rev() {
            rotated[c].push(grid[r][c].clone());
        }
    }
    rotated
}

pub fn part3(input: &str) -> String {
    let (header, text) = input.split_once("\n\n").unwrap();
    let words = header.split_once(':').unwrap().1.split(',').collect::<Vec<_>>();
    let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);
    let matcher = MultiStringMatcher::new(words);
    let mut text_grid = text.lines().map(|line| line.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut match_grid = vec![vec![false; text_grid[0].len()]; text_grid.len()];
    for i in 0..4 {
        let width = match_grid[0].len();
        for j in 0..text_grid.len() {
            let mut tmp_row = text_grid[j].clone();
            if i % 2 == 0 {
                // extend horizontally for wrap around
                let extra = max_len.min(width);
                tmp_row.extend_from_slice(&text_grid[j][..extra]);
            }
            for (cpos, clen) in matcher.find_all_matches(&tmp_row) {
                for k in 0..clen {
                    match_grid[j][(cpos + k) % width] = true;
                }
            }
        }
        text_grid = rotate_clockwise(&text_grid);
        match_grid = rotate_clockwise(&match_grid);
    }
    // debug print
    // for j in 0..text_grid.len() {
    //     for k in 0..text_grid[0].len() {
    //         if match_grid[j][k] {
    //             print!("\x1b[1;32m{}\x1b[0m", text_grid[j][k] as char);
    //         } else {
    //             print!("{}", text_grid[j][k] as char);
    //         }
    //     }
    //     println!();
    // }
    // println!();
    match_grid.iter().flatten().filter(|&&b| b).count().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "\
WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE";
        assert_eq!(part1(input), "4");
    }

    #[test]
    fn test_part2() {
        let input = "\
WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ";
        assert_eq!(part2(input), "42");
    }

    #[test]
    fn test_part3() {
        let input = "\
WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL";
        assert_eq!(part3(input), "10");
    }
}
