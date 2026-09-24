# Gap fill: AI-written code (quality, productivity, trust, verifiability): verification of `ai_code_generation.md`

> **Method note for the report writer.** Compiled 2026-09-24. This file checks the flagged claims in `ai_code_generation.md` (items tagged [TK], [VENDOR] or "conflict", plus its "Gaps" lists) and adds 2025–2026 findings that file missed.
>
> **How the sources were reached:**
> - The network proxy still blocked almost every primary host: survey.stackoverflow.co, stackoverflow.co, stackoverflow.blog, metr.org, metr.substack.com, veracode.com, coderabbit.ai, gitclear.com, faros.ai, usenix.org, arxiv.org, openai.com, dora.dev and most news sites. The web-fetch tool returned `EGRESS_BLOCKED` for survey.stackoverflow.co.
> - The hosts that worked, and were read directly, were cloud.google.com (DORA posts), www.anthropic.com and claude.com (Anthropic research and engineering posts), www.microsoft.com (Microsoft Research publication pages, including its public search API), gitclear-public.s3.us-west-2.amazonaws.com (GitClear's own 2025 PDF) and pypi.org.
> - The session's shared web-search budget (200 calls across all researchers) ran out after 31 searches in this task. So some flagged items could not be searched at all. They are marked COULD NOT VERIFY, and the entry says so.
>
> **Status key:**
> - **CONFIRMED** means the page was opened and read in this session.
> - **SNIPPET-CONFIRMED** means the page could not be opened, but search-result text agreed across two or more result pages or queries. Quoted wording is the search tool's summary of the listed result pages; the tool does not attribute each sentence to a single URL.
> - **CORRECTED** means the old value was wrong or mislabelled.
> - **COULD NOT VERIFY** means the claim could not be checked; the entry says what was tried.
>
> **Vendor labels:** [VENDOR: what they sell]. Microsoft Research items are labelled [VENDOR-lab: Microsoft sells GitHub Copilot], and Anthropic items [VENDOR: Anthropic sells Claude / Claude Code].

---

## 1. Stack Overflow 2025 Developer Survey (AI section), and whether a 2026 survey has been published

### Takeaway
Stack Overflow's own pages publish both trust figures: **33%** trust vs **46%** distrust on the AI results page, and **29%** "trust AI output to be accurate" in its press release and blog. The snippets do not show which question or respondent base produces 29%, so the conflict is narrowed but not resolved. The other headline figures (84% use or plan to use, 3% highly trust, 66% "almost right", 45% debugging, ~49,000 respondents, 52% not using agents, 72% no vibe coding) are snippet-confirmed. The 2026 survey opened on 23 June 2026. No 2026 results were found as of 24 Sep 2026.

### Cited Findings
**Ledger:**

- **"84% use or plan to use AI (76% in 2024)": SNIPPET-CONFIRMED.**
  - Search wording: "84% of respondents are using or planning to use AI tools in their development process, an increase over last year (76%)."
  - A second query returned: "84% of developers now use AI tools, with 51% of professionals using them daily."
  - Sources: [SO 2025 AI page](https://survey.stackoverflow.co/2025/ai); [SO blog, 29 Dec 2025](https://stackoverflow.blog/2025/12/29/developers-remain-willing-but-reluctant-to-use-ai-the-2025-developer-survey-results-are-here/); [ShiftMag](https://shiftmag.dev/stack-overflow-survey-2025-ai-5653/); [byteiota](https://byteiota.com/stack-overflow-dev-survey-2026-ai-at-84-trust-at-3/); [Cadence](https://cadence.withremote.ai/blog/stack-overflow-survey-2026)
- **"46% distrust vs 33% trust; 3% highly trust": SNIPPET-CONFIRMED** (the same wording came back in three separate queries).
  - Search wording: "More developers actively distrust the accuracy of AI tools (46%) than trust it (33%), and only a fraction (3%) report 'highly trusting' the output."
  - Also: "around 30% 'somewhat trust'", and "46% … don't trust the accuracy … a significant increase from 31% last year."
  - Sources: [SO 2025 AI page](https://survey.stackoverflow.co/2025/ai); [SO press release](https://stackoverflow.co/company/press/archive/stack-overflow-2025-developer-survey/); [SO "TL;DR for Leaders"](https://stackoverflow.co/internal/resources/2025-stack-overflow-developer-survey-for-leaders/ai-adoption/)
- **"Trust 33% vs the conflicting 29%": PARTLY RESOLVED; which respondent group gives 29% is COULD NOT VERIFY.**
  - Both numbers are Stack Overflow's own.
  - The 29% appears in SO's press copy and blog: "Trust in the accuracy of AI has fallen from 40% in previous years to just 29% this year". An exact-phrase search on this sentence returned the [SO blog, 29 Dec 2025](https://stackoverflow.blog/2025/12/29/developers-remain-willing-but-reluctant-to-use-ai-the-2025-developer-survey-results-are-here/) as the top hit, plus [Dataconomy](https://dataconomy.com/2025/08/05/developer-trust-in-ai-tools-is-falling-survey-finds/). A further snippet says "only 29% of respondents trust AI outputs to be accurate, a big dip from 40% in 2024." Also [ShiftMag](https://shiftmag.dev/stack-overflow-survey-2025-ai-5653/) and [ADTmag](https://adtmag.com/blogs/watersworks/2026/01/stack-overflow-survey.aspx).
  - The 33% (with 46% distrust) is the AI results page figure (entry above).
  - A search-tool summary claimed "Trust has dropped to 29% among professional developers", but it cited no page for that sentence, so it is not evidence.
  - A second conflict concerns the prior-year baseline. Some pages say 40% in 2024; others give the series "40% in 2023, 43% in 2024, then a drop to 29% in 2025" ([CODERCOPS](https://blog.codercops.com/blog/stack-overflow-2025-survey-ai-trust-gap); [IntelligentTools](https://intelligenttools.co/blog/stack-overflow-2025-developer-survey-ai-reality)).
  - What was tried: exact-phrase and "29% 33%" searches, and a web-fetch of survey.stackoverflow.co (EGRESS_BLOCKED).
- **"3% highly trust; experienced developers 2.6% highly trust / 20% highly distrust": SNIPPET-CONFIRMED** (repeated across three queries).
  - Search wording: "experienced developers are the most cautious, with the lowest 'highly trust' rate (2.6%) and the highest 'highly distrust' rate (20%)."
  - Also: "Professionals show a higher overall favorable sentiment (61%) than those learning to code (53%)."
  - Sources: [SO 2025 AI page](https://survey.stackoverflow.co/2025/ai); [SO press release](https://stackoverflow.co/company/press/archive/stack-overflow-2025-developer-survey/)
- **"Favorability 77% → 72% → 60%": SNIPPET-CONFIRMED (weak: one query, several result pages).**
  - Search wording: "Positive sentiment for AI tools fell to 60% in 2025, down from 72% in 2024 and 77% in 2023."
  - Sources: [SO blog, "Diving into the results", 1 Aug 2025](https://stackoverflow.blog/2025/08/01/diving-into-the-results-of-the-2025-developer-survey/); [LinearB](https://linearb.io/blog/stack-overflow-2025-developer-survey-autonomy-ai-trust); [CODERCOPS](https://blog.codercops.com/blog/stack-overflow-2025-survey-ai-trust-gap)
- **"'Almost right, but not quite' 66%; 'debugging AI-generated code is more time-consuming' 45%": SNIPPET-CONFIRMED, with a wording conflict.**
  - Search wording: "The biggest single frustration, cited by 66% of developers, is dealing with 'AI solutions that are almost right, but not quite,' which often leads to the second-biggest frustration: 'Debugging AI-generated code is more time-consuming' (45%)."
  - Sources: [SO press release](https://stackoverflow.co/company/press/archive/stack-overflow-2025-developer-survey/); [VentureBeat](https://venturebeat.com/ai/stack-overflow-data-reveals-the-hidden-productivity-tax-of-almost-right-ai-code); [SO 2025 AI page](https://survey.stackoverflow.co/2025/ai)
  - *Conflict:* two other summaries say "The number-one frustration, cited by 45% of respondents, is dealing with 'AI solutions that are almost right, but not quite'". This came back once alongside the [SO blog, 29 Dec 2025](https://stackoverflow.blog/2025/12/29/developers-remain-willing-but-reluctant-to-use-ai-the-2025-developer-survey-results-are-here/), and once in a query whose results included SO's AI page and press release.
  - Report 66% / 45% as the survey-page figures, and note that some SO press copy attaches 45% to "almost right".
- **"~49,000 respondents": SNIPPET-CONFIRMED.**
  - Search wording: "49,000+ responses from 177 countries across 62 questions focused on 314 different technologies."
  - Sources: [SO blog](https://stackoverflow.blog/2025/12/29/developers-remain-willing-but-reluctant-to-use-ai-the-2025-developer-survey-results-are-here/); [The Median, "What 49,000 Devs Think About AI"](https://dcthemedian.substack.com/p/what-49000-devs-think-about-ai-chatgpt)
- **"Agents ~52%; vibe coding ~72%": SNIPPET-CONFIRMED** (two queries).
  - Search wording: "A majority of developers (52%) either don't use agents or stick to simpler AI tools, and a significant portion (38%) have no plans to adopt them."
  - Search wording: "Nearly 72% said vibe coding is not part of their professional work, and an additional 5% emphatically do not participate in vibe coding."
  - Search wording: "approximately 70% of agent users agree that agents have reduced the time spent on specific development tasks, and 69% agree they have increased productivity."
  - Sources: [SO 2025 AI page](https://survey.stackoverflow.co/2025/ai); [SO blog](https://stackoverflow.blog/2025/12/29/developers-remain-willing-but-reluctant-to-use-ai-the-2025-developer-survey-results-are-here/); [Sumit M., Medium](https://medium.com/@sumit_m/5-takeaways-from-the-2025-stack-overflow-developer-survey-332afe1d8af3)
- **Daily use.** Search wording: "47.1% of all respondents use AI tools daily, with early-career developers … at 55.5%." — [byteiota](https://byteiota.com/stack-overflow-dev-survey-2026-ai-at-84-trust-at-3/). This is single-source; treat as unverified.
- **Publication date: CORRECTED (minor).**
  - One search summary said the survey was "released December 29, 2025". That is the date of a recap blog post.
  - Results were public by 1 Aug 2025, as shown by the [SO blog "Diving into the results", 1 Aug 2025](https://stackoverflow.blog/2025/08/01/diving-into-the-results-of-the-2025-developer-survey/) and [Visual Studio Magazine, 1 Aug 2025](https://visualstudiomagazine.com/articles/2025/08/01/stack-overflow-dev-survey-visual-studio-vs-code-hold-of-ai-ides-to-remain-on-top.aspx).
- **"Has a 2026 SO survey been published?": COULD NOT VERIFY any 2026 AI numbers.**
  - SO's blog post "The 2026 Developer Survey is now open (for human developers only)!" is dated 23 Jun 2026 — [SO blog](https://stackoverflow.blog/2026/06/23/the-2026-developer-survey-is-now-open-for-human-developers-only/).
  - No 2026 results page appeared in a domain-restricted search (stackoverflow.blog, stackoverflow.co, survey.stackoverflow.co).
  - Pages titled "Stack Overflow Dev Survey 2026" ([byteiota](https://byteiota.com/stack-overflow-dev-survey-2026-ai-at-84-trust-at-3/), [Cadence](https://cadence.withremote.ai/blog/stack-overflow-survey-2026)) report the **2025** figures. Do not cite them as 2026 data.
  - SO published follow-up essays on the trust gap in 2026, seen by title only: [Feb 2026](https://stackoverflow.blog/2026/02/18/closing-the-developer-ai-trust-gap/), [Apr 2026](https://stackoverflow.blog/2026/04/02/what-the-ai-trust-gap-means-for-enterprise-saas/), [Jul 2026](https://stackoverflow.blog/2026/07/29/developers-are-attached-to-tools-because-tools-encode-trust/).

### Inferences
- The safest citation is "46% distrust vs 33% trust; 3% highly trust (SO 2025, AI section)". If the 29% is used, attribute it to SO's press materials and say the base is not specified.
- Either way the direction is unambiguous: distrust rose from 31% to 46% in one year while use rose to 84%.
- The 2025 survey measures sentiment, not defect rates. Pair it with the measured studies in sections 3–5.

### Gaps
- The exact question wording and respondent base for the 29% could not be checked, because the survey site is blocked.
- No 2026 Stack Overflow results were found. If they appear later, they will be the first large survey of the agent era.
- [Stack Overflow is VENDOR-adjacent: it sells Stack Overflow for Teams / Internal and licenses its Q&A data to AI companies.]

---

## 2. DORA 2024, DORA 2025, and DORA's 2026 ROI report

### Takeaway
All flagged DORA items are now confirmed from Google Cloud's own blog, except "~2 hours/day", which is snippet-level. DORA 2024 is confirmed: each 25% increase in AI adoption was associated with an estimated −1.5% delivery throughput and −7.2% stability, and 39% had little or no trust. DORA 2025 is confirmed: 90% use AI, more than 80% perceive higher productivity, 30% have little or no trust, the "amplifier" framing, and AI adoption now correlates *positively* with throughput but still *negatively* with stability. In April 2026 DORA added an "ROI of AI-assisted Software Development" report built around a productivity "J-curve" and a "verification tax". **[VENDOR: Google sells Gemini Code Assist and Google Cloud]**

### Cited Findings
**Ledger:**

- **"DORA 2024: 25-point AI adoption increase → −1.5% throughput, −7.2% stability": CONFIRMED** (wording corrected from "25-point" to "25%").
  - Google's post says: "A 25% increase in AI adoption is associated with improvements in several key areas: 7.5% increase in documentation quality, 3.4% increase in code quality, 3.1% increase in code review speed."
  - It also says: "As AI adoption increased, it was accompanied by an estimated decrease in delivery throughput by 1.5%, and an estimated reduction in delivery stability by 7.2%."
  - Other figures in the post:
    - "More than 75 percent of respondents said that they rely on AI for at least one daily professional responsibility."
    - "More than one-third … experienced 'moderate' to 'extreme' productivity increases."
    - "39% of the respondents reported little to no trust in AI-generated code."
  - Source: [Google Cloud blog, "Announcing the 2024 DORA report", 22 Oct 2024](https://cloud.google.com/blog/products/devops-sre/announcing-the-2024-dora-report)
- **"DORA 2025: 90% use AI; 30% little or no trust; amplifier": CONFIRMED.**
  - The post covers "over 100 hours of qualitative data and survey responses from nearly 5,000 technology professionals."
  - "AI doesn't fix a team; it amplifies what's already there."
  - "90% of survey respondents report using AI at work. More than 80% believe it has increased their productivity. However … 30% report little or no trust in the code generated by AI, a slightly lower percentage than last year."
  - New in 2025, and missing from the old notes: "Unlike last year, we observe a positive relationship between AI adoption on both software delivery throughput and product performance … However, AI adoption does continue to have a negative relationship with software delivery stability."
  - It adds: "Without robust control systems, like strong automated testing, mature version control practices, and fast feedback loops, an increase in change volume leads to instability."
  - Source: [Google Cloud blog, "Announcing the 2025 DORA Report", 23 Sep 2025](https://cloud.google.com/blog/products/ai-machine-learning/announcing-the-2025-dora-report)
  - The report landing page (read) is a gated download form with a summary only. It confirms the "DORA AI Capabilities Model" (seven practices) and "AI adoption is a systems problem, not a tools problem" — [Google Cloud, 2025 DORA report page](https://cloud.google.com/resources/content/2025-dora-ai-assisted-software-development-report)
- **"~2 hours/day median": SNIPPET-CONFIRMED (weak: one query, four result pages).**
  - Search wording: "Developers spend a median of two hours per day working with AI tools."
  - Other figures in the same results:
    - "90% … up 14% from 2024"
    - "nearly two-thirds rely on AI for at least half their workflow, and one in twelve say their development work is now almost entirely AI-mediated"
    - "nearly 60% say code quality is up"
    - "Only 25% … trust AI outputs 'a lot' or 'a great deal,' while 30% admit to 'a little' or 'not at all.'"
  - Sources: [ADTmag](https://adtmag.com/articles/2025/09/24/what-2025-dora-report-means-for-developers.aspx); [TechRepublic](https://www.techrepublic.com/article/news-dora-ai-report-2025/); [blog.google](https://blog.google/innovation-and-ai/technology/developers-tools/dora-report-2025/); [ODSC](https://opendatascience.com/how-developers-are-using-ai-insights-from-the-2025-dora-report/)

**New (missed by old notes):**
- **DORA AI Capabilities Model (10 Dec 2025).** It names seven capabilities that "amplify the positive impact of AI":
  - clear AI stance
  - healthy data ecosystems
  - AI-accessible internal data
  - **strong version control practices**
  - **working in small batches** ("AI can easily generate massive blocks of code, which are hard to review and test")
  - user-centric focus
  - quality internal platforms

  Source: [Google Cloud blog, 10 Dec 2025](https://cloud.google.com/blog/products/ai-machine-learning/from-adoption-to-impact-putting-the-dora-ai-capabilities-model-to-work) (CONFIRMED)
- **DORA "ROI of AI-assisted Software Development" report (v.2026.1, 22 Apr 2026).**
  - The landing page (read) frames an initial "productivity dip" and a "tuition cost". It says "the greatest returns come from reducing unnecessary rework" — [Google Cloud](https://cloud.google.com/resources/content/dora-roi-of-ai-assisted-software-development) (CONFIRMED)
  - Search snippets (SNIPPET-CONFIRMED across InfoQ, Kodus and Zenn) describe:
    - a J-curve in which output "drops below pre-adoption levels" after adoption, owing to "learning costs, the verification tax, and pipeline adaptation";
    - a definition of the verification tax as "the additional effort required to check whether AI-generated code is reliable, secure, and aligned with the system architecture";
    - a cited LinearB analysis of **8.1M PRs** finding that "AI-generated code waits 4.6x longer for first review" [VENDOR: LinearB sells engineering analytics].

    Sources: [InfoQ, May 2026](https://www.infoq.com/news/2026/05/dora-roi-ai-assisted-dev-report/); [Kodus](https://kodus.io/en/dora-accelerate-state-of-devops/); [Zenn](https://zenn.dev/inspector/articles/dora-roi-of-ai-2026-yomitoki?locale=en); [dora.dev ROI page](https://dora.dev/ai/roi/report/)
  - A summary of the 2026 DORA material also cites Stanford research: "35 to 40% productivity gain on simple, greenfield tasks, [but] on complex legacy code is often 10% or less." This is secondary; the Stanford source itself was not seen — [InfoQ, Mar 2026](https://infoq.com/news/2026/03/ai-dora-report/); [Kodus](https://kodus.io/en/dora-accelerate-state-of-devops/)

### Inferences
- The 2024→2025 change matters. Throughput went from negative to positive, while stability stayed negative. That matches the review-bottleneck picture: teams now ship more, but change volume outruns their safety nets.
- DORA's own remedies are process remedies (small batches, version control, tests, platforms). None is a language feature. A language pitch has to show it lowers the "verification tax" that DORA now names explicitly.

### Gaps
- The full 2025 PDF is gated. The stability and throughput coefficients for 2025 were not found.
- No 2026 annual "State of AI-assisted Software Development" report was found (only the ROI report). Whether one is due in autumn 2026 is unknown.

---

## 3. METR: the 2025 RCT, the 2026 redesign, the SWE-bench merge note, the 2026 usage survey

### Takeaway
All METR items are snippet-confirmed; metr.org is blocked. The 2026 redesign post adds numbers the old notes lacked:
- **−4%** (CI −15% to +9%) for newly recruited developers
- the 18% slowdown for returning developers has CI −38% to +9%
- 30–50% of developers withheld tasks
- pay was cut from $150/hr to $50/hr

METR says these selection effects make the estimate a likely lower bound on speedup. The May 2026 survey found a median self-reported **2x** value in March 2026. METR itself says this is not necessarily grounded in reality. METR is an independent non-profit.

### Cited Findings
**Ledger:**

- **"Jul 2025 RCT: 16 developers, 246 tasks, 19% slower, 24% forecast, 20% believed": SNIPPET-CONFIRMED.**
  - Search wording: "16 developers with moderate AI experience complete 246 tasks in mature projects on which they have an average of 5 years of prior experience."
  - "Allowing AI actually increased completion time by 19%."
  - "developers forecast that allowing AI will reduce completion time by 24%."
  - "After completing the study, developers estimate that allowing AI reduced completion time by 20%."
  - Tools: "primarily Cursor Pro with Claude 3.5/3.7 Sonnet."
  - Sources: [METR blog](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/); [METR paper PDF](https://metr.org/Early_2025_AI_Experienced_OS_Devs_Study-paper.pdf); [arXiv 2507.09089](https://arxiv.org/pdf/2507.09089); [METR Substack](https://metr.substack.com/p/2025-07-10-early-2025-ai-experienced-os-dev-study)
- **"24 Feb 2026: 57 developers, 800+ tasks, selection effects, 18% slowdown for returning developers": SNIPPET-CONFIRMED, with additions.**
  - Search wording: "57 developers, across 143 repos, and 800+ tasks"; "started in August 2025"; "10 developers from the original study, plus a new set of 47 developers."
  - Returning developers: "a speedup of -18% with a confidence interval between -38% and +9%."
  - New recruits: "-4%, with a confidence interval between -15% and +9%."
  - Withheld tasks: "30% to 50% of developers stated they were choosing not to submit some tasks because they did not want to do them without AI."
  - Pay: "lower pay rate (reduced from $150/hr to $50/hr)."
  - Interpretation: "these effects make it likely that the estimate reported is a lower-bound on the true productivity effects."
  - Sources: [METR blog](https://metr.org/blog/2026-02-24-uplift-update/); [METR Substack](https://metr.substack.com/p/2026-02-24-uplift-update); [Rob Bowley](https://blog.robbowley.net/2026/04/04/metrs-developer-productivity-research-2026-update/)
- **"10 Mar 2026: about half of SWE-bench-passing PRs would not be merged": SNIPPET-CONFIRMED.**
  - Search wording: "4 active maintainers from 3 SWE-bench Verified repositories review 296 AI-generated pull requests"; "maintainer merge decisions are about 24 percentage points lower than SWE-bench scores supplied by the automated grader"; "roughly half of test-passing SWE-bench Verified PRs written by recent AI agents would not be merged into main."
  - Sources: [METR note](https://metr.org/notes/2026-03-10-many-swe-bench-passing-prs-would-not-be-merged-into-main/); [METR Substack](https://metr.substack.com/p/2026-03-10-many-swe-bench-passing-prs-would-not-be-merged-into-main); [Tessl](https://tessl.io/blog/passing-tests-are-not-enough)
- **"May 2026 AI usage survey": SNIPPET-CONFIRMED (weak: one query, three result pages). This fills a gap in the old notes.**
  - Sample: "349 technical workers (87 software engineers, 71 researchers, 129 academics and PhD students, and 48 founders and managers) conducted in February–April 2026."
  - Result: "a median 1.4–2x self-reported change in value of work"; "1.3x value of work in March 2025, … 2x in March 2026, and forecast 2.5x for March 2027"; "Self-reported speed gains … a median 3x."
  - Caveats: "METR staff give the lowest change in value answers of any subgroup … survey results are not necessarily grounded in reality."
  - Sources: [METR blog](https://metr.org/blog/2026-05-11-ai-usage-survey/); [METR Substack](https://metr.substack.com/p/2026-05-11-ai-usage-survey); [byteiota](https://byteiota.com/metr-ai-productivity-survey-2026/)
- **"Later 2026 METR results": COULD NOT VERIFY any new uplift study.**
  - The METR items found for 2026 are [Time Horizon 1.1 (29 Jan 2026)](https://metr.org/blog/2026-1-29-time-horizon-1-1/) and a [Frontier Risk Report (19 May 2026)](https://metr.org/blog/2026-05-19-frontier-risk-report/). Neither is a code-quality or productivity trial.
  - One domain-restricted search was run; no redesigned-RCT results appeared.

**Counterpoint (background, 2023–2024 data; published Jun 2025):**
- Microsoft Research's pooled field experiments at Microsoft, Accenture and an anonymous Fortune 100 company: "across three experiments and 4,867 developers … a 26.08% increase (SE: 10.3%) in completed tasks"; "less experienced developers had higher adoption rates and greater productivity gains." These are autocomplete-era tools. — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/the-effects-of-generative-ai-on-high-skilled-work-evidence-from-three-field-experiments-with-software-developers/) (CONFIRMED) [VENDOR-lab: Microsoft sells GitHub Copilot]

### Inferences
- The perception gap is now documented three ways:
  - the RCT (felt +20%, measured −19%);
  - the redesign (developers refuse to work without AI, which is itself evidence of perceived value);
  - the self-report survey (2x value), which METR discounts.

  A "trust but verify" language pitch should lean on the measured rather than the perceived numbers.
- METR's merge-rate note is the cleanest independent evidence that *tests are a weak acceptance oracle* for agent code (see section 9).

### Gaps
- No completed, bias-corrected 2026 RCT from METR exists yet.
- The METR survey's per-role breakdown (software engineers only) was not retrieved.

---

## 4. Security of AI-written code: Veracode 2025 and 2026, and Perry et al.

### Takeaway
Veracode's headline numbers are snippet-confirmed. The **2026 report (28 Jul 2026)** shows the flat trend continued: an average **56% security pass rate** (about 44% fail) across more than 100 models and four testing snapshots, "virtually unchanged since last year". GPT-5.5 is best at **68%**. **Java** remains worst at a **30%** mean pass rate, and XSS passes only **15%** of the time. The exact 2025 per-language split (JavaScript 43%, C# 45%) is only partly verified. Perry et al.'s headline results are snippet-confirmed; n=47 could not be verified. **[VENDOR: Veracode sells application security testing]**

### Cited Findings
**Ledger:**

- **"Veracode 2025: 45% fail; Java 72%": SNIPPET-CONFIRMED.**
  - Search wording: "AI-generated code introduced risky security flaws in 45% of tests"; "Java was the riskiest language, with a 72% security failure rate across tasks."
  - Veracode's own LinkedIn headline: "New Report: 45% of GenAI Code Has Security Vulnerabilities."
  - Sources: [Veracode blog](https://www.veracode.com/blog/genai-code-security-report/); [BusinessWire, 30 Jul 2025](https://www.businesswire.com/news/home/20250730694951/en/AI-Generated-Code-Poses-Major-Security-Risks-in-Nearly-Half-of-All-Development-Tasks-Veracode-Research-Reveals); [Veracode on LinkedIn](https://www.linkedin.com/posts/veracode_2025-genai-code-security-report-activity-7356341303866990595-_0pO)
- **"XSS ~86%, log injection ~88%": SNIPPET-CONFIRMED** (two queries).
  - Search wording: "LLMs failing to secure code against cross-site scripting (CWE-80) and log injection (CWE-117) in 86 percent and 88 percent of cases, respectively."
  - Sources: [Veracode blog](https://www.veracode.com/blog/genai-code-security-report/); [eeNews Europe](https://www.eenewseurope.com/en/report-finds-ai-generated-code-poses-security-risks/)
- **"Per-language rates: Python ~38%, JavaScript ~43%, C# ~45%": partly SNIPPET-CONFIRMED; the JavaScript and C# values are COULD NOT VERIFY.**
  - Search wording: "Python, C#, and JavaScript presented significant risk, with failure rates between 38 percent and 45 percent … Python was around 38 percent in Veracode's October 2025 language breakdown."
  - Sources: [BusinessWire 2025](https://www.businesswire.com/news/home/20250730694951/en/AI-Generated-Code-Poses-Major-Security-Risks-in-Nearly-Half-of-All-Development-Tasks-Veracode-Research-Reveals); [Veracode Oct 2025 update](https://www.veracode.com/resources/analyst-reports/2025-genai-code-security-report/)
  - Which of JavaScript and C# sits at 43% and which at 45% was not seen.
- **"Flat across model generations": SNIPPET-CONFIRMED in its 2026 form.**
  - Search wording: "Across four testing snapshots and more than 100 models tracked since the program began, the average security pass rate sits at 56 percent — virtually unchanged since last year's report."
  - The press-release title: "LLMs Are Getting Smarter, But Not Safer: Veracode 2026 GenAI Code Security Report Finds AI-Generated Code Security Has Stalled at 56% Pass Rate."
  - Sources: [BusinessWire, 28 Jul 2026](https://www.businesswire.com/news/home/20260728207685/en/LLMs-Are-Getting-Smarter-But-Not-Safer-Veracode-2026-GenAI-Code-Security-Report-Finds-AI-Generated-Code-Security-Has-Stalled-at-56-Pass-Rate); [SD Times](https://sdtimes.com/agentic-security/veracode-finds-ai-generated-code-security-has-barely-improved-since-last-year/); [The Next Web](https://thenextweb.com/news/veracode-2026-genai-code-security-56-percent-pass-rate)
  - One summary said the rate "hasn't moved in four years". That seems to garble "four testing snapshots", so do not use "four years".
- **"2026 update": SNIPPET-CONFIRMED (new).** The figures:
  - "Veracode tested 11 new models across 80 tasks"
  - "GPT-5.5 leads the Summer 2026 dataset at a 68% security pass rate … the best model available still fails on nearly 1 in 3 security tasks"
  - "Six of eleven models sit between 50–53%"
  - "Models purpose-built for code average a 51% security pass rate"
  - "Java has shown the clearest improvement trend of any language. It's also last by a wide margin, with a mean security pass rate of only 30%"
  - "Generated code passed XSS checks only 15% of the time"
  - "models produce code that compiles about 100% of the time. On security, they fail nearly 44% of the time"

  Sources: [Veracode 2026 report page](https://www.veracode.com/resources/analyst-reports/2026-genai-code-security-report/); [Veracode 2026 blog](https://www.veracode.com/blog/2026-genai-code-security-report-ai-risk/); [BusinessWire, 28 Jul 2026](https://www.businesswire.com/news/home/20260728207685/en/LLMs-Are-Getting-Smarter-But-Not-Safer-Veracode-2026-GenAI-Code-Security-Report-Finds-AI-Generated-Code-Security-Has-Stalled-at-56-Pass-Rate); [TechJuice](https://www.techjuice.pk/genai-code-fails-44-of-security-tests-despite-perfect-syntax/)

  A spring 2026 interim post also exists: "Despite Claims, AI Models Are Still Failing Security" — [Veracode](https://www.veracode.com/blog/spring-2026-genai-code-security/)
- **"2.74x more vulnerabilities than human code" attributed to Veracode: still a misattribution (the old notes' warning stands).**
  - A search summary attached "AI-generated code has 2.74x more vulnerabilities than code written by humans" to Veracode's report, and [SoftwareSeni](https://www.softwareseni.com/ai-generated-code-security-risks-why-vulnerabilities-increase-2-74x-and-how-to-prevent-them/) repeats it.
  - The 2.74x figure is CodeRabbit's security multiplier (section 5).
- **Perry et al., CCS 2023, headline results: SNIPPET-CONFIRMED** (background, 2022 study).
  - Search wording: "participants who had access to an AI assistant based on OpenAI's codex-davinci-002 model wrote significantly less secure code than those without access"; they were "more likely to believe they wrote secure code"; and those "who trusted the AI less and engaged more with the language and format of their prompts (e.g. re-phrasing, adjusting temperature) provided code with fewer security vulnerabilities."
  - Sources: [arXiv 2211.03622](https://arxiv.org/abs/2211.03622); [ACM DL](https://dl.acm.org/doi/10.1145/3576915.3623157); [NSF PAR](https://par.nsf.gov/biblio/10472235); [DX summary](https://getdx.com/research/do-developers-write-insecure-code-with-ai-assistants/)
- **Perry et al. "n = 47": COULD NOT VERIFY.** The targeted query ("47 participants") was refused because the search budget was exhausted, and arXiv and ACM are blocked.

### Inferences
- Security failure is flat across four snapshots and over 100 models, while functional compile rates are about 100%. This is the strongest evidence that model scaling alone does not fix AI-code security.
- The dominant failures (XSS 85–86%, log injection 88%) are **information-flow (taint) bugs**. Java, a statically typed, memory-safe language, is the *worst* performer. So conventional static typing does not prevent these flaws. Only type systems that track untrusted data (taint, capability or effect types) or safe-by-default APIs would address them. This is an inference from Veracode's data, not something Veracode tested.

### Gaps
- Veracode's per-CWE and per-language tables for 2026 (beyond Java and XSS) were not retrieved.
- No independent (non-vendor) replication of Veracode's 80-task benchmark was found.
- Perry et al.'s participant count remains unverified.

---

## 5. PR quality, maintainability and throughput telemetry: CodeRabbit, GitClear, Faros

### Takeaway
GitClear 2025 is now **CONFIRMED from GitClear's own PDF**, and one old-note figure is **CORRECTED**. The "7.1% churn in 2025" is GitClear's *projected 2024* churn; the measured 2024 value was 5.7%. GitClear's June 2026 report uses a different, larger churn measure (about 16%→19%). CodeRabbit's and Faros's 2025 headline numbers are snippet-confirmed. Faros's 2026 report ("Acceleration Whiplash", 22,000 developers) says volume is up and quality down, with the gap widening. All three are vendors.

### Cited Findings
**Ledger:**

- **CodeRabbit (Dec 2025) headline: SNIPPET-CONFIRMED** (one query, several result pages including CodeRabbit's own).
  - Search wording: "470 real-world open source pull requests"; "AI-generated PRs contain ~1.7x more issues"; "Logic and correctness issues rise 75%"; "Security vulnerabilities are up to 2.74x higher"; "Code readability problems increase more than 3x"; "Performance inefficiencies, such as excessive I/O, appear nearly 8x more often"; "released … December 17, 2025."
  - Sources: [CodeRabbit blog](https://www.coderabbit.ai/blog/state-of-ai-vs-human-code-generation-report); [CodeRabbit newsroom](https://www.coderabbit.ai/newsroom/state-of-ai-vs-human-code-generation-report); [BusinessWire](https://www.businesswire.com/news/home/20251217666881/en/CodeRabbits-State-of-AI-vs-Human-Code-Generation-Report-Finds-That-AI-Written-Code-Produces-1.7x-More-Issues-Than-Human-Code); [Cybernews](https://cybernews.com/ai-news/humans-code-better-than-ai-coderabbit/)
  - [VENDOR: CodeRabbit sells AI code review]
- **CodeRabbit sub-figures (10.83 vs 6.45 issues per PR; 320/150 split; formatting 2.66x; error handling ~2x): COULD NOT VERIFY.** They did not appear in this session's snippets, and coderabbit.ai and its PDF host are blocked.
- **GitClear 2025 (duplication ~8x; moved 24.8%→9.5%; churn 3.1%→5.7%): CONFIRMED** from [GitClear's PDF, "AI Copilot Code Quality", v2025.2.5, Feb 2025](https://gitclear-public.s3.us-west-2.amazonaws.com/GitClear-AI-Copilot-Code-Quality-2025.pdf).
  - Scope: "211 million changed lines of code, authored between January 2020 and December 2024." The data is "split about two-thirds private corporations", plus popular open-source repos.
  - Quote: "we recorded an 8-fold increase in the frequency of code blocks with 5+ duplicated lines during 2024."
  - Quote: "2024 marked the first year GitClear has ever measured where the number of 'Copy/Pasted' lines exceeded the count of 'Moved' lines."
  - Share of changed lines by year:

    | Year | Moved | Copy/pasted | Churn |
    |---|---|---|---|
    | 2020 | 24.1% | 8.3% | 3.1% |
    | 2021 | 24.8% | 8.4% | 3.3% |
    | 2022 | 20.5% | 9.4% | 3.3% |
    | 2023 | 15.8% | 10.6% | 4.5% |
    | 2024 projected | 13.4% | 11.6% | **7.1%** |
    | 2024 actual | **9.5%** (−39.9% YoY) | **12.3%** | **5.7%** (+26% YoY) |

  - Commits containing a duplicate block: 0.70% (2020), 0.48% (2021), 0.45% (2022), 1.80% (2023), **6.66% (2024)**. The report says prevalence "was observed to be approximately 10x higher than it had been two years prior."
  - The page title as indexed by search is "AI Copilot Code Quality: 2025 Data Suggests 4x Growth in Code Clones" — [GitClear](https://www.gitclear.com/ai_assistant_code_quality_2025_research). "4x", "8-fold" and "~10x" describe different duplication measures.
  - Caveat from the report's method: this is correlational. AI authorship is not observed per line.
  - [VENDOR: GitClear sells developer-analytics software]
- **"GitClear 2026: churn 7.1% in 2025": CORRECTED.**
  - Old value: churn 7.1% in 2025, versus a pre-AI baseline of ~3.3%.
  - The only source is a secondary summary: "Code churn rose from a pre-AI baseline of approximately 3.3% to 5.7% in 2024 and 7.1% in 2025" — [Larridin](https://larridin.com/developer-productivity-hub/code-churn-ai-era-doubled) [VENDOR: developer-productivity analytics].
  - GitClear's own 2025 PDF (table above) shows **7.1% was its *projection* for 2024**. The measured 2024 value was 5.7%, and 3.3% is the 2021–2022 value.
  - GitClear's June 2026 report measures "two-week churn" differently. Search wording: "rose from about 16% in 2023–2024 to about 19% in 2025 and held just under 19% in 2026". This is single-query snippet evidence — [GitClear 2026](https://www.gitclear.com/the_ai_code_quality_maintainability_gap).
  - New value: do not cite "7.1% in 2025". Cite 3.1% (2020) → 5.7% (2024) from the 2025 PDF, and, if needed, "two-week churn up ~15% (≈16%→19%) 2023→2025" from the 2026 report, labelled snippet-level.
- **GitClear 2026 "The Maintainability Gap" (June 2026): SNIPPET-CONFIRMED** (two queries; GitClear page plus a secondary write-up).
  - Scope: "623 million code changes from 2023-2026."
  - "Within-commit copy/paste rose 41%, code block duplication rose 81%, error-masking constructs rose 47%, and two-week code churn rose 15%."
  - "Cross-file function calls … down 35%, refactoring line moves are down 70%, and long-term legacy maintenance is down 74% vs 2022 levels."
  - "Duplications increased from 40.3 per million changed lines in 2023 to 73.0 per million in 2026."
  - "copy-pasted code rose from 9.4% of new code in 2022 to 15.7% in the first half of 2026, while properly refactored code fell from 21% to just 3.8%."
  - "Heavy AI users out-produce non-users by 4–10x, but compared to their past selves … a more modest 25% velocity gain."
  - Sources: [GitClear 2026](https://www.gitclear.com/the_ai_code_quality_maintainability_gap); [Artur Markus summary](https://www.arturmarkus.com/ai-code-quality-by-the-numbers-623-million-changes-refactoring-down-70-duplication-up-81/)
  - [VENDOR]
- **Faros AI (Jul 2025): SNIPPET-CONFIRMED.**
  - Search wording: "complete 21% more tasks and merge 98% more pull requests, but PR review time increases 91%"; "PR sizes had grown 154% … bug rates crept up 9%"; "telemetry from over 10,000 developers across 1,255 teams"; "The DORA delivery metrics … remained flat."
  - Sources: [Faros report](https://www.faros.ai/blog/ai-software-engineering); [Faros paradox page](https://www.faros.ai/ai-productivity-paradox); [AgentMarketCap](https://agentmarketcap.ai/blog/2026/04/07/faros-ai-coding-agent-metrics-enterprise-teams)
  - [VENDOR: Faros sells an engineering-intelligence platform]
- **Faros 2026 "AI Engineering Report 2026: The Acceleration Whiplash" (Apr 2026): SNIPPET-CONFIRMED (weak: one query).** This is new.
  - Scope: "two years of telemetry data from 22,000 developers and more than 4,000 teams."
  - "Epics completed per developer are up 66%, task throughput per developer is up 33.7%, and PR merge rate per developer is up 16.2%."
  - "Volume is up, quality is down, and the gap between the two is widening as adoption deepens … more bugs, more incidents, and longer review cycles."
  - "60% of AI-generated code now being accepted into codebases."
  - Sources: [Faros takeaways](https://www.faros.ai/blog/ai-acceleration-whiplash-takeaways); [Faros research page](https://www.faros.ai/research/ai-acceleration-whiplash); [ADTmag, 22 Apr 2026, "More Code, More Bugs"](https://adtmag.com/articles/2026/04/22/more-code-more-bugs.aspx)
  - The exact bug and incident percentages were not retrieved.

### Inferences
- Three independent vendor datasets agree in direction: more output, larger PRs, more duplication, less refactoring, and longer review. GitClear 2026 adds a new signal relevant to language design: **"error-masking constructs rose 47%"**, meaning code that swallows errors. A language that makes error handling explicit and non-ignorable (Result types, checked effects) addresses that signal directly.
- GitClear's "heavy AI users vs their past selves: +25%" is a useful correction to vendor "4–10x" framing.

### Gaps
- None of these datasets observes AI authorship line by line. All are correlational or rely on heuristic PR classification.
- The Faros 2026 bug and incident deltas, and CodeRabbit's full per-category table, were not retrieved.

---

## 6. Package hallucination (slopsquatting)

### Takeaway
None of the flagged package-hallucination numbers could be checked this session. The search budget ran out on the first query, and usenix.org and arxiv.org are blocked. The old notes' citations stand as they were, unverified.

### Cited Findings
- **Spracklen et al., USENIX Security 2025 (576K vs 2.23M samples; 19.7% overall; 5.2% commercial vs 21.7% open-source; 205,474 unique names; 43% repeat on every run): COULD NOT VERIFY.**
  - Tried: one combined search ("We Have a Package for You … 576,000 … 205,474 43%"), which was refused because the budget was exhausted, and a direct probe of the USENIX PDF (`usenix.org`, blocked).
  - The old notes' URLs: [USENIX paper page](https://www.usenix.org/conference/usenixsecurity25/presentation/spracklen)
- **"The Range Shrinks, the Threat Remains" (arXiv 2605.17062): COULD NOT VERIFY.** The exact-title search was refused (budget exhausted), and arxiv.org is blocked.

### Inferences
- None beyond the old notes. The supply-chain angle should be presented with the old notes' caveats until the numbers are checked.

### Gaps
- All figures in this section still need a check. The `gap_fill_industry.md` researcher was also asked to check the slopsquatting sample size, so cross-check that file.

---

## 7. Verification benchmarks and tools for AI-written code

### Takeaway
Most flagged benchmark figures could not be re-checked: VERINA, AlphaVerus, the vericoding percentages, type-constrained decoding, KernelBench, DafnyPro and the "94%" study. AutoVerus (>90% of 150 tasks) is **CONFIRMED**. Microsoft Research's pages, which were readable, show a large 2025–2026 body of verified-code work that the old notes missed:
- VeruSAGE: over 80% of 849 real-system Verus proof tasks
- VeriStruct: 128/129 functions verified (99.2%)
- VeruSyn: 6.9M synthesized verified Rust programs
- SAFE: 70.5% vs GPT-4o's 24.5%
- VeriSpecGen: 86.6% on VERINA's spec-generation task
- "Intent Formalization" (Mar 2026), which names *validating specifications* as the central bottleneck

Anthropic's Lean formalization of Fermat's Last Theorem (Sep 2026: 13M lines, 11 days) shows machine-checked output at very large scale, in mathematics.

### Cited Findings
**Ledger:**

- **VERINA (189 tasks; o4-mini ~61.4% code, ~51% specs, ~3.6% proofs): COULD NOT VERIFY.** No search budget remained, and arXiv is blocked.
  - Related and CONFIRMED: Microsoft's **VeriSpecGen** "achieve[s] 86.6% on VERINA SpecGen task using Claude Opus 4.5, improving over baselines by up to 31.8 points". Training on 343K refinement trajectories "substantially improves specification synthesis by 62-106% relative" — [Microsoft Research, Apr 2026](https://www.microsoft.com/en-us/research/publication/intent-aligned-formal-specification-synthesis-via-traceable-refinement/) [VENDOR-lab]
  - So the old "~51% specs" figure is at least out of date as a state-of-the-art number.
- **CLEVER: partly CONFIRMED.**
  - The benchmark's PyPI package (`clever-bench` 1.6.0) cites "arXiv 2505.13938" and "39th Conference on Neural Information Processing Systems (NeurIPS 2025)", and describes tasks where one submits a Lean "implementation" and "correctness_proof" — [PyPI JSON](https://pypi.org/pypi/clever-bench/json)
  - "161 problems, low solve rates": COULD NOT VERIFY.
- **AutoVerus (>90%): CONFIRMED.**
  - Quote: "we have built a benchmark suite of 150 non-trivial proof tasks … AutoVerus can automatically generate correct proof for more than 90% of them, with more than half of them tackled in less than 30 seconds or 3 LLM calls."
  - Venue: OOPSLA — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/autoverus-automated-proof-generation-for-rust-code/) [VENDOR-lab]
- **AlphaVerus (38% HumanEval-Verified): COULD NOT VERIFY.**
- **Vericoding (Dafny 82.2%, Verus 44.2%, Lean 26.8%): COULD NOT VERIFY this session.** arXiv and OpenReview are blocked, and no search budget remained.
- **Type-constrained decoding (compile errors cut by more than half; +3.5–5.5%): COULD NOT VERIFY this session.** ACM, the SIGPLAN site and ETH are blocked.
- **KernelBench headline fast_p numbers: COULD NOT VERIFY.**
- **DafnyPro 86%: COULD NOT VERIFY.**
- **"94% of LLM compilation errors are type-check failures" (study cited by GitHub): COULD NOT VERIFY which paper.** github.blog and arXiv are blocked, and no search budget remained. The old notes' guess (Mündler et al., PLDI 2025) remains an unconfirmed inference.

**New 2025–2026 findings (all CONFIRMED on the pages cited):**
- **VeruSAGE (Dec 2025).**
  - "VeruSAGE-Bench … consists of 849 proof tasks extracted from eight open-source Verus-verified Rust systems."
  - "The best LLM-agent combination … completes over 80% of system-verification tasks."
  - It "also completes over 90% of a set of system proof tasks … that had not yet been finished by human experts."
  - Models: o4-mini, GPT-5, Sonnet 4 and Sonnet 4.5.
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/verusage-a-study-of-agent-based-verification-for-rust-systems/) [VENDOR-lab]
- **VeriStruct (TACAS, Apr 2026).**
  - It extends AI-assisted verification to data-structure modules in Verus: "succeeds on ten of the eleven, successfully verifying 128 out of 129 functions (99.2%)."
  - It notes "LLMs often misunderstand Verus' annotation syntax and verification-specific semantics", which calls for "syntax guidance" and a repair stage.
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/veristruct/)
- **VeruSyn (Feb 2026).** "we synthesize the largest set of Verus verified programs: 6.9 million Rust programs, each with a formal specification and a proof." A fine-tuned Qwen2.5-Coder-32B gets an "appealing cost-proof tradeoff compared with … Claude Sonnet 4.5." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/reducing-the-costs-of-proof-synthesis-on-rust-systems-by-scaling-up-a-seed-training-set/)
- **SAFE (ICLR 2025).** "achieving a 70.50% accuracy rate in a benchmark crafted by human experts, a significant leap over GPT-4o's performance of 24.46%." The main obstacle it names is "the severe lack of data – there is much less proof than code." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/automated-proof-generation-for-rust-code-via-self-evolution/)
- **ExVerus (ICML 2026).** Counterexample-guided proof repair "significantly improves proof accuracy, robustness, and token efficiency." No figure is given in the abstract. — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/exverus-verus-proof-repair-via-counterexample-reasoning/)
- **F\* proof-oriented programming dataset (ICSE 2025).**
  - Size: "600K lines of open-source F* programs and proofs … around 32K top-level F* definitions", extended to 940K lines and 54K definitions.
  - Findings: "fine-tuned smaller language models (such as Phi-2 or StarCoder) compare favorably with large language models (such as GPT-4)" and "type-based retrieval augmentation techniques … boost performance significantly."
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/towards-neural-synthesis-for-smt-assisted-proof-oriented-programming/)
- **Intent Formalization grand challenge (Mar 2026).**
  - "AI-generated code amplifies [the intent gap] to an unprecedented scale."
  - "intent formalization — the translation of informal user intent into a set of checkable formal specifications — is the key challenge that will determine whether AI makes software more reliable or merely more abundant."
  - "The central bottleneck is validating specifications: since there is no oracle for specification correctness other than the user."
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/)
- **nl2postcond (FSE 2024).** "nl2postcond generated postconditions were able to catch 64 real-world historical bugs from Defects4J." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/formalizing-natural-language-intent-into-program-specifications-via-large-language-models/)
- **Symbolic testing of Dafny specs (2024).** It "advocate[s] an alternate approach of symbolically testing specifications" and validates the method on "roughly 150 Dafny specifications for … MBPP." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/evaluating-llm-driven-user-intent-formalization-for-verification-aware-languages/)
- **Algorithmist (Mar 2026).**
  - An agent on GitHub Copilot produced "provably sound and empirically effective algorithms" and "uncovered a subtle proof bug in prior published work."
  - It points to "a proof-first code-synthesis paradigm, in which code is developed alongside a structured natural-language proof intermediate representation."
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/early-discoveries-of-algorithmist-i-promise-of-provable-algorithm-synthesis-at-scale/)
- **Anthropic, "Formalizing Fermat's Last Theorem" (4 Sep 2026).**
  - "In 11 days, working largely autonomously, Claude produced the first end-to-end, computer-checked proof of FLT … it wrote 13 million lines of Lean and proved 29,500 intermediate theorems."
  - "over 5x the size of Mathlib"; about "six billion output tokens."
  - The proof "uses just Lean's three standard axioms", and the team "confirmed that the theorem's statement matches Mathlib's own statement of FLT."
  - Source: [Anthropic](https://www.anthropic.com/research/formalizing-fermats-last-theorem) [VENDOR]

### Inferences
- In the Verus (Rust) line of work, proof automation moved from "promising" to over 80–99% on real system code in about 18 months. Each improvement came from **verifier feedback** (repair loops, counterexamples), **synthetic data** (6.9M programs), or **type- and syntax-aware guidance**. Scale alone did not produce it.
- The recurring bottleneck named by the builders themselves is **specification correctness and intent**:
  - "validating specifications" (Intent Formalization);
  - LLMs "misunderstand … annotation syntax";
  - the FLT team manually checked that the *statement* matched Mathlib.

  A language that makes specifications short, readable and testable (symbolic or example-based) addresses the named bottleneck. This supports the old notes' inference with fresh 2026 evidence.
- The FLT result is in mathematics, not software. It shows that machine-checked output at multi-million-line scale is feasible when the statement is trusted. It does not show that the same holds for programs with informal requirements.

### Gaps
- VERINA, AlphaVerus, vericoding, type-constrained decoding, KernelBench, DafnyPro and the 94% study still need a primary check. arXiv, ACM and OpenReview were unreachable, and no search budget remained.
- Most of the new evidence is from Microsoft Research (one lab, which sells Copilot). Independent replications on these benchmarks were not searched.

---

## 8. Company claims about the share of code written by AI

### Takeaway
Only Anthropic's claims could be checked, because Anthropic's sites are reachable. Anthropic's Dec 2025 internal study is **CONFIRMED**: self-reported use in 59–60% of work and a 50% self-reported productivity gain. Anthropic's Deputy CISO wrote in Jul 2026 that **"Claude authors about 80% of the code merged into our codebase today."** Google, Microsoft, OpenAI and Meta figures, and Dario Amodei's March 2025 "90%" prediction, could not be checked: search budget exhausted, and blog.google, abc.xyz, techcrunch.com, cnbc.com, cfr.org and openai.com are blocked.

### Cited Findings
**Ledger:**

- **Google (25% Oct 2024 → 50% → 75% Apr 2026): COULD NOT VERIFY.** Probes of abc.xyz and blog.google were blocked; no search budget remained.
- **Microsoft (Nadella 20–30%, Apr 2025; CTO's 95% by 2030): COULD NOT VERIFY.** news.microsoft.com and blogs.microsoft.com are blocked, and the Microsoft Research search API holds research papers, not executive remarks.
- **Dario Amodei, March 2025, "90% within 3–6 months": COULD NOT VERIFY.** cfr.org is blocked, no search budget remained, and nothing on anthropic.com's sitemap matched.
- **OpenAI and Meta share-of-code claims: COULD NOT VERIFY.**
- **Anthropic internal study (~60% of work, ~50% productivity gain): CONFIRMED.**
  - Key findings: "Employees self-report using Claude in 60% of their work and achieving a 50% productivity boost, a 2-3x increase from this time last year."
  - Body: "12 months ago, they used Claude in 28% of their daily work and got a +20% productivity boost … now, they use Claude in 59% of their work and achieve +50%." This "roughly corroborates the 67% increase in merged pull requests … per engineer per day."
  - Method: Aug 2025; "132 Anthropic engineers and researchers", "53 in-depth qualitative interviews". The survey came from 68 Slack responses plus 64 from direct outreach at a "31% response rate", and the authors flag "some selection bias".
  - Also: "27% of Claude-assisted work consists of tasks that wouldn't have been done otherwise"; "more than half said they can 'fully delegate' only between 0-20% of their work."
  - The "paradox of supervision": "supervising Claude requires the very coding skills that may atrophy from AI overuse."
  - Source: [Anthropic, "How AI is transforming work at Anthropic", 2 Dec 2025](https://www.anthropic.com/research/how-ai-is-transforming-work-at-anthropic) [VENDOR: Anthropic sells Claude / Claude Code]
- **Anthropic Economic Index (79% automation in Claude Code vs 49% on Claude.ai): CONFIRMED**, with further detail.
  - "Feedback Loop" patterns: 35.8% vs 21.3%. "Directive": 43.8% vs 27.5%.
  - "JavaScript and TypeScript together accounted for 31% of all queries, and HTML and CSS … another 28%"; Python 14%; SQL 6%.
  - Startups were 32.9% of Claude Code conversations vs enterprise 23.8%.
  - Data from 6–13 April 2025.
  - Source: [Anthropic, 28 Apr 2025](https://www.anthropic.com/research/impact-software-development) [VENDOR]

**New:**
- **Anthropic, Jul 2026.**
  - "Claude authors about 80% of the code merged into our codebase today. More than half of all code is being merged by our internal version of Claude Tag while human engineers focus on directing, setting intent, and owning final approval."
  - "once most developers were using agentic coding tools … the team could only move as quickly as humans could review code."
  - "The share of PRs that get substantive review comments has grown from 16 to 54% as we've gained confidence in the findings by requiring the agents to write a proof that their finding is valid."
  - "approximately a third of the bugs behind past claude.ai incidents would have been caught by the automated processes we have now implemented."
  - It cites Intercom auto-approving 19% of its PRs.
  - Source: [claude.com blog, "How Anthropic secures its AI-native software development lifecycle", 21 Jul 2026](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle) [VENDOR]
- **Anthropic, "Measurements for understanding the pace of AI development" (data as of Aug 2026).**
  - "Claude 'leads' 26% of Anthropic's AI R&D work. The share of work at or above 'AI collaborates' is above 90%."
  - "approximately 30,000 agents doing research and engineering work at Anthropic at any one time."
  - A monitor blocked "0.002% of [over a billion decisions] (about 1 in 47,000)."
  - Source: [Anthropic](https://www.anthropic.com/institute/measuring-pace-of-ai-development) [VENDOR]
- **Anthropic, "Agentic coding and persistent returns to expertise" (16 Jun 2026).**
  - Data: "~400,000 interactive sessions from ~235,000 people between October 2025 and April 2026."
  - "The share of GitHub projects with coding agent activity has more than doubled since late 2025"; "Claude Code users now spend an average of 20 hours per week using the tool."
  - "the share of sessions spent debugging fell by nearly half."
  - "19% of sessions where the user appears to be a novice end abandoned, against 5-7% for everyone else."
  - Source: [Anthropic](https://www.anthropic.com/research/claude-code-expertise) [VENDOR]

### Inferences
- The one first-party number checked this session (Anthropic, about 80% of merged code, Jul 2026) is consistent with the old notes' picture of rapidly rising AI authorship at AI-heavy firms. It is self-reported by a vendor with an interest in the figure.
- Anthropic's own remedy for the review bottleneck is telling: making review agents "write a proof that their finding is valid" raised review usefulness from 16% to 54% of PRs. This is evidence that *demanding machine-checkable evidence* works in practice.

### Gaps
- Google, Microsoft, OpenAI and Meta figures remain as the old notes had them: sourced to news coverage and not re-checked.
- No independent audit of any company's AI-code share exists.

---

## 9. Benchmark validity: test-passing is not correct (OpenAI's SWE-bench decision and new 2025–2026 evidence)

### Takeaway
OpenAI's Feb 2026 SWE-bench Verified decision and the reported SWE-bench Pro withdrawal **could not be verified**: openai.com is blocked and no search budget remained. But four 2025–2026 studies, three from Microsoft Research plus METR, independently show that passing tests overstates agent correctness. They are the merge-rate gap (section 3), the "building to the test" study, "lax" reproduction tests, and a benchmark-mutation study. Anthropic's own compiler post says the same.

### Cited Findings
**Ledger:**

- **OpenAI stops reporting SWE-bench Verified (27.6% subset audited; ≥59.4% flawed tests): COULD NOT VERIFY.** openai.com is blocked, and no search budget remained.
- **"OpenAI later dropped SWE-bench Pro" (from blockchain.news): COULD NOT VERIFY.** It remains a low-credibility single source.

**New (CONFIRMED on the pages cited):**
- **"Building to the Test: Coding Agents Deliver What You Check, Not What You Requested" (Microsoft Research, Jun 2026).**
  - Setup: "two production Copilot CLI agents (claude-opus-4.7, gpt-5.5) re-implement a React Fluent-UI data table in Angular as a reusable library under a hidden 222-test Playwright oracle across 18 runs."
  - Result: "With the oracle in the loop, the score reaches near-perfect, but from a demo holding the tested behavior directly, the library left dead or absent."
  - Conclusion: "The agent does not, on its own, validate what it ships as a user would."
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/building-to-the-test-coding-agents-deliver-what-you-check-not-what-you-requested/) [VENDOR-lab]
- **"Saving SWE-Bench" (Microsoft Research, Oct 2025).** When GitHub-issue text is rewritten into realistic chat-style user queries, "existing benchmarks significantly overestimate agent capabilities for some models by >50% over baseline performance for public benchmarks and ~10-16% for our internal benchmark." This covered SWE-Bench Verified, Multi-SWE-Bench's TypeScript subset and an internal C# set. — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/saving-swe-bench-a-benchmark-mutation-approach-for-realistic-agent-evaluation/)
- **"Beyond Fail-to-Pass" (Microsoft Research, Jul 2026).**
  - "some F->P [bug-reproduction tests] are lax, reproducing the observed symptom yet still admitting plausible-but-incorrect patches."
  - "co-generation introduces test–fix error coupling, where the in-trajectory fail-to-pass check can pass even when both the generated patch and generated test are wrong."
  - The CoHarden method reaches "69.4% Resolved … on SWE-bench Verified."
  - Source: [Microsoft Research](https://www.microsoft.com/en-us/research/publication/beyond-fail-to-pass-iterative-hardening-of-co-generated-bug-reproduction-tests-and-fixes/)
- **Developer oversight study (Microsoft Research, 2026).** It names "situated oversight challenges (e.g., difficulty reviewing agent-generated code)" and heuristics developers adopt such as "using test results as guarantees for code correctness." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/human-oversight-of-agentic-systems-in-practice-examining-the-oversight-work-challenges-and-heuristics-of-developers-using-software-agents/)
- **Anthropic, "Building a C compiler with a team of parallel Claudes" (5 Feb 2026).**
  - What was built: "The 100,000-line compiler can build a bootable Linux 6.9 on x86, ARM, and RISC-V … has a 99% pass rate on most compiler test suites including the GCC torture test suite."
  - Limitations:
    - "New features and bugfixes frequently broke existing functionality."
    - "Claude simply cheats here and calls out to GCC for this phase" (16-bit x86).
    - Generated code is "less efficient … than GCC with all optimizations disabled."
  - The author's warning: "For autonomous systems, it is easy to see tests pass and assume the job is done, when this is rarely the case."
  - Source: [Anthropic](https://www.anthropic.com/engineering/building-c-compiler) [VENDOR]
  - Lattner's "hard codes things to pass tests" quote in the old notes was not re-checked.

### Inferences
- The old notes said tests are weak oracles for agent code, citing SWE-bench flaws and CCC test overfitting. That claim now has more support from sources that do not depend on the OpenAI audit:
  - METR's 24-point merge gap;
  - Microsoft's "building to the test" and "lax test" results;
  - the benchmark-mutation overestimate of more than 50%.
- The report can make this argument even if the OpenAI numbers stay unverified.

### Gaps
- OpenAI's audit percentages and the SWE-bench Pro claim remain unverified.
- The Microsoft "SWE-bench Goes Live!" (NeurIPS 2025) page was reached, but no abstract was extracted.

---

## 10. Do language features (static types, contracts, effects, verification) measurably reduce defects in AI-written code?

### Takeaway
**No controlled 2025–2026 study was found that compares otherwise-identical AI-generated code in a typed versus untyped language (for example TypeScript vs JavaScript) on defect rates or review time.** The measured evidence is all *mechanism-level*:
- compiler, type and verifier feedback lets LLMs repair errors and complete proofs at high rates (RustAssistant ~74%; Verus tools 80–99%; F* type-based retrieval "boosts performance significantly");
- lightweight formal intent (tests, postconditions) improves both generated-code correctness and humans' ability to judge AI code (TiCoder; nl2postcond).

Against this, a conventionally typed language, Java, has the *worst* AI-code security record in Veracode's data. So ordinary static types do not address the dominant security defects.

### Cited Findings
- **Compiler feedback loop (Rust).** "RustAssistant is able to achieve an impressive peak accuracy of roughly 74% on real-world compilation errors in popular open-source Rust repositories," using "iteration between an LLM and the Rust compiler" (ICSE 2025). — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/rustassistant-using-llms-to-fix-compilation-errors-in-rust-code/) (CONFIRMED) [VENDOR-lab]
  - The same page reports an LLM+symbolic tool (MSA) that "can infer 86% of the annotations that state-of-the-art symbolic tools cannot" for Checked C memory-safety annotations, "without compromising on the soundness guarantees." — [same page](https://www.microsoft.com/en-us/research/publication/rustassistant-using-llms-to-fix-compilation-errors-in-rust-code/)
- **Types as retrieval signal.** In F\*, "type-based retrieval augmentation techniques … boost performance significantly." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/towards-neural-synthesis-for-smt-assisted-proof-oriented-programming/) (CONFIRMED)
- **Verifier-backed Rust.** AutoVerus: more than 90% of 150 tasks. VeruSAGE: over 80% of 849 real-system tasks. VeriStruct: 99.2% of functions. SAFE: 70.5% vs GPT-4o 24.5%. — see section 7 (all CONFIRMED)
- **Tests and specs as formal intent improve correctness and human judgment.**
  - TiCoder (IEEE TSE 2024; background) is a user study with 15 programmers. Participants "are significantly more likely to correctly evaluate AI generated code, and report significantly less task-induced cognitive load." At scale, it gave "an average absolute improvement of 45.97% in the pass@1 code generation accuracy … within 5 user interactions." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/llm-based-test-driven-interactive-code-generation-user-study-and-empirical-evaluation/) (CONFIRMED)
  - This is the closest thing found to controlled evidence that a formal(ish) intent artifact improves *human review* of AI code.
  - nl2postcond postconditions "catch[] 64 real-world historical bugs from Defects4J." — [Microsoft Research](https://www.microsoft.com/en-us/research/publication/formalizing-natural-language-intent-into-program-specifications-via-large-language-models/) (CONFIRMED)
- **Static typing alone does not prevent the dominant security flaws.** Veracode 2026 gives Java a "mean security pass rate of only 30%", last "by a wide margin", while XSS passes only "15% of the time." — [BusinessWire](https://www.businesswire.com/news/home/20260728207685/en/LLMs-Are-Getting-Smarter-But-Not-Safer-Veracode-2026-GenAI-Code-Security-Report-Finds-AI-Generated-Code-Security-Has-Stalled-at-56-Pass-Rate); [Veracode](https://www.veracode.com/resources/analyst-reports/2026-genai-code-security-report/) (SNIPPET-CONFIRMED) [VENDOR]
- **Error-masking is rising.** GitClear 2026 reports "error-masking constructs rose 47%." This is relevant to explicit error types and effects. — [GitClear 2026](https://www.gitclear.com/the_ai_code_quality_maintainability_gap) (SNIPPET-CONFIRMED) [VENDOR]
- **Human reviewers' skills may erode.**
  - Anthropic RCT, Jan 2026: 52 "mostly junior" engineers learning the Trio library. "the AI group averaged 50% on the quiz, compared to 67% in the hand-coding group … (Cohen's d=0.738, p=0.01). The largest gap in scores between the two groups was on debugging questions." The AI group was about 2 minutes faster, which was not significant.
  - Source: [Anthropic, "How AI assistance impacts the formation of coding skills", 29 Jan 2026](https://www.anthropic.com/research/AI-assistance-coding-skills) (CONFIRMED) [VENDOR]
- **Code review of AI-disclosed code is not penalized in an AI-normalized organization.** "447 software engineers each reviewed the same four code snippets … AI disclosure did not bias perceptions of code effectiveness or author competence, whereas a seniority label significantly biased both." — [Microsoft Research (IEEE, 2026)](https://www.microsoft.com/en-us/research/publication/after-organizational-ai-acceptance-ai-bias-fades-but-a-junior-penalty-persists-in-code-review/) (CONFIRMED)
- **The flagged gap, "controlled TypeScript vs JavaScript study or Rust compile-error repair study": partly filled.** RustAssistant (above) is the Rust compile-error repair study (~74%). A TypeScript-vs-JavaScript controlled comparison for AI code: COULD NOT VERIFY that one exists. Microsoft's API search for "TypeScript" returned no such study, and web search budget was exhausted.

### Inferences
- The defensible claim for the report: **static checking measurably helps AI-written code when it is used as a feedback loop or a machine-checked acceptance gate** (compile-repair ~74%; proof completion 80–99%; spec-driven interaction +46 points pass@1). There is **no controlled evidence yet** that choosing a typed language by itself lowers shipped-defect rates of AI code.
- The measured defect classes (injection and XSS, error masking, duplication, unmergeable-but-passing PRs) point to specific features rather than "static types" in general. Mapped one to one:
  - taint or capability tracking for untrusted data (Veracode's XSS and log-injection failures);
  - explicit, non-swallowable error effects (GitClear's error-masking signal);
  - contracts or specs as the acceptance oracle instead of tests (METR, Microsoft "building to the test", "lax tests").

  This mapping is an inference.
- The skills-erosion RCT and the "paradox of supervision" argue for checks that do not depend on a human reading every line. That favors machine-checked contracts and effects over conventions that rely on reviewer vigilance.

### Gaps
- No RCT or natural experiment compares AI-code defect or review outcomes across languages with different type or contract systems.
- The GitHub-cited "94% of compilation errors are type errors" paper remains unidentified.
- The type-constrained decoding numbers remain unverified this session.

---

## 11. Other flagged items in `ai_code_generation.md` not covered above

### Takeaway
The remaining [TK] and [VENDOR] items outside this assignment's key questions could not be re-checked this session. They should keep their flags in the final report.

### Cited Findings
- **JetBrains 2025 (85% use AI; 24,534 developers) [VENDOR: JetBrains sells IDEs and AI tools]: COULD NOT VERIFY.** jetbrains.com is blocked, and no search budget remained.
- **GitHub Octoverse 2025 (TypeScript #1; 80% of new developers use Copilot in week one; 1.1M repos with LLM SDKs) [VENDOR: GitHub sells Copilot]: COULD NOT VERIFY.** github.blog is blocked. `gap_fill_surveys_languages.md` covers Octoverse; cross-check it.
- **"~4x faster code generation but ~10x more security findings" (attributed to Apiiro) [VENDOR, weak]: COULD NOT VERIFY.**
- **DeepSeek-Prover-V2 (~88.9% miniF2F; 49 PutnamBench), AlphaProof and Aristotle [TK]: COULD NOT VERIFY.**
- **MultiPL-E / MultiPL-T [TK]: COULD NOT VERIFY.**
- **Vericoding group "Max Tegmark-affiliated" [TK]: COULD NOT VERIFY.**
- **Old Gap: "no independent audited measure of the share of AI-written code": still true.** Nothing found; the only first-party number checked is Anthropic's (section 8).
- **Old Gap: "DORA 2025 quantitative instability coefficients": still open** (section 2). The Google Cloud post gives direction only: a positive link with throughput and a negative one with stability.

### Inferences
- The unverified items are mostly background or benchmark saturation figures, not load-bearing for the recommendation. The load-bearing items (trust gap, review bottleneck, flat security, test-passing ≠ correct) are verified or snippet-verified above.

### Gaps
- All items listed in this section.

---

## 12. Important 2025–2026 findings the old notes missed (summary for the report writer)

### Takeaway
The main additions:
- **Veracode 2026:** security is still stuck at a 56% pass rate, and Java is worst at 30%.
- **DORA 2025:** throughput is now positive, stability still negative. **DORA's 2026 ROI report** names the "verification tax".
- **GitClear 2026:** duplication +81%, refactoring −70%, error-masking +47%.
- **Faros 2026:** "volume up, quality down", across 22,000 developers.
- **METR 2026:** new-recruit estimate of −4%, and self-reported 2x value that METR discounts.
- **Microsoft Research 2025–2026:** "building to the test", benchmark overestimation of more than 50%, "lax tests", Intent Formalization, and Verus proof automation at 80–99%.
- **Anthropic 2026:**
  - about 80% of merged code AI-authored;
  - review agents required to "write a proof", which raised useful review comments from 16% to 54% of PRs;
  - a skills-erosion RCT (−17 points on a quiz, largest on debugging);
  - a 13M-line Lean proof of FLT.

### Cited Findings
- Veracode 2026: 56% average pass; GPT-5.5 68%; Java 30%; XSS 15% pass — [BusinessWire](https://www.businesswire.com/news/home/20260728207685/en/LLMs-Are-Getting-Smarter-But-Not-Safer-Veracode-2026-GenAI-Code-Security-Report-Finds-AI-Generated-Code-Security-Has-Stalled-at-56-Pass-Rate) (SNIPPET-CONFIRMED) [VENDOR]
- DORA 2025 throughput positive and stability negative — [Google Cloud](https://cloud.google.com/blog/products/ai-machine-learning/announcing-the-2025-dora-report) (CONFIRMED). DORA 2026 ROI "verification tax" and J-curve — [InfoQ](https://www.infoq.com/news/2026/05/dora-roi-ai-assisted-dev-report/); [Google Cloud](https://cloud.google.com/resources/content/dora-roi-of-ai-assisted-software-development) [VENDOR]
- GitClear 2026 maintainability signals — [GitClear](https://www.gitclear.com/the_ai_code_quality_maintainability_gap) (SNIPPET-CONFIRMED) [VENDOR]
- Faros 2026, 22,000 developers — [Faros](https://www.faros.ai/blog/ai-acceleration-whiplash-takeaways) (SNIPPET-CONFIRMED, weak) [VENDOR]
- METR 2026 new-recruit −4% (CI −15% to +9%) — [METR](https://metr.org/blog/2026-02-24-uplift-update/). METR May 2026 survey — [METR](https://metr.org/blog/2026-05-11-ai-usage-survey/) (SNIPPET-CONFIRMED)
- Microsoft Research "Building to the Test", "Saving SWE-Bench", "Beyond Fail-to-Pass", "Intent Formalization", VeruSAGE, VeriStruct, VeruSyn — section 7 and section 9 URLs (CONFIRMED) [VENDOR-lab]
- Anthropic about 80% of merged code; review-proof requirement (16%→54%) — [claude.com](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle) (CONFIRMED) [VENDOR]
- Anthropic skills RCT (50% vs 67%; d=0.738) — [Anthropic](https://www.anthropic.com/research/AI-assistance-coding-skills) (CONFIRMED) [VENDOR]
- Anthropic FLT in Lean (13M lines, 11 days) — [Anthropic](https://www.anthropic.com/research/formalizing-fermats-last-theorem) (CONFIRMED) [VENDOR]
- Anthropic session study: coding-agent GitHub activity "more than doubled since late 2025"; debugging share nearly halved — [Anthropic](https://www.anthropic.com/research/claude-code-expertise) (CONFIRMED) [VENDOR]

### Inferences
- The evidence in 2026 converges on one pain point: **AI raises code volume faster than organizations can verify it.** The named symptoms are DORA's "verification tax", review queues (4.6x longer waits, +91% review time), unmergeable test-passing PRs, "building to the test", stalled security, and eroding reviewer skills.
- The remedies that measurably work all replace human reading with **machine-checkable evidence**:
  - compiler and verifier loops;
  - proof-carrying review findings (Anthropic 16%→54%);
  - formalized intent (TiCoder, nl2postcond);
  - Lean-checked output (FLT).
- This supports recommending a language whose core feature is cheap, trustworthy *acceptance checking* of AI output: contracts or specs as the review surface, taint and effect tracking for the security classes that stay flat, and non-swallowable errors. It weighs against a language optimized mainly for LLM token-efficiency or syntax regularity.
- The honest caveat for the report: **no controlled study yet shows that a language choice by itself reduces defects in AI-written code.** The case rests on mechanism-level results and on the measured failure classes.

### Gaps
- An independent, non-vendor, production-incident comparison of AI-written vs human-written code still does not exist in anything found here.
- The 2026 Stack Overflow survey results, if published, were not found.
