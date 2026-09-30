export type Account = { id: string; balance: number; frozen: boolean };
export type Transfer =
  | { tag: "ok"; from: Account; to: Account }
  | { tag: "frozen" }
  | { tag: "insufficient"; short: number };

export function transfer(from: Account, to: Account, amount: number): Transfer {
  //@ verify
  //@ requires from.id !== to.id
  //@ requires amount >= 0 && from.balance >= 0 && to.balance >= 0
  //@ ensures \result.tag === "ok" ==> \result.from.balance === from.balance - amount && \result.to.balance === to.balance + amount
  //@ ensures \result.tag === "frozen" <==> (from.frozen || to.frozen)
  //@ ensures \result.tag === "ok" ==> \result.from.balance >= 0
  if (from.frozen || to.frozen) return { tag: "frozen" };
  if (from.balance < amount) return { tag: "insufficient", short: amount - from.balance };
  return { tag: "ok", from: { ...from, balance: from.balance - amount }, to: { ...to, balance: to.balance + amount } };
}
