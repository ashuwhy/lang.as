pub fn gcd(a: i64, b: i64) -> Option<i64> { let (mut x, mut y) = (a.unsigned_abs(), b.unsigned_abs()); while y != 0 { let t = x % y; x = y; y = t; } i64::try_from(x).ok() }
