use num_complex::Complex;
use rayon::prelude::*;

pub fn cycle1(num: &Complex<i64>) -> Complex<i64> {
    let mut result = Complex::new(0, 0);
    for _ in 0..3 {
        result = result * result;
        result = result / 10;
        result = result + num;
    }
    result
}

pub fn read_complex(input: &str) -> Complex<i64> {
    let parts: Vec<&str> = input[3..input.len() - 1].split(',').collect();
    let x: i64 = parts[0].parse().unwrap();
    let y: i64 = parts[1].parse().unwrap();
    Complex::new(x, y)
}

pub fn part1(input: &str) -> String {
    let c = read_complex(input);
    let res = cycle1(&c);
    format!("[{},{}]", res.re, res.im)
}

pub fn cycle2(num: &Complex<i64>) -> i32 {
    let mut result = Complex::new(0, 0);
    let mut old = result;
    let mut period = 0;
    for d in 0..100 {
        result = result * result;
        result = result / 100000;
        result = result + num;
        if result.re.abs() > 1000000 || result.im.abs() > 1000000 {
            return d;
        }
        if result == old {
            return 100;
        }
        period += 1;
        if period == 10 {
            old = result;
        }
    }
    100
}

pub fn part2(input: &str) -> String {
    let c = read_complex(input);
    let mut total = 0;
    for x in 0..101 {
        for y in 0..101 {
            let test_c = Complex::new(c.re + x * 10, c.im + y * 10);
            if cycle2(&test_c) == 100 {
                total += 1;
            }
        }
    }
    format!("{}", total)
}

pub fn part3(input: &str) -> String {
    let c = read_complex(input);
    let total: i32 = (0..1001)
        .into_par_iter()
        .map(|x| {
            let mut local_total = 0;
            for y in 0..1001 {
                let test_c = Complex::new(c.re + x, c.im + y);
                if cycle2(&test_c) == 100 {
                    local_total += 1;
                }
            }
            local_total
        })
        .sum();
    format!("{}", total)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = "A=[25,9]";
        assert_eq!(part1(input), "[357,862]");
    }

    #[test]
    fn test_cycle2() {
        assert_eq!(cycle2(&Complex::new(35630, -64880)), 100);
        assert_eq!(cycle2(&Complex::new(35630, -64870)), 100);
        assert_eq!(cycle2(&Complex::new(35640, -64860)), 100);
        assert_eq!(cycle2(&Complex::new(36230, -64270)), 100);
        assert_eq!(cycle2(&Complex::new(36250, -64270)), 100);

        assert_eq!(cycle2(&Complex::new(35460, -64910)), 26);
        assert_eq!(cycle2(&Complex::new(35470, -64910)), 27);
        assert_eq!(cycle2(&Complex::new(35480, -64910)), 29);
        assert_eq!(cycle2(&Complex::new(35680, -64850)), 94);
        assert_eq!(cycle2(&Complex::new(35630, -64830)), 99);
    }

    #[test]
    fn test_part2() {
        let input = "A=[35300,-64910]";
        assert_eq!(part2(input), "4076");
    }
    #[test]
    fn test_part3() {
        let input = "A=[35300,-64910]";
        assert_eq!(part3(input), "406954");
    }
}
