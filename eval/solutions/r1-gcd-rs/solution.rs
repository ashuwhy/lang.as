/// Greatest common divisor of |a| and |b|.
///
/// gcd(x, 0) = |x|, gcd(0, 0) = 0. Returns `None` when the result does not
/// fit in an `i64` (only possible when the result is 2^63, i.e. for
/// (i64::MIN, 0), (0, i64::MIN) and (i64::MIN, i64::MIN)).
pub fn gcd(a: i64, b: i64) -> Option<i64> {
    // Work in u64 so |i64::MIN| = 2^63 is representable.
    let mut x = a.unsigned_abs();
    let mut y = b.unsigned_abs();
    while y != 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    i64::try_from(x).ok()
}
