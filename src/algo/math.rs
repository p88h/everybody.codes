pub fn inverse_modulus(mut a: i64, mut m: i64) -> i64 {
    let m0 = m;
    let (mut x0, mut x1) = (0, 1);
    if m == 1 {
        return 0;
    }
    while a > 1 {
        let q = a / m;
        (m, a) = (a % m, m);
        (x0, x1) = (x1 - q * x0, x0);
    }
    if x1 < 0 {
        x1 + m0
    } else {
        x1
    }
}

pub fn compute_crt(remainders: &Vec<i64>, moduli: &Vec<i64>) -> i64 {
    let prod: i64 = moduli.iter().product();
    let mut result = 0;
    for (i, &modulus) in moduli.iter().enumerate() {
        let pp = prod / modulus;
        result += remainders[i] * inverse_modulus(pp, modulus) * pp;
    }
    result % prod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse_modulus_basic() {
        assert_eq!(inverse_modulus(3, 11), 4); // 3 * 4 ≡ 1 (mod 11)
        assert_eq!(inverse_modulus(10, 17), 12); // 10 * 12 ≡ 1 (mod 17)
    }

    #[test]
    fn test_inverse_modulus_edge_cases() {
        assert_eq!(inverse_modulus(1, 5), 1); // 1 * 1 ≡ 1 (mod 5)
        assert_eq!(inverse_modulus(2, 1), 0); // m == 1 case
    }

    #[test]
    fn test_compute_crt_simple() {
        let remainders = vec![2, 3, 2];
        let moduli = vec![3, 5, 7];
        let result = compute_crt(&remainders, &moduli);
        assert_eq!(result % 3, 2);
        assert_eq!(result % 5, 3);
        assert_eq!(result % 7, 2);
    }

    #[test]
    fn test_compute_crt_two_equations() {
        let remainders = vec![1, 4];
        let moduli = vec![3, 5];
        let result = compute_crt(&remainders, &moduli);
        assert_eq!(result, 4); // x ≡ 1 (mod 3) and x ≡ 4 (mod 5)
    }

    #[test]
    fn test_compute_crt_all_zeros() {
        let remainders = vec![0, 0, 0];
        let moduli = vec![3, 5, 7];
        assert_eq!(compute_crt(&remainders, &moduli), 0);
    }

    #[test]
    fn test_compute_crt_single_equation() {
        let remainders = vec![5];
        let moduli = vec![7];
        assert_eq!(compute_crt(&remainders, &moduli), 5);
    }
}
