/// Returns the distinct values of `a` (sorted non-decreasing) in increasing order.
pub fn dedupe_sorted(a: &[i64]) -> Vec<i64> {
    let mut out: Vec<i64> = Vec::with_capacity(a.len());
    for &x in a {
        if out.last() != Some(&x) {
            out.push(x);
        }
    }
    out
}
