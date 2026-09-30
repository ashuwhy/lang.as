method SafeDivide(a: int, b: int) returns (r: int)
  requires b != 0
  ensures r == a / b
{
  r := a / b;
}
