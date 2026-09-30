export function limitOf(limits: Map<string, number>, id: string): number {
  //@ verify
  return limits.get(id)!;
}
