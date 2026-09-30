# Token benchmark (seed)

Two measurements behind `research_notes/New programming language pain points/why_llm_languages_flopped.md`.

```sh
npm install
# 1. Tokens per task on Rosetta Code tasks solved in all 20 languages
git clone --filter=blob:none https://github.com/acmeism/RosettaCodeData.git /tmp/rc
node rosetta.mjs /tmp/rc Python,JavaScript,Go,Rust,C,C++,Java,Kotlin,Swift,Haskell,OCaml,F-Sharp,Ruby,Julia,Nim,Elixir,Clojure,Scala,C-sharp,Zig
# 2. The same four programs, with the same contracts, in Touchmark draft syntax and seven other languages
node corpus.mjs
```

`rosetta.mjs` strips comments and blank lines and takes the first solution file per task and
language. `corpus/*.vera` are Vera's own examples (MIT, Copyright (c) 2026 Alasdair Allan) with
comments and test entry points removed; every other file in `corpus/` was written for this
comparison. The `.tmk` files use the draft syntax in `docs/DESIGN.md`; the compiler does not yet
accept all of them (strings and `for` loops are v0.2).
