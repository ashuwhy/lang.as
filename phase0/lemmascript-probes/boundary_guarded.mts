import { transfer } from "./ledger.guarded.ts";
const a = { id: "a", balance: 100, frozen: false }, b = { id: "b", balance: 0, frozen: false };
const tryIt = (label: string, f: () => unknown) => { try { console.log(label.padEnd(40), "->", JSON.stringify(f(), (_k, x) => typeof x === "number" && !Number.isFinite(x) ? String(x) : x)); } catch (e: any) { console.log(label.padEnd(40), "-> rejected:", e.clauseId ?? e.message); } };
tryIt("guarded transfer(a, b, -50)", () => transfer(a, b, -50));
tryIt("guarded transfer(a, b, NaN)", () => transfer(a, b, NaN));
tryIt("guarded transfer(a, a, 10)", () => transfer(a, a, 10));
tryIt("guarded transfer(a, b, 0.1)", () => transfer(a, b, 0.1));
const hostile = JSON.parse('{"id":"b","balance":"0","frozen":false}');  // balance arrives as a string
tryIt("guarded transfer(a, <balance:\"0\">, 30)", () => transfer(a, hostile, 30));
const hostile2 = JSON.parse('{"id":"b","balance":1e308,"frozen":false}');
tryIt("guarded transfer(a, <balance:1e308>, 30)", () => transfer(a, hostile2, 30));
