export function safeDivide(a: number, b: number): number {
  if (b === 0) throw new Error("division by zero");
  return Math.trunc(a / b);
}
