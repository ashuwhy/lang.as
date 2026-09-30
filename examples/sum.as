// Sum of 1..n with a loop, proved equal to the closed form n(n+1)/2.
pub fn sum_to(n: int) -> int
  requires 0 <= n && n <= 1_000_000
  ensures 2 * result == n * (n + 1)
{
  var total = 0
  var i = 0
  while i < n
    invariant 0 <= i && i <= n
    invariant 2 * total == i * (i + 1)
    decreases n - i
  {
    i += 1
    total += i
  }
  total
}

fn main() uses io {
  io.print("sum_to(1000) =", sum_to(1000))
}
