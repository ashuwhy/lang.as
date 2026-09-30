// 1_000_000 elements of magnitude at most 2^63 sum to under 2^83, so an i128
// accumulator cannot overflow; only the final narrowing can fail.
pub fn exact_sum(a: &[i64]) -> Option<i64> {
    let total: i128 = a.iter().map(|&x| x as i128).sum();
    i64::try_from(total).ok()
}
