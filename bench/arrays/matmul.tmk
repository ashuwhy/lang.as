// Multiply two 400x400 integer matrices stored as flat arrays, then checksum the result.
fn matmul(a: [int], b: [int], n: int) -> [int] {
  var c = [0; n * n]
  for i in 0..n {
    for j in 0..n
      invariant forall t in 0..c.len: 0 <= c[t] && c[t] <= n * 1_000_000
    {
      var s = 0
      for k in 0..n {
        s += a[i * n + k] * b[k * n + j]
      }
      c[i * n + j] = s
    }
  }
  c
}

fn fill(n: int, seed: int) -> [int] {
  var m = [0; n * n]
  var x = seed
  for i in 0..n * n {
    x = (x * 37 + 11) % 1_000
    m[i] = x
  }
  m
}

fn main() uses io {
  let n = 400
  let c = matmul(fill(n, 1), fill(n, 2), n)
  var check = 0
  for i in 0..c.len {
    check = (check + c[i]) % 1_000_000_007
  }
  io.print(check)
}
