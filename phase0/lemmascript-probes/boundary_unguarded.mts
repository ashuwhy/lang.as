import { parsePayout } from "./jsonauto.ts";
import { price } from "./effectsauto.ts";
import { transfer } from "./ledger.ts";
const show = (label: string, v: unknown) => console.log(label.padEnd(44), JSON.stringify(v, (_k, x) => (typeof x === "number" && !Number.isFinite(x) ? String(x) : x)));
// parsePayout is proved to ensure result.cents >= 0
for (const body of ['{"user":"a","cents":250}', '{"user":"a","cents":"abc"}', '{"user":"a"}', '{"user":"a","cents":0.5}', '{"user":"a","cents":1e400}']) {
  const r = parsePayout(body); show(`parsePayout(${body})`, r);
  console.log("   proved `cents >= 0` holds:", typeof r.cents === "number" && r.cents >= 0, "| model's safe-integer assumption holds:", Number.isSafeInteger(r.cents));
}
globalThis.fetch = (async () => new Response("")) as typeof fetch;
show("price(10, 3)  [proved pure-looking, did I/O]", price(10, 3));
const a = { id: "a", balance: 100, frozen: false }, b = { id: "b", balance: 0, frozen: false };
// transfer is proved under requires amount >= 0; no guard exists at run time
show("transfer(a, b, -50)  [violates requires]", transfer(a, b, -50));
show("transfer(a, b, NaN)", transfer(a, b, NaN));
show("transfer(a, b, 0.1)  [model says int]", transfer(a, b, 0.1));
show("transfer(a, a, 10)   [violates from.id != to.id]", transfer(a, a, 10));
