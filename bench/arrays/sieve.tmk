// Count primes below 50,000,000 with the sieve of Eratosthenes.
fn count_primes(n: int) -> int {
  var composite = [false; n]
  var count = 0
  for i in 2..n {
    if !composite[i] {
      count += 1
      var j = i * i
      while j < n
        decreases n - j
      {
        composite[j] = true
        j += i
      }
    }
  }
  count
}

fn main() uses io {
  io.print(count_primes(50_000_000))
}
