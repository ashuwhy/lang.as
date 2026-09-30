/// Returns (head + offset) mod cap in [0, cap), computed exactly without overflow.
/// Requires 1 <= cap <= 1_000_000_000 and 0 <= head < cap.
pub fn ring_index(head: i64, offset: i64, cap: i64) -> i64 {
    // Reduce offset first so the sum cannot overflow; use i128 for full safety.
    let r = (head as i128 + (offset as i128).rem_euclid(cap as i128)).rem_euclid(cap as i128);
    r as i64
}
