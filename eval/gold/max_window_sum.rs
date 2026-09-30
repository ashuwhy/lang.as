pub fn max_window_sum(a: &[i64], k: i64) -> i64 {
    let k = k as usize; let mut s: i64 = a[..k].iter().sum(); let mut best = s;
    for i in k..a.len() { s += a[i] - a[i - k]; if s > best { best = s; } } best
}
