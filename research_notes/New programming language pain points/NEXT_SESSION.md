# Next session: start Phase 0 of Assured Script

Paste the block below into a new session opened on `ashuwhy/lang.as`, branch
`research/language-pain-points`, with `ashuwhy/SepInfer` attached.

```text
Continue my programming-language research in ashuwhy/lang.as (branch research/language-pain-points).
Also attach my private repo ashuwhy/SepInfer (paper at paper/main-pldi.tex).

Read first, in this order:
1. research_notes/New programming language pain points/HANDOFF.md (git rules and context).
2. reports/New programming language pain points.md, especially "Pinned contracts and sealed
   boundaries in a sound TypeScript dialect", "LemmaScript holds the lock; the boundary is still
   open", and "A roadmap that earns the right to become a language".

Settled problem statement (do not reopen it without new evidence): AI agents now write a large
share of code, and teams cannot cheaply establish that it does what was intended. Tests get
gamed, agents weaken specifications until checks pass, and TypeScript's erased types let
unchecked data cross trust boundaries. The proposed answer is Assured Script: a small, sound,
TypeScript-shaped language for critical modules (money, permissions, parsing), with SMT-checked
requires/ensures contracts and effects, compiled to TypeScript, .d.ts and Standard Schema
validators, whose killer feature is a contract lock over sealed boundaries. It is unproven.
LemmaScript (npm, Midspiral) is the closest rival.

This session's job is Phase 0 preparation, not a compiler. The SepInfer PLDI 2027 submission
(deadline 12 Nov 2026) comes first, so keep this to design, checks and small spikes.

Steps. Commit and push after each one.
1. Network check: curl -s -o /dev/null -w '%{http_code}' https://arxiv.org/ (000 = blocked).
   If primary sites are blocked, say so, work from web search, and tag snippet-only facts.
2. LemmaScript hands-on assessment (most important; it can flip the plan). Install it from npm,
   read its source and docs, and run it on two or three small annotated TypeScript functions.
   Answer: how it treats any, casts, non-null assertions and unchecked JSON; whether it pins
   contracts across versions or only checks additions-only proof edits; whether it has effects,
   taint or boundary validators; what verifier latency looks like; and who outside Midspiral
   uses it. Write phase0/lemmascript_assessment.md with a clear verdict: does it already cover
   the sealed-boundary edge the report claims for Assured Script?
3. Pre-registration. Write phase0/PREREGISTRATION.md for the study in the report's Phase 0 row:
   60-100 tasks from real TypeScript modules (money, permissions, parsing), at least a third of
   them boundary or effect bugs; four arms (TypeScript plus tests; TypeScript plus property-tested
   contracts; TypeScript annotated for LemmaScript; Dafny with pinned contracts and generated
   boundary validators compiled to JavaScript as the stand-in for the sound core); metrics
   (escaped defects by hidden tests and mutation, how often agents weaken contracts, use of any,
   casts and !, cost, latency, a 10-12 developer review-time sub-study); the exit thresholds and
   stop or pivot rules from the report, stated before any data is collected; the analysis plan.
   Revise the design if step 2 changes it.
4. Task sources. Write phase0/task_sources.md listing candidate modules and bugs with links:
   fix commits for 2025-2026 npm advisories (n8n CVE-2026-25049, axios, handlebars, sequelize,
   tmp, node-tar), SepInfer's corpora and its zod 4.5.4 defect, and payments, entitlement and
   parser code in popular open-source TypeScript repos.
5. Small spike, only if time remains: phase0/contract-lock/, a command that takes two versions
   of a function contract (requires/ensures over integers, records and booleans) and uses Z3 to
   decide whether the new one refines the old one, printing a counterexample when it weakens.
   Reuse SepInfer's floor-guard idea (lib/agent/spec_floor.ml) and its Node TypeScript oracle
   where they fit.
6. Rewrite the AS Lang README.md and docs/EXPLAINER.md so they claim only what the code does
   (the report lists the false claims: SIMD, async/await, constant folding, "10x faster than
   Python"). Mark the project as being rebuilt as Assured Script.

Rules (from HANDOFF.md): commit as Ashutosh Sharma <ashutoshsharmawhy@gmail.com>; never add
Co-Authored-By, Claude-Session or any Claude or Anthropic line to commits, PRs or comments;
never create branches starting with claude/; work only on research/language-pain-points unless
I name another branch; do not open a pull request unless I ask. No authorship line naming an AI
in any file. At the end, update HANDOFF.md with what was done and what is next, and send me a
short summary.
```
