
## Agent evaluation round 1 (interim, 2026-09-30)

See `eval/RESULTS.md`. 63 of 72 pre-registered runs scored: 0 escaped defects in either arm (ceiling effect;
the defect question is unanswered), Touchmark ~3.0x task tokens (T_work) and ~2.1x tool calls of Rust.
Nine trial-3 runs were cut off by the session limit and must be re-run with the same prompts
(`eval/private/harness.py` `prompt()`); the list is in RESULTS.md. Next: finish those, then round 2 per the
findings (cheap sortedness checks, caller-side sortedness proofs, sums in contracts, 128-bit ints, nonlinear
modulo, shorter reference / fewer turns) and harder, externally chosen tasks.
