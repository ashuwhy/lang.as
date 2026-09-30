pub fn rotate_left(a: &[i64], k: i64) -> Vec<i64> {
    if a.is_empty() { return vec![]; } let k = (k as u64 % a.len() as u64) as usize; let mut v = a.to_vec(); v.rotate_left(k); v
}
