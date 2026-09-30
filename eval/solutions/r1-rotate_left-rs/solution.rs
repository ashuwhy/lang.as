/// Rotate `a` left by `k` positions: result[i] = a[(i + k) mod n].
/// Returns an empty vector when `a` is empty. `k` is expected to be non-negative;
/// a negative `k` is handled as a right rotation via Euclidean remainder.
pub fn rotate_left(a: &[i64], k: i64) -> Vec<i64> {
    let n = a.len();
    if n == 0 {
        return Vec::new();
    }
    // n <= 1_000_000 fits in i64; rem_euclid yields a value in [0, n).
    let shift = k.rem_euclid(n as i64) as usize;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&a[shift..]);
    out.extend_from_slice(&a[..shift]);
    out
}
