pub fn abs(x: i64) -> (r: i64)
    requires x > i64::MIN,
    ensures r >= 0, r == x || r == -x,
{
    if x >= 0 { x } else { -x }
}
