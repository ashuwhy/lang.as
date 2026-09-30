pub fn bucket_counts(values: &[i64], lo: i64, hi: i64, n: i64) -> Vec<i64> {
    let nb = n.max(0) as usize;
    let mut counts = vec![0i64; nb];
    if nb == 0 || hi <= lo {
        return counts;
    }
    let width = (hi as i128) - (lo as i128);
    let n128 = n as i128;
    for &v in values {
        if v < lo || v > hi {
            continue;
        }
        let idx = if v == hi {
            nb - 1
        } else {
            let b = ((v as i128) - (lo as i128)) * n128 / width;
            (b as usize).min(nb - 1)
        };
        counts[idx] += 1;
    }
    counts
}
