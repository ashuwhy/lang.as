/// Rotate `a` left by `k` positions: result[i] = a[(i + k) mod n].
/// Returns an empty vector when `a` is empty.
pub fn rotate_left(a: &[i64], k: i64) -> Vec<i64> {
    let n = a.len();
    if n == 0 {
        return Vec::new();
    }
    // k is specified non-negative; rem_euclid keeps the result in [0, n) regardless.
    // n <= 1_000_000 fits comfortably in i64.
    let shift = k.rem_euclid(n as i64) as usize;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&a[shift..]);
    out.extend_from_slice(&a[..shift]);
    out
}
