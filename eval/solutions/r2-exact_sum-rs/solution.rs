/// Returns the exact sum of `a` if it fits in an i64, otherwise `None`.
///
/// Accumulates in i128: with at most 1_000_000 elements, each of magnitude
/// at most 2^63, the running total is bounded by 2^83 in magnitude, so the
/// i128 accumulator can never overflow. Intermediate overflow of i64 is
/// therefore irrelevant; only the final exact total is checked.
pub fn exact_sum(a: &[i64]) -> Option<i64> {
    let total: i128 = a.iter().map(|&x| x as i128).sum();
    i64::try_from(total).ok()
}
