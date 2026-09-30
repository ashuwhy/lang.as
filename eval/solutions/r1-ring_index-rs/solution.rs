/// Returns (head + offset) mod cap in [0, cap), computed exactly without overflow.
pub fn ring_index(head: i64, offset: i64, cap: i64) -> i64 {
    // offset.rem_euclid(cap) is in [0, cap) and never overflows for cap >= 1.
    // head is in [0, cap), so the sum is < 2 * cap <= 2e9, which fits in i64.
    (head + offset.rem_euclid(cap)) % cap
}
