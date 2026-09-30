// E5: an `any` parameter reaching arithmetic
export function fee(amount: any): number {
  //@ verify
  //@ ensures \result >= 0
  if (amount < 0) return 0;
  return amount;
}
