# Touchmark

**A fast, memory-safe language where code carries its own proof of intent.** Built for the era
in which AI writes most code and humans approve it.

You state what a function must do. The compiler proves the code does it, using the Z3 solver,
before it compiles to native code. Every check it proves is deleted from the binary, so proved
code runs without safety overhead. What it cannot prove it rejects with a concrete
counterexample, or keeps as a run-time check and tells you so. And once a contract is pinned,
an agent cannot quietly weaken it to make a failing check pass.

The name comes from the silversmiths' touchmark: the maker's stamp that vouched for a piece,
recorded on the guild's touchplate, a public register nobody could quietly alter. Here the
proof is the mark on each function, and `touchplate.lock` is the register of pinned contracts.

```tmk
type Cents = int where 0 <= it && it <= 1_000_000_000_000
type Account = { id: int, balance: Cents, frozen: bool }
type Moved = { from: Account, to: Account }
enum TransferError { Frozen, Insufficient(short: Cents) }

pub fn transfer(from: Account, to: Account, amount: Cents) -> Result<Moved, TransferError>
  requires from.id != to.id
  requires to.balance + amount <= 1_000_000_000_000
  ensures result is Ok(m) ==> m.from == ({ ...from, balance: from.balance - amount })
  ensures result is Ok(m) ==> m.to == ({ ...to, balance: to.balance + amount })
  ensures (result is Err(Frozen)) == (from.frozen || to.frozen)
  ensures (result is Ok(m)) == (!from.frozen && !to.frozen && amount <= from.balance)
  ensures result is Err(Insufficient(s)) ==> s == amount - from.balance
{
  if from.frozen || to.frozen { return Err(Frozen) }
  if from.balance < amount { return Err(Insufficient(amount - from.balance)) }
  Ok({ from: { ...from, balance: from.balance - amount },
       to:   { ...to,   balance: to.balance + amount } })
}
```

```text
$ tmk run examples/ledger.tmk
ok ledger.tmk: 26 checks proved, 0 kept at run time
moved, new balance: 30
```

A proof is only as good as the contract, so Touchmark also shows a reviewer what a contract means,
with concrete values from the solver:

```text
$ tmk examples examples/ledger.tmk
transfer
  allowed    transfer(from: { id: -999, balance: 999, frozen: true }, to: { id: -1000, balance: 0, frozen: false }, amount: 1000) -> Err(Frozen)
  decided    every allowed input has exactly one allowed result
  forbidden  transfer(from: { id: -999, balance: 999, frozen: false }, to: { id: -1000, balance: 0, frozen: false }, amount: 1000) -> Err(Insufficient(0))   (breaks `ensures result is Err(Insufficient(s)) ==> s == amount - from.balance`)
  rejected   transfer(from: { id: -999, balance: 999, frozen: false }, to: { id: -999, balance: 0, frozen: false }, amount: 1000)   (breaks `requires from.id != to.id`)
  ...
```

`decided` is proved: this contract fixes the result for every input. The first version of
this example only pinned the two balances, and `tmk examples` showed why that was not
enough: the same transfer "may return" accounts with different ids, then an `Insufficient`
error with any shortfall, then `Err(Insufficient(0))` where the money was there. Each finding
became one `ensures` line. A weak contract such as `sum`'s `ensures result >= 0` shows up as
`sum(a: [2, 2]) -> 0` allowed.

## What it catches

Each line below is a test in `compiler/tests/cli.rs`.

| Mistake | What `tmk check` says |
|---|---|
| Off-by-one in the transfer | `E0204` postcondition may not hold, with the inputs and the wrong `result` |
| A missing bound on a sum | `E0205` value may not be a valid `Cents`, with inputs whose sum exceeds the limit |
| `transfer(from: a, to: a, ...)` | `E0203` this call may break `requires from.id != to.id` |
| Arguments swapped, `transfer(to: b, from: a, ...)` | `E0117`, with the corrected call as the fix |
| Contradictory `requires` (proofs would be vacuous) | `E0209` |
| The midpoint bug that sat in Java's `Arrays.binarySearch` for nine years, `(lo + hi) / 2` | `E0201` overflow, counterexample `lo = hi = int.max` |
| A pure function that starts printing | `E0106` effect not declared |
| A loop invariant that does not hold, a loop that may not end | `E0207`, `E0208` |
| An agent weakens a pinned `ensures` to hide a bug | `E0301` contract WEAKER than the pinned one: the removed and added clauses, a result the new contract allows and the old one forbids, and, if the pinned contract decided every result, an input that now has two |
| A pinned function gains an effect | `E0303` |
| `while j <= n` writing `composite[j]` in a sieve | `E0210` index may be out of bounds |
| An unsorted array passed to a binary search that requires sorted input | `E0203`, at the call |
| A sliding-window sum whose invariant covers one element too many | `E0206`, with `sum` in the invariant |

## Speed

Array workloads written the same way in eight languages, each built with its usual release
settings (full table, flags and caveats: [`bench/arrays/RESULTS.md`](bench/arrays/RESULTS.md)):

| Workload | Touchmark | C | Rust | Go | Java | JavaScript | Python |
|---|---|---|---|---|---|---|---|
| Sieve, n = 50M | **304 ms** | 308 | 316 | 315 | 358 | 446 | 11,749 |
| Matrix multiply 400×400 | 52 ms | **50** | 53 | 113 | 157 | 173 | 5,667 |
| Quicksort, 5M integers | **533 ms** | 556 | 567 | 579 | 667 | 1,138 | 13,958 |
| Sum of 10M integers ×20 | 199 ms | 209 | **179** | 257 | 285 | 381 | 3,835 |

Touchmark runs in the same band as C and Rust (noise here is about 5%) and 2-100 times faster than
Go, Java, JavaScript and Python on these programs. The difference is what is known: in the Touchmark
programs every array index and every arithmetic operation is proved safe before compiling, so
the binary has no bounds or overflow checks at all. C checks nothing; Rust checks bounds at run
time and silently wraps on overflow in release builds.

Three things make proved code fast rather than merely safe: proved checks are deleted, proved
facts are handed to the C optimiser, and proved-safe divisions use cheaper 32-bit or unsigned
instructions. On the integer benchmarks in `bench/perf/` that last one lets Touchmark beat Rust on
`primes` (665 ms against 673).

Proofs used to cost source length, because every contract a proof needed had to be written.
The compiler now infers loop invariants, and the `requires` and `ensures` of private functions
(from their call sites and returns), and proves what it infers like hand-written contracts.
Across the seven benchmark programs, Touchmark source shrank by a third (2,185 to 1,438 tokens) and
one hand-written invariant is left in total. `primes` and `isqrt` are now shorter than the Rust
versions, and all seven together are within 18% of Rust (`bench/arrays/RESULTS.md`).

## Built for models as well as people

- **The whole language fits in 2,688 tokens.** [`llms.txt`](llms.txt) is the complete
  reference; a model that reads it can write Touchmark.
- **Short.** On four small programs with identical proved contracts, Touchmark takes 230 tokens against
  Dafny's 249, Verus's 266 and Vera's 447 (`bench/tokens/`).
- **Errors are instructions.** Every diagnostic has a stable code, a fix, and a counterexample
  where there is one, in text or `--json`; `tmk explain E0201` explains any code.
- **Contracts you do not have to write.** Loop invariants, and the contracts of private
  functions, are inferred and proved automatically, and callers of a small private helper see
  exactly what it computes ([`examples/grid.tmk`](examples/grid.tmk) indexes through
  `at(clamp(...), clamp(...), w)` with every index proved and no contract written).
  `--show-inferred` shows what was inferred. You write contracts for the public API and for
  the properties you care about.
- **Contracts a reviewer can check at a glance.** `tmk examples` prints inputs and
  results a contract allows, forbids and rejects, and says whether it decides the result.
- **A definite finish line.** `N checks proved, 0 kept at run time` tells an agent it is done.
- **Greppable.** Effects are called by name (`io.print`), variant names are global, there are no
  macros, overloading or implicit conversions.

## Does it help agents? Round 1 says: not yet shown

I pre-registered a test ([`eval/PROTOCOL.md`](eval/PROTOCOL.md)): 12 small tasks built to provoke
overflow, off-by-one, negative modulo and empty-array bugs, three trials each, one fresh agent per run
writing Touchmark or Rust, scored on hidden tests. Results in [`eval/RESULTS.md`](eval/RESULTS.md).

| Arm | Runs | Runs with a defect | Hidden tests passed | T_work mean | Tool calls mean |
|---|---|---|---|---|---|
| Touchmark | 36 | 0 | 789/789 | 13,386 | 6.9 |
| Rust | 36 | 0 | 789/789 | 3,765 | 2.5 |

0 of 36 Touchmark runs and 0 of 36 Rust runs shipped an escaped defect (Fisher's exact test p = 1.0).
Both arms are at ceiling, so the defect question is unanswered. Touchmark cost 3.6x the task-specific
tokens of Rust (T_work 13,386 vs 3,765; 3.0x in the first-session runs alone) and 2.7x the tool calls.
The hypothesis that Touchmark lowers token cost is not supported. The runs also found where the
language gets in the way (sortedness preconditions nobody could use, modulo contracts that would not
prove, no sums in contracts), and those drive round 2, which needs harder tasks.

Since then I fixed those findings. Sortedness can be written over adjacent pairs or all pairs, a caller
that builds its array with `push` proves it, and a kept check is one pass. Contracts can use
`sum j in lo..hi: e`, `i128` holds exact sums, and the natural `ring_index` contract
`result == ((head + offset) % cap + cap) % cap` is proved. The reference shrank from 3,225 to 2,688
tokens and now tells agents that a proved `ensures` replaces their tests. Whether any of this
changes the outcome is for round 2 to measure.

## Use it

Requires Rust (to build the compiler), a C compiler (gcc or clang) and `z3` on `PATH`.

```sh
cargo build --release
./target/release/tmk check examples/ledger.tmk     # prove
./target/release/tmk run examples/ledger.tmk       # prove, compile, run
./target/release/tmk lock examples/ledger.tmk      # pin public contracts in touchplate.lock
./target/release/tmk examples examples/ledger.tmk  # what each contract allows and forbids
./target/release/tmk check --json examples/bugs/midpoint.tmk
./target/release/tmk explain E0301
cargo test                                            # end-to-end tests
```

## Status: v0.2 in progress

Working: `int` and `i128` with proved overflow safety, booleans, records, enums, `Option`, `Result`,
refinement types, arrays with proved bounds (reference-counted, copy-on-write, moved on last
use, so passing and returning arrays does not copy), `for` loops with automatic termination
proofs, `forall`/`exists`/`sum` in contracts, functions, `let`/`var`, `if`/`match`/`while`,
`requires`/`ensures`/`invariant`/`decreases`, inference of loop invariants and of private
functions' contracts, effects, named arguments, the contract lock, and native binaries through C. Not yet: strings as values, arrays inside other values, generics,
modules, a standard library beyond `io.print`, FFI, concurrency, and proof of termination for
recursion. The plan, the evidence behind every
design rule, and the stop rules are in [`docs/DESIGN.md`](docs/DESIGN.md).

## Where this came from

Touchmark started in 2021 as AS, a small interpreted language; that prototype is in `legacy/`.
In 2026 it was redesigned from a research study of what a new language could fix now that agents write
most code: [`reports/New programming language pain points.md`](reports/New%20programming%20language%20pain%20points.md),
[`why_llm_languages_flopped.md`](research_notes/New%20programming%20language%20pain%20points/why_llm_languages_flopped.md),
and a hands-on test of the closest rival, [`phase0/lemmascript_assessment.md`](phase0/lemmascript_assessment.md).

## License

MIT
