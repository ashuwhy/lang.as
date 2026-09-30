/// Returns floor(amount * bps / 10_000), computed exactly.
///
/// The product can reach 10^15 * 10^4 = 10^19, which exceeds i64::MAX
/// (~9.22 * 10^18), so the intermediate is computed in i128. Both inputs
/// are non-negative per the spec, so integer division is floor division,
/// and the result is at most `amount`, so it always fits back in i64.
pub fn percent_of(amount: i64, bps: i64) -> i64 {
    let product = (amount as i128) * (bps as i128);
    product.div_euclid(10_000) as i64
}
