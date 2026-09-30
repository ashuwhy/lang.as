/// Returns floor(amount * bps / 10_000), computed exactly.
///
/// The product can reach 10^15 * 10^4 = 10^19, which exceeds i64::MAX,
/// so the intermediate is computed in i128. The result is at most `amount`
/// (since bps <= 10_000), so it always fits back into i64.
pub fn percent_of(amount: i64, bps: i64) -> i64 {
    let product = (amount as i128) * (bps as i128);
    product.div_euclid(10_000) as i64
}
