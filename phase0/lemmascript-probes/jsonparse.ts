// E7a: unchecked JSON crossing into a verified function
export type Payout = { user: string; cents: number };
export function parsePayout(body: string): Payout {
  //@ verify
  //@ ensures \result.cents >= 0
  const p = JSON.parse(body) as Payout;
  if (p.cents < 0) return { user: p.user, cents: 0 };
  return p;
}
