# Array benchmarks: AS against seven languages

Run on 2026-09-30 in a 4-core Intel Xeon @ 2.10 GHz cloud container with `RUNS=5 python3
bench/arrays/run.py`. Every program implements the same algorithm and prints the same answer.
Each language uses its normal release build:

| Language | Toolchain and flags |
|---|---|
| AS | `aslang build` (Z3 proves the checks, then GCC 13.3 `-O2`) |
| C, C++ | GCC 13.3 `-O2` |
| Rust | rustc 1.94 `-C opt-level=3` (Cargo's release default) |
| Go | Go 1.24 `go build` |
| Java | OpenJDK, `javac` + `java` (JIT; includes JVM start-up) |
| JavaScript | Node.js 22 |
| Python | CPython 3.11, one run |

Times are medians. On this machine run-to-run noise is about 5%, and one C result moved from
152 ms to 209 ms between sessions, so differences of a few percent are not meaningful.

| Workload | AS | C | C++ | Rust | Go | Java | JavaScript | Python |
|---|---|---|---|---|---|---|---|---|
| Sieve of Eratosthenes, n = 50M | **304 ms** | 308 | 311 | 316 | 315 | 358 | 446 | 11,749 |
| Matrix multiply, 400×400, flat arrays | 52 ms | **50** | 54 | 53 | 113 | 157 | 173 | 5,667 |
| Quicksort, 5M integers | **533 ms** | 556 | 577 | 567 | 579 | 667 | 1,138 | 13,958 |
| Sum of 10M integers, 20 passes | 199 ms | 209 | 211 | **179** | 257 | 285 | 381 | 3,835 |
| Peak memory (sieve / matmul / quicksort / sum) | 49 / 10 / 40 / 78 MB | same as AS | 10 / 10 / 41 / 79 | 50 / 10 / 40 / 78 | 50 / 10 / 40 / 78 | 87 / 43 / 79 / 116 | 96 / 54 / 89 / 126 | 55 / 23 / 199 / 390 |

After contract inference (below), a re-run in the same container type measured AS at 1.11x, 1.06x,
0.99x and 1.12x the C time on the four workloads. That run was noisier: `sum`, whose source did
not change at all, moved from 0.95x to 1.12x. An interleaved A/B of the sieve binaries built
from the old and new sources gave the same times (about 375 ms each, C 379 ms).

What differs is not the speed but what each program is known not to do:

| | Out-of-bounds access | Integer overflow | Evidence |
|---|---|---|---|
| AS | proved impossible, no run-time checks | proved impossible | 13 / 52 / 47 / 24 checks proved per program (plus every inferred invariant and contract) |
| C, C++ | undefined behaviour, unchecked | undefined behaviour | none |
| Rust | checked at run time (panic) | wraps silently in release builds | none |
| Go, Java, JavaScript, Python | checked at run time | wraps (Go, Java), loses precision (JS) or grows (Python) | none |

Proofs used to cost source length: every contract a proof needed had to be written. The
compiler now infers two kinds of them and proves what it infers like hand-written ones:

- loop invariants (Houdini: template candidates, kept only when they hold on entry and survive
  every iteration);
- the `requires` and `ensures` of private functions (Houdini across the module: a `requires`
  candidate is kept only if every call site proves it, an `ensures` candidate only if every
  return does). `pub` functions are the API and keep their written, pinned contracts.

A greedy pass then removed every hand-written invariant and private contract the proofs no
longer needed. The `bench/perf` functions were `pub` before and are private now, as in the Rust
versions. Source size in o200k tokens:

| Program | AS, all by hand | AS, inferred invariants | AS, inferred invariants and contracts | Rust | Go | Python |
|---|---|---|---|---|---|---|
| sieve | 171 | 151 | 134 | 101 | 93 | 66 |
| matmul | 611 | 528 | 309 | 255 | 239 | 191 |
| quicksort | 411 | 359 | 335 | 270 | 254 | 249 |
| sum | 239 | 148 | 148 | 122 | 122 | 78 |
| primes (`bench/perf`) | 191 | 148 | 130 | 141 | - | - |
| collatz (`bench/perf`) | 299 | 258 | 233 | 174 | - | - |
| isqrt (`bench/perf`) | 263 | 209 | 148 | 158 | - | - |
| total | 2,185 | 1,801 | 1,437 (-34%) | 1,221 | | |

One hand-written line is left in the seven programs: the element range of the result matrix
in `matmul` (`c[t] <= n * 1_000_000`), which bounds the checksum sums. `primes` and `isqrt` are
now shorter than their Rust versions. The rest of the gap to Rust is syntax (for example
`quicksort` swaps through a temporary where Rust calls `v.swap`). `aslang check
--show-inferred` prints every inferred invariant and contract. Inference costs verification
time: `matmul` takes about 9 s and `quicksort` about 8 s to check, the others under 4 s;
`--no-infer` turns it off.
