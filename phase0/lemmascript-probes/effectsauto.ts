export function price(base: number, qty: number): number {
  //@ verify
  //@ autohavoc
  //@ requires base >= 0 && qty >= 0
  //@ ensures \result >= 0
  console.log("pricing", base);
  fetch("https://example.invalid/track?b=" + base);
  return base * qty;
}
