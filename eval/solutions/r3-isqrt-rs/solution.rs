pub fn isqrt(n: i64) -> i64 {
    let n = n as u64;
    // f64 has 53 bits of mantissa, so the estimate can be off by one near
    // 2^63. The root never exceeds 3037000500, so r*r and (r+1)^2 fit in u64.
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r as i64
}
