import fs from "fs"; import path from "path";
import { encode as o200 } from "gpt-tokenizer/encoding/o200k_base";
import { encode as cl100 } from "gpt-tokenizer/encoding/cl100k_base";
import llama3 from "llama3-tokenizer-js";
const R = process.argv[2];  // path to a RosettaCodeData checkout
let qwen = null, ds = null;
try { const m = await import("@lenml/tokenizer-qwen2_5"); qwen = m.fromPreTrained(); } catch (e) { console.error("qwen", e.message); }
try { const m = await import("@lenml/tokenizer-deepseek_v3"); ds = m.fromPreTrained(); } catch (e) { console.error("ds", e.message); }
const toks = {
  o200k: s => o200(s).length, cl100k: s => cl100(s).length, llama3: s => llama3.encode(s, { bos: false, eos: false }).length,
  ...(qwen ? { qwen25: s => qwen.encode(s).length } : {}), ...(ds ? { dsv3: s => ds.encode(s).length } : {}),
};
const LINE = { Python: "#", Ruby: "#", Julia: "#", Elixir: "#", Nim: "#", Haskell: "--", Clojure: ";", "F-Sharp": "//", OCaml: null };
function strip(src, lang) {
  let s = src;
  if (["OCaml", "F-Sharp"].includes(lang)) s = s.replace(/\(\*[\s\S]*?\*\)/g, "");
  if (lang === "Haskell") s = s.replace(/\{-[\s\S]*?-\}/g, "");
  if (!["Python", "Ruby", "Julia", "Elixir", "Nim", "Haskell", "Clojure", "OCaml"].includes(lang)) s = s.replace(/\/\*[\s\S]*?\*\//g, "");
  const lc = LINE[lang] === undefined ? "//" : LINE[lang];
  if (lc) s = s.split("\n").map(l => { const i = l.indexOf(lc); return i >= 0 && !l.slice(0, i).includes('"') && !l.slice(0, i).includes("'") ? l.slice(0, i) : l; }).join("\n");
  if (lang === "Python") s = s.replace(/("""|''')[\s\S]*?\1/g, "");
  return s.split("\n").map(l => l.replace(/\s+$/, "")).filter(l => l.trim()).join("\n");
}
const langs = process.argv[3].split(",");
const tasks = fs.readdirSync(path.join(R, "Task")).filter(t => langs.every(l => fs.existsSync(path.join(R, "Task", t, l))));
const res = {}; for (const l of langs) res[l] = Object.fromEntries(Object.keys(toks).map(k => [k, []]));
for (const t of tasks) for (const l of langs) {
  const d = path.join(R, "Task", t, l); const f = fs.readdirSync(d).filter(x => !x.startsWith(".")).sort()[0];
  const src = strip(fs.readFileSync(path.join(d, f), "utf8"), l);
  for (const [k, fn] of Object.entries(toks)) res[l][k].push(fn(src));
}
const mean = a => a.reduce((x, y) => x + y, 0) / a.length;
const base = res["Python"];
console.log(`tasks in common: ${tasks.length}`);
console.log("lang".padEnd(12) + Object.keys(toks).map(k => k.padStart(9)).join("") + "   ratio-vs-Python(o200k, median per task)");
const rows = langs.map(l => [l, Object.keys(toks).map(k => mean(res[l][k])), (() => { const r = res[l].o200k.map((v, i) => v / base.o200k[i]).sort((a, b) => a - b); return r[Math.floor(r.length / 2)]; })()]);
rows.sort((a, b) => a[1][0] - b[1][0]);
for (const [l, ms, med] of rows) console.log(l.padEnd(12) + ms.map(m => m.toFixed(0).padStart(9)).join("") + "   " + med.toFixed(2));
