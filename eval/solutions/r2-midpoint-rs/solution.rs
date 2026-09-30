/// Returns floor((lo + hi) / 2), computed exactly without overflow.
///
/// Uses the identity lo + hi = 2*(lo & hi) + (lo ^ hi); the arithmetic
/// right shift of (lo ^ hi) floors toward negative infinity, and the sum
/// always lies between lo and hi, so it cannot overflow.
pub fn midpoint(lo: i64, hi: i64) -> i64 {
    (lo & hi) + ((lo ^ hi) >> 1)
}
