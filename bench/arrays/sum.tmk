// Fill 10,000,000 integers from a generator, then sum them 20 times.
fn main() uses io {
  let n = 10_000_000
  var a = [0; n]
  var x = 7
  for i in 0..n {
    x = (x * 1_103 + 12_345) % 1_000_003
    a[i] = x
  }
  var total = 0
  for r in 0..20 {
    var s = 0
    for i in 0..n {
      s += a[i]
    }
    total += s
  }
  io.print(total)
}
