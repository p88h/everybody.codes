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

// Same rule as cycle2, run on LANES points at once in f64 so the compiler can use SIMD. While a
// point is still inside, |re|, |im| <= 1e6, so every product is below 2^53 and exact, and
// trunc(q / 1e5) equals integer division (quotients are at most 2e7, far coarser than f64 spacing).
// Escaped lanes may grow to inf/NaN; the sticky mask makes that harmless.
const LANES: usize = 16;

fn count_inside(re: f64, im: [f64; LANES], valid: usize) -> usize {
    let (mut x, mut y) = ([0.0f64; LANES], [0.0f64; LANES]);
    let mut out = [false; LANES];
    for _ in 0..100 {
        for l in 0..LANES {
            let nx = ((x[l] * x[l] - y[l] * y[l]) / 1e5).trunc() + re;
            let ny = ((2.0 * x[l] * y[l]) / 1e5).trunc() + im[l];
            x[l] = nx;
            y[l] = ny;
            out[l] |= nx.abs() > 1e6 || ny.abs() > 1e6;
        }
        if out.iter().all(|&o| o) {
            return 0;
        }
    }
    out[..valid].iter().filter(|&&o| !o).count()
}

/// Counts points c + step * (x, y), for x, y in 0..n, that never escape.
fn count_grid(c: Complex<i64>, step: i64, n: i64) -> usize {
    (0..n)
        .into_par_iter()
        .map(|x| {
            let re = (c.re + x * step) as f64;
            let mut total = 0;
            for y0 in (0..n).step_by(LANES) {
                let im = std::array::from_fn(|l| (c.im + (y0 + l as i64) * step) as f64);
                total += count_inside(re, im, (n - y0).min(LANES as i64) as usize);
            }
            total
        })
        .sum()
}

pub fn part2(input: &str) -> String {
    count_grid(read_complex(input), 10, 101).to_string()
}

pub fn part3(input: &str) -> String {
    count_grid(read_complex(input), 1, 1001).to_string()
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
