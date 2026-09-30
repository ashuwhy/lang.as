// Sort 5,000,000 pseudo-random integers with quicksort, then checksum.
fn quicksort(a: [int], lo: int, hi: int) -> [int] {
  if hi - lo < 2 { return a }
  var v = a
  let pivot = v[hi - 1]
  var store = lo
  for i in lo..hi - 1 {
    if v[i] < pivot {
      let t = v[i]
      v[i] = v[store]
      v[store] = t
      store += 1
    }
  }
  let t = v[store]
  v[store] = v[hi - 1]
  v[hi - 1] = t
  let left = quicksort(v, lo, store)
  quicksort(left, store + 1, hi)
}

fn main() uses io {
  let n = 5_000_000
  var a = [0; n]
  var x = 12345
  for i in 0..n {
    x = (x * 48271) % 1_000_000_007
    a[i] = x
  }
  let s = quicksort(a, 0, n)
  var check = 0
  for i in 0..s.len {
    check = (check * 31 + (s[i] % 1_000 + 1_000) % 1_000) % 1_000_000_007
  }
  io.print(check)
}
