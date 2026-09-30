/// Integer square root: largest r with r*r <= n, for 0 <= n <= i64::MAX.
pub fn isqrt(n: i64) -> i64 {
    if n <= 0 {
        // Spec guarantees n >= 0; treat n == 0 (and defensively any n < 0) as 0.
        return 0;
    }
    let n = n as u128;
    // Floating-point estimate, then exact correction in u128 (no overflow possible).
    let mut r = (n as f64).sqrt() as u128;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r as i64
}
