/// Returns the smallest index `i` such that `a[i] >= key`, or `a.len()` if none.
/// `a` must be sorted in non-decreasing order.
pub fn lower_bound(a: &[i64], key: i64) -> i64 {
    // Half-open search range [lo, hi); usize arithmetic avoids overflow.
    let mut lo: usize = 0;
    let mut hi: usize = a.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if a[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo as i64
}
