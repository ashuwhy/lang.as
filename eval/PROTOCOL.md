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

(none yet)
