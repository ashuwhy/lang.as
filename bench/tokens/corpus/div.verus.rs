pub fn safe_divide(a: i64, b: i64) -> (r: i64)
    requires b != 0, !(a == i64::MIN && b == -1),
    ensures r == a / b,
{
    a / b
}
