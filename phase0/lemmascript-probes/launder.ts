// E4: string laundered into a number through a double cast (SepInfer Class C shape)
export function priceInCents(raw: string): number {
  //@ verify
  //@ ensures \result >= 0
  const p = raw as unknown as number;
  if (p < 0) return 0;
  return p * 100;
}
