pub fn bucket_counts(values: &[i64], lo: i64, hi: i64, n: i64) -> Vec<i64> {
    let mut out = vec![0i64; n as usize];
    for &v in values { if v < lo || v > hi { continue; } let j = if v == hi { n - 1 } else { ((v - lo) as i128 * n as i128 / (hi - lo) as i128) as i64 }; out[j as usize] += 1; }
    out
}
