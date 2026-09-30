// Sum of integer square roots below 3,000,000, each found by binary search.
fn isqrt(n: int) -> int {
  var lo = 0
  var hi = 1_000_001
  while hi - lo > 1 {
    let mid = lo + (hi - lo) / 2
    if mid * mid <= n { lo = mid } else { hi = mid }
  }
  lo
}

fn main() uses io {
  var total = 0
  var k = 0
  while k < 3_000_000 {
    total += isqrt(k)
    k += 1
  }
  io.print(total)
}
