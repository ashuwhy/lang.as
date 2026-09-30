# Agent evaluation, round 1: results (all 72 runs)

Protocol: `eval/PROTOCOL.md` (pre-registered). The hidden tests, reference implementation and scorer are published in
`eval/private/`; their SHA-256 match the pre-registered values (`tasks.py` 3b1fa64f...2387a07, `harness.py`
a8115c1f...bdd8845). The transcript reader `usage.py` was pre-registered as c9178953...ad708d5; it was later extended
for macOS paths, to record the model, and with a per-cause token breakdown (now 08af0b14...9de3c4). The token
formulas did not change. Every scored agent solution is in `eval/solutions/<run>/`, the experimenter's gold solutions
in `eval/gold/`, and `eval/private/report.py` regenerates every number below.

**Status.** Complete. The session ran out before trial 3 finished, so nine trial-3 runs (`r3-ring_index-tmk`,
`r3-dedupe_sorted-tmk`, `r3-merge_sorted-tmk`, `r3-exact_sum-tmk`, `r3-exact_sum-rs`, `r3-gcd-tmk`, `r3-gcd-rs`,
`r3-isqrt-tmk`, `r3-isqrt-rs`) were re-run in a new session with the same prompts. What differed is listed in the
protocol's change log; the effect on tokens is shown below.

## Primary outcome: runs with at least one escaped defect

| Arm | Runs | Runs with a defect | Hidden tests passed | T_work mean | T_ctx mean | T_proc mean | Tool calls mean | Wall s median | Solution tokens mean |
|---|---|---|---|---|---|---|---|---|---|
| Touchmark | 36 | 0 | 789/789 | 13,386 | 61,147 | 431,848 | 6.9 | 41.0 | 273 |
| Rust | 36 | 0 | 789/789 | 3,765 | 50,298 | 125,172 | 2.5 | 10.9 | 162 |

Fisher's exact test (two-sided) on 0/36 vs 0/36: p = 1.0. **No difference in escaped defects: both arms are at
ceiling.** Every run of both languages passed every hidden test, and no run reported DONE with a solution that did not
build. On these tasks a frontier agent writing Rust did not ship the bugs the tasks were designed to provoke (overflow,
off-by-one, negative modulo, empty arrays), so the experiment cannot show that Touchmark prevents them.

## Secondary outcome: tokens

Touchmark cost **3.6x the task-specific tokens (T_work)** of Rust, 3.5x the processed tokens (T_proc), 2.7x the tool
calls and 3.8x the wall time, and its solutions were 1.7x longer (contracts and invariants are part of the code).
**The hypothesis that Touchmark lowers token cost is not supported.**

The pooled 3.6x overstates the gap. The re-run cost more in both arms, and it re-ran six Touchmark runs but only three
Rust runs:

| Session | Touchmark runs | T_work mean | Tool calls | Rust runs | T_work mean | Tool calls | Ratio |
|---|---|---|---|---|---|---|---|
| First (trials 1-3) | 30 | 10,672 | 5.1 | 33 | 3,543 | 2.4 | 3.0x |
| Re-run (trial 3) | 6 | 26,952 | 15.5 | 3 | 6,199 | 3.3 | 4.3x |

Within the re-run, the three tasks run in both languages cost 5.7x (`exact_sum`), 3.2x (`gcd`) and 2.0x (`isqrt`).
The first-session figure, 3.0x on 63 runs from one environment, is the better estimate of the language effect.

Where the tokens went (re-run transcripts only; the first session's transcripts are not on this host). Tokens of
tool inputs plus results, o200k:

| Run | Reading the reference | Compiler and test runs | Writing files | Compiler runs | with an error | with W0250 |
|---|---|---|---|---|---|---|
| r3-ring_index-tmk | 3,691 | 17,545 | 0 | 22 | 2 | 1 |
| r3-dedupe_sorted-tmk | 3,694 | 5,142 | 0 | 10 | 1 | 1 |
| r3-merge_sorted-tmk | 3,692 | 6,530 | 684 | 14 | 1 | 0 |
| r3-exact_sum-tmk | 3,693 | 7,269 | 0 | 16 | 2 | 2 |
| r3-gcd-tmk | 3,691 | 2,984 | 1,710 | 7 | 1 | 0 |
| r3-isqrt-tmk | 3,691 | 2,207 | 0 | 5 | 1 | 0 |
| r3-exact_sum-rs | 0 | 783 | 0 | 2 | 1 | 0 |
| r3-gcd-rs | 0 | 967 | 250 | 2 | 1 | 0 |
| r3-isqrt-rs | 0 | 379 | 923 | 3 | 1 | 0 |

The reference costs about 3,700 tokens per run, a third of the first session's Touchmark T_work. The larger share is
the compile-and-test loop: Touchmark agents ran the compiler 5 to 22 times, Rust agents 2 or 3 times, and most
Touchmark runs were not fixing verifier errors (1 or 2 per run) but running the agent's own tests. A proof that the
code meets its contract did not stop agents from testing it, because they could not tell whether the contract
said what the specification says.

## Per task (mean over three trials)

| Task | TMK runs | TMK tests | TMK T_work | RS runs | RS tests | RS T_work |
|---|---|---|---|---|---|---|
| midpoint | 3 | 75/75 | 8,833 | 3 | 75/75 | 3,518 |
| percent_of | 3 | 69/69 | 8,474 | 3 | 69/69 | 3,293 |
| ring_index | 3 | 72/72 | 21,954 | 3 | 72/72 | 3,505 |
| lower_bound | 3 | 69/69 | 12,427 | 3 | 69/69 | 3,725 |
| max_window_sum | 3 | 60/60 | 13,691 | 3 | 60/60 | 3,835 |
| rotate_left | 3 | 60/60 | 8,573 | 3 | 60/60 | 3,486 |
| dedupe_sorted | 3 | 48/48 | 17,000 | 3 | 48/48 | 3,289 |
| merge_sorted | 3 | 51/51 | 19,071 | 3 | 51/51 | 3,777 |
| bucket_counts | 3 | 63/63 | 9,574 | 3 | 63/63 | 3,545 |
| exact_sum | 3 | 72/72 | 17,550 | 3 | 72/72 | 4,116 |
| gcd | 3 | 75/75 | 13,159 | 3 | 75/75 | 4,401 |
| isqrt | 3 | 75/75 | 10,321 | 3 | 75/75 | 4,687 |

## What the Touchmark runs taught us (usability findings, drive round 2)

- **Sortedness preconditions are unusable in practice.** Agents on `lower_bound`, `merge_sorted` and
  `dedupe_sorted` deliberately dropped `requires sorted(a)`: a caller that builds its array in a loop cannot prove it
  (compile error E0203), and the kept run-time check compares every pair (O(n^2)). Needed: an O(n) adjacent-pairs
  run-time form, and caller-side proof support for arrays built by `push` in a loop. The re-run reproduced this
  without any hint: `dedupe_sorted` dropped the precondition again, and `merge_sorted` moved it into the postcondition
  as `sorted(a) && sorted(b) ==> sorted(result)`, a third workaround for the same gap.
- **Nonlinear modulo in contracts** (`ring_index`): agents could not get `(head + offset - result) % cap == 0` proved and
  rewrote it as explicit cases. The re-run found a fourth form,
  `exists q in offset / cap - 1..offset / cap + 2: head + offset == cap * q + result`. Diagnosis since: the natural form
  `result == ((head + offset) % cap + cap) % cap` times out because the verifier asks Z3 incrementally (push/pop); the
  same query on its own is proved in 0.6 s.
- **One agent (`dedupe_sorted`) allocated an extra O(n) array only to make a postcondition provable.** Proof burden
  leaking into run-time cost is a design smell.
- From the gold solutions: no sums in contracts (sliding-window invariants inexpressible), no 128-bit integers,
  accumulator lower bounds not inferred.
- No agent read the hidden tests, the gold solutions or another run's files. The audit flagged paths in three re-runs,
  all benign: output files of the agent's own background commands and one scratch copy of its own solution, both in
  the runner's session directory where its system prompt sends temporary files, plus `/usr/bin/time` and `/var` (temp).

## Honest reading

1. The pre-registered tasks were too easy for today's agents in Rust: a ceiling effect, so the defect question is
   unanswered, not answered in Touchmark's favour.
2. Touchmark's real, measured cost is about 3x task tokens (3.6x pooled), from reading the reference and, more, from
   compile-and-test turns that a proof did not remove.
3. Next round must (a) use harder, multi-function tasks or tasks chosen by someone else / an existing benchmark,
   where Rust agents do ship bugs, (b) cut the reference and turn count, (c) fix the findings above first, and
   (d) add a Gemini arm when an API key is available.
