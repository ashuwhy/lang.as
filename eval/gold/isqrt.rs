pub fn isqrt(n: i64) -> i64 { let (mut lo, mut hi) = (0i128, 3_037_000_500i128); while hi - lo > 1 { let m = (lo + hi) / 2; if m * m <= n as i128 { lo = m } else { hi = m } } lo as i64 }
