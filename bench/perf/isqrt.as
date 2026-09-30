// Sum of integer square roots below 3,000,000, each found by binary search.
pub fn isqrt(n: int) -> int
  requires 0 <= n && n <= 1_000_000_000_000
  ensures result * result <= n && n < (result + 1) * (result + 1)
{
  var lo = 0
  var hi = 1_000_001
  while hi - lo > 1
    invariant 0 <= lo && lo < hi && hi <= 1_000_001
    invariant lo * lo <= n && n < hi * hi
  {
    let mid = lo + (hi - lo) / 2
    if mid * mid <= n { lo = mid } else { hi = mid }
  }
  lo
}

fn main() uses io {
  var total = 0
  var k = 0
  while k < 3_000_000
    invariant 0 <= k && k <= 3_000_000 && 0 <= total && total <= k * 1_000_000
  {
    total += isqrt(k)
    k += 1
  }
  io.print(total)
}
