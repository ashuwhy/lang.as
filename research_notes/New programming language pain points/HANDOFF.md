# Handoff: new programming language research

Read this file before doing anything else in a new session.

## The request

Find the single biggest real pain point in software development today (September 2026)
that a new programming language could solve, for millions of developers and industries.
Base it on frontier research (PLDI, POPL, OOPSLA, ICFP, ECOOP, CGO, CC, CAV, ASPLOS,
OSDI/SOSP, MLSys, NeurIPS/ICML/ICLR), developer surveys and complaints (Python, C, C++,
Rust, Go, JavaScript/TypeScript, Java, Zig, Mojo and others), industry-scale costs, and
the history of why new languages succeed or fail. Then:

1. Rank the candidate pain points by size (people and money affected), by whether a
   language rather than a tool is the right fix, and by whether a solo developer or small
   team could realistically win.
2. Recommend ONE concrete language idea: core semantics, killer feature, target users,
   adoption wedge, and the existing work it builds on.
3. Give a realistic roadmap that reuses or replaces the AS Lang codebase in this repo,
   and builds on the author's SepInfer research where it fits.
4. Be honest about risks and about why most new languages fail.

## Status

Done: six research notes in this folder, all on branch `research/language-pain-points`.

| File | Covers |
|---|---|
| `frontier_pl_research.md` | PL conference themes 2023–2026, industry adoption of PL research, open problems |
| `ai_code_generation.md` | AI coding adoption, measured quality problems, verification research, languages for AI |
| `developer_complaints.md` | Stack Overflow, JetBrains, Octoverse and per-language pain points, ranked |
| `industry_scale_pain.md` | Memory safety, supply chain, outages, concurrency, legacy, with costs |
| `performance_and_gpu.md` | Two-language problem, GPU kernel languages, Mojo, Python speed work |
| `new_languages_adoption.md` | 28+ languages' status, adoption research, solo-team timelines, LLM factor, 2026 toolchain bar |

Reachability re-tested on 2026-09-24 (second session): every blocked domain below was still
blocked from the container even after the environment's network setting was changed to allow
all domains, which suggests the change only reaches containers started after it. Results and
the hosts that do work are in `network_check.md`. Start the next session by re-running that
probe (for example `curl -s -o /dev/null -w '%{http_code}' https://arxiv.org/`, where `000`
means blocked) before launching the gap round.

Not done:

1. **Gap-filling research.** The first round ran with most primary sites blocked by the
   network proxy and a search budget that ran out, so many figures come from search-result
   snippets. They are tagged `[TK]`, `[search snippet]`, `[VENDOR]`, "conflicting", or listed
   under each file's "Gaps" sections. Blocked domains were: survey.stackoverflow.co,
   jetbrains.com, github.blog, arxiv.org, dl.acm.org, usenix.org, conf.researchr.org,
   *.sigplan.org, dblp.org, mlsys.org, openreview.net, metr.org, blog.google,
   security.googleblog.com, cisa.gov, darpa.mil, sonatype.com, cloudflare.com. First test
   whether they are reachable now. Then verify or correct the flagged items in four areas,
   writing one notes file per area in this folder:
   - `gap_fill_conferences.md`: award lists and themes for PLDI 2025/2026, POPL 2026,
     OOPSLA 2025, ICFP 2025, ECOOP 2025, CAV 2025, CGO 2025/2026, CC, SOSP 2025,
     OSDI 2025/2026, ASPLOS 2025/2026, MLSys 2025/2026 (including whether FlashInfer won
     an MLSys 2025 award). Also confirm the 2026 preprints the notes cite: Lahiri's
     "Intent Formalization", the vericoding benchmark (Dafny 82% / Verus 44% / Lean 27%),
     VeruSAGE, the Microsoft SymCrypt/Aeneas blog, de Moura's Feb 2026 essay, Vera,
     Descend, HipKittens, Mirage, Exo 2, and the Mojo 1.0 / Qualcomm–Modular deal.
   - `gap_fill_ai_code.md`: Stack Overflow 2025 AI figures (66% "almost right",
     trust 33% vs 29%), DORA 2024/2025, METR 2025 trial and 2026 follow-ups, Veracode,
     CodeRabbit, GitClear, Faros, Perry et al., package hallucination, VERINA, CLEVER,
     AutoVerus, AlphaVerus, type-constrained decoding, KernelBench headline numbers,
     and company claims about AI-written code share.
   - `gap_fill_industry.md`: Google's Android memory-safety figures (2024 and 2025 posts),
     GTIG 2025 zero-days, Safe C++ / Profiles / C++26 status, Carbon, TrapC, Fil-C,
     DARPA TRACTOR, CISA's Jan 2026 roadmap deadline, EU Cyber Resilience Act dates,
     Linux kernel Rust status, the Shai-Hulud scale conflict, slopsquatting sample size,
     outage post-mortems (CrowdStrike, Cloudflare Nov 2025, Google Cloud Jun 2025,
     AWS Oct 2025), Uber's Go data-race study, COBOL and SSA status, developer population.
   - `gap_fill_surveys_languages.md`: Stack Overflow 2025 usage and frustration data (and
     a 2026 survey if published), JetBrains 2025, Python Developers Survey 2025, State of JS
     2025, Rust/Go/C++ 2025 surveys, Java/Kotlin/C# complaints, Octoverse 2025 (TypeScript
     #1?), MoonBit 1.0 (a partial check found its docs at 2026-09-18 still say "beta-preview",
     latest compiler tag v0.10.14), Verse, Roc, Zig and Gleam funding, Anthropic–Bun,
     agent-oriented languages (Vercel Zero and others), LLM results on low-resource languages.
   For each item record CONFIRMED (with the URL actually read), CORRECTED (old vs new), or
   COULD NOT VERIFY.
2. **The final report**, saved to `reports/New programming language pain points.md`, built
   from all notes, preferring verified figures and flagging anything still unverified or
   conflicting. No authorship line naming Claude, Anthropic, or an AI assistant.

## Context the report must use

### AS Lang (this repo)

An early prototype, about 1,900 lines of Rust in `src/core/`:

- **Pipeline:** lexer (f64-only numbers, no string escapes) → Pratt-style parser
  (let/fn/if/elseif/else/while/for/return/import/output/input, optional annotations
  Number/String/Boolean/Any/Void) → simple type checker (`types.rs`) → compiler to a flat
  stack bytecode → stack VM with one global variable table.
- **Around the core:** CLI with REPL and `--debug` trace, a minimal LSP (parse errors only),
  a C FFI (`as_execute` / `as_free_string`) used by the Go and Julia bindings, a PyO3 module
  (`aslang.core.run_code`), and a wasm-bindgen crate.
- **Works when run:** let, output, print, `+ - * /`, `== < >`, if/while, import, type
  annotations.
- **Does not work:**
  - Assignment `x = 2`: the parser gives `=` a precedence but has no infix rule for it.
  - `%`, `!=`, `<=`, `>=`, `&&`, `||` and `!`: parsed, but not compiled.
  - User-defined function calls: the runtime only knows `print`.
  - Arrays and indexing: no `MakeArray` in the VM and no compiled indexing.
  - For-loops.
  - Error locations: always reported as 0:0.
  - Tests: one of four lexer tests fails, on escaped quotes.
- **Docs vs code:** the README and `docs/EXPLAINER.md` claims (C++ SIMD linked in,
  async/await, constant folding, "10x faster than Python") do not match the code.
  `build.rs` is a stub.
- **History:** started in 2021 as a Python/sly interpreter; most files in `examples/` still
  use that old syntax.
- **Worth keeping:** the multi-host embedding story (C ABI, PyO3, WASM), the Rust
  implementation, and the LSP scaffold.

### The author's SepInfer research (repo `ashuwhy/SepInfer`, private)

Paper: `paper/main-pldi.tex`, "Ground or Nothing: Characterizing Sound Refutation of
TypeScript Type Assertions", targeting PLDI 2027. Read the README, and in the paper the
abstract, Introduction, "The Refutation Boundary", "Discussion and Limitations",
"Related Work", "Conclusion", and the "AI-Generated Code Probe" appendix.

- **What it is:** an OCaml tool with a Lean-mechanized soundness proof. It derives
  separation-type specs for TypeScript from declared types by forward symbolic execution
  (three rules: cast transparency, strong update, must-alias). It does not guess specs with
  an LLM. It flags "Class C" unsoundness, where TypeScript accepts code that a sound
  verifier rejects, e.g. a string laundered into `bigint` via `as unknown as bigint`.
- **Finding: LLMs weaken specs until the bug disappears.** When asked to make a sound
  verifier pass, an LLM weakens the spec until the bug disappears, so SepInfer needed a
  "floor guard" that forbids relaxing declared contracts.
- **Finding: retrofitting soundness hits a limit.**
  - In 22 production SDKs (15,139 files), 94.7% of `as unknown as` uses target a type the
    domain cannot refute.
  - In two further corpora, 44–48% of all type assertions are beyond any sound analysis.
- **Finding: AI-generated code probe.** 38 strict-clean AI-generated modules contained
  0 soundly-detectable faults.
- **The author's skills:** separation logic, symbolic execution, the Heifer verifier, Lean,
  OCaml, and a TypeScript AST oracle.

## Rules for git and GitHub (from the repo owner)

- Commit as `Ashutosh Sharma <ashutoshsharmawhy@gmail.com>`. Set this in the local git
  config before committing.
- Never add `Co-Authored-By: Claude…`, `Claude-Session:` or any other Claude or Anthropic
  line to commit messages, PR descriptions, or GitHub comments.
- Never create branches whose names start with `claude/`. Work on
  `research/language-pain-points`.
- Do not open a pull request unless the owner asks.
