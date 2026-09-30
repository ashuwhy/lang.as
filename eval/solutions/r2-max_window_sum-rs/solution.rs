/// Returns the largest sum of `k` consecutive elements of `a`.
///
/// Preconditions (per spec): 1 <= a.len() <= 1_000_000, |a[i]| <= 10^12,
/// 1 <= k <= a.len(). Any window sum is bounded by 10^18 in magnitude, which
/// fits in i64; accumulation is done in i128 so intermediate values can never
/// overflow regardless.
pub fn max_window_sum(a: &[i64], k: i64) -> i64 {
    assert!(!a.is_empty(), "a must be non-empty");
    assert!(k >= 1 && (k as u64) <= a.len() as u64, "k must satisfy 1 <= k <= a.len()");
    let k = k as usize;

    let mut sum: i128 = a[..k].iter().map(|&x| x as i128).sum();
    let mut best = sum;
    for i in k..a.len() {
        sum += a[i] as i128 - a[i - k] as i128;
        if sum > best {
            best = sum;
        }
    }
    best as i64
}
