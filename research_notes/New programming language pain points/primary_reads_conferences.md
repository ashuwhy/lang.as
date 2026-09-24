# Gap fill: conferences and 2026 preprints

Checked on 2026-09-24 from the owner's own machine through Chrome, not from the research
container. Every item below was read on the page named in its URL. Tags:

- **CONFIRMED**: the page says what the notes say.
- **CORRECTED**: the page says something different. Old and new are both given.
- **ADDED**: not in the notes, found while checking, and worth having in the report.
- **COULD NOT VERIFY**: no primary page found.

Accepted-paper counts are rows in the "Accepted Papers" table of the researchr track page.
They can differ by one or two from counts in university press releases, which sometimes
leave out withdrawn or journal-first papers.

Provenance: saved from the local session's commit `f8e4353`, which was never pushed because
the session hit its usage limit in the middle of a merge. These are primary reads and take
precedence over `gap_fill_conferences.md`, which was built from search summaries.

## Headline changes for the report

1. PLDI 2026 had **10** Distinguished Papers, not the 5 the notes list. The 5 listed are all
   correct. The notes' "Accelerator programming is a growing award magnet" still holds and
   gets stronger: Bonsai (tree-traversal query compiler, Ragan-Kelley/Kjolstad) and
   Typed Perspectives are both compiler work for performance hardware.
2. POPL 2026's winners list, marked "not found", is now known (8 papers). Two are on this
   project's axis: "Security Reasoning via Substructural Dependency Tracking" (Gouni,
   Pfenning, Aldrich) and the JavaScript regex mechanized semantics.
3. FlashInfer **did** win an MLSys 2025 Outstanding Paper Award.
4. One quote attributed to de Moura is not in his essay. Drop it.
5. The Qualcomm-Modular "$3.9B" is a Reuters estimate, not a disclosed price.
6. Systems venues keep rewarding language-level safety: OSDI 2025 Best Paper to Omniglot
   (safe FFI from Rust), OSDI 2026 Best Paper to "Controlling Opaque-Component Effects with
   Semisolates and Try", SOSP 2025 Best Paper to CortenMM (memory management "with strong
   correctness guarantees"). These are direct evidence for an effects/capabilities pitch.
7. The two PLDI 2026 keynotes most relevant here are Miryung Kim, "Happiness U-Curve:
   Navigating the AI Validation Bottleneck with Conformance Testing and Proof-Engineering",
   and Saman Amarasinghe, "Programming Language Design and Implementation for the Machine
   Learning Era". The "AI validation bottleneck" is now a PLDI keynote title.

## PLDI

**PLDI 2025**, [track page](https://pldi25.sigplan.org/track/pldi-2025-papers):
- CONFIRMED: 6 Distinguished Papers of 89 accepted.
- CONFIRMED: AWDIT, Tree Borrows, Destabilizing Iris are among them.
- ADDED, the other three: "Practical Type Inference with Levels" (Fan, Xu, Xie);
  "Principal Type Inference under a Prefix: A Fresh Look at Static Overloading" (Leijen,
  Ye); "Verifying General-Purpose RCU for Reclamation in Relaxed Memory Separation Logic"
  (Jung, Park, Lee, Yeon, Kang).
- CORRECTED (keynotes incomplete): all three keynotes, from the
  [program](https://pldi25.sigplan.org/program/program-pldi-2025/):
  - Sukyoung Ryu, "Programming Language Research for Technical and Social Good"
  - Leonardo de Moura, "Lean: Machine-Checked Mathematics and Verified Programming, Past
    and Future" (CONFIRMED)
  - Işıl Dillig, "Neurosymbolic Program Synthesis: Bridging Perception and Reasoning in
    Real-World Applications"

**PLDI 2026**, [track page](https://pldi26.sigplan.org/track/pldi-2026-papers):
- CORRECTED: 10 Distinguished Papers of 106 accepted, not 5.
  - VerusBelt (Hance, Elbeheiry, Matsushita, Dreyer). Also a Distinguished Artifact.
  - Towards Removing Undef Values from LLVM IR (Lobo, McIver, Mitenkov, Lee,
    Sundararajah, Lopes)
  - Modular GPU Programming with Typed Perspectives (Bansal, Sainati, Cutler,
    Amarasinghe, Ragan-Kelley)
  - Categorical Semantics of Probabilistic Symbolic Execution (Li, Czenszak, Holtzen)
  - Verification of Recursively Defined Quantum Circuits (Ying, Zhang)
  - ADDED: Bonsai: Compiling Queries to Pruned Tree Traversals (Root, Gyurgyik, Goel,
    Fatahalian, Ragan-Kelley, Adams, Kjolstad)
  - ADDED: Cobble: Compiling Block Encodings for Quantum Computational Linear Algebra
    (Yuan)
  - ADDED: Enumerating Ill-Typed Programs for Testing Type Analyzers (Sotiropoulos, Su)
  - ADDED: MatchBox: A Semantic Foundation for Data Plane Portability (Campbell, Zhang,
    Saxena, Akella, Dillig)
  - ADDED: Synthesizing Backward Error Bounds, Backward (Zielinski, Hsu)
- The first five authors lists match the notes except the Ying paper, which also has
  Zhicheng Zhang, and the VerusBelt author order (Matsushita before Dreyer).
- ADDED, keynotes from the [keynotes track](https://pldi26.sigplan.org/track/pldi-2026-pldi-keynotes):
  - Saman Amarasinghe (MIT), "Programming Language Design and Implementation for the
    Machine Learning Era: A Personal Perspective"
  - Miryung Kim (UCLA and AWS), "Happiness U-Curve: Navigating the AI Validation
    Bottleneck with Conformance Testing and Proof-Engineering"
  - Aws Albarghouthi (Wisconsin), "The Rise & Collapse of a Quantum State"

## POPL

**POPL 2025**, [track page](https://popl25.sigplan.org/track/POPL-2025-popl-research-papers):
- CONFIRMED: 7 Distinguished Papers. The track page lists 82 papers against the notes' 81
  (from TU Delft). Minor.
- CONFIRMED: Data Race Freedom à la Mode; Relaxed Memory Concurrency Re-executed;
  Barendregt Convenes with Knaster and Tarski.
- CORRECTED: the "tensor-compiler verification paper (Mendis, Arora)" is
  "TensorRight: Automated Verification of Tensor Graph Rewrites" (Arora, Lu, Jain, Xu,
  Houshmand, ...).
- ADDED: "Affect: An Affine Type and Effect System" (van Rooij, Krebbers); "A Primal-Dual
  Perspective on Program Verification Algorithms" (Tsukada, Unno, Padon, Shoham);
  "Guaranteed Bounds on Posterior Distributions of Discrete Probabilistic Programs with
  Loops" (Zaiser, Murawski, Ong).

**POPL 2026**, [track page](https://popl26.sigplan.org/track/POPL-2026-popl-research-papers):
- CORRECTED ("winners list not found"): 8 Distinguished Papers of 92 accepted.
  - All for One and One for All: Program Logics for Exploiting Internal Determinism in
    Parallel Programs (Moine, Westrick, Tassarotti)
  - An Equational Axiomatization of Dynamic Threads via Algebraic Effects (Kammar, ...)
  - Encode the Cake and Eat It Too: Controlling Computation in Type Theory, Locally
    (Leray, Winterhalter)
  - Formal Verification for JavaScript Regular Expressions: A Proven Mechanized Semantics
    and Its Applications (Barrière, Deng, Pit-Claudel)
  - Normalisation for First-Class Universe Levels (Danielsson, Favier, Kubánek)
  - Probabilistic Concurrent Reasoning in Outcome Logic (Zilberstein, Silva, Tassarotti)
  - Quotient Polymorphism (Hewer, Hutton)
  - Security Reasoning via Substructural Dependency Tracking (Gouni, Pfenning, Aldrich)
- Not re-checked: the Dafny 2026 workshop listing for the vericoding paper, and Lean-egg at
  POPL 2026.

## OOPSLA, ICFP, SPLASH 2025

- CONFIRMED: "Incremental Bidirectional Typing via Order Maintenance" won an OOPSLA 2025
  Distinguished Paper Award, [U. Michigan CSE, 31 Oct 2025](https://cse.engin.umich.edu/stories/cse-researchers-win-distinguished-paper-award-at-oopsla-2025).
  The post adds that the OCaml implementation gets "multiple orders of magnitude speedup"
  over non-incremental checking.
- COULD NOT VERIFY: the full OOPSLA 2025 Distinguished Paper list. The
  [OOPSLA track page](https://2025.splashcon.org/track/OOPSLA) (213 papers listed) marks no
  award winners, and its "OOPSLA Awards" item is only a program slot.
- CONFIRMED on the same page: AutoVerus, Tracing JIT for Effects and Handlers, Zero-Overhead
  Lexical Effect Handlers, and Dynamic Wind for Effect Handlers are OOPSLA 2025 papers.
- CONFIRMED: Frank Piessens gave a SPLASH 2025 keynote ("Software Stacks for Confidential
  Computing Hardware"). The other keynote names were not re-checked.
- ADDED, **ICFP 2025**, [track page](https://icfp25.sigplan.org/track/icfp-2025-papers):
  4 Distinguished Papers of 36 accepted.
  - Call-Guarded Abstract Definitional Interpreters (Germane)
  - Effectful Lenses: There and Back with Different Monads (Xie, Schrijvers, Hu)
  - First-Order Laziness (Lorenzen, Leijen, Swierstra, Lindley)
  - Multi-stage Programming with Splice Variables (Chiang, Xie)
- CONFIRMED: LMPL 2025, "The 1st International Workshop on Language Models and Programming
  Languages", co-located with ICFP/SPLASH 2025, [LMPL 2025](https://conf.researchr.org/home/icfp-splash-2025/lmpl-2025).

## ECOOP 2025

- ADDED: the [ECOOP 2025 awards page](https://2025.ecoop.org/track/ecoop-2025-awards) lists
  only the AITO Dahl-Nygaard Prizes: Senior to Mira Mezini (TU Darmstadt), Junior to Amir
  Shaikhha (Edinburgh).
- COULD NOT VERIFY: ECOOP 2025 paper awards. The
  [technical papers track](https://2025.ecoop.org/track/ecoop-2025-technical-papers) lists
  43 papers and marks none as award winners.

## CAV 2025

From the [program](https://conferences.i-cav.org/2025/program) and
[award page](https://conferences.i-cav.org/2025/award):
- ADDED: CAV Award 2025 to Armoni, Beer, Ben-David, Eisner, Fisman, Fix, Havlicek,
  Landver, Miller and Vardi, for the temporal logics behind ForSpec, Sugar, PSL and SVA.
  This is industrial specification languages being honoured, which is relevant to the
  "spec language" argument.
- ADDED: six Distinguished Papers.
  - Introducing Certificates to the Hardware Model Checking Competition (Froleyks, ...)
  - A Misconception-Driven Adaptive Tutor for Linear Temporal Logic (Prasad, ...)
  - The Vampire Diary (Bartek, ...)
  - The rIC3 Hardware Model Checker (Su, ...)
  - Approximating Fixpoints of Approximated Functions (Baldan, ...)
  - Robust Probabilistic Bisimilarity for Labelled Markov Chains (van Breugel, ...)
- ADDED: keynotes by Corina Păsăreanu (semantic analysis of neural networks) and Emina
  Torlak, and an industry panel, "Formal Methods in the GenAI Era".

## CGO and CC

- CONFIRMED: DialEgg (Zayed, Dubach) is a CGO 2025 main-conference paper,
  [CGO 2025 track](https://2025.cgo.org/track/cgo-2025-papers) (48 papers).
- ADDED: CGO 2025 ran a "Distinguished Papers" session with three talks: "Synthesis of
  Sorting Kernels" (Ullrich, Hack); "Tensorize: Fast Synthesis of Tensor Programs from
  Legacy Code using Symbolic Tracing, Sketching and Solving" (Brauckmann et al.); and
  "Enhancing Deployment-time Predictive Model Robustness for Code Analysis and
  Optimization" (Wang, Lenihan, Wang). A session with that name usually holds the
  nominees, so treat these as nominees unless a winners list says otherwise.
- COULD NOT VERIFY: CGO 2026 awards. The
  [CGO 2026 track](https://2026.cgo.org/track/cgo-2026-papers) lists 56 papers and marks
  no winners.
- ADDED: the HPCA/CGO/PPoPP/CC 2026 plenary keynotes included Saman Amarasinghe, "Compiler
  2.0: Building the Next Generation Compilers with Machine Learning" (Ken Kennedy Award
  talk), and Cristina Cifuentes (Oracle Parfait),
  [keynotes track](https://2026.cgo.org/track/hpca-cgo-ppopp-cc-2026-plenary-keynotes).
- COULD NOT VERIFY: CC 2025 awards. The
  [CC 2025 track](https://conf.researchr.org/track/CC-2025/CC-2025-main-conference) lists
  17 papers and no awards. CC 2026 has no papers track page on researchr.

## Systems venues

**OSDI 2025**, [technical sessions](https://www.usenix.org/conference/osdi25/technical-sessions):
- ADDED: Best Papers were "Basilisk: Using Provenance Invariants to Automate Proofs of
  Undecidable Protocols" (Zhang, Singh, Chajed, Kapritsos, Parno) and "Building Bridges:
  Safe Interactions with Foreign Languages through Omniglot" (Schuermann, Toubes,
  Potyondy, Pannuto, Milano, Levy).
- ADDED: Distinguished Artifact to "PoWER Never Corrupts: Tool-Agnostic Verification of
  Crash Consistency and Corruption Detection" (LeBlanc, Lorch, Hawblitzel, ... MSR).
- ADDED: keynote by Emery Berger, "Accelerating Software Development: The LLM (R)evolution".
- CONFIRMED: Mirage: A Multi-Level Superoptimizer for Tensor Programs (Wu, Cheng, ... CMU)
  is an OSDI 2025 paper.

**OSDI 2026**, [technical sessions](https://www.usenix.org/conference/osdi26/technical-sessions):
- ADDED: Best Papers.
  - "Controlling Opaque-Component Effects with Semisolates and Try" (Lamprou, Zhu, Jin,
    Ntousakis, Liargkovas, Eng, Kallas, Greenberg, Vasilakis). Also a Distinguished
    Artifact.
  - "ValScope: Value-Semantics-Aware Metamorphic Testing for Detecting Logical Bugs in
    DBMSs" (Lin, Chen, Wu)
  - "Teaching the Old Dog New Tricks: Building Efficient Data Pipelines for Large-Scale
    LLM Pre-Training" (Operational Systems track)
- ADDED: OSDI 2026 also has "jwmalloc: A Verified Memory Allocator for Mobile Devices" and
  "Neuro-Symbolic Proof Generation for Scaling Systems Software Verification".
- Keynote: "Analysis for Better Resilience" (speaker not extracted).

**SOSP 2025**, [schedule](https://sigops.org/s/conferences/sosp/2025/schedule.html):
- ADDED: three Best Paper Awards.
  - "Prove It to the Kernel: Precise Extension Analysis via Proof-Guided Abstraction
    Refinement" (Hao Sun, Zhendong Su, ETH Zurich)
  - "CortenMM: Efficient Memory Management with Strong Correctness Guarantees" (Zhang,
    ... Tian, Zhou; PKU, Ant Group)
  - "How to Copy Memory? Coordinated Asynchronous Copy as a First-Class OS Service" (He,
    ... Xia, Chen; SJTU, Huawei)
- ADDED: the [accepted list](https://sigops.org/s/conferences/sosp/2025/accepted.html)
  includes "Atmosphere: Practical Verified Kernels with Rust and Verus" and "TickTock:
  Verified Isolation in a Production Embedded OS" (Jhala, Stefan et al.). Verified-Rust
  systems papers are now routine at SOSP.

**ASPLOS 2025**, [awards](https://www.asplos-conference.org/asplos2025/awards/):
- ADDED: Best Papers were CXLfork; xUI: extended User Interrupts; H-Houdini: Scalable
  Invariant Learning; MetaSapiens; Orion (FHE for deep learning); SmoothE: Differentiable
  E-Graph Extraction.
- CONFIRMED: Exo 2: Growing a Scheduling Language is an ASPLOS 2025 paper
  ([arXiv 2411.07211](https://arxiv.org/abs/2411.07211), comment "To appear in ASPLOS
  2025"). The abstract's own figure: scheduling libraries cover "more than 80
  high-performance kernels, reducing total scheduling code by an order of magnitude".

**ASPLOS 2026**, [awards](https://www.asplos-conference.org/asplos2026/awards/):
- ADDED: Best Papers were CounterPoint; Finding Reusable Instructions via E-Graph
  Anti-Unification; Lifetime-Aware Design of Item-Level Intelligence; PF-LLM: LLM Hinted
  Hardware Prefetching; vCXLGen.
- ADDED: honourable mentions include "Linear Layouts: Robust Code Generation of Efficient
  Tensor Computation Using F₂" (the Triton layout work), "Graphiti: Formally Verified
  Out-of-Order Execution in Dataflow Circuits", "Highly Automated Verification of Security
  Properties for Unmodified System Software", MSCCL++ and RedFuser.
- E-graphs won an ASPLOS Best Paper in both 2025 and 2026, which supports the notes'
  "e-graphs keep winning" point.

**MLSys**:
- CONFIRMED: FlashInfer won an **MLSys 2025 Outstanding Paper Award**, alongside "The Hidden
  Bloat in Machine Learning Systems", [MLSys 2025 awards](https://mlsys.org/virtual/2025/awards_detail).
  Honourable mentions: LAVA, Marconi, Seesaw, COMET, APOLLO.
- CONFIRMED: HipKittens: Fast and Furious AMD Kernels (Hu, ... Ré, Arora) is an MLSys 2026
  paper, given as a research-track oral in "Compilers and Kernels",
  [MLSys 2026 papers](https://mlsys.org/virtual/2026/papers.html?search=HipKittens).
- COULD NOT VERIFY: MLSys 2026 awards. The
  [awards page](https://mlsys.org/virtual/2026/awards_detail) exists but is empty.

## 2026 preprints and industry items

- CONFIRMED, **Lahiri, "Intent Formalization"**,
  [arXiv 2603.17150](https://arxiv.org/abs/2603.17150): submitted 17 Mar 2026 by Shuvendu
  Lahiri, 10 pages. The abstract matches the notes: intent formalization is "the key
  challenge that will determine whether AI makes software more reliable or merely more
  abundant"; the bottleneck is "validating specifications"; the five open challenges are
  listed as in the notes.
- CONFIRMED, **vericoding benchmark**, [arXiv 2509.22908](https://arxiv.org/abs/2509.22908):
  submitted 26 Sep 2025 by Max Tegmark. 12,504 specs (3,029 Dafny, 2,334 Verus/Rust, 7,141
  Lean), 6,174 new. Success "27% in Lean, 44% in Verus/Rust and 82% in Dafny". Natural-
  language descriptions do not significantly help. Pure Dafny verification rose "from 68%
  to 96% over the past year". The submitter being Tegmark settles the `[TK]` on the group's
  affiliation. The per-model detail (Opus 4.1 best on Dafny, GPT-5 on Verus and Lean) is
  not in the abstract and was not checked in the body.
- CONFIRMED, **VeruSAGE**, [arXiv 2512.18436](https://arxiv.org/abs/2512.18436): v1 20 Dec
  2025, v2 15 Apr 2026. 849 tasks from eight open-source Verus-verified Rust systems;
  o4-mini, GPT-5, Sonnet 4 and Sonnet 4.5; best combination completes "over 80%", and
  "over 90%" of tasks not yet finished by human experts.
- CONFIRMED, **Microsoft SymCrypt blog**,
  [MSR blog, 13 July 2026](https://www.microsoft.com/en-us/research/blog/verifying-rust-cryptography-in-symcrypt-from-standards-to-code/):
  complete proofs for Rust ML-KEM and SHA3 "being used in insiders builds of Windows
  today"; AES-GCM, FrodoKEM and ML-DSA planned.
- CONFIRMED, **SymCrypt technical report**,
  [arXiv 2609.15648](https://arxiv.org/abs/2609.15648), "Scaling Verification of
  Cryptographic Software with Aeneas, Rust, and Lean", submitted 14 Sep 2026 by Son Ho:
  "Our 237 KLOC Lean development establishes safety, panic-freedom, and functional
  correctness of 16.7 KLOC of Rust". The ~14:1 ratio in the notes is right. The abstract
  also says agents "autonomously write formal proofs" while formalizing standards "still
  requires expert design and review".
- CONFIRMED with corrections, **de Moura, "When AI Writes the World's Software, Who Verifies
  It?"**, 28 Feb 2026,
  [leodemoura.github.io](https://leodemoura.github.io/blog/2026-2-28-when-ai-writes-the-worlds-software-who-verifies-it/).
  The `/blog/2026/02/28/...html` URL in frontier_pl_research.md redirects here.
  - CONFIRMED: 25-30% AI-generated new code at Google and Microsoft; 40 million lines of
    COBOL for Toyota; Microsoft's CTO predicting 95% by 2030; "qualitatively different";
    "trust infrastructure"; a trusted kernel of "a few thousand lines"; "Independent
    verification is not a philosophical preference. It is a security architecture
    requirement." (The notes paraphrase this as "not a philosophy but".)
  - CORRECTED: "Mathematical proof must replace traditional review - not supplement it,
    replace it" **does not appear in the essay**. The nearest line is "replace human
    friction with mathematical friction: let AI move fast, but make it prove its work."
    Remove the quote from frontier_pl_research.md.
  - CORRECTED context: "No one is formally verifying the result" is real, but it refers to
    Anthropic's AI-built C compiler (100,000 lines, two weeks, under $20,000), not to
    industry AI code in general.
  - ADDED: the essay opens with Code Metal raising $125 million to rewrite defense code
    with AI.
  - CONFIRMED with detail: the zlib-to-Lean experiment was run at the Lean FRO by Kim
    Morrison, using "Claude, a general-purpose AI, with no special training for theorem
    proving". The capstone theorem is decompress after compress is identity, for inputs
    under 1 GiB.
  - CONFIRMED: Lean in production at AWS (Cedar) and Microsoft (SymCrypt); Lean received
    the 2025 ACM SIGPLAN Programming Languages Software Award.
- CONFIRMED with updates, **Vera**, [GitHub aallan/vera](https://github.com/aallan/vera):
  415 stars (unchanged). Open issues now 226 (was 232). Commits 2,699 (notes: "2,000+").
  VeraBench is "a 60-problem benchmark across 5 difficulty tiers" covering 9 models from 3
  providers. The self-reported "six of nine models at 100%" was not re-checked.
- CONFIRMED, **Descend**: "Descend: A Safe GPU Systems Programming Language" (Köpcke,
  Gorlatch, Steuwer) is a PLDI 2024 paper,
  [PLDI 2024 track](https://pldi24.sigplan.org/track/pldi-2024-papers). The abstract says
  "no significant runtime overhead ... compared to manually written CUDA programs". The
  notes' "performance matching CUDA" is a slight overstatement.
- CONFIRMED, **HipKittens** at MLSys 2026, see above.
- CONFIRMED, **Mirage** at OSDI 2025, see above. The MPK 1.2x-6.7x figure is from the
  GitHub README and was not re-checked.
- CONFIRMED, **Exo 2** at ASPLOS 2025, see above.
- CONFIRMED, **Mojo 1.0**: the
  [v1.0.0 release notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.0.0.md)
  are dated 2026-08-11 and say most core features are stable and stdlib APIs are being
  marked stable "beginning with a deliberately small set". The modular/modular repo shows
  29.9k stars. Modular's blog index lists "Mojo is now open source!" on 18 Aug 2026.
- CONFIRMED with correction, **Qualcomm-Modular**:
  - Announced 24 June 2026, [Modular blog](https://www.modular.com/blog/qualcomm-to-acquire-modular)
    and [Qualcomm IR](https://investor.qualcomm.com/news-events/press-releases/news-details/2026/Qualcomm-to-Acquire-Modular/default.aspx).
  - Completed 29 July 2026, [Nasdaq/PR Newswire](https://www.nasdaq.com/press-release/qualcomm-completes-acquisition-modular-2026-07-29).
    Mojo, MAX and Modular Cloud continue as brands. Lattner becomes "Executive Vice
    President of Advanced AI Software and Platforms".
  - CORRECTED: neither press release states a price or share count. CNBC:
    "Qualcomm didn't provide the financials", Bloomberg reported talks at "nearly $4
    billion", and "Reuters calculations valued the acquisition at $3.92 billion",
    [CNBC, 24 Jun 2026](https://www.cnbc.com/2026/06/24/qualcomm-ai-chip-modular-software.html).
    The WSJ headline says "$3.9 Billion Stock Deal". Write "about $3.9B (Reuters
    estimate; all-stock)" in the report.
  - COULD NOT VERIFY: the share count. performance_and_gpu.md says "18M shares"; a
    secondary outlet says "up to 19.2 million". Neither primary release gives one. Leave
    it out.
  - The Phoronix-headline caveat in new_languages_adoption.md can be dropped: the
    acquisition is confirmed.
