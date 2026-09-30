# AS: a native language where AI writes the code and proves it

*Working name AS, file extension `.as`, command `aslang`. Status: design for v0.1, with the
compiler under construction in `compiler/`. Nothing below is claimed as working unless the
"v0.1 scope" table says so.*

## The bet

Code is now cheap and trust is expensive. Agents write most new code, yet teams cannot cheaply
tell whether it does what was meant: tests get gamed, agents weaken specifications until checks
pass, and review time has grown faster than output (see
`reports/New programming language pain points.md`). The same research shows that languages
pitched only at the *writer*, with regular syntax and fewer tokens, have gone nowhere, while the
evidence favours making AI output *checkable by the reviewer*.

AS does both. It is a compiled, native, memory-safe language whose programs are short enough to
be cheap to generate and whose contracts are machine-checked, pinned, and small enough for a human
to review. It competes with Rust, Go and Zig on safety and speed, and beats them on one thing
none of them has: **a program's intent is part of the program, the compiler proves the code
meets it, and an agent cannot quietly weaken it.**

## Nine design rules

1. **Safe without a borrow checker.** Values have *mutable value semantics* (Hylo's model):
   variables own their values, assignment copies, and there are no references in the surface
   language. So there are no lifetimes, no aliasing bugs, no null, no use-after-free, and no
   data races by construction. The research notes found "ownership without Rust's complexity"
   (Hylo, OxCaml modes, Swift `~Copyable`) to be the award-winning frontier, and that type-system
   complexity is what separates Dafny's 82% agent success from Verus's 44% and Lean's 27%.
2. **Copies are free when nobody can see them.** Heap values (strings, arrays, maps, recursive
   enums, from v0.2) are reference-counted with Perceus-style precise drops and in-place reuse
   (Koka, Lean 4): updating a value whose count is one mutates it in place. Immutable values
   cannot form cycles, so reference counting is complete: no garbage collector and no pauses.
   Scalars, records and non-recursive enums are unboxed and passed by value.
3. **Every safety check is proved or visible, never silent.** Integer overflow, division by
   zero, array bounds, unwrapping, refinement types and contracts all become proof obligations
   for an SMT solver. Each gets one of three verdicts: *proved*, so it costs nothing at run time;
   *refuted*, a compile error with a concrete counterexample; or *unknown* within the time
   budget, so the compiler emits a run-time check and lists it in the build report.
   `--proved` makes any unknown an error. Nothing is ever silently unchecked.
4. **Intent is code.** Functions carry `requires` and `ensures`, types carry refinements
   (`int where it >= 0`), loops carry `invariant` and `decreases`. The spec language is the
   expression language, so an agent writes one language, not two.
5. **Contracts are pinned.** `aslang lock` records every public function's contract, effects
   and signature in `aslang.lock`. Each later build proves the new contract *refines* the pinned
   one: the old precondition implies the new one, and under the old precondition the new
   postcondition implies the old one. A weakening fails with a counterexample the reviewer can
   read, and lands only through an approved lock change (CODEOWNERS). Two holes found in
   LemmaScript's guard (`phase0/lemmascript_assessment.md`, probes E2b, E3a, E3c) are closed by
   rule: an unsatisfiable `requires` is always an error, and there is no `assume` or axiom
   outside an explicit, locked `trusted` block.
6. **Effects are part of the signature.** Functions are pure unless they declare
   `uses io, fs, net, clock, rand, env, ffi`. A function may call only functions whose effects
   are a subset of its own, and effects are pinned in the lock. A dependency update that gives
   a parsing library network access fails the build: the supply-chain attack surface becomes a
   reviewable diff.
7. **Nothing unchecked crosses a boundary.** Data from files, sockets, JSON or foreign code is
   `Untrusted<T>` until a decoder the compiler generates from `T` validates its shape, ranges
   and refinements (v0.3). Output sinks (shell, SQL, HTML, logs) accept only values built by
   escaping constructors.
8. **Token-efficient, and familiar before clever.** Programs should cost fewer tokens than
   Rust, Go or TypeScript for the same verified behaviour, measured with real tokenizers, not
   asserted. The savings come from removing ceremony, not from inventing symbols that
   tokenizers split badly: newlines end statements, local types are inferred, blocks and
   `if`/`match` are expressions, `T?` is `Option<T>`, `?` propagates errors, and there are no
   lifetimes, no `mut`/`&`/`&mut`, no `unwrap()`, no header files. Keywords and punctuation are
   the ones models already know from Rust, Swift, TypeScript and Dafny. There is one canonical
   formatting, so generated diffs stay minimal.
9. **Built for the agent loop, and future-proof.** Every diagnostic has a stable code, a span,
   and a JSON form with the counterexample and a suggested fix. Checking is modular (each
   function against its callees' contracts), so the loop an agent runs thousands of times stays
   in seconds. The solver interface is plain SMT-LIB text, so Z3 can be swapped for cvc5 or a
   future prover, and the core calculus is small enough to mechanise in Lean.

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
  match transfer(a, b, 30) {
    Ok(m) => print(m.to.balance)
    Err(e) => print(-1)
  }
}
```

What the compiler does with it:

- proves the three `ensures`, that no arithmetic overflows a 64-bit integer, and that every
  `Cents` it builds is in range, so the emitted machine code has no checks at all. Delete the
  second `requires` and it reports the counterexample (`to.balance = 1_000_000_000_000`,
  `amount = 1`) instead of guessing;
- rejects `transfer(a, a, 30)` at the call site in `main`, because `requires from.id != to.id`
  fails for `a.id == a.id`;
- pins the contract. If an agent later rewrites the first `ensures` as
  `m.from.balance <= from.balance`, `aslang check` prints the pinned and proposed clauses and an
  input the new contract allows and the old one forbids, and fails until the lock is approved.

## Semantics, briefly

| Area | Rule |
|---|---|
| Integers | `int` is a 64-bit two's-complement integer; every `+ - *` and conversion has an overflow obligation; `/` and `%` truncate as in C, with obligations for a zero divisor and `MIN / -1`. Sized types (`i32`, `u8`, …) arrive in v0.2 |
| Values | Records `{ f: T }` are structural in literals and nominal when named; `{ ...r, f: e }` copies with an update. Enums carry named payloads. `T?` is `Option<T>`; `Result<T, E>` has `Ok`/`Err`. No null, no exceptions, no implicit conversions |
| Variables | `let` binds, `var` binds a mutable local, `x = e` and `x += e` update it. Parameters are immutable. There are no references, so an update can never be seen through another name |
| Control | `if`, `match` and blocks are expressions; `return` exits early; `while` loops carry `invariant` and `decreases`. `match` must be exhaustive |
| Patterns | `x is Ok(m)` is a boolean test that binds `m` on the true side of `&&`, `==>` and `if` |
| Specs | `requires`, `ensures` (with `result`), `invariant`, `decreases`, `==>` (implies). Spec expressions are ordinary expressions evaluated over mathematical integers (no overflow), and are erased from the binary |
| Effects | `uses` lists effects; pure is the default; `main` may use any effect; `print` needs `io` |
| Errors | `Result` must be used; `?` returns an `Err` early (v0.2). A run-time check that fails aborts with the source location and the clause, never with undefined behaviour |

## Toolchain

| Command | Does |
|---|---|
| `aslang check f.as` | Parse, type-check, effect-check, verify; compare with `aslang.lock` if present |
| `aslang build f.as -o f` | `check`, then emit C and compile it with the system C compiler at `-O2` |
| `aslang run f.as` | `build` to a temporary binary and run it |
| `aslang lock f.as` | Pin the current contracts; refuses if any public contract is vacuous |
| `aslang emit-c`, `aslang emit-smt` | Show the generated C or the SMT-LIB queries |
| `--json` | Every command reports machine-readable diagnostics and verdicts |

The native backend emits C and hands it to `cc`, the route Lean 4, Koka and Nim use: it gets
GCC's and Clang's optimisers and every platform they target, including WebAssembly through
`clang --target=wasm32`, on day one. A Cranelift backend for fast debug builds can come later
from the same checked IR.

## v0.1 scope (this milestone)

| Feature | v0.1 |
|---|---|
| Lexer with spans, automatic statement ends | yes |
| `int`, `bool`, records, record update, enums with payloads, `Option`/`Result` | yes |
| Refinement type aliases (`where`) | yes |
| Functions, recursion, `let`/`var`, assignment, `if`/`match`/`while`, `return` | yes |
| `requires`/`ensures`/`invariant`, `is` patterns, `==>` | yes |
| Overflow and division obligations; three verdicts; counterexamples | yes |
| Effects (`uses`), `print` for `int`, `bool` and string literals | yes |
| C backend to a native binary | yes |
| Contract lock with refinement check and vacuity guard | yes |
| JSON diagnostics | yes |
| Strings, arrays, maps, recursive enums (Perceus RC) | v0.2 |
| Generics, modules and packages, `?`, sized integers, `decreases` checking | v0.2 |
| `Untrusted<T>` decoders, taint sinks, FFI, capabilities as values | v0.3 |
| Concurrency (structured, message passing of immutable values) | v0.4 |
| Token benchmark against Rust, Go, TypeScript and Python | v0.2, before any public claim |

Known limits of v0.1, stated so nobody over-reads it: correctness is partial (termination is not
yet proved), specs cannot call user functions, and the trusted base is the compiler, Z3 and the
C compiler.

## Roadmap and honest risks

1. **v0.1 (now):** the vertical slice above, on small verified examples.
2. **v0.2:** heap values with Perceus, generics, modules, `?`, the token benchmark, and an agent
   evaluation: the same tasks written by frontier models in AS, Rust and TypeScript, measuring
   escaped defects, contract weakening, tokens and time. This replaces Phase 0's study with an
   evaluation of the language itself, and is the evidence the report says nobody has yet.
3. **v0.3:** boundaries, taint and FFI; a standard library for files, JSON, HTTP clients and
   time; an LSP with proof status on hover; `llms.txt` and an agent skill.
4. **v0.4:** concurrency, the package manager with effect-pinned dependencies, and Wasm.

The research is blunt about the risks and they stand: most new languages fail on ecosystem,
not features; a no-resource language starts behind in model training data (mitigated by
familiar syntax, short docs in context, and diagnostics that teach); SMT verification can be
brittle at scale (mitigated by modular checking, time budgets and run-time fallbacks); and a
pinned wrong contract is a pinned bug. The plan answers each with a measured milestone rather
than a promise.
