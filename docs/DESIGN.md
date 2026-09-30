# AS: a fast, safe systems language whose code carries its own proof of intent

*Working name AS, file extension `.as`, command `aslang`. Status: design for v0.1, with the
compiler under construction in `compiler/`. Nothing below is claimed as working unless the v0.1
scope table says so. Evidence for each rule: `reports/New programming language pain points.md`
and `research_notes/New programming language pain points/why_llm_languages_flopped.md`.*

## The bet

Agents now write most new code, and the scarce resource is knowing it is right. Tests get gamed,
agents weaken specifications until checks pass, and review time grows faster than output.

The languages built "for LLMs" in 2025-2026 answered the wrong question. Frontier models already
write a language they have never seen at about 99% on small problems, given one document in
context; what nobody can do cheaply is *trust* the result. Those languages optimised for the
writer, made code harder for the human reviewer (Vera removes variable names; Nanolang uses
prefix syntax), measured themselves on saturated toy benchmarks, and found no users.

AS is for teams where agents write the code and humans approve it. It is a compiled, native,
memory-safe language, competing with Rust, Go and Zig on safety and speed, with one thing none of
them has: **the program states its intent, the compiler proves the code meets it, and an agent
cannot quietly weaken it.** It is designed to be the cheapest language in which to reach
*verified* code, counted in total agent tokens, not only in source length.

## Ten design rules

1. **Safe without a borrow checker.** Values have mutable value semantics (Hylo's model):
   variables own their values, assignment copies, and there are no references in the surface
   language. So there are no lifetimes, no aliasing bugs, no null, no use-after-free, and no data
   races by construction. Type-system complexity is what separates Dafny's 82% agent success
   from Verus's 44% and Lean's 27% on the same tasks; AS keeps the type system Dafny-simple.
2. **Copies are free when nobody can see them.** Heap values (strings, arrays, maps, recursive
   enums, from v0.2) are reference-counted with Perceus-style precise drops and in-place reuse
   (Koka, Lean 4): updating a value whose count is one mutates it in place. Immutable values
   cannot form cycles, so reference counting is complete: no garbage collector, no pauses.
   Scalars, records and non-recursive enums are unboxed and passed by value.
3. **Every safety check is proved or visible, never silent.** Integer overflow, division by
   zero, array bounds, refinement types and contracts are proof obligations for an SMT solver,
   each with one of three verdicts: *proved*, so it costs nothing at run time; *refuted*, a
   compile error with a concrete counterexample; or *unknown* within the time budget, so the
   compiler emits a run-time check and lists it in the build report. `--proved` turns every
   unknown into an error. A failed run-time check aborts with the source location and clause,
   never with undefined behaviour.
4. **Intent is code, and costs nothing when trivial.** Functions may carry `requires` and
   `ensures`, types may carry refinements (`int where it >= 0`), loops carry `invariant` and
   `decreases`. The spec language is the expression language. Nothing is mandatory: a function
   without contracts is pure, total over its parameter types, and still gets every derived
   obligation (overflow, bounds, division, exhaustive `match`). Vera's mandatory
   `requires(true) ensures(true) effects(pure)` is the boilerplate AS refuses.
5. **Contracts are pinned.** `aslang lock` records every public function's contract, effects and
   signature in `aslang.lock`. Each later build proves the new contract *refines* the pinned
   one: the old precondition implies the new one, and under the old precondition the new
   postcondition implies the old one. A weakening fails with an input the new contract allows
   and the old one forbids, and lands only through an approved lock change (CODEOWNERS). Two holes
   measured in LemmaScript (`phase0/lemmascript_assessment.md`, probes E2b, E3a, E3c) are closed
   by rule: an unsatisfiable `requires` is always an error, and there is no `assume` or axiom
   outside an explicit, locked `trusted` block.
6. **Effects are part of the signature, and greppable.** Functions are pure unless they declare
   `uses io, fs, net, clock, rand, env, ffi`. Effectful operations are called through the
   effect's name (`io.print`, `fs.read`), so `grep 'net\.'` finds every network call. A function
   may call only functions whose effects are a subset of its own, and effects are pinned in the
   lock: a dependency update that gives a parser network access fails the build.
7. **Nothing unchecked crosses a boundary.** Data from files, sockets, JSON or foreign code is
   `Untrusted<T>` until a decoder the compiler generates from `T` validates shape, ranges and
   refinements (v0.3). This is the gap measured in annotated TypeScript, where a verified
   function returned `balance: "030"`. Output sinks (shell, SQL, HTML, logs) accept only values
   built by escaping constructors.
8. **Readable first, then short.** Real names, infix operators, and keywords and punctuation
   models already know from Rust, Swift, TypeScript and Dafny: no slot references, no prefix
   notation, no new glyphs (tokenizers split unusual symbols badly). Braces, not significant
   whitespace. Ceremony is removed instead: newlines end statements, local types are inferred,
   blocks, `if` and `match` are expressions, `T?` is `Option<T>`, `?` propagates errors, and
   there are no lifetimes, `mut`, `&`, `unwrap()`, macros or header files. One canonical format
   keeps generated diffs minimal. Measured on four programs with identical proved contracts,
   the draft costs 230 tokens against Dafny's 249, Verus's 266 and Vera's 447, and unverified
   Rust's 186 (`bench/tokens/`).
9. **Built to end the agent loop.** Total cost is dominated by reasoning and revision, not by
   source length: in unfamiliar languages agents write non-compiling code, revise solutions that
   already pass, and distrust tests. So every diagnostic is an instruction: a stable code, the
   span, what is wrong, why, a concrete fix, and a counterexample where there is one, in text
   and JSON. A build ends with a definitive verdict ("all 14 obligations proved, 0 checked at run
   time") that gives an agent no reason to keep going. Checking is modular, so each iteration
   stays in seconds. Code is greppable by construction: qualified calls, no glob imports, no
   overloading, and every definition starts with `fn`, `type` or `enum` at column zero, because
   agents navigate with grep and a language server usually costs them tokens.
10. **Fits in one document, links into everything, lasts.** The whole language, the essentials
    of the standard library and the diagnostic index fit in one reference of at most 25,000
    tokens, shipped as `llms.txt` and an agent skill; a feature that does not fit waits. Models
    write a new language well from one document; they fail on what is not in it. Compiled code
    exposes a C ABI in both directions, so a verified AS module links into existing C, C++,
    Rust, Go or Python programs as a static library and header, the way Zig entered through C.
    The solver interface is plain SMT-LIB text (Z3 today, cvc5 or a successor tomorrow), and the
    core calculus is small enough to mechanise in Lean.

## What a model needs from a language

What makes code easier for a model to write correctly, as far as can be judged without
measurement; each point is meant to be tested in the v0.2 evaluation.

1. **Everything about a function in its signature.** Types, effects and contracts, so a
   function can be understood and changed without reading other files. Hidden state, implicit
   conversions, macros and overloading force a model to hold context it may not have.
2. **A definite finish line.** The most wasteful habit is re-deriving code that already works.
   "All N checks proved" ends the loop; a green test suite does not, because tests can be wrong.
3. **Swaps caught, not merely unlikely.** Passing `(to, from)` for `(from, to)` is one of the
   most common slips. Labels checked against parameter names (`transfer(from: a, to: b)`) turn
   it into a compile error with the corrected call as the fix. Vera's answer, removing names,
   makes the code unreadable for the human who must approve it.
4. **Errors that say what to do.** A stable code, the exact span, a fix and a counterexample
   are cheaper than a paragraph of prose; `aslang explain` gives the rest on demand.
5. **Edit-robust syntax.** Models edit by replacing exact text. One statement per line, no
   significant indentation, trailing commas allowed and no required semicolons keep edits
   local and diffs small.
6. **An API that cannot be hallucinated.** The whole language and standard library listed in
   one short document, with "did you mean" suggestions for every unknown name.

## The language in one example

```as
type Cents = int where 0 <= it && it <= 1_000_000_000_000

type Account = { id: int, balance: Cents, frozen: bool }
type Moved = { from: Account, to: Account }

enum TransferError { Frozen, Insufficient(short: Cents) }

pub fn transfer(from: Account, to: Account, amount: Cents) -> Result<Moved, TransferError>
  requires from.id != to.id
  requires to.balance + amount <= 1_000_000_000_000
  ensures result is Ok(m) ==> m.from.balance == from.balance - amount
  ensures result is Ok(m) ==> m.to.balance == to.balance + amount
  ensures (result is Err(Frozen)) == (from.frozen || to.frozen)
{
  if from.frozen || to.frozen { return Err(Frozen) }
  if from.balance < amount { return Err(Insufficient(amount - from.balance)) }
  Ok({ from: { ...from, balance: from.balance - amount },
       to:   { ...to,   balance: to.balance + amount } })
}

fn main() uses io {
  let a = { id: 1, balance: 100, frozen: false }
  let b = { id: 2, balance: 0, frozen: false }
  match transfer(from: a, to: b, amount: 30) {
    Ok(m) => io.print(m.to.balance)
    Err(e) => io.print(-1)
  }
}
```

What the compiler does with it:

- proves the three `ensures`, that no arithmetic overflows a 64-bit integer, and that every
  `Cents` it builds is in range, so the machine code has no checks at all. Delete the second
  `requires` and it reports the counterexample (`to.balance = 1_000_000_000_000`, `amount = 1`)
  with the fix "add `requires to.balance + amount <= 1_000_000_000_000`";
- rejects `transfer(a, a, 30)` at the call site, because `requires from.id != to.id` fails;
- pins the contract. If an agent later rewrites the first `ensures` as
  `m.from.balance <= from.balance`, `aslang check` shows the pinned and proposed clauses and an
  input the new contract allows and the old one forbids, and fails until the lock is approved.

## Semantics, briefly

| Area | Rule |
|---|---|
| Integers | `int` is a 64-bit two's-complement integer and `nat` is `int where it >= 0`; every `+ - *` has an overflow obligation; `/` and `%` truncate as in C, with obligations for a zero divisor and `int.min / -1`. Sized types (`i32`, `u8`, …) arrive in v0.2 |
| Values | Records `{ f: T }` are structural in literals and nominal when named; `{ ...r, f: e }` copies with an update. Enums carry named payloads. `T?` is `Option<T>`; `Result<T, E>` has `Ok`/`Err`. No null, no exceptions, no implicit conversions |
| Variables | `let` binds, `var` binds a mutable local, `x = e` and `x += e` update it. Parameters are immutable. There are no references, so an update is never visible through another name |
| Control | `if`, `match` and blocks are expressions; `return` exits early; `while` carries `invariant` and `decreases`; `match` must be exhaustive |
| Patterns | `x is Ok(m)` is a boolean test that binds `m` on the true side of `&&`, `==>` and `if` |
| Specs | `requires`, `ensures` (with `result`), `invariant`, `decreases`, `==>` (implies). Spec expressions are ordinary expressions over mathematical integers (no overflow), erased from the binary |
| Effects | `uses` lists effects; pure is the default; effect operations are qualified (`io.print`) |
| Errors | `Result` must be used; `?` returns an `Err` early (v0.2) |

## Toolchain

| Command | Does |
|---|---|
| `aslang check f.as` | Parse, type-check, effect-check, verify; compare with `aslang.lock` if present |
| `aslang build f.as -o f` | `check`, then emit C and compile it with the system C compiler at `-O2` |
| `aslang run f.as` | `build` to a temporary binary and run it |
| `aslang lock f.as` | Pin the current contracts; refuses if any public contract is vacuous |
| `aslang emit-c`, `aslang emit-smt` | Show the generated C or the SMT-LIB queries |
| `--json` | Machine-readable diagnostics, verdicts and counterexamples for every command |

The native backend emits C and hands it to `cc`, the route Lean 4, Koka and Nim use: it gets
GCC's and Clang's optimisers and every platform they target, including WebAssembly through
`clang --target=wasm32`, from the start. A Cranelift backend for fast debug builds can follow
from the same checked IR.

## v0.1 scope (this milestone)

| Feature | v0.1 |
|---|---|
| Lexer with spans and automatic statement ends | yes |
| `int`, `nat`, `bool`, records, record update, enums with payloads, `Option`/`Result` | yes |
| Refinement type aliases (`where`) | yes |
| Functions, recursion, `let`/`var`, assignment, `if`/`match`/`while`, `return` | yes |
| `requires`/`ensures`/`invariant`/`decreases` (loops), `is` patterns, `==>` | yes |
| Overflow and division obligations; three verdicts; counterexamples | yes |
| Effects (`uses`), `io.print` for `int`, `bool` and string literals | yes |
| C backend to a native binary | yes |
| Contract lock with refinement check and vacuity guard | yes |
| Diagnostics as instructions, in text and JSON; `aslang explain` | yes |
| Named arguments checked against parameter names | yes |
| Proved facts passed to the C optimiser; proved-safe 32-bit and unsigned division | yes |
| `llms.txt`: the complete reference (3,106 tokens at v0.2) | yes |
| Arrays with proved bounds, reference counting, copy-on-write, last-use moves; `for` loops; `forall`/`exists` | done (v0.2) |
| Loop-invariant inference (Houdini over templates; inferred invariants are re-proved) | done (v0.2) |
| Contract inference for private functions (Houdini across the module: `requires` from call sites, `ensures` from returns; re-proved) | done (v0.2) |
| Exact summaries of small private helpers (no loops, no recursion): callers see the value the body computes | done (v0.2) |
| Strings, maps, arrays inside other values, recursive enums | v0.2/v0.3 |
| Generics, modules, `?`, sized integers, recursion termination | v0.2 |
| C ABI export (`.h` + static library) and import through `uses ffi` | v0.2 |
| `Untrusted<T>` decoders, taint sinks, capabilities as values | v0.3 |
| Concurrency (structured, message passing of immutable values) | v0.4 |

Limits of v0.1, stated so nobody over-reads it: correctness of recursive functions is partial
(termination of recursion is not yet proved), specs cannot call user functions, and the trusted
base is the compiler, Z3 and the C compiler.

## How AS relates to the nearest designs

| | Vera | Dafny | Verus | AS |
|---|---|---|---|---|
| Names | Typed slots `@Int.0` | Names | Names | Names |
| Contracts | Mandatory | Optional | Optional | Optional; derived obligations always |
| Contract pinning across versions | No | No | No | Yes |
| Effects in signatures | Yes | No | No | Yes |
| Memory model | Wasm GC | GC (target-dependent) | Rust ownership | Value semantics + Perceus RC |
| Output | Wasm | C#, Java, JS, Go, Python | Native via rustc | Native via C |
| Tokens on the 4-program corpus | 447 | 249 | 266 | 230 |

## Roadmap, evaluation and stop rules

1. **v0.1 (now):** the vertical slice above, on small verified examples.
   *Exit:* the ledger example verifies, compiles to a binary with no run-time checks, and the lock
   catches a seeded weakening with a counterexample.
2. **v0.2:** heap values with Perceus, generics, modules, `?`, C ABI, `llms.txt` under 25,000
   tokens, and the first agent evaluation. The evaluation avoids the saturation that makes toy
   benchmarks useless: bug-prone modules (money, permissions, parsers), at least a third of the
   tasks boundary or effect bugs, written by frontier models in AS, Rust and TypeScript, measuring
   escaped defects (hidden tests and mutation), contract weakening, and total agent tokens to a
   passing result.
   *Exit:* AS matches Rust on task success, uses no more total tokens, and has fewer escaped
   defects. *Stop or rethink* if agents need more than twice Rust's tokens after two rounds of
   diagnostic and documentation fixes.
3. **v0.3:** boundaries, taint and FFI; a standard library for files, JSON, HTTP clients and
   time; `aslang fmt`; an LSP with proof status on hover.
   *Exit:* three real modules (a ledger, a policy evaluator, a parser) ported and linked into
   existing C, Rust or Python programs.
4. **v0.4:** concurrency, a package manager with effect-pinned dependencies, and Wasm.

Scope discipline, learned from Nanolang: no VM, runtime platform or registry before the core has
outside users; every milestone above has an exit test. The remaining risks stand: new languages
usually fail on ecosystem, not features; SMT verification can be brittle at scale (mitigated by
modular checking, time budgets and run-time fallbacks); and a pinned wrong contract is a pinned
bug. The plan answers each with a measured milestone rather than a promise.
