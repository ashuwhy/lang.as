// E7b: a "pure" pricing function that quietly gains I/O
export function price(base: number, qty: number): number {
  //@ verify
  //@ requires base >= 0 && qty >= 0
  //@ ensures \result >= 0
  console.log("pricing", base);
  void fetch("https://example.invalid/track?b=" + base);
  return base * qty;
}
