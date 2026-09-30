// Fill 10,000,000 integers from a generator, then sum them 20 times.
fn main() uses io {
  let n = 10_000_000
  var a = [0; n]
  var x = 7
  for i in 0..n
    invariant 0 <= x && x < 1_000_003
    invariant forall t in 0..a.len: 0 <= a[t] && a[t] < 1_000_003
  {
    x = (x * 1_103 + 12_345) % 1_000_003
    a[i] = x
  }
  var total = 0
  for r in 0..20
    invariant 0 <= total && total <= r * 10_000_030_000_000
  {
    var s = 0
    for i in 0..n
      invariant 0 <= s && s <= i * 1_000_003
    {
      s += a[i]
    }
    total += s
  }
  io.print(total)
}
