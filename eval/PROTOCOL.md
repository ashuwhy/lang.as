# Agent evaluation, round 1: protocol (pre-registered)

Written and committed on 2026-09-30, before any evaluated agent run. Changes after this commit
are listed at the end with their date and reason; nothing above that list is edited.

## Question

When a frontier coding agent implements the same small, bug-prone functions in Touchmark and in
Rust, which language's solutions pass more hidden tests (fewer escaped defects), and at what
token cost?

## Arms

- **Touchmark**: compiler at commit `8716eff`, installed as `tmk`; the agent is told to read
  `llms.txt` (the complete reference, 3,225 o200k tokens), which counts toward its tokens.
- **Rust**: rustc 1.94.1, edition 2021. No reference is given (models already know Rust).
- Agents: fresh sub-agents of the same model as the experimenter's session, one per run, with
  shell access and nothing else in context. A Gemini arm will be added when an API key is
  available; it will use the same tasks, prompts and scoring.

## Tasks

Twelve functions, each specified identically for both languages (only type names differ):
`midpoint`, `percent_of`, `ring_index`, `lower_bound`, `max_window_sum`, `rotate_left`,
`dedupe_sorted`, `merge_sorted`, `bucket_counts`, `exact_sum`, `gcd`, `isqrt`. The full
specification text is in the prompts; the prompt template is below.

Disclosed bias: the tasks were chosen by the language's designer and are deliberately
bug-prone in the classes Touchmark targets (64-bit overflow, off-by-one indexing, negative
modulo, empty arrays, extreme values). A later round must use tasks chosen by someone else or
taken from an existing benchmark. Known asymmetries: Rust has 128-bit integers and Touchmark
does not (`exact_sum`, `percent_of` and `midpoint` are easy with them); Touchmark contracts cannot
yet express sums, which makes some loops harder to prove; models have seen far more Rust.

Before any agent ran, the experimenter wrote a solution to every task in both languages; all 24
pass every hidden test. Writing them found three compiler bugs, fixed in commit `8716eff`
(exponential path conditions on runs of `match`; array elements not known to be 64-bit;
`push` rejected at the end of a block). No language feature was added for the tasks.

## Prompt (identical except for the language lines)

> You are implementing one function for a production codebase, in {Touchmark|Rust}.
> {Touchmark: "Touchmark is a new compiled language. Its complete reference is the file
> /opt/touchmark/llms.txt; read it first. The compiler is `tmk` (on PATH)." |
> Rust: "Use Rust (stable, edition 2021). `rustc` and `cargo` are on PATH."}
> Work only inside the directory {dir} ... Write your final solution to {dir}/solution.{tmk|rs}.
> It must define a public function with exactly this signature: {signature} ... It must not
> define `main` ... Specification: {spec} ... Your function will be reviewed and then run against
> hidden tests covering the whole input domain described above, including its edge cases. It
> must return the specified result for every input the specification allows. You may write and
> run your own tests ... When you are finished, reply with the single word DONE.

## Hidden tests and scoring

Each task has 16 to 25 hidden tests: edge cases taken from the specification plus seeded random
cases inside the specified domain, with expected values from a Python reference implementation.
The scorer appends a `main` that calls the agent's function on every test and prints the result,
builds it (`tmk build`, which verifies; `rustc -O`), runs it with a 30 s limit, and compares
line by line. A crash costs only the test that crashed (the rest are re-run in a new program).
A solution that does not build against the specified signature and domain fails every test.

The hidden tests, reference implementation and scorer stay outside the repository until every
run is scored, and are then committed. Their SHA-256 now:

```
3b1fa64f9d7d34d44885e428dca2fea5b391eec341f6801231d8eeedb2387a07  tasks.py
a8115c1f6938504157d9d350727f0b098aead7afd426e02fd2b2f36f4bdd8845  harness.py
```

## Runs

Three independent runs per task and language: 72 runs. Runs are started in a fixed interleaved
order (task by task, alternating languages) so that time-of-day effects fall on both arms. A
pilot of one task in each language checks the harness and token accounting; pilot runs are
reported separately and not counted.

## Outcomes and analysis

- **Primary**: the share of runs with at least one escaped defect (any hidden test failing),
  Touchmark against Rust, compared with Fisher's exact test (two-sided). Reported with the
  counts, not only the p-value.
- **Secondary**: the share of hidden tests passed; total tokens per run as reported by the
  agent runner (input plus output, including the language reference for Touchmark); wall time;
  size of the final solution in o200k tokens; runs where the agent reported DONE but the
  solution does not build.
- Per-task tables for everything, and every solution committed.
- Whatever the result, it is published in `eval/RESULTS.md` and the README in the same words.

## Changes after pre-registration

- 2026-09-30, after the pilot and before any counted run: the token metrics are fixed as
  follows, because the transcripts record output tokens only as mid-stream snapshots.
  **T_ctx** (primary): the size of the agent's conversation at its last model call, that is,
  everything it read and wrote (the runner's own figure matches it to within the last reply).
  **T_proc**: prompt tokens summed over all its model calls (grows with the number of turns).
  **T_work**: T_ctx minus the first call's prompt (the agent's fixed system prompt plus the
  task, the same size for both arms), i.e. the task-specific part. Wall time and tool calls
  come from the same transcripts. Pilot (`midpoint`, not counted): Rust 25/25 tests, T_ctx
  49,024, T_work 3,703, 2 tool calls, 11.5 s; Touchmark 25/25 tests, T_ctx 54,736, T_work
  9,382, 4 tool calls, 23.3 s. No agent read files outside its directory. Usage script SHA-256:
  `c9178953940760eda8b440af13e845defd3b654c5767ceb27fb5c2b11ad708d5`.
- 2026-10-01, after 63 counted runs and before scoring the rest: the first session hit its usage
  limit during trial 3, so the nine unscored trial-3 runs (`r3-ring_index-tmk`,
  `r3-dedupe_sorted-tmk`, `r3-merge_sorted-tmk`, `r3-exact_sum-tmk`, `r3-exact_sum-rs`,
  `r3-gcd-tmk`, `r3-gcd-rs`, `r3-isqrt-tmk`, `r3-isqrt-rs`) were re-run in a new session with
  the same prompts (`harness.py` `prompt()`, unchanged SHA) in fresh `/work/<run-id>/`
  directories, one fresh sub-agent each. The cut-off agents' IDs are kept in `runs.json` as
  `superseded_agent`; nothing from them was scored. What differed from the first session:
  the host was macOS arm64 instead of the Linux container, with rustc 1.93.1 (not 1.94.1) and
  Z3 4.15.2; `/work` was a symlink into the home directory; `tmk` was rebuilt from the same
  compiler source (`git diff 8716eff` empty) and `llms.txt` was byte-identical; `eval/` was
  made unreadable (mode 000) while the agents ran, since they started in the repository root;
  the nine were started at once rather than interleaved by task; the model was recorded
  (`claude-opus-5-5`; the first session did not record it); and my global instructions for
  the agent runner were in every sub-agent's context. Both arms cost more in the re-run
  (RESULTS.md reports it separately). `usage.py` was extended to read macOS paths, record the
  model and break tokens down by cause, with the token formulas unchanged (SHA-256 now
  `08af0b14303ef19bdcd63a248fb499fad3e638f2374bd5b78931324bd99de3c4`); `score_all.py` reads
  the new session's transcript directory; `report.py` was added to regenerate RESULTS.md.
