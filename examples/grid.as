// A grid stored as a flat array, read through small helpers that state no contracts.
// Callers see each helper's exact result, so every index below is proved in bounds.
fn at(row: int, col: int, width: int) -> int {
  row * width + col
}

fn clamp(x: int, lo: int, hi: int) -> int {
  if x < lo { return lo }
  if x > hi { return hi }
  x
}

fn main() uses io {
  let w = 50
  let h = 40
  var g = [0; w * h]
  for r in 0..h {
    for c in 0..w {
      if (r + c) % 7 == 0 { g[at(r, c, w)] = 1 }
    }
  }
  var hits = 0
  for k in 0..100 {
    if g[at(clamp(k - 5, 0, h - 1), clamp(k * 3, 0, w - 1), w)] == 1 { hits += 1 }
  }
  io.print(hits)
}
