// Integer square root by binary search. The loop invariant carries the proof.
pub fn isqrt(n: int) -> int
  requires 0 <= n && n <= 1_000_000_000_000
  ensures result * result <= n && n < (result + 1) * (result + 1)
{
  var lo = 0
  var hi = 1_000_001
  while hi - lo > 1
    invariant 0 <= lo && lo < hi && hi <= 1_000_001
    invariant lo * lo <= n && n < hi * hi
    decreases hi - lo
  {
    let mid = lo + (hi - lo) / 2
    if mid * mid <= n { lo = mid } else { hi = mid }
  }
  lo
}

fn main() uses io {
  io.print("isqrt(1_000_000_000_000) =", isqrt(1_000_000_000_000))
  io.print("isqrt(99) =", isqrt(99))
}
