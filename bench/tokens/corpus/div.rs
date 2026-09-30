pub fn safe_divide(a: i64, b: i64) -> Option<i64> {
    a.checked_div(b)
}
