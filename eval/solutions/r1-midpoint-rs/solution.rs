/// Returns floor((lo + hi) / 2), with lo + hi computed exactly.
///
/// Uses the identity lo + hi == 2*(lo & hi) + (lo ^ hi). The arithmetic
/// right shift of (lo ^ hi) floors toward negative infinity, and the sum
/// always lies between lo and hi, so no intermediate step can overflow.
pub fn midpoint(lo: i64, hi: i64) -> i64 {
    (lo & hi) + ((lo ^ hi) >> 1)
}
