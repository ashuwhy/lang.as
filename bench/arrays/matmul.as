// Multiply two 400x400 integer matrices stored as flat arrays, then checksum the result.
fn matmul(a: [int], b: [int], n: int) -> [int]
  requires 1 <= n && n <= 2_000
  requires a.len == n * n && b.len == n * n
  requires forall i in 0..a.len: 0 <= a[i] && a[i] <= 1_000
  requires forall i in 0..b.len: 0 <= b[i] && b[i] <= 1_000
  ensures result.len == n * n
  ensures forall t in 0..result.len: 0 <= result[t] && result[t] <= n * 1_000_000
{
  var c = [0; n * n]
  for i in 0..n
    invariant forall t in 0..c.len: 0 <= c[t] && c[t] <= n * 1_000_000
  {
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

fn fill(n: int, seed: int) -> [int]
  requires 1 <= n && n <= 2_000 && 0 <= seed && seed < 1_000
  ensures result.len == n * n
  ensures forall i in 0..result.len: 0 <= result[i] && result[i] <= 1_000
{
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
