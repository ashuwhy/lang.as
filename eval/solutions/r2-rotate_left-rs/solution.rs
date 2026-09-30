/// Rotate `a` left by `k` positions: result[i] = a[(i + k) mod n].
/// Returns an empty vector when `a` is empty. `k` is expected to be
/// non-negative; a negative `k` is handled as a right rotation via
/// Euclidean remainder so the function never panics.
pub fn rotate_left(a: &[i64], k: i64) -> Vec<i64> {
    let n = a.len();
    if n == 0 {
        return Vec::new();
    }
    // Compute k mod n in i128 to avoid any overflow/sign issues.
    let s = (k as i128).rem_euclid(n as i128) as usize;
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&a[s..]);
    out.extend_from_slice(&a[..s]);
    out
}
