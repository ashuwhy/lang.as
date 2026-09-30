// Count primes below 3,000,000 by trial division.
pub fn is_prime(n: int) -> bool
  requires 0 <= n && n <= 100_000_000
{
  if n < 2 { return false }
  var d = 2
  while d * d <= n {
    if n % d == 0 { return false }
    d += 1
  }
  true
}

fn main() uses io {
  var count = 0
  var n = 0
  while n < 3_000_000 {
    if is_prime(n) { count += 1 }
    n += 1
  }
  io.print(count)
}
