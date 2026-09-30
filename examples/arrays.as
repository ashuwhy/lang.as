// Arrays: every index below is proved in bounds, so the binary has no bounds checks.

pub fn sum(a: [int]) -> int
  requires a.len <= 1_000_000
  requires forall i in 0..a.len: 0 <= a[i] && a[i] <= 1_000_000
  ensures result >= 0
{
  var s = 0
  for i in 0..a.len
    invariant 0 <= s && s <= i * 1_000_000
  {
    s += a[i]
  }
  s
}

// Binary search in a sorted array: returns an index holding `key`, or -1 if there is none.
pub fn search(a: [int], key: int) -> int
  requires forall i in 0..a.len: forall j in i..a.len: a[i] <= a[j]
  ensures result == -1 || (0 <= result && result < a.len && a[result] == key)
  ensures result == -1 ==> forall i in 0..a.len: a[i] != key
{
  var lo = 0
  var hi = a.len
  while lo < hi
    invariant 0 <= lo && lo <= hi && hi <= a.len
    invariant forall i in 0..lo: a[i] < key
    invariant forall i in hi..a.len: a[i] > key
    decreases hi - lo
  {
    let mid = lo + (hi - lo) / 2
    if a[mid] < key {
      lo = mid + 1
    } else if a[mid] > key {
      hi = mid
    } else {
      return mid
    }
  }
  -1
}

// Sieve of Eratosthenes: how many primes are below n.
pub fn count_primes(n: int) -> int
  requires 2 <= n && n <= 100_000_000
  ensures 0 <= result && result <= n
{
  var composite = [false; n]
  var count = 0
  for i in 2..n
    invariant 0 <= count && count <= i
  {
    if !composite[i] {
      count += 1
      var j = i * i
      while j < n
        invariant j >= 0
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
  let a = [1, 3, 5, 7, 9, 11]
  io.print("sum =", sum(a))
  io.print("search 7 =", search(a, 7), " search 4 =", search(a, 4))
  io.print("primes below 1000 =", count_primes(1000))
}
