# Why the "languages for LLMs" flopped, and what AS does differently

Research round of 2026-09-30, done before finalising `docs/DESIGN.md`. Sources were read by
cloning repositories, by measurement in this container, and through web-search summaries where
the page itself was blocked (tagged **[snippet]**). The measurement scripts are in `bench/tokens/`.

## Short answer

They did not fail on engineering, and they did not fail because models cannot write them. They
failed on four things:

1. **They optimised for the writer, a problem frontier models already solved.** Models now write
   a language they have never seen at about 99% on small problems, given one document in
   context.
2. **They made code harder for the human reviewer**, where the real bottleneck is.
3. **They measured success on saturated toy benchmarks**, which cannot show a language's value,
   instead of on escaped defects and total cost.
4. **They had no wedge into an existing ecosystem**, and some drowned in scope.

AS keeps what worked in them (contracts, effects, SMT checking, diagnostics written as
instructions) and changes those four things.

## 1. The field, measured

| Language | Design bet | Activity | Adoption |
|---|---|---|---|
| Vera (A. Allan, Feb 2026) | No variable names (typed De Bruijn slots `@Int.0`), mandatory contracts, typed effects, Z3, three-tier verdicts, JSON diagnostics with fixes, Wasm target, compiler in Python | 3,147 commits in 7 months by 11 authors; active to 2026-09-26 | 416 stars, 27 forks (415 on 2026-09-24): flat |
| Nanolang (J. Hubbard, Sep 2025) | Prefix (Lisp-like) syntax, mandatory `shadow` tests, transpiles to C; since grown into a VM, capability runtime, "POSIX fabric", Coq proofs, v5.0 | 7,179 commits, 4,724 of them in September 2026 alone | 629 stars (629 on 2026-09-24): flat |
| Aver, AILANG | Statically typed, zero training data, learned from one document | Benchmarked alongside Vera | Not measured |
| Long tail (Revia, intentlang, Codong, Snow, ALaS, Axis, Kern, …) | Various | Mostly 2025-2026 | 3-95 stars each; a GitHub search found 53 repositories in total (earlier note) |

Commit counts and dates are from `git log` on fresh clones; star counts from the GitHub pages.
None has a known production user.

## 2. Models can write a brand-new language; that is no longer the problem

VeraBench (60 problems, five tiers, nine models from three providers, one run each, v0.0.18 on
Vera 0.1.8): **Vera 98.7% average, Python 96.7%, TypeScript 99.7%**. Six of nine models scored
100% in Vera, a language absent from all training data, learned from a single `SKILL.md` in the
prompt. On the five-model subset the three zero-training-data languages scored Vera 98.2%,
AILANG 96.8% and Aver 92.4%, against Python's 97.0% (`vera-bench/README.md`). Hacker News
commenters on Nanolang made the same point: with a grammar and a single explanatory document in
context, models "one-shot" code; what they lack is knowledge of the *tooling* [snippet].

Consequences:

- The cold-start objection the report worried about is mostly answered for frontier models, **if
  the whole language fits in one document** that ships with it.
- A pass@1 benchmark of small problems is saturated (TypeScript 100% for eight of nine models),
  so it cannot show a language helping. Vera's headline "beats Python" rests on one or two
  problems per model, a gap its own README attributes partly to grader artefacts.

## 3. Token efficiency is total cost to a correct result, not source length

**Source length.** Rosetta Code, 92 tasks solved in all 20 languages, comments and blank lines
stripped, mean tokens per task with five tokenizers (o200k, cl100k, Llama 3, Qwen 2.5,
DeepSeek-V3, which agree within about 4%): Ruby 80, Julia 82, Clojure 85, **Python 86**,
Haskell 102, Nim 111, Scala 114, F# 116, OCaml 119, Elixir 134, Swift 140, Kotlin 141,
**Rust 145**, Java 149, **Go 152**, JavaScript 156, C# 190, **C 193**, C++ 208, **Zig 233**
(`bench/tokens/rosetta.mjs`). Statically typed languages with inference and expression syntax
(Haskell, Nim, F#, OCaml) cost 1.2-1.4 times Python; the native systems languages AS competes
with cost 1.7-2.7 times. This matches published comparisons (a 2.6x spread across 19 languages;
APL's glyphs tokenize badly, so terseness through symbols does not pay) [snippet].

**Total cost.** Three 2026 studies show source length is the smaller term:

- Dan Luu found the claim that dynamic languages are cheaper or more correct "does not hold at
  higher effort levels"; reasoning tokens and extra iterations can swamp conciseness, and
  **language popularity correlates better with correctness and cost than static versus
  dynamic typing** [snippet].
- "The Best Programming Language for Tokenmaxxing" (arXiv 2607.22807; 4 languages, 5 models,
  2,000 agent trajectories): language choice significantly changes output tokens even after
  controlling for difficulty, worst for OCaml, then Rust and Java, best for Python. **"What
  matters is not the number of tokens in the final solution, but the amount of reasoning and
  revision needed."** In unfamiliar languages agents repeatedly produce non-compiling code,
  **revise solutions that already pass**, distrust the given tests, and prototype in Python
  first [snippet].
- LangSelect (arXiv 2609.18959): on a 3,000-task, 8-language verified corpus, choosing the target
  language per task cut tokens by 50.3% at 92.9% pass after fallback [snippet].

**Measured on our draft.** The same four programs (Vera's own abs, factorial, safe_divide and
fizzbuzz examples, re-written with identical contracts), o200k tokens (`bench/tokens/corpus.mjs`):

| Language | Tokens | Contracts proved |
|---|---|---|
| AS draft | **230** | yes |
| Dafny | 249 | yes |
| Verus (verified Rust) | 266 | yes |
| Vera | 447 | yes |
| Python / TypeScript / Rust / Go | 165 / 174 / 186 / 191 | no |

Four small programs are indicative only. They say the draft syntax carries proved contracts for
about a quarter more tokens than unverified Rust, and that Vera's slot references plus mandatory
`requires(true) ensures(true) effects(pure)` boilerplate roughly double the cost.

## 4. What the reviewer pays

Vera's central bet, no variable names, has one supporting data point: Vera beats Aver by 1-10
points on five models. Against it: its own example needs a comment to explain that in
`safe_divide(2, 10)` "the rightmost parameter is `@Int.0`", and a recursive call reads
`loop(@Nat.1, @Nat.0 + 1)`. The research's pain point is review, not generation (report,
"Code got cheap in 2026"), and code a human cannot read defeats contracts meant for humans to
judge. Nanolang's prefix syntax (`(+ "Hello, " name)`) imposes a similar cost on readers used to
infix code.

## 5. What agents need from the language itself

- **Greppable code.** Agents spend most of their context on retrieval and mostly use grep; a
  2026 measurement found a language server usually *costs* tokens (+6% to +118% on symbol
  localisation) and saves them only for the weakest model (arXiv 2608.13568) [snippet].
  Ronacher's "A Language for Agents" asks for Go-style qualified names (`context.Context`), no
  macros, braces rather than significant whitespace, and effect markers on functions [snippet].
- **Diagnostics that end the loop.** The revision loop is the cost. Vera's errors-as-instructions
  (what, why, a concrete fix, a spec reference) and JSON output are the right idea and should be
  copied. The stronger lever is a definitive verdict: an agent told "all 14 obligations proved"
  has no reason to re-derive a passing solution or invent its own tests.

## 6. Adoption lessons

- Commits and benchmarks do not create users: Vera's and Nanolang's star counts did not move in
  a week in which they made hundreds of commits.
- Scope creep is a failure mode. Nanolang went from "tiny experimental language" to a verified
  VM, capability runtime and release contracts in a year, with self-hosting still in progress.
- Models default to Python for project work: in LangChoiceBench, Python is 35.3% of
  implementations but 10.7% of top-three recommendations, and fewer than half of implementations
  use a language the model itself recommended (arXiv 2608.06041) [snippet]. A new language will
  not be chosen by agents on its own; a team or a tool has to choose it.
- The earlier notes' adoption history still holds: languages win through an ecosystem wedge
  (TypeScript in JavaScript, Kotlin on the JVM, Zig through C interop and its C toolchain).

## Design consequences (applied to `docs/DESIGN.md`)

1. Position AS as a safe, fast systems language whose programs carry machine-checked intent, for
   teams where agents write the code and humans approve it; do not position it as "a language
   for LLMs".
2. Keep real names and familiar syntax (Rust, Swift, TypeScript, Dafny tokens). No slot
   references, no prefix notation, no new glyphs.
3. No mandatory boilerplate: `requires`, `ensures` and `uses` are omitted when trivial, while
   derived obligations (overflow, bounds, division, exhaustiveness) always apply.
4. Define token efficiency as tokens to a verified result, including reasoning and repair, and
   design for fewer iterations: instruction-style diagnostics with a fix, counterexamples, and a
   definitive proved verdict.
5. The whole language, standard-library essentials and diagnostic index fit in one reference of
   at most 25,000 tokens, shipped as `llms.txt` and an agent skill; a feature that does not fit
   is deferred.
6. Greppable by construction: qualified calls (`io.print`), no glob imports, no macros, no
   overloading, every definition starts with `fn`, `type` or `enum` at column zero.
7. Ecosystem wedge: a C ABI in both directions from the start, so a verified AS module links into
   existing C, C++, Rust, Go or Python programs as a static library with a header.
8. Scope discipline: no VM, runtime platform or package registry before the core is used; each
   milestone has an exit test and a stop rule.
9. Evaluate on unsaturated, realistic tasks (bug-prone modules, boundary and effect bugs), and
   report escaped defects and total agent tokens, not pass@1 on small problems.

## Sources

- Vera: <https://github.com/aallan/vera> (README, FAQ, examples, cloned 2026-09-30);
  VeraBench: <https://github.com/aallan/vera-bench>
- Nanolang: <https://github.com/jordanhubbard/nanolang>; HN discussion
  <https://news.ycombinator.com/item?id=46684958> [snippet]
- Rosetta Code data: <https://github.com/acmeism/RosettaCodeData>
- Dan Luu, "How does programming language affect token efficiency and correctness?"
  <https://danluu.com/pl-tokens/> [snippet]
- M. Alderson, "Which programming languages are most token-efficient?"
  <https://martinalderson.com/posts/which-programming-languages-are-most-token-efficient/>
  [snippet]
- "The Best Programming Language for Tokenmaxxing", <https://arxiv.org/abs/2607.22807> [snippet]
- "LangSelect: Cost-Aware Target-Language Routing for LLM Code Generation",
  <https://arxiv.org/abs/2609.18959> [snippet]
- "LangChoiceBench", <https://arxiv.org/abs/2608.06041> [snippet]
- "Does a Language Server Save Tokens for Coding Agents?", <https://arxiv.org/abs/2608.13568>
  [snippet]
- A. Ronacher, "A Language For Agents", <https://lucumr.pocoo.org/2026/2/9/a-language-for-agents/>
  [snippet]
