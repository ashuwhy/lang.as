/// Returns the largest sum of `k` consecutive elements of `a`.
///
/// Requires 1 <= k <= a.len(). Sums are accumulated in i128 so no
/// intermediate can overflow; the final result is bounded by
/// |k * 10^12| <= 10^18 and therefore fits in i64.
pub fn max_window_sum(a: &[i64], k: i64) -> i64 {
    let n = a.len();
    assert!(k >= 1 && (k as u64) <= n as u64, "k must satisfy 1 <= k <= a.len()");
    let k = k as usize;

    let mut sum: i128 = a[..k].iter().map(|&x| x as i128).sum();
    let mut best = sum;
    for i in k..n {
        sum += a[i] as i128 - a[i - k] as i128;
        if sum > best {
            best = sum;
        }
    }
    best as i64
}
