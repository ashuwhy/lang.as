pub fn abs(x: int) -> int
  requires x != int.min
  ensures result >= 0
  ensures result == x || result == -x
{
  if x >= 0 { x } else { -x }
}
