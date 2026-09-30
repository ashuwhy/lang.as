# AS

**A fast, memory-safe language where code carries its own proof of intent.** Built for the era
in which AI writes most code and humans approve it.

You state what a function must do. The compiler proves the code does it, using the Z3 solver,
before it compiles to native code. Every check it proves is deleted from the binary, so proved
code runs without safety overhead. What it cannot prove it rejects with a concrete
counterexample, or keeps as a run-time check and tells you so. And once a contract is pinned,
an agent cannot quietly weaken it to make a failing check pass.

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
```

```text
$ aslang run examples/ledger.as
ok ledger.as: 20 checks proved, 0 kept at run time
moved, new balance: 30
```

## What it catches

Each line below is a test in `compiler/tests/cli.rs`.

| Mistake | What `aslang check` says |
|---|---|
| Off-by-one in the transfer | `E0204` postcondition may not hold, with the inputs and the wrong `result` |
| A missing bound on a sum | `E0205` value may not be a valid `Cents`, with inputs whose sum exceeds the limit |
| `transfer(from: a, to: a, ...)` | `E0203` this call may break `requires from.id != to.id` |
| Arguments swapped, `transfer(to: b, from: a, ...)` | `E0117`, with the corrected call as the fix |
| Contradictory `requires` (proofs would be vacuous) | `E0209` |
| The midpoint bug that sat in Java's `Arrays.binarySearch` for nine years, `(lo + hi) / 2` | `E0201` overflow, counterexample `lo = hi = int.max` |
| A pure function that starts printing | `E0106` effect not declared |
| A loop invariant that does not hold, a loop that may not end | `E0207`, `E0208` |
| An agent weakens a pinned `ensures` to hide a bug | `E0301` contract WEAKER than the pinned one, with a result the new contract allows and the old one forbids |
| A pinned function gains an effect | `E0303` |

## Speed

Three integer workloads written the same way in AS and Rust (`bench/perf/`, run with
`python3 bench/perf/run.py`), median of five runs on this container's 4-core x86-64, GCC 13.3
for AS, rustc 1.94:

| Benchmark | AS, every check proved | Rust `-O` (overflow wraps silently) | Rust `-O` with overflow checks |
|---|---|---|---|
| primes (trial division) | **639 ms** | 675 ms | 672 ms |
| collatz (longest chain) | 848 ms | 570 ms | 847 ms |
| isqrt (binary search) | 108 ms | 96 ms | 131 ms |

AS is as safe as Rust with overflow checks and, on these programs, as fast or faster than it.
Two things make that possible: proved checks are removed, and proved facts become optimiser
input. In `primes`, the prover shows both operands of `n % d` fit in 32 bits, so AS emits 32-bit
division, which is what beats Rust. `collatz` is slower than unchecked Rust because AS will not
let `3 * x + 1` overflow silently, so the program carries two guards Rust omits; Rust with the
same guards takes about 980 ms. These are three small programs, not a general claim.

## Built for models as well as people

- **The whole language fits in 2,370 tokens.** [`llms.txt`](llms.txt) is the complete v0.1
  reference; a model that reads it can write AS.
- **Short.** On four small programs with identical proved contracts, AS takes 230 tokens against
  Dafny's 249, Verus's 266 and Vera's 447 (`bench/tokens/`).
- **Errors are instructions.** Every diagnostic has a stable code, a fix, and a counterexample
  where there is one, in text or `--json`; `aslang explain E0201` explains any code.
- **A definite finish line.** `N checks proved, 0 kept at run time` tells an agent it is done.
- **Greppable.** Effects are called by name (`io.print`), variant names are global, there are no
  macros, overloading or implicit conversions.

## Use it

Requires Rust (to build the compiler), a C compiler (gcc or clang) and `z3` on `PATH`.

```sh
cargo build --release
./target/release/aslang check examples/ledger.as     # prove
./target/release/aslang run examples/ledger.as       # prove, compile, run
./target/release/aslang lock examples/ledger.as      # pin public contracts in aslang.lock
./target/release/aslang check --json examples/bugs/midpoint.as
./target/release/aslang explain E0301
cargo test                                            # end-to-end tests
```

## Status: v0.1

Working: integers with proved overflow safety, booleans, records, enums, `Option`, `Result`,
refinement types, functions, `let`/`var`, `if`/`match`/`while`, `requires`/`ensures`/
`invariant`/`decreases`, effects, named arguments, the contract lock, and native binaries through
C. Not yet: strings as values, arrays, generics, modules, a standard library beyond `io.print`,
FFI, concurrency, and proof of termination for recursion. The plan, the evidence behind every
design rule, and the stop rules are in [`docs/DESIGN.md`](docs/DESIGN.md).

## Where this came from

AS started in 2021 as a small interpreted language; that prototype is in `legacy/`. In 2026 it
was redesigned from a research study of what a new language could fix now that agents write
most code: [`reports/New programming language pain points.md`](reports/New%20programming%20language%20pain%20points.md),
[`why_llm_languages_flopped.md`](research_notes/New%20programming%20language%20pain%20points/why_llm_languages_flopped.md),
and a hands-on test of the closest rival, [`phase0/lemmascript_assessment.md`](phase0/lemmascript_assessment.md).

## License

MIT
