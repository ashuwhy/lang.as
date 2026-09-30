// A pure pricing function that quietly started printing. Effects are part of the
// signature, so this does not compile until `price` declares `uses io`.
pub fn price(base: int, qty: int) -> int
  requires 0 <= base && base <= 1_000_000 && 0 <= qty && qty <= 1_000
{
  io.print("pricing", base)
  base * qty
}
