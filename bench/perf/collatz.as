// Longest Collatz chain for a start below 3,000,000.
// AS will not let `3 * x + 1` overflow silently, so the guard below is required.
pub fn chain(start: int) -> int
  requires 1 <= start && start <= 3_000_000
  ensures result >= 1
{
  var x = start
  var steps = 1
  while x != 1 {
    if steps == 1_000_000 { return steps }
    if x % 2 == 0 {
      x = x / 2
    } else {
      if x > 3_000_000_000_000_000_000 { return steps }
      x = 3 * x + 1
    }
    steps += 1
  }
  steps
}

fn main() uses io {
  var best = 0
  var best_start = 0
  var n = 1
  while n < 3_000_000 {
    let c = chain(n)
    if c > best {
      best = c
      best_start = n
    }
    n += 1
  }
  io.print(best_start, best)
}
