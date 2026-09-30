pub fn factorial(n: u64) -> (r: u64)
    requires n <= 20,
    ensures r >= 1,
    decreases n,
{
    if n == 0 { 1 } else { n * factorial(n - 1) }
}
