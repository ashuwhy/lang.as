# Agent evaluation, round 1: results (interim, 63 of 72 runs scored)

Protocol: `eval/PROTOCOL.md` (pre-registered). The hidden tests, reference implementation and scorer are now
published in `eval/private/`; their SHA-256 match the pre-registered values
(`tasks.py` 3b1fa64f...2387a07, `harness.py` a8115c1f...bdd8845, `usage.py` c9178953...ad708d5).
Every scored agent solution is in `eval/solutions/<run>/`, the experimenter's gold solutions in `eval/gold/`.

**Status.** The session ran out before trial 3 finished. Not yet scored (9 runs, to be re-run with
the same prompts in a fresh environment and added here): `r3-ring_index-tmk`, `r3-dedupe_sorted-tmk`, `r3-merge_sorted-tmk`, `r3-exact_sum-tmk`, `r3-exact_sum-rs`, `r3-gcd-tmk`, `r3-gcd-rs`, `r3-isqrt-tmk`, `r3-isqrt-rs`.

## Primary outcome: runs with at least one escaped defect

| Arm | Runs | Runs with a defect | Hidden tests passed | T_work mean | T_ctx mean | T_proc mean | Tool calls mean | Wall s median |
|---|---|---|---|---|---|---|---|---|
| Touchmark | 30 | 0 | 658/658 | 10,672 | 56,043 | 272,784 | 5.1 | 34.6 |
| Rust | 33 | 0 | 715/715 | 3,543 | 48,883 | 113,271 | 2.4 | 10.8 |

Fisher's exact test (two-sided) on 0/30 vs 0/33: p = 1.0. **No difference in escaped defects: both arms
are at ceiling.** Every run of both languages passed every hidden test. On these tasks a frontier agent writing Rust
did not ship the bugs the tasks were designed to provoke (overflow, off-by-one, negative modulo, empty arrays), so the
experiment cannot show that Touchmark prevents them.

## Secondary outcome: tokens

Touchmark cost about **3.0x the task-specific tokens (T_work)** of Rust,
2.4x the processed tokens (T_proc), about 2.1x the tool calls and
3.2x the wall time. Most of the gap is reading the
3,225-token reference and the extra verify-and-fix turns (not yet broken down per cause).
**The hypothesis that Touchmark lowers token cost is not supported in this round.**

## Per task (mean over scored trials)

| Task | TMK runs | TMK tests | TMK T_work | RS runs | RS tests | RS T_work |
|---|---|---|---|---|---|---|
| midpoint | 3 | 75/75 | 8,833 | 3 | 75/75 | 3,518 |
| percent_of | 3 | 69/69 | 8,474 | 3 | 69/69 | 3,293 |
| ring_index | 2 | 48/48 | 10,299 | 3 | 72/72 | 3,505 |
| lower_bound | 3 | 69/69 | 12,427 | 3 | 69/69 | 3,725 |
| max_window_sum | 3 | 60/60 | 13,691 | 3 | 60/60 | 3,835 |
| rotate_left | 3 | 60/60 | 8,573 | 3 | 60/60 | 3,486 |
| dedupe_sorted | 2 | 32/32 | 14,426 | 3 | 48/48 | 3,289 |
| merge_sorted | 2 | 34/34 | 13,740 | 3 | 51/51 | 3,777 |
| bucket_counts | 3 | 63/63 | 9,574 | 3 | 63/63 | 3,545 |
| exact_sum | 2 | 48/48 | 10,763 | 2 | 48/48 | 3,426 |
| gcd | 2 | 50/50 | 9,832 | 2 | 50/50 | 3,516 |
| isqrt | 2 | 50/50 | 8,668 | 2 | 50/50 | 3,566 |

## What the Touchmark runs taught us (usability findings, drive round 2)

- **Sortedness preconditions are unusable in practice.** Agents on `lower_bound`, `merge_sorted` and
  `dedupe_sorted` deliberately dropped `requires sorted(a)`: a caller that builds its array in a loop cannot prove it
  (compile error E0203), and the kept run-time check compares every pair (O(n^2)). Needed: an O(n) adjacent-pairs
  run-time form, and caller-side proof support for arrays built by `push` in a loop.
- **Nonlinear modulo in contracts** (`ring_index`): agents could not get `(head + offset - result) % cap == 0` proved and
  rewrote it as explicit cases.
- **One agent (`dedupe_sorted`) allocated an extra O(n) array only to make a postcondition provable.** Proof burden
  leaking into run-time cost is a design smell.
- From the gold solutions: no sums in contracts (sliding-window invariants inexpressible), no 128-bit integers,
  accumulator lower bounds not inferred.
- No agent in either arm read or touched files outside its directory.

## Honest reading

1. The pre-registered tasks were too easy for today's agents in Rust: a ceiling effect, so the defect question is
   unanswered, not answered in Touchmark's favour.
2. Touchmark's real, measured cost is ~3x task tokens, mostly the reference and verification round trips.
3. Next round must (a) use harder, multi-function tasks or tasks chosen by someone else / an existing benchmark,
   where Rust agents do ship bugs, (b) cut the reference and turn count, (c) fix the findings above first, and
   (d) add a Gemini arm when an API key is available.

