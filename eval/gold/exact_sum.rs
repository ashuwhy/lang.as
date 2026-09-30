pub fn exact_sum(a: &[i64]) -> Option<i64> { let s: i128 = a.iter().map(|&x| x as i128).sum(); i64::try_from(s).ok() }
