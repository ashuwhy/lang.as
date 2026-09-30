pub fn midpoint(lo: i64, hi: i64) -> i64 { ((lo as i128 + hi as i128).div_euclid(2)) as i64 }
