/// Counts values of `values` falling into each of `n` equal-width buckets over [lo, hi].
/// Value v in [lo, hi] goes to bucket floor((v - lo) * n / (hi - lo)); v == hi goes to n - 1.
/// Values outside [lo, hi] are ignored.
pub fn bucket_counts(values: &[i64], lo: i64, hi: i64, n: i64) -> Vec<i64> {
    let nb = n.max(0) as usize;
    let mut counts = vec![0i64; nb];
    if nb == 0 || lo >= hi {
        return counts;
    }
    let lo128 = lo as i128;
    let width = hi as i128 - lo128;
    let n128 = n as i128;
    for &v in values {
        if v < lo || v > hi {
            continue;
        }
        let idx = if v == hi {
            nb - 1
        } else {
            let b = ((v as i128 - lo128) * n128 / width) as usize;
            b.min(nb - 1)
        };
        counts[idx] += 1;
    }
    counts
}
