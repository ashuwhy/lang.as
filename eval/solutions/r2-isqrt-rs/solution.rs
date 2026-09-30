/// Integer square root: the largest r with r * r <= n, for 0 <= n <= i64::MAX.
///
/// Uses a floating-point estimate, then corrects it exactly with 128-bit
/// arithmetic so the result is exact across the whole domain.
pub fn isqrt(n: i64) -> i64 {
    assert!(n >= 0, "isqrt: n must be non-negative");
    let n128 = n as i128;
    let mut r = (n as f64).sqrt() as i128;
    // Correct downward while r*r > n.
    while r * r > n128 {
        r -= 1;
    }
    // Correct upward while (r+1)^2 <= n.
    while (r + 1) * (r + 1) <= n128 {
        r += 1;
    }
    r as i64
}
