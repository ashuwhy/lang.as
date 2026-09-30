function Factorial(n: nat): nat
  ensures Factorial(n) >= 1
  decreases n
{
  if n == 0 then 1 else n * Factorial(n - 1)
}
