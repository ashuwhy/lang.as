// The binary-search midpoint bug that sat in Java's standard library for nine years
// (Arrays.binarySearch, fixed in 2006). `aslang check` rejects it with a counterexample.
pub fn midpoint(lo: int, hi: int) -> int
  requires 0 <= lo && lo <= hi
  ensures lo <= result && result <= hi
{
  (lo + hi) / 2
}

// The fix. This version proves, so the machine code has no checks at all.
pub fn midpoint_fixed(lo: int, hi: int) -> int
  requires 0 <= lo && lo <= hi
  ensures lo <= result && result <= hi
{
  lo + (hi - lo) / 2
}
