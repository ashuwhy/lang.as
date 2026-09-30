/// Returns floor(amount * bps / 10_000), computed exactly.
///
/// The product can reach 10^19, which exceeds i64::MAX, so the
/// intermediate computation is done in i128. The final result is at most
/// `amount` (since bps <= 10_000), so it always fits in i64.
pub fn percent_of(amount: i64, bps: i64) -> i64 {
    let product = (amount as i128) * (bps as i128);
    // Inputs are non-negative, so truncating division equals floor;
    // div_euclid keeps it a true floor regardless.
    product.div_euclid(10_000) as i64
}
