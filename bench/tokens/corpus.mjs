import fs from "fs";
import { encode as o200 } from "gpt-tokenizer/encoding/o200k_base";
import llama3 from "llama3-tokenizer-js";
const q = (await import("@lenml/tokenizer-qwen2_5")).fromPreTrained();
const dir = "corpus"; const tasks = ["abs", "fact", "div", "fizz"];
const langs = [["Touchmark draft", "tmk"], ["Vera", "vera"], ["Dafny", "dfy"], ["Verus", "verus.rs"], ["Python", "py"], ["TypeScript", "ts"], ["Rust", "rs"], ["Go", "go"]];
console.log("language".padEnd(12) + tasks.map(t => t.padStart(6)).join("") + "  total(o200k)  llama3  qwen2.5  contracts?");
for (const [name, ext] of langs) {
  const src = tasks.map(t => fs.readFileSync(`${dir}/${t}.${ext}`, "utf8"));
  const c = src.map(s => o200(s).length);
  const all = src.join("\n");
  console.log(name.padEnd(12) + c.map(x => String(x).padStart(6)).join("") + String(c.reduce((a, b) => a + b)).padStart(13) + String(llama3.encode(all, { bos: false, eos: false }).length).padStart(8) + String(q.encode(all).length).padStart(9) + "  " + (["as", "vera", "dfy", "verus.rs"].includes(ext) ? "proved" : "none"));
}
