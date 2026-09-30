/// Returns (head + offset) mod cap in the range [0, cap), computed exactly.
///
/// Requires 1 <= cap <= 1_000_000_000 and 0 <= head < cap; offset may be any i64.
/// Reducing `offset` first with `rem_euclid` avoids overflow for extreme offsets
/// (e.g. i64::MIN / i64::MAX): the intermediate sum is below 2 * cap <= 2e9.
pub fn ring_index(head: i64, offset: i64, cap: i64) -> i64 {
    let off = offset.rem_euclid(cap);
    (head + off) % cap
}
