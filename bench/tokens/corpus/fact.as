pub fn factorial(n: nat) -> nat
  requires n <= 20
  ensures result >= 1
  decreases n
{
  if n == 0 { 1 } else { n * factorial(n - 1) }
}
