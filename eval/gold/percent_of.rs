pub fn percent_of(amount: i64, bps: i64) -> i64 { ((amount as i128 * bps as i128) / 10_000) as i64 }
