export type Payout = { user: string; cents: number };
export function parsePayout(body: string): Payout {
  //@ verify
  //@ autohavoc
  //@ ensures \result.cents >= 0
  const p: Payout = JSON.parse(body);
  if (p.cents < 0) return { user: p.user, cents: 0 };
  return p;
}
