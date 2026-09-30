function FizzBuzz(n: nat): string
{
  if n % 15 == 0 then "FizzBuzz"
  else if n % 3 == 0 then "Fizz"
  else if n % 5 == 0 then "Buzz"
  else NatToString(n)
}

method Main()
{
  var i := 1;
  while i <= 100
    invariant 1 <= i <= 101
  {
    print FizzBuzz(i), "\n";
    i := i + 1;
  }
}
