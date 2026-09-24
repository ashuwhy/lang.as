# Gap fill: AI code generation

Checked on 2026-09-24 from my own machine through Chrome. Same tags as
gap_fill_conferences.md: CONFIRMED, CORRECTED, ADDED, COULD NOT VERIFY. Items refer to
ai_code_generation.md unless another file is named.

Provenance: saved from the local session's commit `f8e4353`, which was never pushed because
the session hit its usage limit in the middle of a merge. These are primary reads and take
precedence over `gap_fill_ai_code.md`, which was built from search summaries. (The tag
reference above means `primary_reads_conferences.md` in this folder.)

## Headline changes for the report

1. **METR's 2026 update is a speedup, not a slowdown.** The notes say the 10 returning
   developers still showed "an 18% slowdown". METR writes "a speedup of -18%" right after
   "Our raw results show some evidence for speedup", and the post's figure is captioned
   "Late-2025 AI likely accelerated open-source developers". The -18% is the change in task
   time (negative means faster), CI -38% to +9%. The notes flipped the sign. It still
   crosses zero, and METR still calls it "only very weak evidence".
2. **VERINA says the hard part in Lean is the proof, not the spec.** The notes cite VERINA
   for "LLMs are much worse when they must write the spec". Its abstract gives the best
   model (o3) 72.6% on code, 52.3% on spec soundness and completeness, and **4.9% on
   proofs**. Specs are not solved, but proofs are the weaker link in Lean. The report's
   "the spec is the bottleneck" argument has to lean on Lahiri and de Moura, not on VERINA.
3. **DORA 2025 reversed the throughput finding.** DORA 2024 found AI adoption linked to lower
   throughput (-1.5%) and lower stability (-7.2%). DORA 2025 says "Unlike last year, we
   observe a positive relationship between AI adoption on both software delivery throughput
   and product performance", while the negative link to **stability** remains. "End-to-end
   gains are muted" is out of date. The better line is "faster, and less stable".
4. **The Stack Overflow "29% trust" figure is a misread.** The only 29% on the primary page
   is professional developers who say AI tools struggle with complex tasks, down from 35%.
   Trust is 33% (3% highly), distrust 46%. Drop the conflict note.
5. **GitClear's "7.1% churn in 2025" is suspect.** 7.1% is exactly the *projected* 2024 churn
   in GitClear's 2025 report; actual 2024 churn was 5.7%. The 2026 report could not be opened
   (Cloudflare challenge), so I cannot rule out a coincidence, but I would not use 7.1%
   without reading the 2026 report.
6. Amodei's "90% in three to six months" quote is now sourced to the CFR transcript. Its next
   sentence helps this project: "But the programmer still needs to specify ... what are the
   conditions of what you're doing".

## Q1. Adoption and trust

**Stack Overflow 2025**, [AI section](https://survey.stackoverflow.co/2025/ai):
- CONFIRMED: 84% use or plan to use AI tools, up from 76%. 51% of professional developers
  use them daily (ADDED).
- CONFIRMED: more distrust (46%) than trust (33%); 3% highly trust; experienced developers
  2.6% highly trust and 20% highly distrust.
- CONFIRMED: positive sentiment fell from "70%+ in 2023 and 2024 to just 60%". ADDED:
  professionals 61%, learners 53%.
- CONFIRMED, was [TK]: the biggest frustration, "cited by 66% of developers", is "AI
  solutions that are almost right, but not quite". Second is "Debugging AI-generated code is
  more time-consuming" at 45%.
- CONFIRMED, was [TK]: 52% "either don't use agents or stick to simpler AI tools"; 38% have
  no plans to adopt agents (ADDED).
- CONFIRMED, was [TK], with a correction: vibe coding "No" is 72.2%, plus 5.3% "No,
  emphatically". The notes' ~72% counts only the plain "No".
- CONFIRMED: about 49,000 respondents. One chart shows 33,662 responses as 68.7% of the
  total, which gives ~49,000.
- CORRECTED: "only 29% trust AI outputs, down from 40%" (ShiftMag/ADTmag) is not on the
  primary page. See headline 4.
- ADDED: there is no 2026 survey yet. survey.stackoverflow.co still lists 2025 as the
  latest, and `/2026/ai` returns "Page not found".

**DORA**:
- CONFIRMED, DORA 2025,
  [Google Cloud announcement](https://cloud.google.com/blog/products/ai-machine-learning/announcing-the-2025-dora-report):
  "nearly 5,000 technology professionals", "over 100 hours of qualitative data", 90% use AI
  at work, more than 80% believe it raised productivity, 30% report "little or no trust"
  ("a slightly lower percentage than last year"). "AI doesn't fix a team; it amplifies
  what's already there."
- COULD NOT VERIFY: the +14-point year-on-year rise and the ~2 hours/day median. Neither is
  in the announcement or on the [dora.dev landing page](https://dora.dev/research/2025/dora-report/).
  The full report is a PDF behind a form, which I did not fill in.
- CORRECTED: throughput. See headline 3.
- CONFIRMED, was [TK], DORA 2024,
  [Google Cloud announcement](https://cloud.google.com/blog/products/devops-sre/announcing-the-2024-dora-report):
  AI adoption "was accompanied by an estimated decrease in delivery throughput by 1.5%, and
  an estimated reduction in delivery stability by 7.2%". ADDED from the same page: a 25%
  rise in adoption goes with +7.5% documentation quality, +3.4% code quality and +3.1% code
  review speed.

**Company claims about the share of AI-written code**:
- CONFIRMED, Google Oct 2024, primary:
  [Pichai's Q3 2024 earnings remarks](https://blog.google/company-news/inside-google/message-ceo/alphabet-earnings-q3-2024/):
  "more than a quarter of all new code at Google is generated by AI, then reviewed and
  accepted by engineers".
- CONFIRMED, Google Apr 2026,
  [Fast Company, 23 Apr 2026](https://www.fastcompany.com/91531519/google-ceo-says-75-of-the-companys-code-is-ai-generated),
  quoting a Pichai blog post: "75% of all new code at Google is now AI-generated and
  approved by engineers, up from 50% last fall", and "truly agentic workflows". I did not
  open the Google blog post itself.
- CONFIRMED, Nadella,
  [TechCrunch, 29 Apr 2025](https://techcrunch.com/2025/04/29/microsoft-ceo-says-up-to-30-of-the-companys-code-was-written-by-ai/):
  20% to 30% of code in Microsoft's repositories "written by software"; "more progress in
  Python and less in C++"; Kevin Scott expects 95% by 2030.
- COULD NOT VERIFY: the "30-40% acceptance rate, going up monotonically" line. It is in
  neither the TechCrunch nor the
  [CNBC](https://www.cnbc.com/2025/04/29/satya-nadella-says-as-much-as-30percent-of-microsoft-code-is-written-by-ai.html)
  article. It may be from the video. Leave it out, or cite the video after watching it.
- ADDED, Amodei, [CFR CEO Speaker Series transcript](https://www.cfr.org/event/ceo-speaker-series-dario-amodei-anthropic)
  (10 Mar 2025 per press coverage; the date was not on the extracted page): "I think we'll
  be there in three to six months - where AI is writing 90 percent of the code. And then in
  twelve months, we may be in a world where AI is writing essentially all of the code. But
  the programmer still needs to specify ... what is the overall app you're trying to make".
- Still missing: Altman and Cherny statements, Cursor and Codex usage numbers. Not searched.

## Q2. Measured quality problems

**METR**:
- CONFIRMED, [2025 RCT](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/):
  16 experienced developers, 246 issues, repos averaging 22k+ stars and 1M+ lines, tasks
  about two hours each, "primarily Cursor Pro with Claude 3.5/3.7 Sonnet". 19% longer with
  AI; developers expected a 24% speedup and afterwards believed in 20%. ADDED: CI +2% to
  +39%; developers were paid $150/hr.
- CORRECTED, [Feb 2026 update](https://metr.org/blog/2026-02-24-uplift-update/): 57
  developers, 143 repos, 800+ tasks, median 10 years of experience (all CONFIRMED). The
  returning-developer estimate is an 18% *reduction* in time, not a slowdown. See headline 1.
  ADDED: new recruits -4% (CI -15% to +9%); pay was cut from $150/hr to $50/hr, which METR
  lists as another selection effect; time measurements are unreliable for developers
  running several agents at once.
- CONFIRMED, [Mar 2026 note](https://metr.org/notes/2026-03-10-many-swe-bench-passing-prs-would-not-be-merged-into-main/)
  (Whitfill, Wu, Becker, Rush): 4 maintainers, 3 repos, 296 AI PRs; "roughly half" of
  test-passing PRs would not be merged; merge decisions average "about 24 percentage points
  lower" than the grader. ADDED: maintainer-judged progress is 9.6 pp/yr slower than
  grader-judged progress; agents were from mid-2024 to mid/late-2025.
- ADDED, [May 2026 survey](https://metr.org/blog/2026-05-11-ai-usage-survey/) (the notes had
  "details could not be retrieved"): 349 technical workers surveyed Feb-Apr 2026 (87
  software engineers). Median self-reported value change 1.4-2x, median self-reported speed
  change 3x. Retrospective 1.3x for March 2025, 2x for March 2026, forecast 2.5x for March
  2027. METR is openly sceptical of the magnitudes: its own staff give the lowest answers,
  and "survey results are not necessarily grounded in reality". Given METR's 2025 gap between
  perceived +20% and measured -19%, treat these as perception data.

**Vendor telemetry and reports**:
- CONFIRMED, Faros AI, [report page](https://www.faros.ai/blog/ai-software-engineering):
  "over 10,000 developers across 1,255 teams" (team count ADDED); +21% tasks, +98% merged
  PRs, +91% PR review time, +9% bugs per developer, +154% average PR size; Amdahl's Law
  framing. ADDED: high-adoption developers touch 47% more PRs per day.
- CONFIRMED, CodeRabbit, [report page](https://www.coderabbit.ai/blog/state-of-ai-vs-human-code-generation-report):
  470 PRs (320 AI-co-authored, 150 human-only); 10.83 vs 6.45 issues per PR (~1.7x); logic
  75% more common; readability "more than 3x"; error handling "nearly 2x"; security "up to
  2.74x"; excessive I/O ~8x; formatting 2.66x. ADDED: ~1.4-1.7x more critical and major
  findings; concurrency and dependency errors ~2x.
- CONFIRMED, Veracode, [blog, 30 Jul 2025](https://www.veracode.com/blog/genai-code-security-report/):
  "over 100 large language models across Java, Python, C#, and JavaScript"; 45% of samples
  failed security tests; Java 72%; Python 38%, JavaScript 43%, C# 45% (was [TK]); XSS
  (CWE-80) failed in 86% of relevant samples (was [TK]). Flat across generations: "they
  were no better at writing secure code. Security performance remained flat, regardless of
  model size or training sophistication."
- COULD NOT VERIFY: the 80-task count, and log injection ~88%. Neither is in the blog; both
  are probably in the PDF report, which I did not open.
- CONFIRMED, GitClear 2025, from the
  [PDF](https://gitclear-public.s3.us-west-2.amazonaws.com/GitClear-AI-Copilot-Code-Quality-2025.pdf)
  (the HTML page is behind a Cloudflare challenge): 211 million changed lines; "an 8-fold
  increase in the frequency of code blocks with 5+ duplicated lines during 2024"; moved
  lines 24.8% (2021) to 9.5% (2024); copy/paste 8.4% to 12.3%; churn 3.1% (2020) to 5.7%
  (2024).
- COULD NOT VERIFY, GitClear 2026 ("The Maintainability Gap"): the page is behind the same
  Cloudflare challenge. See headline 5 about the 7.1% figure.

**Security and supply chain**:
- CONFIRMED, Perry et al., [arXiv 2211.03622](https://arxiv.org/abs/2211.03622): codex-
  davinci-002 users "wrote significantly less secure code", "were more likely to believe
  they wrote secure code", and participants who "trusted the AI less and engaged more"
  wrote fewer vulnerabilities.
- CONFIRMED, Spracklen et al., [arXiv 2406.10279](https://arxiv.org/abs/2406.10279),
  comment "To appear in the 2025 USENIX Security Symposium": 16 LLMs, 576,000 samples in
  two languages, hallucinated packages "at least 5.2% for commercial models and 21.7% for
  open-source models", 205,474 unique hallucinated names (was [TK]).
- COULD NOT VERIFY: the 19.7% overall rate. It is not in the abstract; I did not open the
  PDF.

**Benchmarks**:
- CONFIRMED, OpenAI, [Why SWE-bench Verified no longer measures frontier coding capabilities](https://openai.com/index/why-we-no-longer-evaluate-swe-bench-verified/):
  audited a 27.6% subset (138 problems o3 did not consistently solve over 64 runs) and
  found "at least 59.4%" with flawed tests or descriptions; all frontier models tested could
  reproduce gold patches or problem specifics verbatim; "improvements on SWE-bench Verified
  no longer reflect meaningful improvements". ADDED: 35.5% of audited tasks have overly
  strict tests. The model names in the notes (GPT-5.2, Gemini 3 Flash) were not re-checked;
  Claude Opus 4.5 appears in the page's own example.

## Q3. Verified code generation

- CORRECTED, VERINA, [arXiv 2505.23135](https://arxiv.org/abs/2505.23135) (v3 16 Mar
  2026): 189 curated Lean tasks. Best model o3: 72.6% code, 52.3% spec, 4.9% proof, one try
  each. See headline 2.
- CONFIRMED, CLEVER, [arXiv 2505.13938](https://arxiv.org/abs/2505.13938): 161 Lean problems;
  each needs a spec matching a held-out ground truth, then a provably correct
  implementation. It avoids test-case supervision and specs "that leak implementation
  logic or allow vacuous solutions". All methods tried "struggle to achieve full
  verification". No headline number in the abstract.
- CONFIRMED, AutoVerus, [arXiv 2409.13082](https://arxiv.org/abs/2409.13082), comment
  "OOPSLA 2025": 150 proof tasks; correct proofs for "more than 90%", more than half within
  30 seconds or 3 LLM calls.
- CONFIRMED, AlphaVerus, [arXiv 2412.06176](https://arxiv.org/abs/2412.06176): translation
  from a higher-resource language, Treefinement, filtering misaligned specs to prevent
  reward hacking; LLaMA-3.1-70B produces verified code "without human intervention or model
  finetuning". The ICML 2025 venue was not in the arXiv comment field; I did not reopen the
  ICML page.
- CONFIRMED, type-constrained decoding, [arXiv 2504.09246](https://arxiv.org/abs/2504.09246):
  prefix automata plus a search over inhabitable types, formalized on a simply typed core
  and extended to TypeScript; "reduces compilation errors by more than half" on HumanEval and
  MBPP. The PLDI 2025 venue comes from the ACM DOI in the notes, which dl.acm.org's bot
  wall stopped me from opening.
- CONFIRMED, KernelBench, [arXiv 2502.10517](https://arxiv.org/abs/2502.10517): 250 PyTorch
  workloads; new metric fast_p; frontier reasoning models match the PyTorch baseline "in
  less than 20% of the cases" out of the box; execution and profiling feedback helps.

## Not checked in this round

- The vericoding per-model split (in gap_fill_conferences.md, abstract only).
- FVAPPS, VerusBench numbers beyond AutoVerus's own, the grammar-constrained decoding
  libraries (XGrammar, SynCode, Outlines).
- The 2026 constrained-decoding preprints (2607.13921, 2605.30054, 2607.18254).
- The Apiiro "4x faster, 10x more findings" figure. Weakly sourced; leave it out of the
  report.
