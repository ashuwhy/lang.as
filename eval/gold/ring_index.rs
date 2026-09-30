pub fn ring_index(head: i64, offset: i64, cap: i64) -> i64 { ((head as i128 + offset as i128).rem_euclid(cap as i128)) as i64 }
