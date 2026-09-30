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
| AS | proved impossible, no run-time checks | proved impossible | 18 / 72 / 58 / 32 checks proved per program |
| C, C++ | undefined behaviour, unchecked | undefined behaviour | none |
| Rust | checked at run time (panic) | wraps silently in release builds | none |
| Go, Java, JavaScript, Python | checked at run time | wraps (Go, Java), loses precision (JS) or grows (Python) | none |

The cost is on the page: AS sources are longer because they state the contracts the proofs
need. Source size in o200k tokens:

| Workload | AS | C | Rust | Go | Python |
|---|---|---|---|---|---|
| sieve | 171 | 130 | 101 | 93 | 66 |
| matmul | 611 | 350 | 255 | 239 | 191 |
| quicksort | 411 | 334 | 270 | 254 | 249 |
| sum | 239 | 172 | 122 | 122 | 78 |

Most of the extra tokens in `matmul` are element bounds (`forall i in 0..a.len: 0 <= a[i] &&
a[i] <= 1_000`) that exist only to prove the arithmetic cannot overflow. Inferring such
invariants automatically is the most direct way to close this gap and is on the roadmap.

Two compiler changes made during this run mattered: passing proved facts to GCC through
`__attribute__((assume))` instead of branches (branches blocked vectorisation of `sum`), and
leaving out facts about variables the loop changes (they broke GCC's reduction pattern). Before
those fixes AS took 213 ms on `sum`.
