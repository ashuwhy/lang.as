# LemmaScript, hands-on: it seals the inside, not the edges

Assessment for Phase 0 step 2, run on 2026-09-30 against `lemmascript` 0.6.4 (npm `latest`,
source commit `097f18f` of 2026-09-19), `lemmascript-guard` 0.1.0, `lemmascript-crosscheck`
0.1.0 and Dafny 4.11.0. Every result below comes from a run in this container and reproduces
with `phase0/lemmascript-probes/run.sh`; source facts cite files in the LemmaScript repository.
Nothing here relies on search snippets.

## Verdict

**LemmaScript does not cover the sealed-boundary edge the report claims for Assured Script, but
it covers more of it than the report assumed, and what it lacks is tool-shaped.**

- **Already covered, better than expected.** Inside a `//@ verify` function, escape hatches fail
  closed: an `as` cast is stripped so the real type reaches Dafny and the mismatch is rejected,
  `any` becomes an opaque `Unknown` that no arithmetic accepts, and `m.get(k)!` becomes a map
  lookup Dafny must prove safe. Lee et al.'s ninefold `any` gap cannot cause a wrong proof
  there. Midspiral has also published `lemmascript-guard`, which compiles `//@ requires` into
  run-time checks, and `lemmascript-crosscheck`, which found a precision counterexample in our
  ledger in five seconds.
- **Not covered, confirmed by running it.** (1) There is no contract lock. An edit that weakens
  `//@ ensures` in the `.ts` and adds a bug verifies and passes the additions-only rule. (2) The
  additions-only rule itself can be beaten by additions: one added line, `requires false` or
  `assume {:axiom} false;`, hides the same bug with exit code 0. (3) There is no structural
  decoding. `JSON.parse` under `//@ autohavoc` is assumed to return a well-typed record, and a
  guarded, verified `transfer` given a JSON-decoded `balance: "0"` returns `balance: "030"`.
  (4) Effects are not part of any contract. A "pure" pricing function that calls `fetch`
  verifies; `autohavoc` prints a note and exits 0.
- **Consequence for the plan.** Items (1), (2) and (4) are exactly the contract lock, the floor
  guard and the effect floor. Item (3) is decoder generation. All four could ship as companion
  tools over LemmaScript's `//@` annotations. The one property the report said only a language
  can give, no escape hatches, LemmaScript's verified fragment already gives. The case for a
  new language is weaker than the report states. It now rests on a narrower, testable claim:
  that a boundary sealed *by construction*, with effects in the type of every function,
  prevents defects that companion tools over annotated TypeScript still let through. Step 3's
  pre-registration is revised to test exactly that (a fifth arm, "LemmaScript plus companion
  tools"), and the Phase 0 build target becomes the companion tools, not a language.

## Answers to the five questions

### 1. How it treats `any`, casts, non-null assertions and unchecked JSON

Design: `DESIGN.md` §5 excludes `any` and `unknown` "by design", along with `this`, classes,
closures over mutable state and implicit coercions. Implementation and behaviour:

| Construct inside `//@ verify` | Source | Probe | Result |
|---|---|---|---|
| `raw as unknown as number` (string laundered to number) | `extract.ts:701` strips the assertion | `launder.ts`, and `launder2.ts` with an explicit `: number` annotation | Rejected: Dafny type error, `int` vs `seq<char>`. Fail-closed, but the message is in Dafny terms, not TypeScript |
| parameter typed `any` | `extract.ts:2013-2027` maps `any` to `unknown`; `dafny-emit.ts:51` emits opaque `type Unknown` | `anyparam.ts` | Rejected: "no common supertype (Unknown and int)" |
| `limits.get(u.id)! + u.limit!` (map lookup plus optional field) | `resolve.ts:788-796` unwraps the optional type | `nonnull.ts` | Rejected: Dafny type errors (`Option<int>` vs `int`); the unwrap is not emitted for this shape |
| `limits.get(id)!` | same, map case becomes direct indexing | `nonnull2.ts` | Proof obligation `id in limits`: fails without a `requires`, sound |
| `JSON.parse(body) as Payout` | no model for `JSON.parse` | `jsonparse.ts` | Rejected: "Unsupported Dafny method call: .parse()" |
| `JSON.parse(body)` under `//@ autohavoc` | `autohavoc.ts` replaces it with `var p: Payout := *` | `jsonauto.ts` | **Verifies**. Prints "abstracts 1 external call(s) — confirm none is an unguarded sink: JSON.parse" and exits 0 |

The last row is the boundary. `//@ autohavoc` is the documented way to verify brownfield code
that parses input (`SPEC.md` §2.12), and it havocs a value *of the declared type*. The proof then
holds only for well-typed JSON. At run time the verified `parsePayout`, proved to ensure
`cents >= 0`, returned `{"user":"a","cents":"abc"}` and `{"user":"a"}`, which violate the
proved postcondition, plus `0.5` and `Infinity`, which satisfy `>= 0` but break the integer model
the proof assumed (`boundary_unguarded.mts`). Nothing in LemmaScript generates the structural check
that would reject them, and `lemmascript-guard`'s README says so plainly: it "is not structural
input validation … Decode untrusted bytes before the guard".

### 2. Whether it pins contracts across versions

It does not. The additions-only rule (`dafny-commands.ts:22-68`) runs `git diff --no-index`
between `foo.dfy.gen` and `foo.dfy` and fails if any generated line was deleted or changed. Both
files come from the *current* `.ts`, so the rule protects proof work from being edited, not the
contract from being changed. Probes on `ledger.ts`, a verified money transfer:

| Probe | Change | `lsc` exit | Verifier |
|---|---|---|---|
| E2a | bug only (`to.balance + amount - 1`) | 1 | postcondition could not be proved (correct) |
| E2b | same bug, plus the `//@ ensures` for `to.balance` deleted in the `.ts` | **0** | 2 verified, 0 errors; 0 generated lines deleted |
| E3a | bug, `.ts` untouched, one added line `requires false` in the ensures lemma of `ledger.dfy` | **0** | 2 verified, 0 errors |
| E3b | bug, one added line `assume false;` | 1 | Dafny's missing-`{:axiom}` warning fails the build (correct) |
| E3c | bug, one added line `assume {:axiom} false;` | **0** | 2 verified, 0 errors |

E2b is the failure SepInfer's floor guard exists for, moved from the proof file to the source
file: a reviewer sees it only by noticing a deleted `//@` line in a TypeScript diff. E3a and E3c
show that "additions only" is not "strengthening only": an added precondition or axiom can make
the obligation vacuous. `AGENTS.md` tells agents not to use `//@ assume` as a shortcut, but no
check enforces it. LemmaScript's own related tool, `lemmascript-claimcheck`, asks an LLM whether
the prose `//@ contract` matches the formal `//@ ensures`; an agent that weakens both passes it,
and it compares within one version, not across versions.

These are gaps in a Tech Preview, not a verdict on LemmaScript's quality, and the E3 pair is
worth reporting upstream (a draft issue is at the end).

### 3. Effects, taint and boundary validators

| Capability | Status |
|---|---|
| Effects in a function's contract | None. `//@ impure` makes an extern's result vary per call (`SPEC.md` §2.9); it does not declare or restrict what a function may do. A pricing function calling `console.log` and `fetch` verifies under `autohavoc` (`effectsauto.ts`), which only lists the abstracted calls; without `autohavoc` it is rejected as unsupported (`effects.ts`) |
| Taint | No taint types. `autohavoc` guarantees "every *contracted* sink is reached only under its guard, not … every dangerous call is contracted" (`SPEC.md` §2.12). The `guardians-lemmascript` case study proves a taint checker correct; that is an application, not a toolchain feature |
| Semantic preconditions at the boundary | **Yes**, via `lemmascript-guard` 0.1.0 (published 2026-08-23). It emits `foo.guarded.ts`, which checks each `//@ requires` before delegating, throws `PreconditionError`, exits nonzero on any clause it cannot enforce, and supports CI drift checks. On `ledger.ts` it rejected `amount = -50`, `NaN` and a same-account transfer that the unguarded export accepted |
| Structural validation | **No.** The guarded `transfer` accepted `0.1` (the model says `int`), `1e308` (beyond safe integers) and a JSON-decoded `balance: "0"`, returning `balance: "030"` from a function proved to add numbers (`boundary_guarded.mts`) |
| Model-vs-JavaScript differences | Detected by testing, via `lemmascript-crosscheck` 0.1.0 (published 2026-08-22): it runs the compiled Dafny model and the JavaScript export on proof-valid inputs and compares. On `ledger.ts` it reported in about five seconds that `from.balance = 9007199254740993` rounds to `…992`. Faithful `number` semantics are a research proposal (`DESIGN_NUMBERS.md`, "no JavaScript-number mode is implemented") |

### 4. Verifier latency

Latency is not an obstacle. `lsc check` on seven of LemmaScript's own examples (17-89 lines,
1-9 verification conditions each) took 1.8-2.3 s wall time per file, of which 1.1-1.5 s is
`dafny verify` alone, mostly .NET start-up. Our ledger took 2.0-2.7 s end to end. The published
case studies with 700+ obligations run under `--isolate-assertions` with 180-600 s limits, so
large proofs are slow, but the per-function loop an agent iterates on is in the few-seconds
range the report targets. Lean was not measured; it needs Loom and Velvet forks.

### 5. Who outside Midspiral uses it

No independent user was found.

- npm: two maintainers, both `@midspiral.com`. A registry search for "lemmascript" returns five
  packages, all published by Midspiral. The search index reported 3,059 weekly and 7,890
  monthly downloads for `lemmascript` (up from about 1,733 weekly on 2026-09-24), and 2,533
  weekly for `lemmascript-claimcheck`, which `lemmascript` depends on, so most of those are the
  same installs. The guard and crosscheck packages show 2-4 downloads a week. api.npmjs.org was
  blocked, so these are search-index figures.
- GitHub: 1,095 of 1,119 commits are by Nada Amin; three other people have 4-12 commits each.
  All listed case studies live in `midspiral/*` or the maintainer's `metareflection/*`, except
  `CHARM-BDF/charmchat`, whose `lemma` branch is also authored by Nada Amin.
- Release pace is high: 36 versions from 2026-04-01 to 2026-09-19, two companion tools in
  August, and case studies that each drove toolchain changes. It is an active, well-staffed
  research project with no visible outside adopters yet.

## What this changes in the Phase 0 plan

1. **Drop "no escape hatches" as a differentiator.** Inside its verified fragment LemmaScript
   already fails closed on `any`, casts and `!`. Assured Script's soundness is still a
   precondition for its own guarantees, but it is not a selling point against LemmaScript.
2. **Re-centre on four measured gaps**: version-to-version contract refinement (E2b), vacuity
   and axiom additions (E3a, E3c), structural decoding at host and JSON boundaries (E8), and
   effects in the pinned contract (E7). Build them first as a companion tool that reads
   LemmaScript's `//@` annotations, which the report already planned for the contract-lock
   Action.
3. **Add an arm to the study.** The language hypothesis is now "a sound core with sealed
   boundaries beats LemmaScript *plus the companion tools*", not "beats LemmaScript as
   shipped". `PREREGISTRATION.md` adds that arm and states the pivot against it.
4. **Contact Midspiral early.** The E3 finding and the decoder gap are natural first
   contributions, and they test whether "contribute rather than compete" is open.

## Draft upstream issue (not filed)

> **Title:** additions-only check accepts vacuous proof additions (`requires false`, `assume {:axiom} false`)
>
> With lemmascript 0.6.4 and Dafny 4.11.0, take a verified function whose ensures are emitted as
> a `<fn>_ensures` lemma, introduce a bug in the `.ts`, and `lsc regen` fails as expected. Adding
> a single line `requires false` to the generated lemma in the `.dfy`, or a single line
> `assume {:axiom} false;` in its body, makes `lsc check` exit 0 and still pass the additions-only
> check. (A bare `assume false;` is caught by Dafny's warning.) Since agents are told the diff
> must be additions-only, a mechanical check for added `requires` on generated lemmas and
> methods, `{:axiom}`, `assume` and `{:verify false}` in proof additions would close this. A
> reproduction script is available.

The owner should decide whether and when to file it.
