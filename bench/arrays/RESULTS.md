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

What differs is not the speed but what each program is known not to do:

| | Out-of-bounds access | Integer overflow | Evidence |
|---|---|---|---|
| AS | proved impossible, no run-time checks | proved impossible | 14 / 64 / 52 / 24 checks proved per program (plus every inferred loop invariant) |
| C, C++ | undefined behaviour, unchecked | undefined behaviour | none |
| Rust | checked at run time (panic) | wraps silently in release builds | none |
| Go, Java, JavaScript, Python | checked at run time | wraps (Go, Java), loses precision (JS) or grows (Python) | none |

The cost is on the page: AS sources state the contracts their proofs need. Since this table was
first made, the compiler infers loop invariants (Houdini-style: template candidates, kept only
when inductive, then proved like hand-written ones), and a greedy pass removed every
hand-written invariant it made redundant. Source size in o200k tokens:

| Program | AS, invariants by hand | AS, with inference | Rust | Go | Python |
|---|---|---|---|---|---|
| sieve | 171 | 151 | 101 | 93 | 66 |
| matmul | 611 | 528 | 255 | 239 | 191 |
| quicksort | 411 | 359 | 270 | 254 | 249 |
| sum | 239 | 148 | 122 | 122 | 78 |
| primes (`bench/perf`) | 191 | 148 | 141 | - | - |
| collatz (`bench/perf`) | 299 | 258 | 174 | - | - |
| isqrt (`bench/perf`) | 263 | 209 | 158 | - | - |
| total | 2,185 | 1,801 (-18%) | 1,221 | | |

`sieve`, `quicksort`, `sum`, `primes` and `collatz` now need no hand-written invariants at all.
What remains is intent the compiler cannot guess (the square bounds in `isqrt`, sortedness facts
in binary search) and function contracts, such as the element bounds `matmul` requires of its
inputs, which Rust does not state at all. Inferring the contracts of private functions is the
next step. Inference costs verification time: the heaviest programs here take 4-8 s to check
instead of about 1 s; `--no-infer` skips it when every invariant is written by hand.
