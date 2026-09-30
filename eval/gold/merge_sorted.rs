pub fn merge_sorted(a: &[i64], b: &[i64]) -> Vec<i64> { let mut v = a.to_vec(); v.extend_from_slice(b); v.sort(); v }
