## Agent evaluation round 1 (complete, 2026-10-01)

See `eval/RESULTS.md`. All 72 pre-registered runs scored: 0 of 36 Touchmark runs and 0 of 36 Rust runs shipped an
escaped defect (Fisher p = 1.0; ceiling effect, the defect question is unanswered). Touchmark cost 3.6x the task
tokens (T_work) of Rust pooled, 3.0x in the first-session runs alone, and 2.7x the tool calls. Nine trial-3 runs
were re-run in a second session after a usage limit; the differences are in the protocol's change log.
`eval/private/report.py` regenerates every number.

Next: round 2 per the findings (O(n) sortedness checks and caller-side sortedness proofs, sums in contracts,
128-bit ints, nonlinear modulo, shorter reference / fewer compile-and-test turns), then a pre-registered round 2
on harder, multi-function or externally chosen tasks where Rust agents do ship bugs.
