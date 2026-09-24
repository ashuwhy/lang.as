# Frontier Programming-Language Research Trends (PLDI, POPL, OOPSLA/SPLASH, ICFP, ECOOP, CGO, CC, CAV), 2023–2026

Legend for evidence type: **[PR]** = peer-reviewed venue paper; **[Pre]** = preprint / technical report (arXiv, not yet peer-reviewed, or review status unknown); **[Op]** = opinion / blog / essay / keynote; **[Ind]** = industry engineering blog or company statement; **[News]** = trade press (secondary); **[BG]** = background, pre-2024 work. Research date: 2026-09-24.

Method note for the report writer: the web-search budget ran out partway through this task, and the egress proxy blocked direct page fetches for most conference sites (conf.researchr.org, *.sigplan.org, dl.acm.org, arxiv.org, openreview.net, lean-lang.org, aws.amazon.com, amazon.science, most personal blogs). Many findings below therefore rest on search-engine snippets of the cited URL, not on a full read of the page. Pages I did read in full: the Microsoft Research SymCrypt blog, the MSR intent-formalization page, the MSR SOSP 2024 blog, the Verus podcast page, and GitHub READMEs (Vera, Hylo, Verus, Flux, verify-rust-std, WebAssembly/proposals, swift-evolution memory-safety vision, awesome-egraphs). Claims that rest only on snippets are still cited to the original URL, but they should be spot-checked before anything is published.

---

## Q1. Dominant themes at the top PL/compiler/verification venues, 2023–2026 (awards, keynotes, workshops)

### Takeaway
The award lists and keynotes group around six themes. (1) **Rust**: its semantics and its verification (Tree Borrows, VerusBelt, Flux, Aeneas). (2) **Separation-logic foundations** (Iris) that make those proofs possible. (3) **Ownership and "modes" beyond Rust**: OxCaml's data-race-freedom modes, Swift's `~Copyable`/`~Escapable`, Hylo's mutable value semantics. (4) **Effect handlers** moving from theory into runtimes and Wasm. (5) **The LLM + PL intersection**: type-constrained decoding, LLM-written proofs, the new SIGPLAN LMPL workshop. (6) **Accelerators, tensors and GPUs, plus e-graphs** as compiler infrastructure. Probabilistic and quantum programming keep winning awards but look further from mainstream pain points.

### Cited Findings

**PLDI 2025 (Seoul)**
- PLDI 2025 gave the Distinguished Paper award to only 6 of 89 accepted papers (the cap is 10%) — [Aarhus Univ. news](https://cs.au.dk/news-events/news/show-news/artikel/distinguished-paper-award-at-pldi-2025) [PR]
- PLDI 2025 Distinguished Papers include "AWDIT: An Optimal Weak Database Isolation Tester" — [Aarhus Univ. news](https://cs.au.dk/news-events/news/show-news/artikel/distinguished-paper-award-at-pldi-2025) [PR]
- PLDI 2025 Distinguished Papers include "Tree Borrows" (Villani, Hostert, Dreyer, Jung), a new aliasing model for Rust, and "Destabilizing Iris" (Spies, Mück, Zeng, Sammler, Lattuada, Müller, Dreyer) — [Saarland Informatics Campus](https://saarland-informatics-campus.de/en/piece-of-news/derek-dreyer-and-collaborators-receive-three-distinguished-paper-awards-at-pldi25-and-popl25/) [PR]
- PLDI 2025 keynote: Leonardo de Moura, "Lean: Machine-Checked Mathematics and Verified Programming, Past and Future" — [Lean on X](https://x.com/leanprover/status/1935112979097559133) [Op]
- PLDI 2025 had a dedicated "Machine Learning" session. It included "Type-Constrained Code Generation with Language Models" (Mündler, He, Wang, Sen, Song, Vechev), which uses type inference plus prefix automata to force LLM output to be well-typed and extends the approach to TypeScript — [PLDI 2025 program](https://pldi25.sigplan.org/details/pldi-2025-papers/25/Type-Constrained-Code-Generation-with-Language-Models); [ACM DL](https://dl.acm.org/doi/10.1145/3729274); [ETH SRI](https://www.sri.inf.ethz.ch/publications/muendler2025typeawareconstraint) [PR]
- PLDI 2025 ran a tutorial and workshop on egglog ("Unlocking Optimizations with egglog: Equality Saturation Meets Datalog") — [PLDI 2025 tutorials](https://pldi25.sigplan.org/details/pldi-2025-tutorials/4/Unlocking-Optimizations-with-egglog-Equality-Saturation-Meets-Datalog) [Op/event]

**PLDI 2026**
- Reported PLDI 2026 Distinguished Papers:
  - "VerusBelt: A Semantic Foundation for Verus's Proof-Oriented Extensions to the Rust Type System" (Hance, Elbeheiry, Dreyer, Matsushita)
  - "Towards Removing Undef Values From LLVM IR" (Lobo, McIver, Mitenkov, Lee, Sundararajah, Lopes)
  - "Modular GPU Programming with Typed Perspectives" (Bansal, Sainati, Cutler, Amarasinghe, Ragan-Kelley)
  - "Categorical Semantics of Probabilistic Symbolic Execution" (Li, Czenszak, Holtzen)
  - "Verification of Recursively Defined Quantum Circuits" (Ying)

  Sources: [Saarland Informatics Campus, PLDI 2026](https://saarland-informatics-campus.de/en/piece-of-news/mpi-researchers-receive-distinguished-paper-award-at-pldi-2026/); [PLDI 2026 papers track](https://pldi26.sigplan.org/track/pldi-2026-papers); [Steven Holtzen PLDI 2026 reflections](http://sholtzen.dev/articles/pldi26-reflection.html) [PR]. Caveat: this list comes from a search-engine aggregation of these pages. I could not open the track page to confirm it is complete.

**POPL 2025 / POPL 2026**
- POPL 2025 gave 7 Distinguished Paper awards out of 81 accepted papers — [TU Delft research portal](https://research.tudelft.nl/en/prizes/distinguished-paper-award-popl-2025/) [PR]
- POPL 2025 Distinguished Paper: "Data Race Freedom à la Mode" (Georges, Peters, Elbeheiry, White, Dolan, Eisenberg, Casinghino, Pottier, Dreyer). The authors include Jane Street engineers. The paper extends OCaml's modes (locality, uniqueness, affinity) with "contention" and "portability" to guarantee data-race freedom while staying backward compatible with existing OCaml — [ACM DL](https://dl.acm.org/doi/10.1145/3704859); [POPL 2025 program](https://popl25.sigplan.org/details/POPL-2025-popl-research-papers/23/Data-Race-Freedom-la-Mode) [PR]
- Other POPL 2025 Distinguished Papers:
  - a tensor-compiler verification paper (Mendis, Arora) — [Illinois](https://siebelschool.illinois.edu/news/POPL-2025)
  - "Relaxed Memory Concurrency Re-executed" — [TU Delft](https://research.tudelft.nl/en/prizes/distinguished-paper-award-popl-2025/)
  - "Barendregt Convenes with Knaster and Tarski" (binders / rule induction) — [Sheffield](https://www.sheffield.ac.uk/cs/news/dr-andrei-popescu-receives-distinguished-paper-award-popl-third-consecutive-year) [PR]
- POPL 2026: the call for papers kept the rule of at most 10% of papers as Distinguished — [POPL 2026 track](https://popl26.sigplan.org/track/POPL-2026-popl-research-papers). The winners list was **not found** (see Gaps).
- POPL 2026 hosted a co-located Dafny 2026 workshop, where "A benchmark for vericoding: formally verified program synthesis" was presented — [POPL 2026 / Dafny 2026](https://popl26.sigplan.org/details/dafny-2026-papers/13/A-benchmark-for-vericoding-formally-verified-program-synthesis) [PR-workshop]
- Lean-egg, an equality-saturation tactic for Lean, appeared at POPL 2026 — [awesome-egraphs list](https://github.com/philzook58/awesome-egraphs) [PR]

**OOPSLA/SPLASH and ICFP 2025 (Singapore, co-located)**
- OOPSLA 2025 Distinguished Paper: "Incremental Bidirectional Typing via Order Maintenance" (Porter, Wei, Kirisame, Panchekha, Omar). It gives incremental type checking for live-programming environments (Hazel) — [U. Michigan CSE](https://cse.engin.umich.edu/stories/cse-researchers-win-distinguished-paper-award-at-oopsla-2025) [PR]
- AutoVerus (LLM-generated Verus proofs) was published in PACMPL OOPSLA2 2025, Vol. 9, Article 396 — [ACM DL](https://dl.acm.org/doi/10.1145/3763174); [MSR](https://www.microsoft.com/en-us/research/publication/autoverus-automated-proof-generation-for-rust-code/) [PR]
- OOPSLA 2025 effect-handler papers include "Tracing Just-in-time Compilation for Effects and Handlers" (Gaißert, Bolz-Tereick, Brachthäuser), "Zero-Overhead Lexical Effect Handlers", and "Dynamic Wind for Effect Handlers" — [Effekt publications](https://effekt-lang.org/publications); [SPLASH 2025](https://2025.splashcon.org/details/OOPSLA/179/Dynamic-Wind-for-Effect-Handlers) [PR]
- ICFP 2025 keynotes: Christos Dimoulas, Ekaterina Komendantskaya, Satnam Singh. SPLASH 2025 keynotes: Julia Lawall, Frank Piessens, Zhendong Su — [ICFP 2025 keynotes](https://icfp25.sigplan.org/track/icfp-2025-icfp-keynotes); [SPLASH 2025 keynotes](https://icfp25.sigplan.org/track/splash-2025-keynotes) [event]
- **New SIGPLAN workshop:** the 1st International Workshop on Language Models and Programming Languages (LMPL 2025) ran on 15 Oct 2025, co-located with ICFP/SPLASH, and has ACM proceedings. LMPL 2026 is scheduled at SPLASH/ISSTA 2026 — [LMPL 2025](https://conf.researchr.org/home/icfp-splash-2025/lmpl-2025); [ACM proceedings](https://dl.acm.org/doi/proceedings/10.1145/3759425); [LMPL 2026](https://conf.researchr.org/home/splash-issta-2026/lmpl-2026) [event]
- ICFP/SPLASH 2025 included talks by Jane Street and Docker on moving to OCaml 5 — [Anil Madhavapeddy notes](https://anil.recoil.org/notes/icfp25-ocaml5-js-docker) [Op]

**ECOOP / ‹Programming› / CGO / systems venues**
- ECOOP 2025 (PLSS workshop): "Designing Hylo, a programming language for safe systems programming" — [ECOOP 2025](https://2025.ecoop.org/details/plss-2025-papers/12/Designing-Hylo-a-programming-language-for-safe-systems-programming) [event]
- ‹Programming› 2025 hosted an "Effekt: Lexical Effect Handlers In Action" workshop — [‹Programming› 2025](https://2025.programming-conference.org/home/effekt-2025) [event]
- CGO 2025: "DialEgg: Dialect-Agnostic MLIR Optimizer using Equality Saturation with Egglog" connects e-graphs to MLIR — [ACM DL](https://dl.acm.org/doi/10.1145/3696443.3708957) [PR]
- SOSP 2024: Verus ("A Practical Foundation for Systems Verification") won the Distinguished Artifact Award. MSR reports it verifies "up to 61 times faster than existing methods", evaluated on systems with 6,100 lines of code and 31,000 lines of proof — [MSR blog](https://www.microsoft.com/en-us/research/blog/microsoft-at-sosp-2024-innovations-in-systems-research/) [PR/Ind]
- The Verus authors state that "two of three best papers at OSDI 2024 are built on Verus" — [Verus paper (ETH Research Collection)](https://www.research-collection.ethz.ch/server/api/core/bitstreams/b6c5b30f-8192-476a-8dc2-a99fb45d448f/content) [PR]. The MSR SOSP blog post does not repeat this claim (I read it in full).
- PACMPL (June 2026): "Iris-WasmFX: Modular Reasoning for Wasm Stack Switching" — [ACM DL](https://dl.acm.org/doi/10.1145/3808271); [PDF](https://homepages.inf.ed.ac.uk/slindley/papers/iris-wasmfx.pdf) [PR]
- E-graphs keep winning awards in adjacent venues: Isaria (auto-generated vectorizing compilers) won ASPLOS 2024 Best Paper, and MegaLibm was a POPL 2024 Distinguished Paper — [awesome-egraphs](https://github.com/philzook58/awesome-egraphs) [PR]

**Background (pre-2024)**
- "Flux: Liquid Types for Rust", PLDI 2023 — [ACM DL](https://dl.acm.org/doi/10.1145/3591283) [BG][PR]
- "Better Together: Unifying Datalog and Equality Saturation" (egglog), PLDI 2023 — [PLDI 2023](https://pldi23.sigplan.org/details/pldi-2023-pldi/20/Better-Together-Unifying-Datalog-and-Equality-Saturation) [BG][PR]
- "Continuing WebAssembly with Effect Handlers" (WasmFX), OOPSLA 2023 — [ACM DL](https://dl.acm.org/doi/10.1145/3622814) [BG][PR]
- "From Capabilities to Regions: Enabling Efficient Compilation of Lexical Effect Handlers", OOPSLA 2023 — [ACM DL](https://dl.acm.org/doi/10.1145/3622831) [BG][PR]
- "The Verse Calculus", ICFP 2023 (Augustsson, Breitner, Claessen, Jhala, Peyton Jones, Shivers, Steele, Sweeney) — [ACM DL](https://dl.acm.org/doi/10.1145/3607845) [BG][PR]

### Inferences
- **Rust is the centre of gravity for applied PL/verification research.** The evidence: Distinguished Papers on Rust's aliasing model (Tree Borrows, 2025) and on Verus's type-system foundations (VerusBelt, 2026); Verus-based OSDI best papers; Flux; Aeneas; the AWS/Rust Foundation standard-library effort (Q2). The research community is now building proof infrastructure *on top of* Rust rather than designing a Rust replacement.
- **"Ownership without Rust's complexity" is an active, award-winning direction.** OxCaml modes (POPL 2025 DP), Swift noncopyable/nonescapable types, and Hylo's mutable value semantics are three different attempts at safe aliasing control with less annotation burden than Rust's borrow checker. None of them has become the default in a mainstream language (see Q5).
- **The LLM + PL intersection is now institutionalised.** It has a PLDI session, a SIGPLAN workshop (LMPL), a Dafny-workshop benchmark, and OOPSLA papers on LLM-written proofs. The dominant framing is "LLMs generate, formal tools check" rather than "LLMs replace formal tools".
- **Accelerator programming is a growing award magnet**: GPU typed perspectives (PLDI 2026 DP), tensor-compiler verification (POPL 2025 DP), and MLIR/e-graph work at CGO. This connects to industrial moves such as Mojo (Q2).

### Gaps
- I could not retrieve the full Distinguished Paper / Best Paper lists for POPL 2026, ICFP 2025, ECOOP 2025/2026, CGO 2025/2026, CC 2025/2026, or CAV 2024–2026. The search budget ran out, and conf.researchr.org, splashcon.org, dl.acm.org and the CAV sites were blocked by the egress proxy. The CGO/CC/CAV themes above are inferred from individual papers, not from award lists.
- The PLDI 2025 and PLDI 2026 keynote lists are incomplete (only de Moura's 2025 keynote is confirmed).
- I found no 2024–2026 SNAPL edition. The Onward! 2025 essay track exists ([Onward! 2025 papers](https://2025.splashcon.org/track/splash-2025-Onward-papers)), but I could not read its contents.

---

## Q2. Which results have crossed into industry use? Concrete adoption evidence

### Takeaway
Verification-aware programming has real, production-grade deployments:
- **AWS**: the authorization engine rewritten in Dafny and serving ~1B requests/second; Cedar developed with Dafny, then Lean, models.
- **Microsoft**: SymCrypt post-quantum crypto written in Rust, verified in Lean via Aeneas, and shipping in Windows Insider builds.
- **AWS + Rust Foundation**: a funded effort to verify the Rust standard library with Kani, VeriFast, Flux and others.

Language-level features have shipped too: OCaml 5 effect handlers in production at Jane Street and Docker, Swift's ownership/`~Escapable`/`Span` and opt-in strict memory safety, WasmGC in all major browsers, and e-graphs in Cranelift. In almost every case, though, verification is done by **specialist teams bolting tools onto an existing language**. No mainstream language has shipped with verification or typed effects as a first-class, default experience.

### Cited Findings

**AWS: Dafny, Lean, Cedar**
- AWS rebuilt its authorization engine (invoked about **1 billion times per second**) in Dafny over **four years**. The new engine deployed in **2024 without incident** and gave a **threefold performance improvement**. The Dafny code compiles to Java ("AuthV2java") that had to be human-readable for code review — [ICSE 2025 paper, "Formally Verified Cloud-Scale Authorization"](https://dl.acm.org/doi/10.1109/ICSE55347.2025.00166); [PDF](https://assets.amazon.science/bb/40/22ac44f84f6d8eb625ac9666a00f/formally-verified-cloud-scale-authorization.pdf) [PR/Ind]
- Cedar, AWS's open-source authorization policy language, follows "verification-guided development". Formal models and proofs were first written in Dafny; the Rust implementation is checked against the models with millions of differential tests — [Amazon Science](https://www.amazon.science/blog/how-we-built-cedar-with-automated-reasoning-and-differential-testing); [arXiv 2407.01688](https://arxiv.org/pdf/2407.01688) [PR/Ind]
- AWS ported Cedar's models and proofs from Dafny to Lean, which it judged better suited to meta-theory proofs such as validation soundness. The Lean models are about **10× smaller** than production code, and no Cedar version is released unless its model, proofs and differential tests are current — [Cedar RFC 0032](https://github.com/cedar-policy/rfcs/blob/main/text/0032-port-formalization-to-lean.md); [Lean use case: Cedar](https://lean-lang.org/use-cases/cedar/); [AWS Open Source blog](https://aws.amazon.com/blogs/opensource/lean-into-verified-software-development/) [Ind]
- Lean's creator, Leonardo de Moura, is a Senior Principal Applied Scientist in AWS's Automated Reasoning Group and co-founder of the Lean FRO — [Cornell colloquium listing](https://www.cs.cornell.edu/events/computer-science-colloquium/lean-machine-checked-mathematics-and-verified-programming) [Ind]

**Microsoft: Verus, Aeneas + Lean for SymCrypt, Rust migration**
- Microsoft is porting SymCrypt (the crypto library behind Windows and Azure) from C to Rust. The Rust code is verified in Lean via Aeneas, which uses Rust's ownership discipline to extract a pure functional model. ML-KEM and SHA-3 have full proofs and are "being used in insiders builds of Windows today". AES-GCM, FrodoKEM and ML-DSA are planned. AI agents help translate standards into Lean specs (which are then audited) and write proofs, and Lean independently checks every proof. Blog dated 13 July 2026 — [MSR blog](https://www.microsoft.com/en-us/research/blog/verifying-rust-cryptography-in-symcrypt-from-standards-to-code/) [Ind]
- The accompanying technical report covers **16.7 KLOC of Rust** proven safe, panic-free and functionally correct, backed by a **237 KLOC Lean development**, for post-quantum cipher suites on x86-64 and ARM. The ratio of about 14:1 proof to code is notable — [arXiv 2609.15648 (draft TR, Sept 2026)](https://arxiv.org/abs/2609.15648) [Pre]
- The Verus authors claim it is "already seeing industrial use at Microsoft and Amazon" — [Verus paper (ETH Research Collection)](https://www.research-collection.ethz.ch/server/api/core/bitstreams/b6c5b30f-8192-476a-8dc2-a99fb45d448f/content) [PR]. **Caveat:** the MSR SOSP blog and MSR podcast do not name any specific production deployment (I read both in full: [blog](https://www.microsoft.com/en-us/research/blog/microsoft-at-sosp-2024-innovations-in-systems-research/), [podcast](https://www.microsoft.com/en-us/research/podcast/abstracts-november-5-2024/)).
- Verus's own README says it is "under active development", that features "may be broken and/or missing", and that it supports only "a subset of Rust" — [GitHub verus-lang/verus](https://github.com/verus-lang/verus) [Ind]
- Microsoft Distinguished Engineer Galen Hunt posted "My goal is to eliminate every line of C and C++ from Microsoft by 2030", with a "North Star" of "1 engineer, 1 month, 1 million lines of code" using AI plus algorithms — [Windows Central](https://www.windowscentral.com/microsoft/windows-11/my-goal-is-to-eliminate-every-line-of-c-and-c-from-microsoft-by-2030-microsoft-bets-on-ai-to-finally-modernize-windows) [News]. He later clarified that Windows is *not* being rewritten in Rust with AI and that this is a research project on language-to-language migration — [TechRadar](https://www.techradar.com/computing/cloud-computing/microsoft-engineer-clarifies-speculation-around-plans-to-eliminate-c-c-languages-by-2030); [IT Pro](https://www.itpro.com/software/development/microsoft-rust-programming-language-modernization-ai) [News]

**Rust standard-library verification (AWS + Rust Foundation)**
- AWS and the Rust Foundation announced the challenge in November 2024 — [AWS Open Source blog](https://aws.amazon.com/blogs/opensource/verify-the-safety-of-the-rust-standard-library/); [Rust Foundation](https://foundation.rust-lang.org/news/rust-foundation-collaborates-with-aws-initiative-to-verify-rust-standard-libraries) [Ind]
- Current repository state: 29 challenges worth about $355k in total; **7 resolved, 20 open**; open ones include BTreeMap, atomic types, String and Vec/VecDeque. Accepted tools are Flux, GOTO Transcoder (ESBMC), Kani, KMIR and VeriFast — [GitHub model-checking/verify-rust-std](https://github.com/model-checking/verify-rust-std) [Ind]
- Scale so far: year 1 was almost entirely hand-written, with **725 manual Kani harnesses** (694 with formal contracts) and more than 50 VeriFast proofs. Manual contract growth **plateaued around October 2025**. Participation is 450+ PRs from 21+ external contributors — [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/) [Ind]
- Automation: the AWS Kani team's "Autoharness" generated **16,748 proof harnesses**. **989 functions** (295 automatic + 694 manual) are now verified against contracts — [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/); [arXiv 2606.17374 "Verifying the Rust Standard Library"](https://arxiv.org/html/2606.17374) [Ind/Pre]
- Unsolved: **9,600+ generic functions** are skipped because monomorphization defeats automatic harnessing, the atomics and `Arc` challenges remain open, and lock-free code under relaxed memory is "genuinely hard" — [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/) [Ind]
- Minor conflict on tool lists: one summary says four tools are in CI (Kani, ESBMC, VeriFast, Flux) with four more under review (Verus, Creusot, KRust, RAPx) — [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/). The repository README lists five accepted tools, including KMIR — [GitHub](https://github.com/model-checking/verify-rust-std).

**Flux (refinement types for Rust)**
- Flux was used to verify process isolation in the Tock embedded OS, which is used in the Google Security Chip and Microsoft Pluton, and found multiple subtle isolation-breaking bugs — [Galois blog](https://galois.com/blog/2023/05/flux-liquid-types-for-rust/); [PLDI 2023 paper](https://dl.acm.org/doi/10.1145/3591283) [PR]
- Flux is also an accepted tool in the Rust std verification challenge — [GitHub verify-rust-std](https://github.com/model-checking/verify-rust-std) [Ind]

**OCaml 5 effects and Jane Street's OxCaml**
- OCaml 5.0 (December 2022) shipped a rewritten runtime with domains for parallelism and native effect handlers for concurrency — [InfoQ](https://www.infoq.com/news/2022/12/ocaml-5-concurrency-parallelism/) [BG][News]
- Eio 1.0, an effects-based direct-style I/O library that avoids "function colouring", was released March 2024 — [Tarides](https://tarides.com/blog/2024-03-20-eio-1-0-release-introducing-a-new-effects-based-i-o-library-for-ocaml/) [Ind]
- Jane Street has switched production to the multicore runtime, but hit GC pacing issues and resource-usage changes. Docker migrated to direct style with Eio, and Semgrep and Tezos have adopted it — [Anil Madhavapeddy, ICFP/SPLASH 2025 notes](https://anil.recoil.org/notes/icfp25-ocaml5-js-docker) [Op]
- Jane Street is adding a static *effect system* to OCaml, which implies OCaml 5's effects are not yet tracked in types — [Jane Street tech talk "Effective Programming: Adding an Effect System to OCaml"](https://www.janestreet.com/tech-talks/effective-programming/) [Ind]
- Jane Street open-sourced **OxCaml** in mid-2025. It bundles modes (locality for stack allocation, uniqueness, affinity, contention, portability for data-race freedom) and unboxed types/layouts — [Tarides](https://tarides.com/blog/2025-07-09-introducing-jane-street-s-oxcaml-branch/); [OxCaml docs](https://oxcaml.org/documentation/); [POPL 2025 DP](https://dl.acm.org/doi/10.1145/3704859) [Ind/PR]

**Swift ownership and memory safety**
- Swift 6 claims all five memory-safety dimensions: lifetime, bounds, type, initialization and thread safety. Opt-outs remain via `unowned(unsafe)`, `nonisolated(unsafe)`, `Unsafe*Pointer` and C interop. An opt-in "strict memory safety" mode flags unsafe constructs and requires `@unsafe` annotations. `Span<T>` is built on non-escapable types, and C interop can use Clang `__counted_by`/`lifetimebound` — [swift-evolution memory-safety vision](https://github.com/swiftlang/swift-evolution/blob/main/visions/memory-safety.md) [Ind]
- Relevant proposals: SE-0446 introduces `~Escapable` types; SE-0447 introduces `Span` — [SE-0446](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0446-non-escapable.md); [SE-0447](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0447-span-access-shared-contiguous-storage.md) [Ind]
- In Swift 6.2, `-strict-memory-safety` is opt-in, while `@lifetime` dependency annotations (needed to *return* a `Span` from an API) are still experimental — [Michael Tsai: Lifetime Dependencies in Swift 6.2](https://mjtsai.com/blog/2025/03/19/lifetime-dependencies-in-swift-6-2-and-beyond/); [Michael Tsai: Swift 6.2](https://mjtsai.com/blog/2025/10/29/swift-6-2/) [Op]

**WebAssembly: GC and typed stack switching (effect handlers)**
- WasmGC is enabled by default in Chrome — [Chrome for Developers](https://developer.chrome.com/blog/wasmgc) [Ind]
- WasmGC and Wasm tail calls are "Baseline newly available" across browsers — [web.dev](https://web.dev/blog/wasmgc-wasm-tail-call-optimizations-baseline) [Ind]
- Kotlin, Dart/Flutter and Java (J2CL) target WasmGC — [V8 blog](https://v8.dev/blog/wasm-gc-porting) [Ind]
- WebAssembly 3.0 (2025) standardised GC, memory64 and exception handling, and Google Sheets' calc engine reportedly runs 2× faster than its JS version on WasmGC — [State of WebAssembly 2026](https://devnewsletter.com/p/state-of-webassembly-2026/) [News; secondary]
- The Wasm **stack-switching** proposal, derived from the WasmFX effect-handler design (typed continuations), is at **Phase 3 (Implementation)**. Its champions are Francis McCabe and Sam Lindley — [GitHub WebAssembly/proposals](https://github.com/WebAssembly/proposals); [explainer](https://github.com/WebAssembly/stack-switching/blob/main/proposals/stack-switching/Explainer.md); [WasmFX](http://wasmfx.dev/) [Ind]

**E-graphs, MLIR, Mojo**
- Cranelift, Wasmtime's production code generator, uses "ægraphs" (acyclic e-graphs) as its mid-end optimizer. Egglog has Python bindings — [awesome-egraphs](https://github.com/philzook58/awesome-egraphs) [Ind]
- **Mojo 1.0** (Chris Lattner's MLIR-based Python-superset language) shipped on 11 Aug 2026, and the compiler was open-sourced under Apache 2.0 with LLVM exceptions on 18 Aug 2026. This followed Qualcomm's roughly $3.9B acquisition of Modular; Lattner is now Qualcomm EVP for advanced AI software — [The Register](https://www.theregister.com/ai-and-ml/2026/08/12/modulars-mojo-programming-language-hits-10-milestone/5286545); [RuntimeWire](https://runtimewire.com/article/chris-lattner-open-sources-mojo-qualcomm-modular); [Phoronix](https://www.phoronix.com/news/Modular-Mojo-Open-Source) [News; not verified against a Modular/Qualcomm primary source]

**Verse (Epic Games)**
- Simon Peyton Jones has worked at Epic Games since 2021 on Verse, a functional-logic language. Its core calculus (ICFP 2023) gives a confluent rewrite semantics — [SPJ: Verse Calculus](https://simon.peytonjones.org/verse-calculus/); [ACM DL](https://dl.acm.org/doi/10.1145/3607845) [BG][PR]

**Lean and AI-produced verified code**
- De Moura reported that Claude Code, "with no special training for theorem proving, converted zlib to Lean and proved the roundtrip correct" (decompress ∘ compress = id) with minimal human guidance — [de Moura on X](https://x.com/Leonard41111588/status/2028834053240279435) [Op]
- A follow-up ran 105 million fuzzing executions against the verified lean-zip. It found zero bugs in the verified code, but did find a heap buffer overflow in the **Lean runtime** (`lean_alloc_sarray`), outside the proof boundary. Commenters disputed the post's "found a bug" framing — [kirancodes.me](https://kirancodes.me/posts/log-who-watches-the-watchers.html); [HN discussion](https://news.ycombinator.com/item?id=47760024) [Op]

**DARPA TRACTOR (C → safe Rust)**
- DARPA's TRACTOR programme aims to automate translation of legacy C to safe, idiomatic Rust. MIT Lincoln Lab is the independent evaluator and has released a public benchmark — [arXiv 2609.25121 TRACTOR Benchmark](https://arxiv.org/abs/2609.25121); [The New Stack](https://thenewstack.io/can-darpas-tractor-pull-c-to-rust-for-memory-safe-overhaul/) [Pre/News]
- Performers publish agentic translation results; for example, UW HARVEST reports results from kiro-cli runs, April 2026 — [GitHub UW-HARVEST](https://github.com/UW-HARVEST/harvest-agentic-translate-results) [Ind]

### Inferences
- The strongest industrial pattern is **"write a small, verification-friendly model or implementation, prove it, then connect it to production code"**. Examples: AWS AuthV2 (Dafny → Java), Cedar (Lean model + differential testing against Rust), SymCrypt (Rust → Aeneas → Lean). Each needs a separate verification language and toolchain, plus a translation or differential-testing bridge. That integration seam is itself a pain point a new language could remove.
- Proof burden is still very large. SymCrypt has about 14 lines of Lean per line of Rust, and the Rust std effort plateaued on manual proofs within a year. Automation (Autoharness, LLM agents) is what is now moving the numbers, not better human ergonomics.
- The adopters are hyperscalers with dedicated formal-methods groups (AWS ARG, MSR, Jane Street's compiler team). I found no evidence of mid-sized companies adopting Verus, Flux, Dafny or Lean in production.
- Effect handlers have shipped at the runtime level (OCaml 5, and Wasm stack switching at Phase 3), but **typed** effects have not shipped in any top-20 language.

### Gaps
- I found no primary source naming a *specific* production system verified with Verus at Microsoft or AWS; the claim comes from the Verus authors.
- I found no adoption evidence for Prusti or Creusot beyond Creusot being "under review" for the Rust std challenge.
- Kani's AWS-internal usage (e.g., Firecracker, s2n-quic) was not verified this session because the search budget ran out.
- Verse's shipping status in Fortnite/UEFN is not verified here.
- The Google Sheets 2× WasmGC figure comes from a secondary newsletter.

---

## Q3. What prominent PL/FM researchers call the biggest open problems or "grand challenges"

### Takeaway
The loudest 2025–2026 argument from senior figures (de Moura, Kleppmann, Lahiri) is that **AI is about to make code nearly free, so the scarce resource becomes trust.** Mechanically checked proofs are the way to get it, and the hard, unsolved part is **specification, or "intent formalization"**: getting from what a human meant to a formal, checkable spec. Secondary open problems: scaling proofs past manual effort (generics, concurrency, relaxed memory), trusted-computing-base gaps (runtimes, specs), and making ownership, effects and verification ergonomic enough for ordinary developers.

### Cited Findings
- **Leonardo de Moura (AWS / Lean FRO), essay, 28 Feb 2026** [Op]:
  - Google and Microsoft report that 25–30% of their new code is AI-generated, and Microsoft's CTO predicts 95% by 2030.
  - "Mathematical proof must replace traditional review — not supplement it, replace it."
  - Everything (AI, automation, humans) should sit outside a small trusted kernel of a few thousand lines.
  - An AI that generates provably correct code is "qualitatively different" from one that generates plausible code.

  Source: [de Moura blog](https://leodemoura.github.io/blog/2026/02/28/when-ai-writes-the-worlds-software.html)
- **Martin Kleppmann, blog, 8 Dec 2025, "Prediction: AI will make formal verification go mainstream"** [Op]:
  - Formal verification is about to become "vastly cheaper".
  - AI-generated code needs formal verification so that human review can be skipped.
  - Formal precision counteracts LLMs' probabilistic nature.
  - He names Dafny, Nagini and Verus as the languages likely to go mainstream, and cites a March 2025 JetBrains Research study with promising Claude 3.5 Sonnet results on them.

  Sources: [Kleppmann](https://martin.kleppmann.com/2025/12/08/ai-formal-verification.html); [Simon Willison summary](https://simonwillison.net/2025/Dec/9/formal-verification/)
- Related commentary: "The Coming Need for Formal Specification" — [Ben Congdon, Dec 2025](https://benjamincongdon.me/blog/2025/12/12/The-Coming-Need-for-Formal-Specification/) [Op; title only, content not read]
- **Shuvendu Lahiri (Microsoft Research), "Intent Formalization: A Grand Challenge for Reliable Coding in the Age of AI Agents", March 2026** [Pre/position paper]:
  - Intent formalization means translating informal intent into checkable formal specifications. It is "the key challenge that will determine whether AI makes software more reliable or merely more abundant".
  - It offers a spectrum from lightweight disambiguating tests, through full functional specs, to DSLs that synthesize correct code.
  - The bottleneck is *specification validation*, because only users can confirm a spec is right.
  - Open challenges: scaling beyond benchmarks, compositionality over changes, metrics for validating specs, rich logics, and human–AI spec interaction design.

  Sources: [MSR publication page (read in full)](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/); [arXiv 2603.17150](https://arxiv.org/html/2603.17150v1); [RiSE blog](https://risemsr.github.io/blog/2026-03-05-shuvendu-intent-formalization/)
- **Verus team (MSR podcast)** [Op]:
  - Proving software correct still takes "significant effort" even with advanced tools.
  - AI code generation "lacks verification capabilities for realistic code".
  - Verifier latency matters: "The difference between one second and 10 seconds… it's much nicer to get immediate feedback".

  Source: [MSR Abstracts podcast](https://www.microsoft.com/en-us/research/podcast/abstracts-november-5-2024/)
- **Rust std verification lessons** [Ind]: manual proof engineering "was never going to reach tens of thousands of functions"; generics, atomics/`Arc` and relaxed-memory lock-free code remain open — [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/); [arXiv 2510.01072 "Lessons Learned So Far From Verifying the Rust Standard Library"](https://arxiv.org/html/2510.01072v1) [Pre]
- **Trusted-computing-base gap** [Op]: a verified Lean program was still exposed to a runtime memory-safety bug. Proofs cover only the code and spec, not the runtime or FFI — [kirancodes.me](https://kirancodes.me/posts/log-who-watches-the-watchers.html)
- **Niko Matsakis (Rust)** [Op]: frames Rust's mission as "foundational software" ("Rust in 2025: Targeting foundational software"), and published lessons from the Rust Vision Doc effort in Nov 2025 — [corrode.dev](https://corrode.dev/blog/foundational-software/); [corrode podcast](https://corrode.dev/podcast/s04e04-rust/). Detailed open-problem lists were not retrieved.
- **Community/agency white papers**: the CCC/CRA-I joint white paper "The Future of Programming in the Age of Large Language Models" (April 2025) and the CCC white paper "The Imperative for Grand Challenges in Computing" (July 2025) exist — [CCC/CRA-I 2025](https://cra.org/ccc/wp-content/uploads/sites/2/2025/05/CCC_CRA-I-Whitepaper_-The-Future-of-Programming-in-the-Age-of-Large-Language-Models.pdf); [CCC July 2025](https://cra.org/ccc/wp-content/uploads/sites/2/2025/07/The-Imperative-for-Grand-Challenges-in-Computing.pdf) [Op; contents not read, fetch blocked]
- **Hylo designers** [Op]: the design has "solidified into a coherent system", but the implementation is still experimental. A new compiler is under way ([hylo-new](https://github.com/hylo-lang/hylo)), and the README warns "expect things to break" — [GitHub hylo-lang/hylo](https://github.com/hylo-lang/hylo); [Hylo introduction](https://hylo-lang.org/introduction/)

### Inferences
- There is broad consensus that the **specification problem** is now the binding constraint. Proof generation is increasingly automatable (see the Q4 numbers); knowing *what* to prove is not. A language that makes specifications cheap to write, read and validate (contracts, refinement types, executable specs, property tests as a spec ramp) directly targets the named grand challenge.
- The second-order problem is **end-to-end trust**: runtimes, FFI to C, generics and concurrency are where proofs stop. A language whose runtime and FFI are designed to sit inside the proof boundary (small, verified runtime; safe FFI with `__counted_by`-style contracts as in Swift) addresses a gap flagged in practice.

### Gaps
- I could not retrieve 2024–2026 statements from Simon Peyton Jones, Chris Lattner, Graydon Hoare, Adrian Sampson or Jonathan Aldrich on grand challenges. Search was exhausted and their blogs or pages were blocked. Chris Lattner's direction can only be inferred from Mojo 1.0 (Q2).
- I found no SNAPL 2025/2026; Onward! 2025 essay contents were not retrieved.
- The full recommendation lists of the CCC/CRA-I white papers were not read.

---

## Q4. Work on "languages designed for AI agents to write" and "languages that make generated code verifiable"

### Takeaway
There are three strands:
- **(a) Constrain generation with the language's static semantics.** Type-constrained decoding (PLDI 2025) and a wave of 2026 preprints on compiler-in-the-loop and "projectional" decoding.
- **(b) Generate code in verification-aware languages (Dafny, Verus, Lean) and let the checker be the judge ("vericoding").** Off-the-shelf LLMs already succeed about 82% of the time in Dafny, versus 44% in Verus and 27% in Lean. Agentic systems now finish over 80% of real Verus systems-proof tasks.
- **(c) New languages explicitly designed for LLM authors.** The clearest example is Vera: mandatory contracts, algebraic effects, refinement types, De Bruijn-indexed variables, Z3 verification with runtime fallback, compiled to Wasm. It is a single-author project, not peer-reviewed, and self-benchmarked.

The research signal is that the **simpler and more automated the verifier (Dafny/SMT), the better LLMs do**. Richer type systems (Verus's Rust ghost/tracked modes, Lean's dependent types) hurt LLM success.

### Cited Findings

**Vericoding and verified code generation**
- **Vericoding benchmark** (Sept 2025; presented at the Dafny 2026 workshop at POPL 2026):
  - 12,504 formal specs: 3,029 Dafny, 2,334 Verus/Rust and 7,141 Lean, of which 6,174 are new, unseen problems.
  - Off-the-shelf LLM success: **Dafny 82%, Verus/Rust 44%, Lean 27%**.
  - The Verus shortfall is attributed to its complex type system and thin training data; Lean's to LLMs being trained mostly on math rather than code verification.
  - Adding natural-language descriptions does *not* significantly help.
  - LLM progress lifted pure Dafny verification from **68% to 96% in one year**.

  Sources: [arXiv 2509.22908](https://arxiv.org/abs/2509.22908); [POPL 2026 Dafny workshop](https://popl26.sigplan.org/details/dafny-2026-papers/13/A-benchmark-for-vericoding-formally-verified-program-synthesis) [Pre / PR-workshop]
- **AutoVerus** (OOPSLA 2025): a multi-agent LLM system that mimics human proof phases (draft, refine, debug from verifier errors). It proves more than 90% of benchmark tasks, over half in under 30 s or 3 LLM calls — [MSR](https://www.microsoft.com/en-us/research/publication/autoverus-automated-proof-generation-for-rust-code/); [ACM DL](https://dl.acm.org/doi/10.1145/3763174) [PR]
- **AlphaVerus**: self-improving verified Verus generation that bootstraps by translating Dafny programs and uses verifier-guided tree search ("Treefinement"). It reaches 38% on HumanEval-Verified and beats AutoVerus by 10% on proof annotation — [AlphaVerus site](https://alphaverus.github.io/); [OpenReview PDF](https://openreview.net/pdf?id=AyjhgkQDeY) [Pre/PR status unclear]
- **VeruSAGE** (Dec 2025): 849 proof tasks drawn from eight real Verus-verified Rust systems (OS, allocators, storage, distributed). The best LLM-agent combination (o4-mini, GPT-5, Sonnet 4 and Sonnet 4.5 were tested) completes over 80% of tasks, and over 90% of tasks that human experts had *not yet finished* — [arXiv 2512.18436](https://arxiv.org/abs/2512.18436); [MSR](https://www.microsoft.com/en-us/research/publication/verusage-a-study-of-agent-based-verification-for-rust-systems/) [Pre]
- Proof agents in production: Microsoft's SymCrypt team uses agents to translate standards into Lean specs (audited) and to write and maintain proofs, with Lean as the independent checker — [MSR blog](https://www.microsoft.com/en-us/research/blog/verifying-rust-cryptography-in-symcrypt-from-standards-to-code/) [Ind]
- Benchmark and spec-generation proliferation (titles and venues only; contents not read) [Pre]:
  - [VERINA](https://arxiv.org/pdf/2505.23135), benchmarking verifiable code generation
  - [AlgoVeri](https://arxiv.org/html/2602.09464v2)
  - [VeriContest](https://arxiv.org/pdf/2605.08553)
  - [Verus-SpecGym](https://arxiv.org/pdf/2605.26457), specification autoformalization
  - [ATLAS](https://arxiv.org/pdf/2512.10173), large-scale verified code synthesis
  - [MiniF2F-Dafny](https://arxiv.org/pdf/2512.10187)
  - [Interactive proof mode for Dafny](https://arxiv.org/pdf/2512.20486)
  - [A Rust-to-Lean verification pipeline with AI provers](https://arxiv.org/pdf/2605.30106)
  - [Axon verified compiler in Lean built with Claude Code](https://arxiv.org/pdf/2605.01660)
  - [HarnessLLM](https://arxiv.org/html/2607.22161), LLM-generated Kani harnesses

**Constrained decoding and language semantics in the loop**
- **Type-constrained decoding** (PLDI 2025): LLMs "frequently produce uncompilable output" because next-token inference ignores types. Prefix automata plus search over inhabitable types enforce well-typedness during decoding; formalized on a simply-typed calculus and extended to TypeScript — [ACM DL](https://dl.acm.org/doi/10.1145/3729274); [code](https://github.com/eth-sri/type-constrained-code-generation) [PR]
- 2026 follow-ons (titles only, not read) [Pre]:
  - [Generative Compilation: On-the-Fly Compiler Feedback as AI Generates Code](https://arxiv.org/pdf/2607.13921)
  - [Projectional Decoding: Towards Semantic-Aware LLM Generation](https://arxiv.org/pdf/2605.30054)
  - [Schema-derived constrained decoding for MLIR dialects](https://arxiv.org/pdf/2607.18254)
  - [The Alignment Problem in Constrained Code Generation](https://arxiv.org/pdf/2606.21619)
  - [A Completion Semantics for LLM-Based Code Inference](https://arxiv.org/pdf/2607.12490)
  - [Verifiable Literate Programming for human validation of LLM code](https://arxiv.org/abs/2607.02333)

**Languages designed for LLM authors**
- **Vera** (Alasdair Allan; GitHub, v0.1.13, 2,000+ commits):
  - Design thesis: "the model doesn't need to be right, it needs to be checkable".
  - Features: typed De Bruijn indices instead of variable names (`@Int.0`), which removes naming errors; mandatory `requires`/`ensures`/`effects` clauses on every function; algebraic effects tracking IO, HTTP, DB and inference calls; refinement types.
  - Three-tier verification: decidable SMT, then Z3-guided (specified but *not yet implemented*), then runtime checks when static proof fails.
  - Compiles to WebAssembly (CLI, browser, WASI P2, HTTP server).
  - Self-reported: on VeraBench (60 problems), six of nine tested models write 100% correct Vera code without having seen the language in training.

  Source: [GitHub aallan/vera](https://github.com/aallan/vera) [Ind/self-reported; not peer-reviewed]
- Practitioner essays on "programming languages for AI", both 2026 [Op; content not read]:
  - [ploeh blog, "Programming languages for AI" (Mark Seemann, Mar 2026)](https://blog.ploeh.dk/2026/03/30/programming-languages-for-ai/)
  - [AkitaOnRails, "What would be the best programming language for LLMs?" (Feb 2026)](https://akitaonrails.com/en/2026/02/09/ai-agents-best-programming-language-for-llms/)

**AI-driven migration into safe languages**
- AI plus algorithms for C/C++ → Rust migration: Microsoft's research project (Galen Hunt) and DARPA TRACTOR; see Q2 — [Windows Central](https://www.windowscentral.com/microsoft/windows-11/my-goal-is-to-eliminate-every-line-of-c-and-c-from-microsoft-by-2030-microsoft-bets-on-ai-to-finally-modernize-windows); [TRACTOR benchmark](https://arxiv.org/abs/2609.25121)
- Many 2025–2026 preprints on agentic C→Rust translation [Pre]:
  - [ORBIT](https://arxiv.org/pdf/2604.12048)
  - [&inator: Correct, Precise C-to-Rust Interface Translation](https://arxiv.org/pdf/2604.17261)
  - [Mostly Automatic Translation of Language Interpreters from C to Safe Rust](https://arxiv.org/pdf/2606.27122)
  - [Project-Level C-to-Rust Translation via Pointer Knowledge Graphs](https://arxiv.org/pdf/2510.10956)

### Inferences
- **Evidence for "language design matters to LLM success"**: in the same benchmark, the same LLMs show a threefold spread (Dafny 82% vs Lean 27%). The paper's explanations are type-system complexity and training data, both properties of the language and its ecosystem. This is the most direct quantitative evidence that a language built for machine authorship plus automatic checking could beat retrofitted verifiers.
- The research favours **SMT-backed auto-active verification (Dafny style)** over interactive proof (Lean) for LLM-driven code, and **fast verifier feedback** as part of the agent loop. The Verus podcast's 1 s vs 10 s point is the human version of the same argument.
- **Vera** shows the design space is being explored, but only by a hobbyist or independent project with self-reported results. I found no peer-reviewed, general-purpose "language for LLM authors" at PLDI/POPL/OOPSLA 2024–2026. This looks like open space for a small team, grounded in the peer-reviewed pieces: type-constrained decoding, vericoding, AutoVerus, intent formalization.
- Natural-language descriptions did not improve vericoding success. That suggests the formal spec, not prose, is the right interface between humans and agents, which is consistent with Lahiri's intent-formalization agenda.

### Gaps
- I found no peer-reviewed empirical study that compares LLM error rates across *mainstream* languages (e.g., Rust vs Go vs TypeScript) as a function of specific language features (explicit effects, nominal vs structural types, named vs De Bruijn binders). Vera's benchmark is the only direct evidence, and it is self-reported.
- The JetBrains Research March 2025 study that Kleppmann cites was not located directly.
- The contents of the 2026 constrained-decoding preprints were not read (arXiv fetch blocked).

---

## Q5. Promising research ideas still unshipped in a mainstream language (gaps a new language could fill)

### Takeaway
Several peer-reviewed, award-winning ideas are mature enough to build on but have **no mainstream home**:
1. **Typed, lexically scoped effect handlers / effects-as-capabilities** (Effekt, Koka). The runtime mechanism shipped in OCaml 5 and is at Phase 3 for Wasm, but effect *typing* is still being added even at Jane Street.
2. **Lighter-weight ownership**: mutable value semantics (Hylo), modes for data-race freedom (OxCaml), and Swift's `~Escapable`/lifetime dependencies (still experimental).
3. **Refinement types and contracts checked by SMT as a default, not a bolt-on** (Flux, Liquid Haskell, Dafny).
4. **Spec-first / intent-formalization workflows** built into the language and toolchain.
5. **Incremental, live type checking** (Hazel).
6. **Equality saturation (egglog) as the optimizer architecture.**

The combination that research points to, and that nobody ships, is a language where **agents generate code, the language forces explicit specs and effects, and an SMT-backed checker gives fast, automatic verdicts.** Its ownership model should be simpler than Rust's.

### Cited Findings
- **Effect handlers, typed and lexical:**
  - Effekt's design treats effect types as the capabilities a computation needs, and lexical scoping gives modular local reasoning without losing control-flow power — [Effekt](https://effekt-lang.org/); [ACM DL: Effects as capabilities](https://dl.acm.org/doi/10.1145/3428194) [BG][PR]
  - Compilation-efficiency work continues, with "From Capabilities to Regions" (OOPSLA 2023) and "Zero-Overhead Lexical Effect Handlers" and tracing JIT for handlers (OOPSLA 2025) — [ACM DL](https://dl.acm.org/doi/10.1145/3622831); [Effekt publications](https://effekt-lang.org/publications) [PR]
  - A 2025 preprint unifies the row-based and capability-based effect-typing styles — [arXiv 2507.10301, "Rows and Capabilities as Modal Effects"](https://arxiv.org/pdf/2507.10301) [Pre]
  - Mainstream status: OCaml 5 ships handlers, while Jane Street is still "adding an effect system" — [Jane Street](https://www.janestreet.com/tech-talks/effective-programming/). Wasm stack switching is at Phase 3 — [WebAssembly/proposals](https://github.com/WebAssembly/proposals).
- **Mutable value semantics (Hylo)**: a safe, efficient systems language without a borrow checker's reference lifetimes. It is still experimental; the new compiler is in progress and a 1.0 is likely "years" away per community commentary — [GitHub hylo-lang/hylo](https://github.com/hylo-lang/hylo); [Native Implementation of Mutable Value Semantics, arXiv 2106.12678](https://arxiv.org/pdf/2106.12678) [BG]; [The New Stack](https://thenewstack.io/what-you-need-to-know-about-carbon-python-and-val/) [News]
- **Modes for data-race freedom and stack allocation (OxCaml)**: a peer-reviewed POPL 2025 DP, but it lives only in Jane Street's OCaml branch — [ACM DL](https://dl.acm.org/doi/10.1145/3704859); [Tarides](https://tarides.com/blog/2025-07-09-introducing-jane-street-s-oxcaml-branch/)
- **Swift lifetime dependencies**: `@lifetime` is still experimental in 6.2, and strict memory safety is opt-in because legacy unsafe pointer APIs and C interop dominate — [Michael Tsai](https://mjtsai.com/blog/2025/03/19/lifetime-dependencies-in-swift-6-2-and-beyond/); [Swift memory-safety vision](https://github.com/swiftlang/swift-evolution/blob/main/visions/memory-safety.md)
- **Refinement types for a systems language**: Flux shows refinements work "hand in glove" with Rust ownership (strong updates on mutable locations), but it remains an external research checker. The Rust std verification contest shows how far such tools still are from covering a real standard library (7 of 29 challenges solved; 9,600+ generic functions skipped) — [ACM DL Flux](https://dl.acm.org/doi/10.1145/3591283); [verify-rust-std](https://github.com/model-checking/verify-rust-std); [Rust Foundation](https://rustfoundation.org/media/how-the-rust-standard-library-verification-contest-scaled-past-manual-proof-engineering/). A 2025 preprint studies "Refinement-Types Driven Development" — [arXiv 2509.15005](https://arxiv.org/pdf/2509.15005) [Pre]
- **Verification-aware language as primary implementation language**: AWS already writes production logic in Dafny and compiles it to Java — [ICSE 2025](https://dl.acm.org/doi/10.1109/ICSE55347.2025.00166). Dafny is also the language LLMs verify best in (82%) — [arXiv 2509.22908](https://arxiv.org/abs/2509.22908). Neither result has produced a mainstream general-purpose language with Dafny-style contracts as the default.
- **Incremental, live typing**: the OOPSLA 2025 DP gives incremental bidirectional typing for live programming environments (Hazel) — [U. Michigan](https://cse.engin.umich.edu/stories/cse-researchers-win-distinguished-paper-award-at-oopsla-2025)
- **Equality saturation as optimizer architecture**: egglog unifies Datalog and EqSat. It has Python bindings, an MLIR integration (DialEgg, CGO 2025) and a Lean tactic (POPL 2026). The only production compiler use I found is Cranelift's ægraphs — [awesome-egraphs](https://github.com/philzook58/awesome-egraphs); [DialEgg](https://dl.acm.org/doi/10.1145/3696443.3708957)
- **Intent formalization tooling**: Lahiri's open challenges (spec validation metrics, compositional specs over changes, human–AI spec interaction) are explicitly unsolved — [MSR](https://www.microsoft.com/en-us/research/publication/intent-formalization-a-grand-challenge-for-reliable-coding-in-the-age-of-ai-agents/)
- **Typed accelerator programming**: GPU "typed perspectives" (PLDI 2026 DP) and verified tensor compilers (POPL 2025 DP) are research-stage. Mojo 1.0 is the main industrial MLIR-based language — [Saarland PLDI 2026](https://saarland-informatics-campus.de/en/piece-of-news/mpi-researchers-receive-distinguished-paper-award-at-pldi-2026/); [Illinois POPL 2025](https://siebelschool.illinois.edu/news/POPL-2025); [The Register](https://www.theregister.com/ai-and-ml/2026/08/12/modulars-mojo-programming-language-hits-10-milestone/5286545)

### Inferences
- **Candidate pain point with the strongest research backing: "AI writes most new code, but nobody can cheaply trust it."** The research ingredients are peer-reviewed and individually mature:
  - SMT-backed contracts and refinements (Dafny, Flux, Liquid Haskell)
  - typed effects or capabilities to bound what generated code can do (Effekt, Koka, OxCaml modes)
  - a simpler-than-Rust ownership story (mutable value semantics or modes)
  - type- or verifier-constrained decoding
  - Wasm or LLVM as a mature backend

  No mainstream language combines them. Industry currently gets this only by stitching separate tools together (Dafny → Java, Rust → Aeneas → Lean, Rust + Kani). Verus is Rust plus proof annotations, and LLMs do markedly worse on it than on Dafny.
- **Feasibility for a small team**: the pieces a small team can build on are open source and actively developed: Z3/SMT, egglog, Wasm with GC and (soon) stack switching, Lean as an optional backend prover, and the vericoding, VeruSAGE and AutoVerus benchmarks for evaluation. Vera shows a single developer can get a contract- and effect-heavy, LLM-targeted language to a usable prototype. It also shows the hard part, SMT guidance (its "Tier 2"), is still unimplemented there.
- **Risks the report should flag**:
  1. The spec bottleneck: if humans can't validate specs, verified code can still be wrong.
  2. TCB gaps: the runtime or FFI still has to be trusted (the lean-zip example).
  3. Generics and concurrency remain unsolved even for well-funded Rust efforts.
  4. Incumbents are moving: Swift's strict safety, Rust's verification ecosystem, Lean/Dafny with LLM agents. A new language must beat "LLM + Dafny/Verus + existing language" workflows, not just C/C++.

### Gaps
- I found no systematic survey (peer-reviewed) ranking "unshipped PL ideas by readiness"; the gap analysis above is my synthesis.
- I found no quantitative data on developer demand for typed effects or refinement types in mainstream languages (e.g., survey data). The report writer should pull such data from another researcher's notes if available.
- Status of probabilistic and differentiable programming languages in industry (e.g., Stan/Pyro/JAX-style differentiable languages) was not researched because the search budget ran out. Their award presence (PLDI 2026 DP on probabilistic symbolic execution) suggests they are active but, on this evidence, further from a broad developer pain point.
- Query-based and incremental compiler architecture (Salsa/rust-analyzer, Roslyn) was not researched beyond the Hazel OOPSLA 2025 result.
