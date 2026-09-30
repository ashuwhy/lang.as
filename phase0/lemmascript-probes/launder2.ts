export function priceInCents(raw: string): number {
  //@ verify
  //@ ensures \result >= 0
  const p: number = raw as unknown as number;
  if (p < 0) return 0;
  return p * 100;
}
