/// Greatest common divisor of |a| and |b|, with gcd(x, 0) = |x| and gcd(0, 0) = 0.
///
/// Returns `None` only when the answer is 2^63, which happens when both inputs are
/// in {0, i64::MIN} and at least one is i64::MIN. The arithmetic runs in u64 so that
/// |i64::MIN| is representable while the gcd is computed.
pub fn gcd(a: i64, b: i64) -> Option<i64> {
    let mut x = a.unsigned_abs();
    let mut y = b.unsigned_abs();
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    i64::try_from(x).ok()
}
