# Gap fill: conference award lists, themes, and checks on flagged 2025–2026 claims

As of 24 September 2026. This file checks the flagged claims in `frontier_pl_research.md` (all of it) and section 4 of `performance_and_gpu.md`, then adds the award lists and themes those notes were missing. 2025–2026 items come first. Older items are marked [BG].

**How this was done, and its limits (read first):**
- **Network.** The egress proxy still blocks every conference, publisher and preprint host. I re-probed on 24 Sep 2026, and all of these returned `000`: arxiv.org, dl.acm.org, usenix.org, conf.researchr.org, pldi26.sigplan.org, mlsys.org, openreview.net, sigops.org, the asplos-conference.org and cgo.org sites, conferences.i-cav.org, leodemoura.github.io, veralang.dev, developer.nvidia.com, and every university news host I tried. Only **www.microsoft.com** and **pypi.org** gave primary-source reads. So "CONFIRMED" below means I opened one of those pages.
- **Search tool.** It returns a machine-written summary of the results plus a list of result URLs, not raw page snippets. Where I "quote" a search result, I am quoting that summary text. **SNIPPET-CONFIRMED** means that two or more separate queries, or two or more independent result pages, gave the same fact.
- **Search budget.** The session-wide web-search cap (200 calls) ran out partway through this task, and later queries were refused. Items I had not searched by then (Exo 2, Descend, AlphaVerus, the vericoding benchmark, type-constrained decoding, de Moura's essay, Qualcomm–Modular, the PLDI 2025, POPL 2026 and SPLASH 2025 keynotes) are marked **COULD NOT VERIFY**. They are not refuted. A later session with open network access should re-check them first.
- **GitHub.** I did not read any GitHub repository (session rule). Vera was checked through its PyPI package page instead.

---

## Q1. Verification ledger for the flagged claims

### Takeaway
Rows 1–28 below are flagged claims. Rows 29–30 are old "Gaps" that this file partly fills.
- **6 are CONFIRMED** from a page I read in full: Lahiri's position paper, VeruSAGE, AutoVerus at OOPSLA 2025, the SymCrypt blog's date and content, the Mojo 1.0 release date, and Vera's core design (from PyPI).
- **4 are SNIPPET-CONFIRMED**: the PLDI 2025 Distinguished Papers, HipKittens at MLSys 2026, Mirage at OSDI 2025, and FlashInfer's MLSys 2025 Best Paper award.
- **1 is CORRECTED**: the PLDI 2026 Distinguished Paper list in the notes is incomplete. It has at least seven papers, not five, and one author list was cut short.
- **17 are COULD NOT VERIFY.** Among the load-bearing ones, the SymCrypt technical-report figures (16.7 KLOC Rust / 237 KLOC Lean) are *not* in the MSR blog, and the blog does not link a technical report. The vericoding numbers and the de Moura essay URL conflict remain unverified.

### Cited Findings

**Ledger** (status, evidence, and what changed)

| # | Claim in the old notes | Status | Evidence (URL actually used) and wording |
|---|---|---|---|
| 1 | PLDI 2025: 6 of 89 accepted papers got Distinguished Paper (DP); DPs include AWDIT, Tree Borrows, Destabilizing Iris | **SNIPPET-CONFIRMED** | [Aarhus Univ. news](https://cs.au.dk/news-events/news/show-news/artikel/distinguished-paper-award-at-pldi-2025): "Only 6 papers were given the Distinguished Paper Award at PLDI 2025 out of 89 accepted papers"; AWDIT by "Lasse Møldrup and Andreas Pavlogiannis". [Saarland Informatics Campus](https://saarland-informatics-campus.de/en/piece-of-news/derek-dreyer-and-collaborators-receive-three-distinguished-paper-awards-at-pldi25-and-popl25/) lists Tree Borrows and Destabilizing Iris. [MPI-SWS news](https://mpi-sws.org/news/2026/) summary: "Dreyer and his collaborators received Distinguished Paper Awards for two papers at PLDI 2025 and one at POPL 2025." The other 3 PLDI 2025 DPs were not found. |
| 2 | PLDI 2025 keynote by Leonardo de Moura ("Lean: Machine-Checked Mathematics…") | **COULD NOT VERIFY** | A search for PLDI 2025 keynotes returned only [pldi25.sigplan.org](https://pldi25.sigplan.org/), with no speaker names in the result. The notes' source is an X post, which is blocked. |
| 3 | Type-constrained decoding (Mündler et al.), PLDI 2025 | **COULD NOT VERIFY** | The search budget ran out before this query. pldi25.sigplan.org and dl.acm.org are blocked. |
| 4 | PLDI 2026 DPs = VerusBelt; Undef in LLVM IR; Typed Perspectives; Categorical Semantics of Probabilistic Symbolic Execution; Verification of Recursively Defined Quantum Circuits (Ying) | **CORRECTED**: the list is incomplete, and one author list is incomplete | **Old:** 5 papers, with the quantum paper credited to "Ying". **New:** at least **7** DPs. Add "**Synthesizing Backward Error Bounds, Backward**" (Laura Zielinski, Justin Hsu; Cornell) and "**Cobble: Compiling Block Encodings for Quantum Computational Linear Algebra**" (Charles Yuan). The quantum-circuits paper is by **Mingsheng Ying and Zhicheng Zhang**. Sources for the two additions: a query summary citing the [pldi26 track](https://pldi26.sigplan.org/track/pldi-2026-papers) said "Synthesizing Backward Error Bounds by Laura Zielinski and Justin Hsu – This paper was also listed among the distinguished papers", and a second query said it "has been selected for a Distinguished Paper Award" ([ACM DL 10.1145/3808333](https://dl.acm.org/doi/10.1145/3808333); [arXiv 2604.15633](https://arxiv.org/abs/2604.15633); [Justin Hsu](https://www.justinhsu.net/)). For Cobble, one summary said "Cobble … by Charles Yuan among the distinguished papers", and another said "The paper received a Distinguished Paper Award … published in PLDI 2026 in June 2026" ([pldi26 details](https://pldi26.sigplan.org/details/pldi-2026-papers/12/Cobble-Compiling-Block-Encodings-for-Quantum-Computational-Linear-Algebra); [ACM DL 10.1145/3808255](https://dl.acm.org/doi/10.1145/3808255); [arXiv 2511.01736](https://arxiv.org/abs/2511.01736)). Source for the author fix: [pldi26 details page](https://pldi26.sigplan.org/details/pldi-2026-papers/30/Verification-of-Recursively-Defined-Quantum-Circuits), via the summary "by Mingsheng Ying and Zhicheng Zhang". The original five are SNIPPET-CONFIRMED by [Saarland Informatics Campus](https://saarland-informatics-campus.de/en/piece-of-news/mpi-researchers-receive-distinguished-paper-award-at-pldi-2026/) and repeated query summaries. [Holtzen](http://sholtzen.dev/articles/pldi26-reflection.html) wrote: "The highlight of the trip was John Li's and Jack Czenszak's distinguished paper award for the symbolic sets paper." The total DP count for PLDI 2026 is still unknown. |
| 5 | POPL 2026 DP winners "not found" | **COULD NOT VERIFY** (still not found) | Six queries returned only the 10% rule ([POPL 2026 track](https://popl26.sigplan.org/track/POPL-2026-popl-research-papers)). One summary wrongly said POPL 2026 "hasn't taken place yet", so the search index looks stale for this venue. |
| 6 | Vericoding benchmark presented at the Dafny 2026 workshop at POPL 2026 | **COULD NOT VERIFY** | Not reached before the search cap. arXiv and popl26 are blocked. |
| 7 | Lean-egg at POPL 2026 | **COULD NOT VERIFY** | Not searched (budget). |
| 8 | OOPSLA 2025 DP: "Incremental Bidirectional Typing via Order Maintenance" | **COULD NOT VERIFY** (one snippet source only) | [U. Michigan CSE](https://cse.engin.umich.edu/stories/cse-researchers-win-distinguished-paper-award-at-oopsla-2025) summary: "honored for advancing type-checking for live programming environments". That is one source, so it does not meet the two-source rule, but nothing contradicts it. |
| 9 | AutoVerus published at OOPSLA 2025; proves >90% of benchmark tasks, over half in <30 s or 3 LLM calls | **CONFIRMED** | [MSR publication page](https://www.microsoft.com/en-us/research/publication/autoverus-automated-proof-generation-for-rust-code/) (read). Venue: "Object-Oriented Programming, Systems, Languages & Applications (OOPSLA) 2025"; date October 2025. The page says AutoVerus "can automatically generate correct proof for more than 90% of them, with more than half of them tackled in less than 30 seconds or 3 LLM calls." The notes' "Vol. 9, Article 396" was not checked (ACM blocked). |
| 10 | ICFP 2025 keynotes (Dimoulas, Komendantskaya, Singh); SPLASH 2025 keynotes (Lawall, Piessens, Su) | **COULD NOT VERIFY** | Not reached before the search cap. |
| 11 | Lahiri, "Intent Formalization: A Grand Challenge for Reliable Coding in the Age of AI Agents", MSR, March 2026, arXiv 2603.17150 | **CONFIRMED** | [MSR publication page](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/) (read). Sole author Shuvendu Lahiri; "March 2026"; type arXiv; link `arxiv.org/abs/2603.17150`. Quotes: "intent formalization — the translation of informal user intent into a set of checkable formal specifications — is the key challenge"; "The central bottleneck is validating specifications: since there is no oracle for specification correctness other than the user, we need semi-automated metrics". Open challenges, verbatim: "scaling beyond benchmarks, achieving compositionality over changes, metrics for validating specifications, handling rich logics, designing human-AI specification interactions". This is a preprint, not a peer-reviewed venue paper. |
| 12 | Vericoding benchmark (arXiv 2509.22908): 12,504 specs; Dafny 82% / Verus 44% / Lean 27%; Dafny 68%→96% in a year; venue (ICLR 2026? Dafny 2026 workshop?) | **COULD NOT VERIFY** (all sub-claims) | arXiv, openreview and popl26 are blocked, and the search cap was hit before this query. An MSR site search for "vericoding" returned 0 results. The numbers stay as reported in the old notes and should be treated as unverified. |
| 13 | VeruSAGE (arXiv 2512.18436): 849 tasks, >80% solved | **CONFIRMED** | [MSR publication page](https://www.microsoft.com/en-us/research/publication/verusage-a-study-of-agent-based-verification-for-rust-systems/) (read). Authors Chenyuan Yang, Natalie Neamtu, Chris Hawblitzel, Jay Lorch, Shan Lu; December 2025. Quotes: "849 proof tasks extracted from eight open-source Verus-verified Rust systems"; models o4-mini, GPT-5, Sonnet 4, Sonnet 4.5; "the best LLM-agent combination in our study completes over 80% of system-verification tasks"; "over 90% of a set of system proof tasks not part of VeruSAGE-Bench because they had not yet been finished by human experts." No peer-reviewed venue is shown; treat it as a preprint. |
| 14 | MSR SymCrypt/Aeneas/Lean blog, July 2026 | **CONFIRMED** | [MSR blog](https://www.microsoft.com/en-us/research/blog/verifying-rust-cryptography-in-symcrypt-from-standards-to-code/) (read): dated **13 July 2026**; authors Son Ho, Cédric Fournet, Antoine Delignat-Lavaud, Samuel Lee, Jason Fisher, Jessica Krynitsky. Quotes: "This first release includes complete proofs for the Rust ML-KEM and SHA3 code that is being used in insiders builds of Windows today"; planned: "verified Rust code for, e.g., AES-GCM, FrodoKEM, and ML-DSA"; "agents help translate standards into Lean specifications" and "agents help write and maintain proofs". Links in the body: an earlier [SymCrypt-in-Rust blog](https://www.microsoft.com/en-us/research/blog/rewriting-symcrypt-in-rust-to-modernize-microsofts-cryptographic-library/), lean-lang.org, the Aeneas repository, and a SymCrypt `feature/verifiedcrypto` branch. |
| 15 | SymCrypt technical report: 16.7 KLOC Rust, 237 KLOC Lean (arXiv 2609.15648) | **COULD NOT VERIFY**, and **the attribution is weaker than the notes imply** | I read the blog twice. It gives **no KLOC figures** and **does not link any technical report or arXiv paper**; its body links are listed in row 14. The figures rest only on the blocked arXiv page. The report must not present "about 14 lines of Lean per line of Rust" as confirmed. |
| 16 | de Moura, "When AI Writes the World's Software, Who Verifies It?", 28 Feb 2026. The notes give two URLs: `…/blog/2026/02/28/when-ai-writes-the-worlds-software.html` (frontier_pl_research.md) and `…/blog/2026-2-28-when-ai-writes-the-worlds-software-who-verifies-it/` (ai_code_generation.md) | **COULD NOT VERIFY** (URL conflict unresolved) | leodemoura.github.io is blocked (curl `000`), and the search cap was reached before this query. Neither URL could be tested, so the report should cite the essay by title and date and flag the URL. The quoted figures (25–30% AI-written code at Google/Microsoft; "95% by 2030") are also unverified here. |
| 17 | Vera (aallan/vera), a language designed for LLMs to write | **CONFIRMED** (core design) / **COULD NOT VERIFY** (benchmarks and details) | [PyPI `veralang`](https://pypi.org/project/veralang/) (read through the PyPI JSON API). Summary: "Vera: a programming language designed for LLMs, with full contracts, algebraic effects, and typed slot references". Description: "designed for large language models to write. It has mandatory contracts, algebraic effects, typed slot references instead of variable names, and a compiler that emits WebAssembly. Contracts are verified statically with Z3 where possible, and SQL injection is a compile-time error." Author Alasdair Allan; licence MIT; latest version **0.1.13 (uploaded 21 Aug 2026)**; PyPI releases run 0.1.5 (17 Jul 2026) to 0.1.13; an LSP extra exists. The example code uses `@Int.0`/`@Int.1` slot references with `requires`/`ensures`/`effects(pure)`. **Not verified:** "2,000+ commits", the three-tier verification design and Tier 2 being unimplemented, VeraBench (60 problems; "six of nine models write 100% correct Vera"), and WASI P2/browser/HTTP targets. Those claims come from the GitHub README, which I could not read. Everything is still self-reported and not peer-reviewed. |
| 18 | Descend (PLDI 2024), safe GPU programming | **COULD NOT VERIFY** | Search cap reached; ACM and arXiv blocked. [BG] |
| 19 | HipKittens at MLSys 2026 | **SNIPPET-CONFIRMED** | Result titles: "[MLSys Oral HipKittens: Fast and Furious AMD Kernels](https://mlsys.org/virtual/2026/oral/3735)"; [slides PDF](https://mlsys.org/media/mlsys-2026/Slides/3735.pdf); session "[Research Track Oral Presentation: Compilers and Kernels](https://mlsys.org/virtual/2026/session/3718)". Summary: "HK kernels compete with AMD's hand-optimized assembly kernels for GEMMs and attention, and consistently outperform compiler baselines." It was an oral presentation, and I saw no evidence of an award. |
| 20 | Mirage: A Multi-Level Superoptimizer for Tensor Programs (OSDI 2025) | **SNIPPET-CONFIRMED** | [USENIX presentation page](https://www.usenix.org/conference/osdi25/presentation/wu-mengdi); [USENIX PDF](https://www.usenix.org/system/files/osdi25-wu-mengdi.pdf); [ACM DL](https://dl.acm.org/doi/10.5555/3767901.3767914); [YouTube "OSDI '25 - Mirage…"](https://www.youtube.com/watch?v=CKrQsMHUh8M). Summary: "Mirage outperforms existing approaches by up to 3.3× even for DNNs that are widely used and heavily optimized"; it uses "µGraphs … at the kernel, thread block, and thread levels". The notes' "1.2×–6.7×" figure belongs to the separate MPK megakernel work (arXiv 2512.22219, "MPK: A Compiler and Runtime for Mega-Kernelizing Tensor Programs"; title only). Keep the two numbers apart. |
| 21 | Exo 2: Growing a Scheduling Language (ASPLOS 2025) | **COULD NOT VERIFY** | The query was refused at the search cap. The only indirect evidence: the notes' DOI 10.1145/3669940.3707218 has the prefix of the ASPLOS 2025 Vol. 1 proceedings ([dl.acm.org/doi/proceedings/10.1145/3669940](https://dl.acm.org/doi/proceedings/10.1145/3669940), seen in a result list). That is consistent with the claim but does not confirm it. |
| 22 | AlphaVerus (ICML 2025) | **COULD NOT VERIFY** | Search cap reached; openreview blocked. |
| 23 | FlashInfer won an MLSys 2025 best paper award | **SNIPPET-CONFIRMED (yes)** | [Zihao Ye on X](https://x.com/ye_combinator/status/1922421121657549249) (title): "We're thrilled that FlashInfer won a Best Paper Award at MLSys 2025!" [UW Allen School news, 1 Jul 2025](https://news.cs.washington.edu/2025/07/01/allen-school-researchers-receive-best-paper-award-for-speeding-up-llm-performance-with-flashinfer) (title): "allen school researchers receive best paper award for speeding up llm performance with flashinfer". Result lists also included [MLSys 2025 Awards](https://mlsys.org/virtual/2025/awards_detail) and [UW Award Papers](https://www.cs.washington.edu/research/award-papers/). One summary called it an "Outstanding Paper Award"; the primary-author wording is "Best Paper Award". Paper: "FlashInfer: Efficient and Customizable Attention Engine for LLM Inference Serving" (Ye, Chen, Lai, Lin, Zhang, Wang, Chen, Kasikci, Grover, Krishnamurthy, Ceze). Results per summary: "29-69% inter-token-latency reduction compared to compiler backends … 28-30% latency reduction for long-context inference, and 13-17% speedup for LLM serving with parallel generation." |
| 24 | Mojo 1.0 released 11 Aug 2026 | **CONFIRMED** | [PyPI `mojo` JSON](https://pypi.org/pypi/mojo/json) and [1.0.0 JSON](https://pypi.org/pypi/mojo/1.0.0/json) (read). The `mojo-1.0.0` wheels were uploaded **2026-08-11T14:33Z** (macOS arm64, Linux aarch64 and x86_64). Before that came 1.0.0b1 (2026-05-07) and 1.0.0b2 (2026-06-18); 1.1.0 followed on 2026-09-17. Author field: "Modular Inc". |
| 25 | Mojo compiler open-sourced 18 Aug 2026 (Apache-2.0 with LLVM exceptions) | **COULD NOT VERIFY** (one caution) | modular.com, mojolang.org and docs.modular.com are blocked, and the search cap was reached. **Caution:** every Modular wheel on PyPI (mojo 1.1.0 from 17 Sep 2026, mojo-compiler 1.1.0, modular 26.6.0, max 26.6.0) still carries the licence label `LicenseRef-MAX-Platform-Software-License`, not Apache-2.0. That label covers the binary wheels, so it does not disprove a source release, but the report should not state the licence as confirmed. PyPI lists "Source: https://github.com/modular/modular/tree/main/mojo" and new docs at mojolang.org. |
| 26 | Qualcomm acquired Modular (~$3.9B; announced 24 Jun 2026; closed 29 Jul 2026) | **COULD NOT VERIFY** | Nasdaq, PR Newswire, sec.gov and qualcomm.com are all blocked, and the search cap was reached. PyPI metadata still names "Modular Inc" as author, which is consistent with the notes' claim that the Modular brand continues but proves nothing about the deal. |
| 27 | FlashInfer, TileLang, KernelBench and the 2026 arXiv GPU preprints (Section 4 of performance_and_gpu.md) | **COULD NOT VERIFY** (except row 23) | Search cap reached. |
| 28 | CGO 2025 DialEgg | **COULD NOT VERIFY** (not re-searched) | The CGO 2025 and CGO 2026 award lists were not found either (see Q2). |
| 29 | PLDI 2026 keynote list "incomplete" (gap) | **Filled in part** (new finding) | See Q4: Saman Amarasinghe gave the opening keynote. |
| 30 | CGO/CC/CAV award lists missing (gap) | **Filled in part** (CAV 2025; see Q2) | CGO and CC are still missing. |

### Inferences
- The best-supported verification claims in the notes are the Microsoft ones (Lahiri, AutoVerus, VeruSAGE, SymCrypt), because www.microsoft.com is reachable. The weakest are the non-Microsoft numbers that the recommendation leans on: vericoding's 82/44/27 split, the SymCrypt 14:1 proof ratio, and de Moura's AI-share figures. The final report should either mark these unverified or re-check them in a session with open network access.
- The PLDI 2026 correction matters for the theme analysis. The two missing DPs are both numerical or quantum work. That makes PLDI 2026 lean even more toward quantum, probabilistic and numerical topics than the notes implied (see Q5).

### Gaps
- None of the COULD NOT VERIFY rows above was refuted; they are simply unchecked. Priority for a re-run: vericoding (row 12), SymCrypt TR figures (15), de Moura URL (16), Qualcomm–Modular and the Mojo licence (25–26), type-constrained decoding (3), Exo 2 (21).

---

## Q2. Award lists and themes at the PL and verification venues, 2025–2026

### Takeaway
The recoverable 2025–2026 PL award lists cluster in four areas: **foundations for verifying real code** (Rust aliasing, Iris, Verus type-system semantics), **compiler IR semantics** (removing `undef` from LLVM), **typed GPU programming**, and a strong **quantum, probabilistic and numerical** group that makes up 4 of the 7 known PLDI 2026 DPs. Language usability shows up too: live incremental typing (OOPSLA 2025) and a diagramming DSL (ECOOP 2025). **I found no DP-winning paper in 2025–2026 on LLMs writing code**, although the topic has a keynote, a workshop and several OOPSLA papers. The lists are partial: POPL 2026, ICFP 2025, CGO and CC winners could not be recovered.

### Cited Findings

**PLDI 2026** (Boulder, Colorado; main conference 17–19 June 2026, per [PLDI 2026 attendee info](https://pldi26.sigplan.org/attending/Information-for-Attendees))
- Selection rule: "In 2026, distinguished papers were chosen via nominations and votes from the review committee, with the review committee chair and general chair making final decisions"; cap of 10% — [PLDI 2026 track](https://pldi26.sigplan.org/track/pldi-2026-papers)
- Known Distinguished Papers (7; the list may still be incomplete) — [Saarland Informatics Campus](https://saarland-informatics-campus.de/en/piece-of-news/mpi-researchers-receive-distinguished-paper-award-at-pldi-2026/); [PLDI 2026 track](https://pldi26.sigplan.org/track/pldi-2026-papers); [Holtzen reflections](http://sholtzen.dev/articles/pldi26-reflection.html) [search summaries]:
  1. "VerusBelt: A Semantic Foundation for Verus's Proof-Oriented Extensions to the Rust Type System" (Hance, Elbeheiry, Dreyer, Matsushita)
  2. "Towards Removing Undef Values From LLVM IR" (Lobo, McIver, Mitenkov, Lee, Sundararajah, Lopes)
  3. "Modular GPU Programming with Typed Perspectives" (Bansal, Sainati, Cutler, Amarasinghe, Ragan-Kelley)
  4. "Categorical Semantics of Probabilistic Symbolic Execution" (Li, Czenszak, Holtzen): "develops symbolic sets as a new semantic domain" — [pldi26 details](https://pldi26.sigplan.org/details/pldi-2026-papers/100/Categorical-Semantics-of-Probabilistic-Symbolic-Execution)
  5. "Verification of Recursively Defined Quantum Circuits" (Mingsheng Ying, Zhicheng Zhang): "a proof system … with soundness and relative completeness established" — [pldi26 details](https://pldi26.sigplan.org/details/pldi-2026-papers/30/Verification-of-Recursively-Defined-Quantum-Circuits)
  6. **(new)** "Synthesizing Backward Error Bounds, Backward" (Laura Zielinski, Justin Hsu; Cornell). A "formal framework that enables sound, automated backward error analysis for a broad class of numerical programs", with a category "Shel" and a tool "eggshel" — [ACM DL](https://dl.acm.org/doi/10.1145/3808333); [arXiv 2604.15633](https://arxiv.org/abs/2604.15633). It follows "Bean: A Language for Backward Error Analysis" ([ACM DL 10.1145/3729324](https://dl.acm.org/doi/10.1145/3729324); title from a result list only).
  7. **(new)** "Cobble: Compiling Block Encodings for Quantum Computational Linear Algebra" (Charles Yuan). A "language for programming with quantum computational linear algebra" that compiles block encodings to circuits, with "2.6x-25.4x speedups" over an unoptimized baseline — [pldi26 details](https://pldi26.sigplan.org/details/pldi-2026-papers/12/Cobble-Compiling-Block-Encodings-for-Quantum-Computational-Linear-Algebra); [arXiv 2511.01736](https://arxiv.org/abs/2511.01736)
- Aarhus reported six of its papers accepted at PLDI 2026 — [Aarhus news](https://cs.au.dk/news-events/news/show-news/artikel/six-papers-accepted-at-pldi-2026) [title only]

**PLDI 2025** (Seoul, 18–20 June 2025 — [PLDI 2025](https://pldi25.sigplan.org/))
- 6 DPs out of 89 accepted papers. Known winners: AWDIT (Møldrup, Pavlogiannis), Tree Borrows (Villani, Hostert, Dreyer, Jung) and Destabilizing Iris (Spies et al.) — [Aarhus](https://cs.au.dk/news-events/news/show-news/artikel/distinguished-paper-award-at-pldi-2025); [Saarland](https://saarland-informatics-campus.de/en/piece-of-news/derek-dreyer-and-collaborators-receive-three-distinguished-paper-awards-at-pldi25-and-popl25/) [SNIPPET-CONFIRMED]
- Pavel Panchekha's blog post "Distinguished (for me) Papers of PLDI'25" is a personal list, *not* the award list; do not cite it as one — [pavpanchekha.com](https://pavpanchekha.com/blog/pldi25.html)

**POPL 2026** (Rennes, France; January 2026)
- The DP list was not recovered (see ledger row 5).
- **2026 Most Influential POPL Paper Award** (for POPL 2016): "Dependent Types and Multi-monadic Effects in F*", Nikhil Swamy (Microsoft), Cătălin Hrițcu (MPI-SP) and co-authors — [MPI-SP news](https://www.mpi-sp.org/94694/news_publication_26021689_transferred) [search summary; two queries agree]
- SIGPLAN's **2026 Distinguished Service Award** went to Derek Dreyer. MPI-SWS had 5 papers at POPL 2026, "the ninth year in a row" with 5+ — [MPI-SWS news 2026](https://mpi-sws.org/news/2026/) [search summary]

**OOPSLA 2025** (SPLASH, Singapore)
- DP: "HeapBuffers: Why Not Just Using a Binary Serialization Format for Your Managed Memory?" (Bonetta, VU Amsterdam; Löff, Basso, Binder, USI). It is about serialization and deserialization cost in managed languages and VMs — [VU Amsterdam](https://vu.nl/en/news/2025/distinguished-paper-award-at-oopsla-2025-for-daniele-bonetta) [single snippet source]
- DP: "Incremental Bidirectional Typing via Order Maintenance" (Porter, Wei, Kirisame, Panchekha, Omar) — [U. Michigan](https://cse.engin.umich.edu/stories/cse-researchers-win-distinguished-paper-award-at-oopsla-2025) [single snippet source]
- An "OOPSLA Awards" session page exists — [SPLASH 2025](https://2025.splashcon.org/details/OOPSLA/218/OOPSLA-Awards). The full list was not recovered.

**ICFP 2025** (co-located with SPLASH 2025, Singapore)
- ICFP 2025 had a Distinguished Papers Committee — [ICFP 2025 committee page](https://icfp25.sigplan.org/committee/icfp-2025-distinguished-papers-committee). The winners were not recovered.

**ECOOP 2025** (Bergen)
- DP and Distinguished Artifact: "Cope and Drag" (CnD), "a novel lightweight diagramming language" whose design is "driven by cognitive science principles" (Prasad, Greenman, Nelson, Krishnamurthi) — [Brown CS awards](https://awards.cs.brown.edu/2025/07/16/brown-university-programming-languages-team-receives-ecoops-distinguished-paper-and-distinguished-artifact-awards/)
- Test of Time Award (ECOOP 2005 paper): "Towards type inference for JavaScript" (Anderson, Giannini, Drossopoulou) — [ECOOP 2025 awards track](https://2025.ecoop.org/track/ecoop-2025-awards) [search summary]

**CAV 2025** (Zagreb, 23–25 July 2025)
- DP: Fatmi, Kiefer, Parker (Oxford) and van Breugel (York U.). The paper "develops a new method for simplifying probabilistic models in formal verification" — [Oxford CS news](https://www.cs.ox.ac.uk/news/2471-full.html)
- DP: "Introducing Certificates to the Hardware Model Checking Competition" (Froleyks, Yu, Preiner, Biere, Heljanko) — [JKU research portal](https://research.jku.at/en/prizes/cav-distinguished-paper-award)
- CAV Award 2025 (lifetime-style award): Armoni, Beer, Ben-David, Eisner, Fisman, Fix, Havlicek, Landver, Miller and Vardi, "for fundamental contributions in designing temporal logics that led to highly successful industry-standard property-specification languages based on temporal logics such as ForSpec, Sugar, PSL, and SVA" — [CAV 2025 award page](https://conferences.i-cav.org/2025/award/); [Rice CS](https://csweb.rice.edu/news/moshe-vardi-receives-2025-computer-aided-verification-cav-award)
- A CAV Facebook post "Congratulations to CAV 2025 distinguished papers" exists, but its contents were not visible — [Facebook](https://www.facebook.com/groups/cavconference/posts/24128751976812843/)

**CGO 2025 / CGO 2026 / CC 2025 / CC 2026**
- CGO 2025: Las Vegas, 1–5 March 2025; up to 10% DPs, open to regular and tool papers — [CGO 2025 track](https://2025.cgo.org/track/cgo-2025-papers). Winners were not found. Paper titles seen: "Synthesis of Sorting Kernels"; "Tensorize: Fast Synthesis of Tensor Programs from Legacy Code using Symbolic Tracing, Sketching and Solving" — [CGO 2025 proceedings TOC](https://www.conference-publishing.com/toc/CGO25)
- CGO 2026: Sydney, 31 Jan – 4 Feb 2026; same 10% rule — [CGO 2026](https://2026.cgo.org/). Winners were not found. Papers seen by title:
  - [QIGen, a kernel generator for inference on nonuniformly quantized LLMs](https://2026.cgo.org/details/cgo-2026-papers/7/QIGen-A-Kernel-Generator-for-Inference-on-Nonuniformly-Quantized-Large-Language-Mode)
  - [Towards Threading the Needle of Debuggable Optimized Binaries](https://2026.cgo.org/details/cgo-2026-papers/30/Towards-Threading-the-Needle-of-Debuggable-Optimized-Binaries)
  - [Multidirectional Propagation of Sparsity Information across Tensor Slices](https://2026.cgo.org/details/cgo-2026-papers/14/Multidirectional-Propagation-of-Sparsity-Information-across-Tensor-Slices)
  - [SkeleShare: Algorithmic Skeletons and Equality Saturation for Hardware Resource Sharing](https://2026.cgo.org/details/cgo-2026-papers/49/SkeleShare-Algorithmic-Skeletons-and-Equality-Saturation-for-Hardware-Resource-Shari)
- CC 2026: the 35th CC, Sydney, 31 Jan – 1 Feb 2026 — [CC 2026](https://conf.researchr.org/home/CC-2026); [proceedings](https://dl.acm.org/doi/proceedings/10.1145/3771775). No award list was found. CC 2025 was not reached before the search cap.

### Inferences
- PLDI 2026's awards split into two groups. One is "trustworthy foundations for real languages and compilers" (VerusBelt, LLVM `undef`, typed GPU perspectives). The other is "specialised semantics" (quantum ×2, probabilistic, numerical). Only the first group maps onto a broad developer pain point.
- The award-winning language-design work is **domain-specific and small**: Cobble for quantum linear algebra, Bean/eggshel for numerical stability, CnD for diagrams, Hazel for live typing. There is no award-winning general-purpose new language. The award signal favours precise semantics for narrow domains, or foundations for existing languages (Rust, LLVM).
- The 2026 Most Influential POPL award to F* ("dependent types and multi-monadic effects") is a ten-year retrospective. It rewards a verification-oriented language whose core idea is *typed effects*, which supports the notes' claim that typed effects plus verification is an established, respected direction that has not shipped in the mainstream.

### Gaps
- Complete DP lists for PLDI 2025 (3 missing), PLDI 2026 (total count unknown), POPL 2026 (none), OOPSLA 2025, ICFP 2025, ECOOP 2025, CAV 2025 (partial), CGO 2025/2026 (none), CC 2025/2026 (none). The cause was blocked sites plus the exhausted search budget.

---

## Q3. Award lists and themes at the systems and ML-systems venues (SOSP, OSDI, ASPLOS, MLSys), 2025–2026

### Takeaway
Systems best papers in 2025–2026 are dominated by two themes: **mechanised correctness for real systems code** (proof-guided eBPF verification, a Rust OS memory manager "with strong correctness guarantees", automated distributed-protocol proofs, invariant learning) and **infrastructure for LLMs** (FlashInfer attention kernels, LLM pre-training data pipelines, vector indexes). E-graphs won at ASPLOS in both years. One OSDI 2026 best paper is about **controlling the side effects of opaque components**, which is essentially an effect-isolation problem.

### Cited Findings

**SOSP 2025** (31st SOSP)
- Best Paper: "Prove It to the Kernel: Precise Extension Analysis via Proof-Guided Abstraction Refinement" (Hao Sun, Zhendong Su; ETH Zurich). Its technique, BCF, "enables the verifier to accept 403 out of 512 real-world eBPF programs that were previously rejected erroneously" — [ETH AST news](https://ast.ethz.ch/ast-news/2025/10/best-paper-award-at-sosp-2025-for-hao-sun-and-zhendong-su.html) [two queries agree]
- Best Paper: "CortenMM: Efficient Memory Management with Strong Correctness Guarantees" (Asterinas project; Peking University TELOS lab). One result claims performance "up to 26 times that of Linux" — [Ronghui Gu on X](https://x.com/RonghuiGu/status/1979219795137474736): "Excited to see the Asterinas paper co-authored by @CertiK win the SOSP'25 Best Paper Award!"; [C114Pro](https://www.c114pro.com/ainews/118320.html): "SOSP Unveils Best Paper Award, with 'XingZhan' OS Taking Center Stage"; [CortenMM artifact](https://github.com/TELOS-syslab/CortenMM-Artifact) [title only]. Linking "the Asterinas paper" to CortenMM relies on the search summary, so confidence is medium.
- Asterinas background, from a search summary: a "framekernel" OS with "unsafe Rust confined to a small, auditable framework called OSTD", while the rest is safe Rust. Its USENIX ATC 2025 papers are "Asterinas: A Linux ABI-Compatible, Rust-Based Framekernel OS with a Small and Sound TCB" and "Converos: Practical Model Checking for Verifying Rust OS Kernel Concurrency" [search summary; no primary read]
- The full SOSP 2025 award list was not recovered — [SOSP 2025](https://sigops.org/s/conferences/sosp/2025/)

**OSDI 2025**
- Best Paper: "Basilisk: Using Provenance Invariants to Automate Proofs of Undecidable Protocols" (lead author Tony Zhang, U. Michigan). Per the U-M story, "Two winning papers were selected from 53 total accepted submissions" (I did not cross-check that count) — [U. Michigan CSE](https://cse.engin.umich.edu/stories/cse-researchers-win-best-paper-award-at-osdi-2025). The second winner was not found.
- Mirage was an OSDI 2025 paper (ledger row 20).

**OSDI 2026** (Seattle, 13–15 July 2026; 136 accepted papers; 3 Jay Lepreau Best Paper Awards; 9 runners-up)
- Best Paper and Best Artifact: "Controlling Opaque-Component Effects with Semisolates and Try" (Lamprou, Zhu, Jin, Ntousakis, Liargkovas, Eng, Kallas, Greenberg, Vasilakis). The tool `try` is "a new kind of container called a semisolate, in that it is semi-isolated from the system" — [Brown CS awards](https://awards.cs.brown.edu/2026/07/16/strong-representation-at-osdi-including-three-honors-shows-the-impact-of-brown-systems/); [USENIX page](https://www.usenix.org/conference/osdi26/presentation/lamprou) [two queries agree]
- Best Paper: "Teaching The Old Dog New Tricks: Building Efficient Data Pipelines for Large-Scale LLM Pre-training" (Li Cheng, USTC, with ByteDance Seed) — [USTC news (EN)](https://en.ustc.edu.cn/info/1007/5181.htm); [USTC news (CN)](http://news.ustc.edu.cn/info/1055/95731.htm)
- Best Paper Runner-Up: "RT: Regular Types for the Streaming Shell" (Brown) — [Brown CS awards](https://awards.cs.brown.edu/2026/07/16/strong-representation-at-osdi-including-three-honors-shows-the-impact-of-brown-systems/)
- The third best paper was not found.

**ASPLOS 2025** (Rotterdam, 30 Mar – 3 Apr 2025; the first joint ASPLOS/EuroSys)
- Six Best Paper Awards — [ASPLOS 2025 awards](https://www.asplos-conference.org/asplos2025/awards/index.html); [Cornell Zhang lab](https://zhang.ece.cornell.edu/blog) ("SmoothE Wins the ASPLOS 2025 Best Paper Award"); [UMD ECE](https://ece.umd.edu/news/story/yu-joins-recipients-of-best-paper-award-at-asplos) [SNIPPET-CONFIRMED for SmoothE; the others come from one summary of the awards page]:
  - CXLfork: Fast Remote Fork over CXL Fabrics
  - xUI: extended User Interrupts
  - H-Houdini: Scalable Invariant Learning
  - MetaSapiens: Real-Time Neural Rendering with Efficiency-Aware Pruning and Accelerated Foveated Rendering
  - Orion: A Fully Homomorphic Encryption Framework for Deep Learning
  - SmoothE: Differentiable E-Graph Extraction

**ASPLOS 2026** (Pittsburgh, 22–26 Mar 2026)
- The Best Paper list includes the following three; the page may list more — [ASPLOS 2026 awards](https://www.asplos-conference.org/asplos2026/awards/index.html) [one search summary]:
  - CounterPoint: Using Hardware Event Counters to Refute and Refine Microarchitectural Assumptions (Lindsay, Trippel, Khandelwal, Bhattacharjee)
  - **Finding Reusable Instructions via E-Graph Anti-Unification** (Xiao, Yin, Sun, Zou, Liang; Peking U.)
  - Lifetime-Aware Design of Item-Level Intelligence (Prakash et al.; Harvard/PragmatIC)

**MLSys 2025**
- Best Paper: FlashInfer (ledger row 23).
- Outstanding Paper Honorable Mention: "APOLLO: SGD-like Memory, AdamW-level Performance" (UT Austin, Meta) — [UT Austin ECE](https://ece.utexas.edu/news/researchers-receive-outstanding-paper-award-mlsys-2025)

**MLSys 2026** (Bellevue, WA, 18–22 May 2026 per one summary; another said "Seattle")
- Best Research Paper: "LEANN: A Low-Storage Overhead Vector Index" (Wang, Li, Liu, Wu, Mao, Zhao, Yan, Xu, Zhou) — [MLSys award certificate PDF](https://mlsys.org/media/mlsys-2026/best-papers/Best-Research-Paper-LEANN.pdf)
- Best Research Paper: "StreamDiffusionV2", per the same summary; its source was not identified (possibly the [Zhihao Jia X post](https://x.com/JiaZhihao/status/2056784016209302014)). Single, unclear source.
- HipKittens was an oral in the "Compilers and Kernels" session (ledger row 19). I found no award for it.

### Inferences
- **Verification is winning at systems venues, not just PL venues.** BCF (SOSP 2025), CortenMM (SOSP 2025), Basilisk (OSDI 2025) and H-Houdini (ASPLOS 2025) all won best papers for making proofs or checkers work on real systems code. The BCF result is itself a pain point: a production verifier (the Linux eBPF verifier) *wrongly rejected* 403 of 512 real programs. Checkers that cannot see the programmer's reasoning cause false rejections, and that is a language-design problem as much as an analysis problem.
- **Effect control is resurfacing as a systems problem.** OSDI 2026's `try`/semisolates award is about containing what opaque components do to the system, and the runner-up adds types to shell pipelines. These are runtime or tool fixes for the lack of *typed, enforced effects* at the language level.
- **E-graphs keep winning at the architecture venue**: SmoothE (ASPLOS 2025) and E-Graph Anti-Unification (ASPLOS 2026); CGO 2026 also has SkeleShare. This is compiler infrastructure a new language can reuse, not a user-facing pain point.
- **The LLM-related best papers are about serving and training LLMs** (FlashInfer, the OSDI 2026 data pipelines, LEANN), not about LLMs writing code.

### Gaps
- The second OSDI 2025 best paper, the third OSDI 2026 best paper, the full SOSP 2025 list, possibly more ASPLOS 2026 winners, and the MLSys 2026 honorable mentions were not found.

---

## Q4. Keynotes on AI and programming languages (PLDI 2025/2026, POPL 2026, SPLASH 2025)

### Takeaway
The one confirmed AI-and-PL keynote is the **PLDI 2026 opening keynote by Saman Amarasinghe**, "Programming Language Design and Implementation for the Machine Learning Era: A Personal Perspective". It asks directly what languages should look like if LLMs generate tomorrow's code. The other keynote lists (PLDI 2025, POPL 2026, ICFP/SPLASH 2025) could not be recovered.

### Cited Findings
- **PLDI 2026, Wed 17 June 2026: Saman Amarasinghe (MIT).** According to the search summary, he "questioned what languages should look like if tomorrow's code is generated by LLMs, what compilers should optimize, verify, or compile, and whether programming languages and compilers will exist at all". He posed "a key provocation … why machine learning compilers aren't a part of the PL community" and discussed "paths that either preserve familiar abstractions or tear them apart" — [PLDI 2026 program entry](https://pldi26.sigplan.org/details/pldi-2026-papers/1/Programming-Language-Design-and-Implementation-for-the-Machine-Learning-Era-A-Person); [Holtzen, PLDI 2026 Reflections](http://sholtzen.dev/articles/pldi26-reflection.html) [SNIPPET-CONFIRMED]. Amarasinghe is also a co-author of the PLDI 2026 DP on typed GPU perspectives (Q2).
- PLDI 2025 keynotes: not recovered; the de Moura keynote in the notes is unverified (ledger row 2) — [PLDI 2025](https://pldi25.sigplan.org/)
- POPL 2026 keynotes: not recovered — [POPL 2026](https://popl26.sigplan.org/home)
- SPLASH 2025 and ICFP 2025 keynotes: not recovered (ledger row 10)

### Inferences
- The PLDI 2026 opening keynote frames the same question as this project's report ("what should a language look like when LLMs write the code?") as an open question for the field. That supports the claim that the design space is recognised but unsettled at the top venue.

### Gaps
- PLDI 2025, POPL 2026, ICFP 2025 and SPLASH 2025 keynote speakers and abstracts. Search budget exhausted; conference sites blocked.

---

## Q5. Which themes won the most awards, which point to pain a new language could fix, and 2026 work on languages and specs for AI-written code

### Takeaway
Counting the roughly 32 award winners recovered for 2025–2026 across the PL, verification, systems and ML-systems venues, **formal verification and correctness of real code is the largest cluster (about 10 awards)**. The next groups are **ML/GPU performance (about 7)** and **quantum/probabilistic/numerical semantics (about 5)**, then **e-graphs (2)**, **Rust semantics and systems (3, overlapping with verification)**, and **effect control (1 winner and 1 runner-up)**. **No recovered 2025–2026 award went to LLMs writing code or to LLM-plus-verification.** That work is visible as papers (AutoVerus at OOPSLA 2025), preprints (VeruSAGE, Lahiri), a keynote (Amarasinghe) and self-published languages (Vera), but not yet in the award lists I could see. The award-backed pains that a *language* (not a tool) could address are: checkers that reject correct code, unclear aliasing and undefined semantics, untyped side effects of components, and non-modular, vendor-specific GPU code.

### Cited Findings

**Theme tally (known 2025–2026 winners only; the lists are partial, so treat the counts as rough)**

| Theme | Award winners found (venue) | Count |
|---|---|---|
| Verification / correctness of real code | Destabilizing Iris (PLDI 25); VerusBelt (PLDI 26); Recursively Defined Quantum Circuits verification (PLDI 26); Backward Error Bounds (PLDI 26, sound analysis); 2 CAV 25 DPs; BCF eBPF (SOSP 25); CortenMM (SOSP 25); Basilisk (OSDI 25); H-Houdini (ASPLOS 25) | ~10 |
| ML / GPU / LLM-serving performance | Typed Perspectives GPU (PLDI 26); FlashInfer (MLSys 25); Orion FHE-for-DL (ASPLOS 25); MetaSapiens (ASPLOS 25); LLM pre-training pipelines (OSDI 26); LEANN (MLSys 26); StreamDiffusionV2 (MLSys 26, weak source) | ~7 |
| Quantum / probabilistic / numerical | Prob. symbolic execution, quantum circuits, Cobble, backward error (PLDI 26); probabilistic model simplification (CAV 25) | ~5 |
| Rust semantics / Rust systems | Tree Borrows (PLDI 25); VerusBelt (PLDI 26); CortenMM on Asterinas (SOSP 25) | 3 (overlaps verification) |
| E-graphs / equality saturation | SmoothE (ASPLOS 25); E-Graph Anti-Unification (ASPLOS 26) | 2 |
| Compiler IR semantics | Removing `undef` from LLVM IR (PLDI 26) | 1 |
| Effect control / isolation | `try`/semisolates (OSDI 26); RT regular types for shell (OSDI 26 runner-up); F* effects (POPL 2026 Most Influential, retrospective) | 1 + runner-up + retrospective |
| Language usability / live programming | Incremental bidirectional typing, Hazel (OOPSLA 25); Cope and Drag (ECOOP 25) | 2 |
| Managed runtimes | HeapBuffers (OOPSLA 25) | 1 |
| LLMs writing code / LLM + verification | none found | 0 |

Sources: the rows of Q2 and Q3 above.

**2026 work on languages for LLM-generated code, and on spec languages for AI agents (what could be confirmed)**
- **Vera** (Alasdair Allan) is the only confirmed "language designed for LLMs to write". It has mandatory contracts, algebraic effects, typed slot references instead of names, and Z3 checking "where possible"; it compiles to WebAssembly; v0.1.13 dates from 21 Aug 2026; licence MIT. It is self-published and not peer-reviewed — [PyPI veralang](https://pypi.org/project/veralang/) [CONFIRMED]
- **Intent formalization** (Lahiri, March 2026, preprint) argues that checkable specifications, from tests through full specs to DSLs, are "the key challenge" for AI-written code, with spec validation as the bottleneck — [MSR](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/) [CONFIRMED]
- **Industrial agent-plus-spec practice:** at SymCrypt, "agents help translate standards into Lean specifications" and "agents help write and maintain proofs", while Lean independently checks the proofs — [MSR blog, 13 Jul 2026](https://www.microsoft.com/en-us/research/blog/verifying-rust-cryptography-in-symcrypt-from-standards-to-code/) [CONFIRMED]
- **Agent proof success on real Rust systems:** over 80% of 849 VeruSAGE tasks, and over 90% of tasks humans had not finished — [MSR](https://www.microsoft.com/en-us/research/publication/verusage-a-study-of-agent-based-verification-for-rust-systems/) [CONFIRMED]
- **PLDI 2026 keynote:** asks what languages should look like if LLMs generate the code — [PLDI 2026](https://pldi26.sigplan.org/details/pldi-2026-papers/1/Programming-Language-Design-and-Implementation-for-the-Machine-Learning-Era-A-Person) [SNIPPET-CONFIRMED]
- I found **no 2025–2026 peer-reviewed, award-winning paper** proposing a general-purpose language or type system designed for LLM-generated code, or a contract language for AI agents. The search was cut short, so this is "not found", not "does not exist".

### Inferences

Award-backed pain points that a new language could address, strongest first:
1. **Checkers that reject correct code, or cannot see the programmer's reasoning.** Evidence: BCF (SOSP 2025 Best Paper) showed the eBPF verifier wrongly rejected 403 of 512 real programs, and VerusBelt (PLDI 2026 DP) had to give Verus's proof extensions a semantic foundation after the fact. A language where specifications and proof hints are first-class and travel with the code would target this. It fits the notes' "AI generates, a small trusted checker verifies" thesis, and agents can now write many of the hints (VeruSAGE >80%).
2. **Unclear aliasing and undefined semantics in systems languages.** Tree Borrows (PLDI 2025 DP) needed a new model just to say what unsafe Rust may do. Removing `undef` from LLVM IR (PLDI 2026 DP) shows that undefined-value semantics are still being repaired at the IR level. A new language with a fully specified, simpler aliasing model and no undefined behaviour avoids inheriting both problems.
3. **Uncontrolled side effects of components, packages or generated code.** OSDI 2026's best paper builds a *runtime* semi-isolation container (`try`) to contain what opaque components do. The POPL 2026 retrospective award honours F*'s *typed* multi-monadic effects. A language with typed effects or capabilities enforces at compile time what `try` enforces at run time. This is directly relevant to AI-generated code, whose effects are the thing a reviewer most needs to bound.
4. **Non-modular, vendor-specific GPU and ML kernels.** Evidence: Typed Perspectives (PLDI 2026 DP), FlashInfer's JIT templates (MLSys 2025 Best Paper), HipKittens needing an AMD-specific rethink (MLSys 2026 oral). This is real pain, but it is contested by well-funded incumbents (see performance_and_gpu.md on Mojo), so it is a weaker wedge for a small team.
5. **Fast, incremental type feedback for editing loops** (Hazel, OOPSLA 2025 DP). This matters for agent loops that call the checker thousands of times. It is a toolchain property a new language can build in from day one.

**Themes that point *away* from a general-purpose language:**
- Quantum, probabilistic and numerical semantics (about 5 awards) win through precise, narrow DSLs (Cobble, Bean/eggshel), not broad languages.
- ML-serving performance awards (FlashInfer, LEANN, the data pipelines) are systems engineering; a new language is not the fix.

**Caution for the report:** the absence of LLM-writes-code papers from the award lists means the report's central thesis (a language designed for AI authorship plus verification) is supported by **preprints, industry practice, a keynote and one self-published language**, not yet by peer-reviewed award winners. That is consistent with an open opportunity, but it is weaker evidence than the notes' framing suggests.

### Gaps
- The tally is built from partial award lists: POPL 2026, ICFP 2025, CGO and CC are missing, and the others are incomplete. With full lists the counts could shift, especially for effects and ownership, which POPL and ICFP often reward.
- LMPL 2025/2026 workshop papers, Onward! 2025 essays, and 2026 preprints on constrained decoding and verifier-in-the-loop generation were not re-checked (search cap reached).
