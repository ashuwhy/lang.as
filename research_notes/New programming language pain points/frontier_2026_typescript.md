# TypeScript's unsound escape hatches and AI-generated TypeScript: demand-side evidence for a sound, contract-checked language on npm (as of 24 September 2026)

_Labels. **CONFIRMED**: the page or API response was read in this session. **SNIPPET-CONFIRMED**: page not opened, but two or more search results agree (quoted). **UNVERIFIED**: one search summary or one secondary source. **LOCAL**: the author's own SepInfer repo, read on disk (not independent evidence). **NOTES**: carried over from an earlier note in this folder and not rechecked. Items before 2024 are marked "background"._

_Access this session. Reachable: `registry.npmjs.org`. That covers package metadata, the search endpoint `/-/v1/search`, which reports `downloads.weekly` and `downloads.monthly`, full tarballs, and the npm audit endpoint `/-/npm/v1/security/advisories/bulk`, which returns GitHub-advisory metadata (title, severity, CWE, affected range, but no dates). Blocked (EGRESS_BLOCKED or HTTP 000): api.npmjs.org, www.npmjs.com, arxiv.org, nvd.nist.gov, services.nvd.nist.gov, cveawg.mitre.org, www.cve.org, api.osv.dev, osv.dev, zenodo.org, conf.researchr.org, 2026.msrconf.org, www.researchgate.net, www.themoonlight.io, www.emergentmind.com, www.endorlabs.com, thehackernews.com, www.opswat.com, socradar.io, hetmehta.com, www.tenable.com, advisories.gitlab.com, www.staicu.org, publications.cispa.de, www.typescriptlang.org, devblogs.microsoft.com, tc39.es, www.infoq.com, socket.dev, rescript-lang.org, gleam.run, dafny.org, effect.website, midspiral.com, typescript-eslint.io. github.com/advisories answered 403. The search tool returns one merged summary per query, not a separate snippet per URL. 39 web searches were used._

_How to reproduce the registry figures: `curl 'https://registry.npmjs.org/-/v1/search?text=<name>&size=20'` and take the exact-name match. All download figures below are the registry's own numbers as of 2026-09-24 (the packages' search rows were "updated" 2026-09-22)._

---

## 1. How often do real TypeScript bugs and CVEs come from unchecked types at runtime boundaries (JSON, network input, casts)?

### Takeaway
By bug count, type problems are a minority. The only large 2026 study of real TypeScript projects (Tang et al., MSR '26, 633 bugs) puts "Type Error" at 12.4%. That is below API misuse (14.5%), and the study describes the landscape as dominated by tooling and configuration faults. It gives no breakdown by casts, `any` or JSON, and older work found `any` use uncorrelated with bug proneness. By severity, the picture reverses. The clearest 2026 case is n8n's critical CVE-2026-25049: a sanitizer trusted a `string` annotation that nothing enforced at run time. Primary advisory data pulled for this note shows the pattern recurring in 2026 advisories for tmp, handlebars, sequelize and node-tar, and a large volume of prototype-pollution advisories, the JSON/object-input cousin: 51 in a 51-package sample, 14 in axios alone. CWE labels undercount the class, since the n8n CVE is filed as CWE-913, not CWE-843. TypePatrol, the dynamic tool tied to the n8n CVE, has almost no web footprint yet. The author's own corpus results say soundly detectable cast laundering is rare in shipped code.

### Cited Findings

**Tang, Alimadadi, Sumner, "From Logic to Toolchains" (MSR '26)**
- Authors "TianYi Tang, Saba Alimadadi, and Nick Sumner from Simon Fraser University". The study covers "633 bug reports from 16 popular open-source repositories". It finds that "the fault landscape is dominated not by logic or syntax errors but by tooling and configuration faults, API misuses, and asynchronous error-handling issues". It concludes that "while static typing in TypeScript has reduced traditional runtime and type errors, it has shifted fragility toward build systems and toolchains". It appeared at "MSR '26, April 13–14, 2026, in Rio de Janeiro". SNIPPET-CONFIRMED (three searches; result lists include arXiv abs/HTML/PDF, ResearchGate, emergentmind and a Moonlight review) — [arXiv 2601.21186](https://arxiv.org/abs/2601.21186); [ResearchGate](https://www.researchgate.net/publication/400236972_From_Logic_to_Toolchains_An_Empirical_Study_of_Bugs_in_the_TypeScript_Ecosystem)
- Category shares: "Type-related errors dropped from approximately 33% in JavaScript to 12.4% in TypeScript". "API Misuse (14.5%): Incorrect usage of internal or third-party APIs". "Type Error (12.4%): Incorrect, incomplete, or unsafe type annotations". "Tooling / Configuration bugs and Test Faults, virtually absent in prior JavaScript taxonomies, are now prevalent". SNIPPET-CONFIRMED: one search this session plus the earlier note's independent search; both summaries draw on the same aggregator pages, and one other search this session did not surface the 12.4% figure — [arXiv HTML](https://arxiv.org/html/2601.21186v1); [emergentmind](https://www.emergentmind.com/papers/2601.21186); [Moonlight review](https://www.themoonlight.io/en/review/from-logic-to-toolchains-an-empirical-study-of-bugs-in-the-typescript-ecosystem)
- The "Type Error" category bundles "incorrect, incomplete, or unsafe" annotations. No sub-split for `as`, `as unknown as`, `any`, `!` or unvalidated JSON was found. Tooling/configuration and asynchronous shares were not retrievable as numbers.
- A LinkedIn post by Greg Wilson about the paper appears in the results ("TypeScript Ecosystem Bug Study Reveals Tooling Faults"). Title only — [LinkedIn](https://www.linkedin.com/posts/gvwilson_from-logic-to-toolchains-an-empirical-study-activity-7423792111139004416-gpw5)

**TypePatrol (Galipelli, Staicu, Patra; ESEM 2026) and the n8n link**
- A search summary drawn from Staicu's researchr profile lists "TypePatrol ... Adversarial Testing for Uncovering Security-Relevant Type Inconsistencies in JavaScript Libraries" in "ESEM's Technical Track". UNVERIFIED (one search; researchr blocked) — [Staicu profile](https://conf.researchr.org/profile/cristianalexandrustaicu); [ESEM 2026 Technical Track](https://conf.researchr.org/track/eseiw-2026/eseiw-2026-esem---technical-track)
- A second query (`"TypePatrol" type inconsistencies CVE`) returned nothing about the tool. Like the earlier note, this session found no paper PDF, preprint or press coverage.
- The n8n connection is through a co-author. The Hacker News lists "Endor Labs' Cris Staicu" among "as many as 10 security researchers" credited for CVE-2026-25049, alongside Fatih Çelik (who reported the earlier CVE-2025-68613), Pillar Security's Eilon Cohen and SecureLayer7's Sandeep Kamble. UNVERIFIED (one search summary; page blocked) — [The Hacker News](https://thehackernews.com/2026/02/critical-n8n-flaw-cve-2026-25049.html)
- SepInfer's description of TypePatrol: it "adversarially mutates unit-test values to surface the same primitive-type laundering in JS/TS libraries, finding hundreds of inconsistencies and a critical n8n CVE". The author cross-ran TypePatrol's artifact: 15 libraries per language, 70 found sites (32 TS, 38 JS). Of 27 typed TS sites, SepInfer derived 25 in-fragment and flagged 0 ("the two detection surfaces are thus disjoint"). One return site (form-data `getBoundary`, "annotated `string` but returning a `number`") certified as a type-safety disagreement. LOCAL — `/home/user/sepinfer/paper/main-pldi.tex` (Related Work; appendix "Cross-Run Against a Dynamic Detector (TypePatrol)"); `/home/user/sepinfer/benchmarks/e1/OUTCOME.md`
- Caution: the author's `refs.bib` records that a September 2026 check found the DOI previously given for `typepatrol2026` "does not exist". The entry now has no DOI. LOCAL — `/home/user/sepinfer/paper/refs.bib`, lines 1–5

**"Typed and Confused" (Troppmann, Fass, Staicu; ASE 2024; background to TypePatrol)**
- Hypothesis: "type hints in code can mislead developers into thinking they are enforced consistently by the compiler, resulting in a lack of explicit runtime checks". Scope: "30,000 open-source repositories" analysed statically. It notes that when "user input is involved, it can render input validation mechanisms ineffective, resulting in type confusion problems", and that "Developers must implement explicit type checks". SNIPPET-CONFIRMED (two searches; result lists include ACM DL, CISPA, Zenodo, staicu.org and ResearchGate) — [ACM DL](https://dl.acm.org/doi/10.1145/3691620.3695549); [CISPA](https://publications.cispa.de/articles/conference_contribution/Typed_and_Confused_Studying_the_Unexpected_Dangers_of_Gradual_Typing/27051859); [PDF](https://www.staicu.org/publications/ase2024.pdf)
- The paper's quantitative result (how much less often typed code checks types at run time) was not retrievable.

**The author's corpus results (SepInfer, PLDI 2027 submission)**
- "Across 22 production SDKs at pinned commits (15,139 non-test files, analyzed unmodified), 320 of 338 double-cast uses - 94.7% - target a type nothing in our domain can refute". "Classifying every assertion in two further corpora puts 44 to 48% beyond any sound analysis". LOCAL — `/home/user/sepinfer/paper/main-pldi.tex`, abstract
- "The shape a detector would target, a double cast landing on a primitive, occurs 18 times in those 15,139 files, two of them with a source decided enough for anything to refute". Against 150 unseen repositories, "the analyzer reports six Class C sites in four, none false". The first real-world Class C was the `helius-sdk` export `"1_000_000_000" as unknown as bigint`. LOCAL — same abstract; `/home/user/sepinfer/README.md`

**Other empirical studies (2025) and counter-evidence**
- "An empirical study on bugs in TypeScript programming language" (JSS 2025) is about bugs in the TypeScript compiler itself, not in user code: "8814 bug reports and 7974 pull requests in the repository of TypeScript". The most affected features are "Name, Binding and Scope" and "data types". SNIPPET-CONFIRMED (result list includes ACM DL and ScienceDirect) — [ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S016412122500113X); [ACM DL](https://dl.acm.org/doi/10.1016/j.jss.2025.112445)
- **Counter-evidence (background, 2022).** Bogner and Merkel mined 604 GitHub projects (299 JavaScript, 305 TypeScript, over 16M lines). They found "bug proneness and bug resolution time of the TypeScript sample were not significantly lower than for JavaScript". `any` frequency "was significantly correlated with all metrics except bug proneness", and "the correlations were of small strengths (Spearman's rho between 0.17 and 0.26)". SNIPPET-CONFIRMED (result list includes arXiv, VU Amsterdam, Semantic Scholar and ResearchGate) — [arXiv 2203.11115](https://arxiv.org/abs/2203.11115); [VU Amsterdam](https://research.vu.nl/en/publications/to-type-or-not-to-type-a-systematic-comparison-of-the-software-qu/)
- **Counter-evidence (practitioner and official, background).** Effective TypeScript: "for every soundness-related bug that sneaks through, TypeScript will probably save developers from hundreds of 'cannot read property of undefined' errors". The TypeScript handbook's type-compatibility page says of an unsound case: "in practice, this sort of error is rare, and allowing this enables many common JavaScript patterns". UNVERIFIED (one search summary; result list includes effectivetypescript.com and typescriptlang.org) — [Effective TypeScript](https://effectivetypescript.com/2021/05/06/unsoundness/); [TS handbook: Type Compatibility](https://www.typescriptlang.org/docs/handbook/type-compatibility.html)
- **Counter-evidence (the author's own).** The paper says the npm sweep shows "soundly-detectable unsoundness is scarce in shipped human code", and that this scarcity comes from "how these constructs are used, not the code's provenance". LOCAL — `/home/user/sepinfer/paper/main-pldi.tex`, appendix "AI-Generated Code Probe"
- Supporting (background, 2019, weak): a vendor blog repeats that "Airbnb famously found that 38% of their production bugs would have been caught by TypeScript's compiler". That is about TypeScript versus JavaScript, not about soundness. UNVERIFIED (single secondary source) — [Bacancy blog](https://www.bacancytechnology.com/blog/typescript-best-practices)

**CVE-level evidence** is in section 6: n8n's advisory history and a 51-package census.

### Inferences
- The bug-count case for "TypeScript is unsound, so replace it" is weak. Type errors are about 1 in 8 TypeScript bugs in the best 2026 sample, and nobody has split out the share caused by casts or unchecked JSON. The security case is stronger and specific: at trust boundaries (sanitizers, template engines, ORMs, path checks, config merges), an unenforced annotation becomes an exploitable assumption, and these bugs are critical when they occur.
- Static and dynamic evidence agree that the dangerous cases are rare in the source and live at the boundary. SepInfer finds almost no soundly refutable laundering in the source (18 double casts onto primitives in 15,139 files). TypePatrol finds wrong runtime types induced by inputs. The two tools' surfaces are disjoint. This points a new language at checked boundaries (inputs, JSON, FFI and `.d.ts` imports) rather than at banning `as` inside well-typed code.
- CWE-843 counts would understate the problem, because the flagship case (n8n) is filed as CWE-913. Any quantitative claim should use titles or descriptions, not CWE alone.

### Gaps
- Tang et al.'s full taxonomy table and any cast/`any`/JSON sub-split (arXiv blocked).
- TypePatrol's own numbers (sites found, libraries, CVEs credited), and whether the paper is public yet. No preprint was found.
- Typed and Confused's quantitative result.
- No 2024–2026 study was found that counts CVEs caused by TypeScript casts or unchecked JSON across the ecosystem.

---

## 2. How often does AI-generated TypeScript use `any`, `as` casts or non-null assertions, compared with human code?

### Takeaway
There is now one direct study, and it points one way. Lee, Ul Hassan and Hindle (MSR '26 Mining Challenge, AIDev dataset) find AI agents "9x more prone to use the 'any' keyword compared to humans" and more likely to introduce non-null assertions and type assertions. Agent TypeScript pull requests are nevertheless accepted 1.8x more often than human ones. Against that, the author's own probe of 38 strict-clean modules from one model found 0 soundly detectable faults and only benign `as unknown as` plumbing. The honest summary: agents reach for escape hatches more often, reviewers merge them anyway, but no study yet shows that this produces more runtime faults.

### Cited Findings
- **Lee, Ul Hassan, Hindle, "Mining Type Constructs Using Patterns in AI-Generated Code"** (University of Alberta; arXiv 2602.17955, last updated 22 April 2026; MSR 2026 Mining Challenge):
  - "AI agents are 9x more prone to use the 'any' keyword compared to humans"
  - "Agentic pull requests (PRs) are more likely to introduce overly type-related anti-patterns, such as non-null assertions and type assertions"
  - agents "use advanced type constructs, including those that ignore type checks, more often compared to humans", and this finding is "quite statistically significant"
  - "Agentic pull requests have 1.8x higher acceptance rates compared to humans for TypeScript"
  - "If AI agents use a method to bypass compile-time checks carelessly, they can introduce subtle runtime errors"; developers "should carefully confirm the type safety of their codebases whenever they coordinate with AI agents"
  - The replication package holds "a Python-based parser and Large Language Model prompts" and extracts constructs "using pattern-based methods"

  SNIPPET-CONFIRMED (five searches; result lists include arXiv abs/HTML/PDF, the MSR 2026 Mining Challenge page, ResearchGate, Zenodo and awesomepapers) — [arXiv 2602.17955](https://arxiv.org/abs/2602.17955); [MSR 2026 page](https://2026.msrconf.org/details/msr-2026-mining-challenge/18/Mining-Type-Constructs-Using-Patterns-in-AI-Generated-Code); [Zenodo replication package](https://zenodo.org/records/19685727)
- **AIDev dataset** (the corpus behind the MSR 2026 challenge): "932,791 agent-authored pull requests from 116,211 repositories involving five AI coding agents". A testing-focused subset has "6,582 human-agent pull requests and 3,122 human pull requests spanning four programming languages including TypeScript". UNVERIFIED (one search summary; Lee et al.'s own subset size was not found) — [AIDev, arXiv 2602.09185](https://arxiv.org/abs/2602.09185); [arXiv 2601.21194](https://arxiv.org/html/2601.21194)
- **AgenticTyper** (arXiv 2602.21251, 2026) is an agent that adds types to legacy JS/TS. "Evaluation on two proprietary repositories (81K LOC) shows that AgenticTyper resolves all 633 initial type errors in 20 minutes". "When type errors cannot be automatically fixed, the system inserts suppression comments annotated as either bugs (@ts-expect-error BUG) or valid untyped code patterns". It "only adds type annotations — it never modifies runtime behavior", which is checked by comparing transpiled output. In other words, the agent resolves errors partly by suppressing them. UNVERIFIED (one search summary; result list includes arXiv, alphaXiv, Bytez and the GitHub repo) — [arXiv 2602.21251](https://arxiv.org/abs/2602.21251)
- **The "94% of LLM compile errors are type-check failures" figure** cited by GitHub is attributed by secondary blogs to "Recent academic research from ETH Zurich and UC Berkeley". That fits the earlier notes' guess of Mündler et al. (PLDI 2025, type-constrained decoding), but no source names the paper. UNVERIFIED — [GitHub blog](https://github.blog/ai-and-ml/llms/why-ai-is-pushing-developers-toward-typed-languages/); [yuv.ai](https://yuv.ai/blog/ai-pushing-typed-languages)
- **Counter-evidence: the author's AI probe.** One model (gemini-flash) generated 40 modules from neutral tickets. 38 were `tsc --strict` clean (4,517 LoC), and they "contained 0 soundly-detectable type-soundness faults". "Its `as unknown as` casts are benign generic-default and branded-type plumbing, not data laundering". Stated limit: "one model under clean-framed prompts, not a survey or adversarial generation". LOCAL — `/home/user/sepinfer/paper/main-pldi.tex`, appendix "AI-Generated Code Probe"
- **LLMs weaken specs.** In SepInfer's ablation, the evolutionary LLM baseline "verifies" every Class C function "only via a spec the floor guard rejects - it passes by weakening the spec past the declared cell". LOCAL — `/home/user/sepinfer/README.md` (Phase D). LemmaScript (section 5) independently requires that the LLM-edited Dafny file differ from the generated one by "additions-only", the same safeguard. CONFIRMED — [registry lemmascript README](https://registry.npmjs.org/lemmascript)
- Security of AI code generally (not TypeScript-specific): Veracode's 2026 report puts the average security pass rate at 56%, "virtually unchanged since last year". NOTES (`gap_fill_ai_code.md`, SNIPPET-CONFIRMED there)

### Inferences
- The two pieces of evidence are compatible. Agents write `any`, `!` and `as` more often (Lee et al.), yet in clean, single-module settings those escape hatches rarely amount to a soundly detectable fault (SepInfer probe). The risk shows up at the boundary: agents type external data as `any` or cast it, and reviewers accept 1.8x more readily. That is exactly where n8n-style bugs live.
- In a language where `any`, unchecked casts and `!` do not exist, and where boundary data must pass a generated check, the 9x gap cannot arise. That is a cleaner story than "detect bad casts afterwards", which SepInfer shows hits a ceiling.
- Agents resolve type errors partly by suppressing them (AgenticTyper's `@ts-expect-error`) and by weakening specs (SepInfer, LemmaScript's additions-only rule). A language for AI-written code needs non-weakenable contracts as a first-class rule, not a CI convention.

### Gaps
- Lee et al.'s sample size, per-construct rates (casts, `!`, `@ts-ignore`) and per-agent breakdown (arXiv, Zenodo and MSR pages all blocked).
- No study links agent escape-hatch use to runtime failures or CVEs.
- No benchmark measures `any`/`as` rates in model output under strict settings across several models.

---

## 3. How large is the runtime-validation ecosystem that exists because TypeScript types are erased, and how is it trending?

### Takeaway
Runtime validation is at compiler scale on npm. The registry reports Zod at 211.9M weekly downloads, slightly above the `typescript` package itself (209.2M), with 128,822 dependent packages. Ajv (JSON Schema) is at 287.9M, TypeBox (`@sinclair/typebox`) at 81.2M and Effect at 26.3M. A shared validator interface, `@standard-schema/spec`, is at 80.7M. Growth is concentrated in the newer libraries (Zod 4, Valibot, ArkType, TypeBox 1.x, Effect); io-ts, superstruct and fp-ts have stopped releasing. Much of the volume is transitive, so downloads overstate direct use.

### Cited Findings

**Weekly downloads on 2026-09-24** (CONFIRMED — `https://registry.npmjs.org/-/v1/search?text=<name>`, exact-name match)

| Package | Weekly | Monthly | Dependents | Latest version (date) |
|---|---|---|---|---|
| ajv | 287,892,010 | 1,331,257,345 | 19,693 | 8.20.0 (2026-04-24) |
| zod | 211,933,203 | 1,022,521,791 | 128,822 | 4.6.5 (2026-09-13) |
| _typescript (for scale)_ | _209,215,221_ | _1,004,355,023_ | _66,420_ | _7.0.2 (2026-07-08)_ |
| @sinclair/typebox (TypeBox 0.x) | 81,212,632 | 410,407,371 | 6,708 | 0.34.52 (2026-07-11) |
| @standard-schema/spec | 80,710,923 | 401,689,688 | 1,326 | 1.1.0 (2025-12-15) |
| typescript-eslint (lint rules incl. `no-explicit-any`, `no-non-null-assertion`) | 64,393,188 | 314,193,488 | 3,484 | 8.70.1 (2026-09-21) |
| effect | 26,315,867 | 120,527,897 | 3,329 | 3.22.2 (2026-09-09) |
| joi | 19,571,079 | 94,284,822 | 13,587 | 18.2.9 (2026-09-11) |
| valibot | 13,370,944 | 67,422,815 | 1,643 | 1.5.0 (2026-09-09) |
| yup | 8,462,501 | 41,457,118 | 7,204 | 1.7.1 (2025-09-21) |
| class-validator | 8,212,234 | 42,031,397 | 7,398 | 0.15.1 (2026-02-26) |
| typebox (TypeBox 1.x, new name) | 8,186,082 | 43,631,539 | 1,948 | 1.3.34 (2026-09-18) |
| superstruct | 4,705,564 | 23,267,145 | 806 | 2.0.2 (2024-07-06) |
| fp-ts | 3,322,939 | 16,245,014 | 2,683 | 2.16.11 (2025-08-18) |
| io-ts | 2,049,283 | 9,245,156 | 1,774 | 2.2.22 (2024-12-10) |
| arktype | 1,345,148 | 6,818,393 | 548 | 2.2.3 (2026-07-07) |
| @effect/platform | 1,180,243 | 5,681,559 | 594 | 0.97.2 (2026-09-09) |
| @total-typescript/ts-reset | 847,650 | 4,462,451 | 108 | 0.6.1 (2024-09-02) |
| typia (validators generated from TS types) | 241,879 | 1,163,880 | 331 | 15.0.0 (2026-09-22) |
| runtypes | 186,342 | 905,309 | 344 | 7.0.5 (2026-08-14) |

- Zod companions: `zod-to-json-schema` 44,434,863/week, `zod-validation-error` 34,454,554/week. CONFIRMED (same endpoint)
- Third-party trackers agree on Zod: Socket reports "219,173,224 weekly downloads". Earlier notes had npm trends at 211,601,986 and Snyk at 213,873,599. SNIPPET-CONFIRMED — [Socket](https://socket.dev/npm/package/zod); [npm trends](https://npmtrends.com/zod); NOTES (`frontier_2026_candidates.md`)

**Trend**
- Zod: "When Zod v3.0 was released in May 2021, it had 2700 stars on GitHub and 600k weekly downloads". UNVERIFIED (one search summary, apparently from Zod's v4 release notes) — [Zod v4 notes](https://zod.dev/v4). PkgPulse says Zod grew "about 360% in the same-week comparison (comparing May 2026 to May 2025)". UNVERIFIED (single source) — [PkgPulse](https://www.pkgpulse.com/guides/20-fastest-growing-npm-packages-2026). An older DEV post said "139M weekly downloads". NOTES. The registry dates Zod 4.0.0 to 2025-07-09 and 4.6.0 to 2026-09-09. CONFIRMED — [registry zod](https://registry.npmjs.org/zod)
- Effect: "5M weekly downloads (November 2025), passing 10M weekly by mid-April 2026" according to the project blog. NOTES (UNVERIFIED there) — [This Week in Effect](https://effect.website/blog/this-week-in-effect/2026/04/17/). The registry now shows 26.3M, which would be about 2.6x since April if the blog figure is right. Effect 4.0 is at release-candidate stage (`rc` 4.0.0-rc.117; `latest` still 3.22.2). CONFIRMED — [registry effect](https://registry.npmjs.org/effect)
- Corrections to the earlier note (`frontier_2026_candidates.md`, figures from PkgPulse): TypeBox "5.5M/week" is **CORRECTED** to 81.2M (`@sinclair/typebox`) plus 8.2M (`typebox`). ArkType "about 400K/week" is **CORRECTED** to 1.35M. Valibot "15.6M/week" becomes 13.4M (same order). Source: registry, above; old figures from [PkgPulse](https://www.pkgpulse.com/guides/zod-v4-vs-arktype-vs-typebox-vs-valibot-2026)
- Stagnant incumbents: io-ts (last release 2024-12-10), superstruct (2024-07-06) and fp-ts (2025-08-18). CONFIRMED (registry release dates)
- Inflation caveat: Zod's count is "inflated by its inclusion as a peer/transitive dependency of tRPC, React Hook Form resolvers, and Drizzle ORM". NOTES (from PkgPulse)

**Survey-side demand for runtime types**
- State of JS 2025 (survey November 2025, published February 2026): "40 percent of respondents writing exclusively in TypeScript, up from 34 percent in 2024". SNIPPET-CONFIRMED (result lists include InfoQ, devclass and 2025.stateofjs.com) — [InfoQ](https://www.infoq.com/news/2026/03/state-of-js-survey-2025); [devclass](https://www.devclass.com/development/2026/02/10/javascript-survey-reveals-gripes-against-date-handling-webpack-and-nextjs-and-that-typescript-has-won/4090262)
- One summary adds: "Lack of static typing remains the number one language pain point". On native types in JavaScript, "TypeScript-like type annotations came in first with 5,380 votes, ahead of runtime types at 3,524", and "32% of respondents are hoping for in-browser runtime types support". UNVERIFIED (one search summary; it may mix the 2024 and 2025 surveys) — [State of JS 2025 Features](https://2025.stateofjs.com/en-US/features/); [reptile.haus](https://reptile.haus/journal/state-of-javascript-2025-what-the-survey-results-mean-for-your-development-team/)

### Inferences
- Erased types have already produced a validation layer as large as the compiler. A new language does not compete with nothing; it competes with, or should emit, these validators. Emitting Standard Schema-compatible validators from declared contracts would plug into what frameworks (tRPC, form libraries, ORMs) already accept.
- Fragmentation is the pain signal: ten or more validators with millions of weekly downloads, and a shared interface built so frameworks can accept any of them. Developers keep a type and a schema in sync by hand, or derive the type from the schema. typia (types → validators at compile time) exists but is small (242K a week), so "derive checks from types" is not yet mainstream.
- Ajv should not be counted as caused by TypeScript's erasure; it predates wide TypeScript use and serves JSON Schema generally.

### Gaps
- True time series (api.npmjs.org blocked). The trend rests on snapshots from earlier notes and single-source claims.
- The direct-versus-transitive split of validator downloads.

---

## 4. TypeScript itself in 2026: TypeScript 7 release and soundness, the TC39 "type annotations" proposal, the official stance on soundness

### Takeaway
TypeScript 7.0 (the native Go compiler) went GA as 7.0.2 on 8 July 2026. It keeps 6.0's type-checking options and does not change soundness. The real 2026 change came in TypeScript 6.0 (March 2026), which made `strict` the default: a strictness change, not a soundness one. `as`, `any` and `!` are untouched. The design non-goal of a sound type system stands. The TC39 type-annotations proposal is still at Stage 1, and it would erase types at run time anyway. Node.js now runs `.ts` files by stripping types without checking them. Every official path in 2026 keeps types erased at run time.

### Cited Findings
- **TypeScript 7.0 GA.** The registry `latest` tag is 7.0.2, published 2026-07-08T15:55:18Z. 7.0.1-rc was published 2026-06-18; 6.0.2 on 2026-03-23; 6.0.3 on 2026-04-16. `next` is 7.1.0-dev.20260924.1. CONFIRMED — [registry typescript](https://registry.npmjs.org/typescript)
- **It is a Go binary.** 7.0.2 ships per-platform binaries as optional dependencies (about 20 packages, e.g. `@typescript/typescript-linux-x64`). The linux-x64 tarball holds a 24 MB `lib/tsc` whose strings name `go1.26.4`. CONFIRMED — [registry tarball](https://registry.npmjs.org/@typescript/typescript-linux-x64/-/typescript-linux-x64-7.0.2.tgz)
- **No soundness change in 7.0 (name-level check).** All type-checking options of 6.0.2 are present in the 7.0.2 binary, including `strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `useUnknownInCatchVariables`, `strictBuiltinIteratorReturn`, `noPropertyAccessFromIndexSignature` and `erasableSyntaxOnly`. New options `checkers`, `builders` and `singleThreaded` appear. Seven old options are gone: `importsNotUsedAsValues`, `keyofStringsOnly`, `noImplicitUseStrict`, `noStrictGenericChecks`, `preserveValueImports`, `suppressExcessPropertyErrors` and `suppressImplicitAnyIndexErrors`. Three of those (`noStrictGenericChecks`, `suppressExcessPropertyErrors`, `suppressImplicitAnyIndexErrors`) were switches that weakened checking. CONFIRMED (string scan of the tarball; not a behavioural test)
- Performance and API: 7.0 has "8–12x faster full builds" and "no stable programmatic API until 7.1". NOTES (SNIPPET-CONFIRMED there) — [InfoQ](https://www.infoq.com/news/2026/08/typescript-7-released/). The 7.0.2 `package.json` exports only `./unstable/sync` and similar API entry points. CONFIRMED (tarball). The compatibility package `@typescript/typescript6` (6.0.2) has 3,907,396 weekly downloads, and `@typescript/native-preview` still has 6,079,557. CONFIRMED (registry)
- **TypeScript 6.0 made `strict` the default.** In 5.9.3's `lib/typescript.js`, `getStrictOptionValue` returns `!!compilerOptions.strict` for an unset strict sub-flag (off unless requested). In 6.0.2 it returns `compilerOptions.strict !== false` (on unless disabled). CONFIRMED (compiler source in the registry tarballs [5.9.3](https://registry.npmjs.org/typescript/-/typescript-5.9.3.tgz) and [6.0.2](https://registry.npmjs.org/typescript/-/typescript-6.0.2.tgz))
- Secondary sources on 6.0: it "shipped in March 2026 as the last JavaScript-based compiler release". "Strict mode is now default. Projects that relied on strict: false implicitly will now see errors from strictNullChecks, noImplicitAny, strictFunctionTypes, strictBindCallApply, strictPropertyInitialization, noImplicitThis, useUnknownInCatchVariables, and alwaysStrict". Other new defaults: `module: "esnext"`, a floating `target` (es2025), `types: []` and `noUncheckedSideEffectImports: true`. Deprecations such as `baseUrl` and `moduleResolution: node` "will be removed in TypeScript 7.0". SNIPPET-CONFIRMED (result list includes typescriptlang.org release notes, Socket, jsmanifest and OpenReplay) — [TS 6.0 release notes](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html); [Socket](https://socket.dev/blog/typescript-6-0-released-final-javascript-based-version); [jsmanifest](https://jsmanifest.com/typescript-6-breaking-changes)
- 6.0.2 added only two options over 5.9.3 (`ignoreConfig`, `stableTypeOrdering`); neither concerns soundness. CONFIRMED (option-declaration diff)
- **Official stance.** TypeScript's design non-goals include: "Apply a sound or 'provably correct' type system. Instead, strike a balance between correctness and productivity." SNIPPET-CONFIRMED (result list includes the Microsoft TypeScript-wiki page and secondary write-ups) — [TypeScript Design Goals](https://github.com/microsoft/TypeScript-wiki/blob/main/TypeScript-Design-Goals.md); [dsebastien.net](https://www.dsebastien.net/2020-04-25-typescript-non-goals/). No 2026 statement revising it was found.
- **TC39 Type Annotations.** It "reached Stage 1 in March 2022, and since then, development has effectively halted. No updates have appeared in TC39 meeting notes." Engines would parse the syntax "but ignor[e] it at runtime"; "The proposal never intended to add type checking to JavaScript itself". TC39's March 2026 meeting advanced eight proposals, including Temporal to Stage 4, and type annotations was not among them. The proposal repo has an issue titled "This proposal is probably dead?". SNIPPET-CONFIRMED for "still Stage 1" (this search plus the earlier note; result list includes jsmanifest on DEV and Medium, Socket, and the TC39 GitHub repo and issue #178). UNVERIFIED for "no 2026 activity" — [jsmanifest](https://dev.to/jsmanifest/tc39-type-annotations-proposal-in-2026-what-javascript-developers-need-to-know-before-it-hits-5fie); [Socket: Temporal Stage 4](https://socket.dev/blog/tc39-advances-temporal-to-stage-4); [issue #178](https://github.com/tc39/proposal-type-annotations/issues/178)
- **Node.js runs TypeScript by erasure.** "type stripping lets Node run .ts files with no build step since v22.18, default in v24". It "removes erasable TypeScript syntax, such as type annotations and interfaces, then runs the remaining JavaScript", using Amaro, a wrapper around `@swc/wasm-typescript`. "Node.js does not type check your code when it runs TypeScript files". "TypeScript 5.8 added the erasableSyntaxOnly compiler option". SNIPPET-CONFIRMED (result list includes nodejs.org Learn and several guides) — [Node.js: Running TypeScript Natively](https://nodejs.org/learn/typescript/run-natively); [DEV guide](https://dev.to/pockit_tools/nodejs-native-typescript-the-complete-guide-to-running-ts-files-without-a-compiler-mpa)

### Inferences
- The 2026 platform direction is toward erasure. Node strips types, TC39 would ignore them, and TypeScript 7 is faster, not sounder. Nothing in the official roadmap closes the gap between annotation and runtime at boundaries, so the validator ecosystem and the n8n-class bug will persist. That makes the gap stable ground for a new language rather than a moving target that Microsoft is about to close.
- 6.0's strict-by-default shows that Microsoft will tighten defaults when the ecosystem can absorb it. The escape hatches themselves (`as`, `any`, `!`) remain, by design.
- 7.0 has no stable API until 7.1, and the old JS compiler lives on as `@typescript/typescript6` with 3.9M weekly downloads. Tools that depend on the compiler API (linters, codegen, and any TypeScript emitter or checker a new language builds on) face a transition cost in 2026–2027.

### Gaps
- Behavioural confirmation that 7.0's checker matches 6.0's on unsound constructs (only option names were compared).
- The official 7.0 announcement text (devblogs.microsoft.com blocked).
- TC39 meeting notes for 2026 (tc39.es and GitHub notes not read).

---

## 5. Sound or verified languages that compile to JavaScript/TypeScript and publish to npm: status and adoption

### Takeaway
All are alive but tiny beside TypeScript. ReScript (12.3.1, August 2026; 34K weekly downloads), Elm (0.19.2 in July 2026, its first compiler release since 2019; 27K) and PureScript (0.15.16; 7.6K) together reach about 68K weekly downloads, about 0.03% of `typescript`'s 209M. Flow's parsers are large only because of React Native and Meta tooling. Gleam and Dafny do not distribute through npm, and F*'s JavaScript backend is unmaintained. The newest and most relevant entrant is LemmaScript (npm, April 2026, "Tech Preview"): it verifies annotated TypeScript through Dafny or Lean and pitches itself as "the correctness layer for AI-generated software". It has about 1.7K weekly downloads.

### Cited Findings

**Registry figures on 2026-09-24** (CONFIRMED — registry search endpoint and packuments)

| Package | What it is | Weekly | Dependents | Latest (date) |
|---|---|---|---|---|
| hermes-parser | Meta's parser for Flow-typed JS (React Native tooling) | 56,654,019 | 330 | 0.37.0 (2026-07-14) |
| flow-parser | Flow's parser | 5,318,962 | 372 | 0.333.0 (2026-09-24) |
| flow-bin | Flow checker binary | 433,924 | 465 | 0.333.0 (2026-09-24) |
| assemblyscript | TypeScript-like language to Wasm (comparison) | 124,967 | 159 | 0.28.20 (2026-07-22) |
| rescript | ReScript compiler | 33,972 | 207 | 12.3.1 (2026-08-23) |
| @rescript/runtime | ReScript 12 runtime | 29,356 | 18 | 12.3.1 (2026-08-23) |
| elm | Elm compiler installer | 26,908 | 37 | 0.19.2-0 (2026-07-06) |
| elm-test | Elm test runner | 23,251 | 9 | 0.19.2-1 (2026-08-21) |
| @rescript/core | ReScript 11 stdlib | 15,330 | 75 | 1.6.1 (2024-10-16) |
| purescript | PureScript compiler installer | 7,563 | 11 | 0.15.16 (2026-03-15) |
| spago | PureScript build tool | 4,860 | 4 | 1.0.4 (2026-03-30) |
| lemmascript | TS → Dafny/Lean verification toolchain | 1,733 | 0 | 0.6.4 (2026-09-19) |

- **ReScript.** Releases: 11.0.0 on 2024-01-10; 12.0.0 on 2025-11-25; 12.1.0 2026-01-13; 12.2.0 2026-02-27; 12.3.0 2026-05-19; 12.3.1 2026-08-23. A `next-13` tag points to 13.0.0-alpha.6. CONFIRMED — [registry rescript](https://registry.npmjs.org/rescript). ReScript 12 "complet[ed] a multi-year development roadmap to modernize the compiler toolchain". Its pitch: "you won't be able to write code that passes type compilation but produces type errors at runtime", while it "covers only a curated subset of JavaScript". SNIPPET-CONFIRMED (result list includes InfoQ, rescript-lang.org and codecentric) — [InfoQ](https://www.infoq.com/news/2025/12/rescript-12-release/); [codecentric](https://www.codecentric.de/en/knowledge-hub/blog/rescript-compare-typescript-elm). No company-level adoption figures were found.
- **Elm.** The npm `latest` tag moved to 0.19.2-0 on 2026-07-06. CONFIRMED — [registry elm](https://registry.npmjs.org/elm). "Elm 0.19.2 was released on July 6, 2026 — the first compiler release since 0.19.1 in 2019", a "performance-only patch release with no language changes". SNIPPET-CONFIRMED (result list includes the elm/compiler release page, Elm Discourse, Devtalk and newreleases.io) — [Elm Discourse](https://discourse.elm-lang.org/t/elm-0-19-2-is-live/10841); [elm/compiler 0.19.2](https://github.com/elm/compiler/releases/tag/0.19.2)
- **PureScript**: 0.15.16 on 2026-03-15, 7.6K weekly. CONFIRMED (registry)
- **Flow**: `flow-bin` 0.333.0 was published on 2026-09-24, the day of this note. It replaces the earlier note's unverified "0.326.0 on 2026-08-05". CONFIRMED (registry). **Hegel** (a sound JS checker) was closed by its author in 2024. NOTES
- **Gleam**: its compiler is not on npm (`gleam` on npm is an unrelated 2014 package with 5 weekly downloads). CONFIRMED (registry). In the 2024 survey (841 responses), "Gleam's JavaScript target is used a lot, and among production uses Gleam's JavaScript target is even more widely used". UNVERIFIED (one search summary) — [Gleam survey 2024](https://gleam.run/news/developer-survey-2024-results/). Earlier notes: 297 respondents used the JS target and 571 the Erlang target. NOTES. A 2025 survey exists; its results were not found — [developer-survey.gleam.run](https://developer-survey.gleam.run/)
- **Dafny → JavaScript**: Dafny "can compile to multiple different backends including JavaScript", producing "Javascript source consistent with Node.js v 16.0.0" via `dafny run --target:js`. `dafny2js` "generates JavaScript/TypeScript adapters from Dafny sources". No `dafny` package appears in npm search. UNVERIFIED (one search summary) / CONFIRMED (registry absence) — [Dafny reference](https://dafny.org/dafny/DafnyRef/DafnyRef); [dafny2js](https://github.com/metareflection/dafny2js)
- **F\* → JavaScript**: "the JavaScript backend's branch is not up-to-date with master". To run F* code today "one needs to translate it to OCaml or F#". Background: POPL 2013 "Fully Abstract Compilation to JavaScript". UNVERIFIED (one search summary; result list includes the F* wiki and GitHub issue #332) — [F* wiki: Executing F* code](https://github.com/FStarLang/FStar/wiki/Executing-F*-code); [issue #332](https://github.com/FStarLang/FStar/issues/332)
- **LemmaScript (Midspiral)**, the closest 2026 prior art to "contract-checked TypeScript":
  - npm description: "A verification toolchain for TypeScript — generates Lean 4 or Dafny from annotated TS". First published 2026-04-01; 36 versions. CONFIRMED — [registry lemmascript](https://registry.npmjs.org/lemmascript)
  - The README (read from the packument): "Write ordinary TypeScript with `//@ ` specification annotations"; it is a "**Tech Preview**: the core idea is there, but support, semantics, and ergonomics are still evolving". The Dafny backend's LLM- or user-edited `foo.dfy` must differ from the generated `foo.dfy.gen` by "additions-only". A reusable GitHub Actions workflow "fails the build if any committed generated file is out of date". CONFIRMED
  - Case studies, all in the vendor's own repos: brownfield, in-place verification of Hono's security middleware ("Four CVEs covered", including CVE-2026-39409, an IP-restriction bypass), opencode's permission system and patch parser, Infisical's "privilege-escalation guard" glob check, xyflow utilities, rallly's `validateRedirectUrl`, balanced-match, and the pi and Flue agent harnesses. Greenfield apps include one with "123 Dafny lemmas" and a "16-conjunct invariant". CONFIRMED (README text; not independent adoption)
  - Midspiral's site title: "The correctness layer for AI-generated software". The launch post dates to about April 2026 and was discussed on Lobsters. SNIPPET-CONFIRMED (result list includes the Midspiral blog, lemmascript.com, E-Ink News, Lobsters and jsDelivr) — [Midspiral blog](https://midspiral.com/blog/lemmascript-a-verification-toolchain-for-typescript/); [Lobsters](https://lobste.rs/s/4tuujf/lemmascript_verification_toolchain_for)
- Gleam popularity signal: "In the 2025 Stack Overflow Developer Survey, Gleam scored 70% admiration, landing second only to Rust's 72%". UNVERIFIED (one search summary) — [Pullflow](https://www.pullflow.com/blog/gleam-functional-language-developers-actually-want-to-use/)

### Inferences
- Sound compile-to-JS languages have been available for a decade and have stayed at tens of thousands of weekly downloads. Soundness alone does not pull developers off TypeScript. The ones that survive trade JS-interop breadth for soundness (ReScript's "curated subset", Elm's closed world).
- LemmaScript shows that the 2026 niche for "verified TypeScript for AI-written code" is real enough for a startup, and occupied. Its design keeps TypeScript as the source language and adds comment contracts, the opposite of a new language. It targets deep functional correctness via external provers; it does not target runtime boundary checks or erasure. A new language would need to beat it on boundary safety (generated runtime checks at JSON/FFI edges), on non-weakenable contracts, and on not needing Dafny or Lean installed.
- The distribution lesson: ReScript, Elm and PureScript ship compilers through npm and emit plain JS. Gleam and Dafny do not use npm for the compiler, which is one reason they are invisible in npm data. A new language aimed at npm should ship its compiler as an npm package, emit `.js` plus `.d.ts`, and ideally be callable from `tsc`-based toolchains.

### Gaps
- Company-level adoption for ReScript, Elm, PureScript or Gleam's JS target in 2025–2026 (sites blocked; no survey figures found).
- Gleam 2025 survey results.
- Independent (non-vendor) users of LemmaScript.

---

## 6. Notable 2024–2026 incidents traced to type confusion or unvalidated input in TypeScript/JavaScript, and what post-mortems recommended

### Takeaway
n8n is the case study: a TypeScript `string` annotation on a sanitizer input was trusted but never checked, and the result was a critical RCE (CVE-2026-25049). It sits inside a longer 2026 run of n8n expression-sandbox escapes: 14 sandbox/expression escapes and 11 prototype-pollution advisories among 149 advisories in total. Beyond n8n, 2026 advisories show the same "non-string where a string was assumed" pattern in tmp (path traversal), handlebars (three AST type-confusion code-injection advisories, one CVSS 9.8), sequelize (CVSS 9.9) and node-tar. Prototype pollution is common: 51 advisories in a 51-package sample, 14 in axios. The fixes and recommendations are all runtime: add explicit type checks, cover more AST shapes, and isolate execution (n8n's external task runners). None proposes static types as the fix.

### Cited Findings

**n8n CVE-2026-25049 (GHSA-6cqr-8cfr-67f8)**
- npm audit metadata: "n8n Has Expression Escape Vulnerability Leading to RCE", severity critical, **CWE-913** (not CWE-843), affected `>=2.0.0 <2.5.2` and `<1.123.17`. CONFIRMED — npm audit endpoint; [GHSA-6cqr-8cfr-67f8](https://github.com/advisories/GHSA-6cqr-8cfr-67f8)
- Root cause, quoted identically by several outlets: "the sanitization function assumes keys in property accesses are strings in attacker-controlled code. This assumption is correctly encoded in a TypeScript annotation, but it is not enforced through runtime type checking." Also: "Even when developers correctly annotate the sanitizer's input parameter as a string, attackers can still pass non-string values at runtime, thereby bypassing the security controls." The fix was "implementing proper runtime type checking in the sanitization functions and expanding AST coverage to handle destructuring patterns". SNIPPET-CONFIRMED (three searches; result lists include Endor Labs, OPSWAT, SOCRadar, Fidelis, Tenable, The Hacker News, PurpleOps and hetmehta.com) — [Endor Labs](https://www.endorlabs.com/learn/cve-2026-25049-n8n-rce); [OPSWAT](https://www.opswat.com/blog/cve-2026-25049-expression-sandbox-escape-leading-to-remote-code-execution-in-n8n); [hetmehta.com, "How TypeScript Types Failed n8n's Security"](https://hetmehta.com/posts/n8n-type-confusion-rce); [SOCRadar](https://socradar.io/blog/cve-2026-25049-n8n-expression-escape/)
- Reporters: about 10 credited, including Endor Labs' Cris Staicu (a TypePatrol co-author). UNVERIFIED (section 1)
- Earlier January 2026 n8n sandbox CVEs: JFrog found "CVE-2026-1470 (rated 9.9 Critical) impacting the expression evaluation engine, and CVE-2026-0863 (rated 8.5 High) affecting Python execution in the Code node". UNVERIFIED (one search summary) — [JFrog Security Research](https://research.jfrog.com/post/achieving-remote-code-execution-on-n8n-via-sandbox-escape/)
- n8n's structural response: "task runners can run in external mode, launching as separate containers providing a fully isolated environment to execute the JavaScript defined in the Code node". An n8n blog adds that "runtime isolation alone isn't enough—workflow-level controls, credential scoping, and layered enforcement are what keep agents safe when any single layer fails". A security write-up concludes that "blocking one execution backend or one escape technique does not remove the risk; it only shifts the attack surface". UNVERIFIED (one search summary; sources in list) — [n8n docs: Hardening task runners](https://docs.n8n.io/hosting/securing/hardening-task-runners/); [n8n blog: AI agent sandboxes](https://blog.n8n.io/ai-agent-sandbox/); [Smartkeyss](https://www.smartkeyss.com/post/cve-2026-0863-python-sandbox-escape-in-n8n-via-exception-formatting-and-implicit-code-execution)

**n8n's full advisory record** (CONFIRMED — npm audit endpoint, 2026-09-24, n8n versions 1.0.0, 1.123.16 and 2.5.1; deduplicated by GHSA)
- 149 distinct advisories (225 rows before de-duplication across the 1.x and 2.x branches), 23 of them critical.
- 14 are expression or code sandbox escapes or sanitizer bypasses. Several came after CVE-2026-25049: "Expression Sandbox Escape Leads to RCE" (GHSA-vpcf-gvg4-6qwr, CVSS 9.9), "Expression sandbox escape via arrow-function bodies" (GHSA-gv7g-jm28-cr3m), "Expression Sandbox Escape via Class-Field Sanitizer Rebinding" (GHSA-hw8v-xxg5-vvvx), "Expression Sandbox Escape via Shared Builtin Tampering and Code-Printer Injection" (GHSA-6xcw-7xm6-48c6), and "Legacy Expression Evaluator Sanitizer Bypass" (GHSA-pm35-fqvh-cq5g).
- 11 are prototype pollution (CWE-1321), including "Prototype Pollution in XML Webhook Body Parser that Leads to RCE" (GHSA-q5f4-99jv-pgg5, CVSS 10), "HTTP Request Node Pagination Prototype Pollution to RCE" (GHSA-c8xv-5998-g76h) and "XML Node Prototype Pollution Patch Bypass" (GHSA-wrwr-h859-xh2r).
- Only one carries CWE-843 explicitly: "Send Email Node Arbitrary File Read and SSRF via Nodemailer Content-Object Type Confusion" (GHSA-2x35-3fw4-9jr4).
- The largest single classes are conventional: XSS 16, SQL injection 13, incorrect authorization 13.

**Census of 51 popular npm packages** (CONFIRMED — npm audit endpoint, 2026-09-24; every published version submitted; deduplicated by GHSA). This is a convenience sample chosen for this note: tmp, binary-parser, qs, express, body-parser, axios, lodash, undici, ws, jsonwebtoken, multer, formidable, sanitize-html, dompurify, xml2js, fast-xml-parser, handlebars, ejs, pug, next, nuxt, vite, @angular/core, react-dom, vm2, node-fetch, got, cookie, path-to-regexp, semver, minimist, yargs-parser, json5, flatted, mongoose, sequelize, typeorm, knex, pg, mysql2, socket.io, tar, jose, validator, class-transformer, superagent, koa, fastify, hono, @trpc/server, graphql.
- 551 distinct advisories. Top CWEs: XSS 66, **prototype pollution 51**, CWE-400 49, code injection 33, CWE-770 33, CWE-200 28, path traversal 27, **improper input validation (CWE-20) 26**.
- Explicit type confusion:
  - **tmp**: "Type-confusion bypass of `_assertPath` allows path traversal via non-string prefix/postfix/template" (GHSA-7c78-jf6q-g5cm, high, 8.2, affects 0.2.6). A write-up names it CVE-2026-49982 and says the library "relies on the .includes() method without verifying the input type", so arrays or duck-typed objects pass, and "downstream string coercion subsequently restor[es] the traversal sequence". The CVE number and quote are UNVERIFIED (single source) — [CVEReports](https://cvereports.com/reports/CVE-2026-49982)
  - **handlebars**: "JavaScript Injection via AST Type Confusion" in three advisories (GHSA-2w6w-674q-4c4q critical 9.8; GHSA-3mfm-83xf-c92r and GHSA-xhpv-hc6g-r9c6 high 8.1), CWE-843 + CWE-94, affecting `>=4.0.0 <=4.7.8`.
  - **sequelize**: "Unsafe fall-through in getWhereConditions" (GHSA-vqfx-gj96-3w95, critical 9.9, CWE-843).
  - **node-tar**: "Process crash via PAX numeric path type confusion" (GHSA-w8wr-v893-vjvp, moderate, CWE-704).
- Prototype pollution: 51 advisories across 19 packages, 14 in axios. Examples: "Authentication Bypass via Prototype Pollution Gadget in `validateStatus` Merge Strategy", "Invisible JSON Response Tampering via Prototype Pollution Gadget in `parseReviver`", "Full Man-in-the-Middle via Prototype Pollution Gadget in `config.proxy`", and "Patch Bypass: Proxy-Authorization Header Injection via Prototype Pollution — Incomplete Null-Prototype Fix".
- Dating is an inference. The endpoint gives no dates, but its numeric IDs rise over time. n8n's February 2026 advisory has ID 1112923. On that basis, 25 of the 51 prototype-pollution advisories, and all the tmp, handlebars, sequelize and tar type-confusion advisories (IDs 1115538–1123939), are probably from 2026.
- Remediation guidance (general): "If one of these methods is used to validate user input, the validation could be prone to a potential bypass if an input of different type is provided (for example an array) and no checks are performed." UNVERIFIED (search summary of Snyk material) — [Snyk blog](https://snyk.io/blog/remediate-javascript-type-confusion-bypassed-input-validation/); [Snyk Learn](https://learn.snyk.io/lesson/type-confusion/)
- LemmaScript's README lists four Hono CVEs it "covers" by verifying the middleware (IP restriction bypass CVE-2026-39409 and others). This shows vendors using 2026 CVEs to market verification of TypeScript. CONFIRMED (README) — [registry lemmascript](https://registry.npmjs.org/lemmascript)

### Inferences
- Every recommended fix is a runtime check or runtime isolation. That is what a language with checked boundaries would generate automatically: a declared `string` parameter on a function reachable from untrusted input compiles to a guard. n8n's fix ("proper runtime type checking in the sanitization functions") is exactly the code such a compiler would emit.
- n8n shows the limit of patching bypass by bypass: after 25049 came class-field rebinding, arrow-function bodies, builtin tampering and a legacy evaluator bypass. Sandboxing JavaScript by sanitizing an AST is a losing game that no type system fixes. The type-confusion part is one link in the chain, not the whole chain. A new language would remove the "annotation trusted but not enforced" link; it would not make n8n's sandbox safe by itself.
- Prototype pollution (JSON and object input merged into objects without shape checks) is the most frequent boundary-typing class in the sample. A language whose records are closed and whose JSON decoding is schema-checked removes it by construction. That is a concrete, countable selling point.

### Gaps
- Dates and CVE numbers for the census advisories (NVD, OSV and GitHub advisory pages blocked). IDs are used only as a recency proxy.
- Primary post-mortems from n8n (blog and docs blocked).
- No ecosystem-wide count of 2024–2026 npm advisories whose root cause is an unenforced TypeScript annotation. It would need advisory text, not CWE labels.
