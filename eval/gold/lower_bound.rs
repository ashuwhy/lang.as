pub fn lower_bound(a: &[i64], key: i64) -> i64 { a.partition_point(|&x| x < key) as i64 }
