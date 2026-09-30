/// Returns the largest sum of `k` consecutive elements of `a`.
///
/// Preconditions (per spec): 1 <= a.len() <= 1_000_000, 1 <= k <= a.len(),
/// |a[i]| <= 10^12. Any window sum is bounded by 10^18 in magnitude, and the
/// intermediate `sum + a[i] - a[i-k]` stays within 10^18 + 2*10^12, so i64
/// arithmetic never overflows. i128 is used anyway for defensive robustness.
pub fn max_window_sum(a: &[i64], k: i64) -> i64 {
    let n = a.len();
    assert!(n >= 1, "array must be non-empty");
    assert!(k >= 1 && (k as u64) <= n as u64, "k must satisfy 1 <= k <= len");
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
