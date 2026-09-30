// E6: non-null assertion on a map lookup and on an optional field
export type User = { id: string; limit?: number };
export function limitOf(limits: Map<string, number>, u: User): number {
  //@ verify
  return limits.get(u.id)! + u.limit!;
}
