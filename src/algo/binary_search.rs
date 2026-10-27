/// Finds the smallest `n >= start` such that `g(n) >= limit`, for a monotonically
/// non-decreasing generator `g`. Uses exponential probing followed by binary search.
pub fn lower_bound(start: i64, limit: i64, g: impl Fn(i64) -> i64) -> i64 {
    if g(start) >= limit {
        return start;
    }
    // invariant: g(lo) < limit
    let mut lo = start;
    let mut step = 1;
    let mut hi = lo + step;
    while g(hi) < limit {
        lo = hi;
        step *= 2;
        hi = lo + step;
    }
    // invariant: g(lo) < limit <= g(hi)
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if g(mid) < limit {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    hi
}
