# Recent New Programming Languages and the History of Language Adoption

*How these notes were gathered (2026-09-24): Most web pages could not be fetched because the egress proxy blocked them, and the session's web-search budget ran out partway through. Where possible, primary sources were therefore read directly from the language projects' own website and source repositories on GitHub (git clone / raw.githubusercontent.com). Examples: ziglang.org news, gleam.run news, blog.rust-lang.org, go.dev/blog, swift.org blog, nim-lang.org, crystal-lang.org, elixir-lang.org, unison-lang.org, kotlinlang.org docs, Modular's Mojo release notes, and the Carbon roadmap. I cite the canonical public URL of each page, which is built from the site's source path. Facts that come only from web-search snippets (and could not be opened) are marked "[search snippet]". GitHub star counts come from the GitHub search API on 2026-09-24. Release dates marked "tag date" are the commit dates of the git release tags, which is a proxy for the release date.*

---

## Q1. Language-by-language profiles: goal, backing, team/funding, 2025–2026 status, and why each succeeded or stalled

### Takeaway
The languages that reached real adoption either rode a platform or ecosystem they did not have to build (Kotlin/Android, Swift/iOS, TypeScript/JavaScript, Gleam/BEAM+JS, Elixir/BEAM, Mojo/Python syntax, Zig as a C/C++ toolchain) or had a well-funded corporate sponsor (Go, Rust, Swift, Kotlin, TypeScript). Independent languages that stalled usually lacked such a host ecosystem, never shipped a stable release (Jai, Roc, Carbon, Pony, Odin), or lost funding or momentum (Vale was archived and relaunched as Valen; Austral has been dormant since mid-2025; Hylo lost its lead designer to retirement). 2026's newest entrants are overwhelmingly "languages for AI agents" (Vercel Zero, BAML, Weft, Bend 2) or languages built *by* AI agents (Rue).

### Cited Findings

#### Snapshot: GitHub stars and latest tagged versions (retrieved 2026-09-24)
- Go 138,970★ ([golang/go](https://github.com/golang/go)); Rust 119,104★ ([rust-lang/rust](https://github.com/rust-lang/rust)); TypeScript 111,178★, and the repo's primary language is now listed as Go after the TS 7 native port ([microsoft/TypeScript](https://github.com/microsoft/TypeScript)); Swift 70,395★ ([swiftlang/swift](https://github.com/swiftlang/swift)); Kotlin 53,453★ ([JetBrains/kotlin](https://github.com/JetBrains/kotlin)); Julia 49,149★ ([JuliaLang/julia](https://github.com/JuliaLang/julia)) — GitHub search API, 2026-09-24.
- Zig 43,303★ on the now read-only GitHub repo, description "Moved to Codeberg" ([ziglang/zig](https://github.com/ziglang/zig)); V 37,911★ ([vlang/v](https://github.com/vlang/v)); Carbon 33,901★ ([carbon-language/carbon-lang](https://github.com/carbon-language/carbon-lang)); Modular (Mojo + MAX) 29,875★ ([modular/modular](https://github.com/modular/modular)); Elixir 26,677★ ([elixir-lang/elixir](https://github.com/elixir-lang/elixir)); Bend 22,621★ ([bendlang/bend](https://github.com/bendlang/bend)); Gleam 21,940★ ([gleam-lang/gleam](https://github.com/gleam-lang/gleam)); Crystal 20,421★ ([crystal-lang/crystal](https://github.com/crystal-lang/crystal)); Nim 18,243★ ([nim-lang/Nim](https://github.com/nim-lang/Nim)); Odin 12,004★ ([odin-lang/Odin](https://github.com/odin-lang/Odin)).
- Pkl 11,531★ ([apple/pkl](https://github.com/apple/pkl)); Lean 4 9,287★ ([leanprover/lean4](https://github.com/leanprover/lean4)); BAML 9,280★ ([BoundaryML/baml](https://github.com/BoundaryML/baml)); Elm 7,904★ ([elm/compiler](https://github.com/elm/compiler)); Unison 6,737★ ([unisonweb/unison](https://github.com/unisonweb/unison)); Pony 6,191★ ([ponylang/ponyc](https://github.com/ponylang/ponyc)); Roc 6,069★, with Zig now its primary implementation language ([roc-lang/roc](https://github.com/roc-lang/roc)); C3 5,835★ ([c3lang/c3c](https://github.com/c3lang/c3c)); Vercel Zero 5,373★, created 2026-05-15 ([vercel-labs/zerolang](https://github.com/vercel-labs/zerolang)); Borgo 4,634★ ([borgo-lang/borgo](https://github.com/borgo-lang/borgo)); Koka 4,071★ ([koka-lang/koka](https://github.com/koka-lang/koka)); Dafny 3,556★ ([dafny-lang/dafny](https://github.com/dafny-lang/dafny)); jank 3,333★ ([jank-lang/jank](https://github.com/jank-lang/jank)).
- MoonBit: docs 2,440★, core 1,217★, compiler 706★, and the `moon` build tool 423★ ([moonbitlang org search](https://github.com/moonbitlang)). Vale 2,017★ ([ValeLang/Vale](https://github.com/ValeLang/Vale)); Weft 1,978★, created 2026-04-15 ([WeaveMindAI/weft](https://github.com/WeaveMindAI/weft)); Austral 1,579★, last push 2025-07-28 ([austral/austral](https://github.com/austral/austral)); Hylo 1,557★ ([hylo-lang/hylo](https://github.com/hylo-lang/hylo)); Inko 1,304★ ([inko-lang/inko](https://github.com/inko-lang/inko)); Rue 1,198★, created 2025-06-05 ([rue-language/rue](https://github.com/rue-language/rue)); Gren (an Elm fork) 504★ ([gren-lang/compiler](https://github.com/gren-lang/compiler)); Valen 337★, created 2026-08-15 ([valen-lang/Valen](https://github.com/valen-lang/Valen)).
- Latest stable tags seen via `git ls-remote`: Gleam v1.18.1; Elixir v1.20.4; Crystal v1.21.0; Nim v2.2.12; Lean v4.34.0; Dafny v4.11.0; Koka v3.2.9; C3 v0.8.4; Pkl 0.32.1; Bend v2.0.27; Pony 0.72.1 (tag date 2026-09-13); Zig 0.15.2 on the GitHub mirror (later releases, if any, are on Codeberg and were not checked). V publishes weekly tags (`weekly.2026.08`). Carbon and Roc have no release tags — the respective GitHub repos: [gleam](https://github.com/gleam-lang/gleam), [elixir](https://github.com/elixir-lang/elixir), [crystal](https://github.com/crystal-lang/crystal), [Nim](https://github.com/nim-lang/Nim), [lean4](https://github.com/leanprover/lean4), [dafny](https://github.com/dafny-lang/dafny), [koka](https://github.com/koka-lang/koka), [c3c](https://github.com/c3lang/c3c), [pkl](https://github.com/apple/pkl), [bend](https://github.com/bendlang/bend), [ponyc](https://github.com/ponylang/ponyc), [zig](https://github.com/ziglang/zig), [v](https://github.com/vlang/v).

#### Corporate-, institution- or VC-backed newcomers
- **Mojo (Modular; Chris Lattner and Tim Davis).**
  - *Goal:* Python syntax with systems-level performance and Rust-like memory safety, built on MLIR to target CPUs, GPUs, TPUs and ASICs — [Open Source For You](https://www.opensourceforu.com/2026/08/modular-launches-mojo-language/) [search snippet].
  - *1.0 release:* Mojo 1.0 shipped on 2026-08-11 with stability policies. Most core language features are now stable, standard-library APIs are being marked stable starting with "a deliberately small set", and nearly every breaking change ships with a deprecated alias plus a compiler fix-it — [Mojo v1.0.0 release notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.0.0.md). Mojo 1.1.0 followed on 2026-09-17 ([v1.1.0 notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.1.0.md)).
  - *Open source:* the compiler was open-sourced under Apache 2.0 on 2026-08-18. Modular will not accept compiler contributions until about the end of 2026, and Windows support is coming through a collaboration with Microsoft — [linuxiac](https://linuxiac.com/mojo-programming-language-goes-fully-open-source/), [The Register](https://www.theregister.com/ai-and-ml/2026/08/12/modulars-mojo-programming-language-hits-10-milestone/5286545) [search snippets]. The repo tree now contains `Mojo/lib`, `Mojo/include` and `Mojo/tools`, consistent with a compiler-source release ([modular/modular](https://github.com/modular/modular)).
  - *Distribution:* Mojo is installed via the `modular` package from pip or conda, i.e. through Python's own distribution channels — [Mojo README](https://github.com/modular/modular/blob/main/Mojo/README.md).
  - *Unverified:* a Phoronix headline says the open-sourcing came "Following Qualcomm Acquisition" of Modular — [Phoronix](https://www.phoronix.com/news/Modular-Mojo-Open-Source) [headline only; not verified].
- **Carbon (Google-led, experimental C++ successor).**
  - *2025 goals:* demonstrate C++ interop (most non-template C++ APIs usable from Carbon) and produce a concrete memory-safety design.
  - *Timeline:* "the end of 2026 is now the *soonest* that 0.1 could realistically be ready," because memory safety was added to the 0.1 scope. 0.2 is planned for 2027–2028, 1.0 for "beyond 2028", plus a future foundation "separate from any corporate entities that fund work on Carbon" — [Carbon roadmap](https://github.com/carbon-language/carbon-lang/blob/trunk/docs/project/roadmap.md).
  - *Reach vs. use:* 33.9k★ with no 0.1 release, 6 years after the repo was created ([repo](https://github.com/carbon-language/carbon-lang)).
- **Verse (Epic Games).**
  - A functional-logic language and the main scripting language for Unreal Editor for Fortnite (UEFN), shipped in UEFN on 2023-03-23 and presented at Haskell eXchange in December 2022.
  - Simon Peyton Jones joined Epic in December 2021 with Lennart Augustsson to work on it, and it is expected to converge into the general Unreal codebase — [Wikipedia: Verse](https://en.wikipedia.org/wiki/Verse_(programming_language)) [search snippet].
  - Its adoption is captive: it is the scripting language of a platform with a built-in creator economy.
- **MoonBit (IDEA — International Digital Economy Academy, Shenzhen; led by Hongbo Zhang).**
  - Zhang was an OCaml core contributor, created ReScript, and contributed to Flow at Meta. MoonBit is billed as "AI-native". Its compiler, including the WASM backend, was open-sourced in December 2024, and 1.0 was targeted for 2026 — [MoonBit about](https://www.moonbitlang.com/about-us/), [Hivemind Medium](https://medium.com/@hivemind_tech/moonbit-language-in-10-features-4dc41a3a1d6c) [search snippets].
  - There is an academic design paper, "MoonBit: Explore the Design of an AI-Friendly Programming Language" (LLM4Code 2024) — [dblp](https://dblp.org/rec/conf/llm4code/FeiZZWL24.html).
  - It ships official Agent Skills for Codex CLI, Claude Code and Copilot — [moonbit-agent-guide](https://github.com/moonbitlang/moonbit-agent-guide) — and a "DeepSeek-backed MoonBit coding agent", openseek, created 2026-05 ([moonbitlang/openseek](https://github.com/moonbitlang/openseek)).
- **Lean 4 (Lean FRO).**
  - The Lean Focused Research Organization launched in July 2023 under Convergent Research, funded by the Simons Foundation, Sloan and Richard Merkin — [Lean FRO about](https://lean-lang.org/fro/about/) [search snippet].
  - In July 2025, Alex Gerko (XTX Markets) gave $10M: $5M for a new Mathlib Initiative and $5M to the Lean FRO — [Renaissance Philanthropy](https://www.renaissancephilanthropy.org/insights/lean-fro-and-mathlib-receive-10m-from-xtx-markets-founder-alex-gerko-to-further-advance-the-use-of-ai-for-mathematical-research) [search snippet].
  - In 2025 the FRO shipped the `grind` tactic and a new compiler, and the Lean VS Code extension passed 100,000 installs. As of June 2024, Mathlib had 1.56M lines from 300+ mathematicians — [Lean FRO Year 2 roadmap](https://lean-lang.org/fro/roadmap/y2/), [Wikipedia: Lean](https://en.wikipedia.org/wiki/Lean_(proof_assistant)) [search snippets].
  - *Why it succeeded:* philanthropic funding, plus a niche (formal mathematics and AI-for-math) where no mainstream language competes.
- **Dafny (verification-aware; heavily used at AWS).**
  - AWS rewrote its authorization engine in Dafny, compiled it to Java, and deployed it in 2024 "without incident", with 3× better performance. The engine handles about 1 billion calls per second and was differentially tested against about 10^15 production samples — [Amazon Science, ICSE 2025](https://www.amazon.science/publications/formally-verified-cloud-scale-authorization) [search snippet].
  - LLM assistance works well for verification annotations: DafnyPro reaches 86% correct proofs on DafnyBench with Claude 3.5 Sonnet — [POPL 2026 Dafny workshop](https://popl26.sigplan.org/details/dafny-2026-papers/12/DafnyPro-LLM-Assisted-Automated-Verification-for-Dafny-Programs) [search snippet].
  - *Interop lesson:* Dafny compiles to existing targets (Java etc.), so it plugs into an existing runtime rather than replacing it.
- **Bend / HVM (Higher Order Company — Victor Taelin).**
  - HOC closed a $5M seed "in about one week of meetings". In January 2025 it announced a post-seed round to turn its SupGen program synthesizer into a "Symbolic Transformer" aimed at ARC-AGI, and it later raised on Wefunder ($4M at a $60M valuation) "to bring Bend2 to market", including an LSP, editor and docs — [Wefunder](https://wefunder.com/higherorderco/), [HOC on X](https://x.com/higherordercomp/status/1885319917932605501?lang=en) [search snippets].
  - The repo moved to bendlang/bend (22.6k★) and is now "Bend 2: a fast language that blocks AI mistakes via proof", i.e. repositioned from GPU parallelism to a language for verifying AI-written code — [bendlang/bend](https://github.com/bendlang/bend); [Bend README](https://raw.githubusercontent.com/HigherOrderCO/Bend/main/README.md) ("In the post-AGI economy, humans will eventually stop writing and reading code...").
- **Unison (Unison Computing).**
  - Code is stored as a content-addressed AST in a database rather than as text files, giving "perfect" incremental compilation. Unison 1.0 was announced 2025-11-25, about 10 years after the repo was created in 2015, alongside Unison Cloud BYOC (run on your own infrastructure) — [Announcing Unison 1.0](https://www.unison-lang.org/unison-1-0/), [InfoWorld](https://www.infoworld.com/article/4100673/futuristic-unison-functional-language-debuts.html) [search snippet], [repo](https://github.com/unisonweb/unison).
  - *Business model:* the language is funded by a hosted-cloud product.
- **Koka (Microsoft Research; Daan Leijen).**
  - A research language with effect types and handlers and Perceus reference counting. The README says it is "not quite ready for production use"; latest release v3.2.9 on 2026-09-17 — [Koka README](https://github.com/koka-lang/koka).
  - *Impact:* its value has been as a source of ideas (effect handlers, Perceus) rather than as a language people adopt.

#### Community-, solo- or small-team systems languages
- **Zig (Andrew Kelley; Zig Software Foundation, a 501(c)(3)).**
  - *2024 finances:* income $670,672.59 — GitHub Sponsors $170,656; Mitchell Hashimoto $150,000, half of a $300k pledge; Every.org $90,097; Bun $60,000; TigerBeetle $60,000; ZML $33,000; Blacksmith $14,000. Expenses $520,748.91, of which 92% went to contributors: contractors at $60/hour ($306,362) plus one employee, Kelley ($154,263).
  - *Funding pressure:* donations were in "slow decline", and "with our current level of recurring income, we will not be able to renew everyone's contracts" — [ZSF 2025 Financial Report](https://ziglang.org/news/2025-financials/).
  - *Demand pressure:* by the same report, average time to close issues grew to over a year in the past month, and stars went from 22 (Jan 2016) to 40,757 (Aug 2025) — [same report](https://ziglang.org/news/2025-financials/).
  - *Codeberg move:* on 2025-11-26 Zig moved its canonical repo from GitHub to Codeberg, citing GitHub Actions neglect and AI pushes. It treats GitHub Sponsors as "a liability" and asks donors to move to Every.org — [Migrating from GitHub to Codeberg](https://ziglang.org/news/migrating-from-github-to-codeberg/).
  - *AI policy:* Zig enforces a "Strict No LLM / No AI Policy" for issues, PRs and comments — [Zig Code of Conduct](https://ziglang.org/code-of-conduct/#strict-no-llm-no-ai-policy).
  - *Why it succeeded:* the `zig cc` drop-in C/C++ cross-compiler (Zig bundles libcs for many targets) plus flagship users that fund it (Bun, TigerBeetle, Ghostty's Hashimoto) — [ZSF report income table](https://ziglang.org/news/2025-financials/).
  - *Status:* 0.1.0 tag dated 2017-10-17, 0.15.2 on the GitHub mirror; still no 1.0 about 11 years after `git init` ([ziglang/zig](https://github.com/ziglang/zig)). 64% "admired" in the Stack Overflow 2025 survey ([SO 2025 Technology](https://survey.stackoverflow.co/2025/technology)) [search snippet].
- **Odin (Bill "gingerBill" Hall).**
  - Started July 2016 as a data-oriented C alternative. Its anchor user is JangaFX, whose EmberGen, GeoGen and LiquiGen are written in Odin and used by studios including Bethesda, CAPCOM, Codemasters, Warner Bros and Weta — [Wikipedia: Odin](https://en.wikipedia.org/wiki/Odin_(programming_language)) [search snippet]; [Odin showcase: EmberGen](https://odin-lang.org/showcase/embergen/).
  - Not self-hosted, deliberately, and still pre-1.0: "self hosting before a stable language and compiler exists is masturbatory pleasure" — [Odin FAQ](https://odin-lang.org/docs/faq/).
- **Jai (Jonathan Blow / Thekla).**
  - Development began in 2014 and it is still a closed, invite-only beta. The 0.2.026 beta shipped 2026-02-21, and a public release is expected "soon after" Blow's game *Order of the Sinking Star*, which is written in Jai and scheduled for 2026-10-08 — [Wikipedia: Jai](https://en.wikipedia.org/wiki/Jai_(programming_language)), [Mr. Phil Games, "Jai in 2026"](https://www.mrphilgames.com/blog/jai-in-2026) [search snippets].
  - *Pattern:* the language's "killer app" is the author's own game, but about 12 years without public access means almost no ecosystem.
- **V (vlang; Alexander Medvednikov).**
  - 37.9k★, but its 2019 launch drew "vaporware" criticism over unmet claims (e.g. a 400 KB compiler claim vs a 3.7 MB binary) — [Xe, "V is for Vaporware"](https://christine.website/blog/v-vaporware-2019-06-23), [Lobsters](https://lobste.rs/s/1ogeev/v_is_for_vaporware) [search snippets].
  - Stable release 0.5.1 on 2026-03-09 — [Wikipedia: V](https://en.wikipedia.org/wiki/V_(programming_language)) [search snippet]; weekly tags continue ([vlang/v](https://github.com/vlang/v)).
  - *Lesson:* stars can be earned by marketing, but over-promising costs credibility.
- **Nim (Andreas Rumpf "Araq"; backed partly by Status).**
  - 1.0 tag dated 2019-09-23 ([nim-lang/Nim](https://github.com/nim-lang/Nim)). A new compiler, Nimony, is meant to become Nim 3.0 ("extend Nimony until it compiles most Nim 2.0 code out there and then call it Nim 3.0") — [Nimony](https://nim-lang.org/araq/nimony.html) [search snippet].
  - *2024 community survey:* only 367 responses, "less than in previous years". 17% of respondents had stopped using Nim; only 15% were new users ("we're not attracting enough new users"); tooling became the #1 requested priority. Non-users cite "immature, not ready for production", "doesn't have libraries I need", and "not enough learning materials" — [Nim Community Survey 2024 Results](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
- **Crystal (Manas.Tech; Ruby-like syntax, compiled).**
  - Repo from 2012; 1.0.0 tag dated 2021-03-22 ([crystal-lang/crystal](https://github.com/crystal-lang/crystal)).
  - *Funding model:* the sponsoring company Manas.Tech plus paid work from users. 84codes (maker of the LavinMQ message broker) funds multi-threading support — [84codes and Manas partner](https://crystal-lang.org/2024/02/09/84codes-manas-mt/) — and in July 2025 Manas launched "Crystal Compass", a paid support/code-review subscription — [Crystal Compass](https://crystal-lang.org/2025/07/08/crystal-compass/).
  - *Leadership:* the project lead stepped down in September 2025 ("over the past year I've already been a bit distant from the project due to Manas' own dynamics") — [Wind of change](https://crystal-lang.org/2025/09/29/wind-of-change/).
- **Hylo (formerly Val; Dave Abrahams and Dimi Racordon).**
  - Mutable value semantics plus generic programming. Effort has moved to a new compiler (the hylo-new repo has only 43★) — [Hylo README](https://github.com/hylo-lang/hylo), [hylo-new](https://github.com/hylo-lang/hylo-new).
  - Abrahams "is retired and focused on his music" while keeping a hand in — [search summary of hylo-lang.org / talks](http://hylo-lang.org/) [search snippet].
- **Vale (Evan Ovadia).**
  - Proposed generational references and regions. Donations and sponsorships were paused — [vale.dev](https://vale.dev/), [GitHub Sponsors](https://github.com/sponsors/ValeLang) [search snippets].
  - The README now says "Vale is archived and no longer being worked on"; it lives on as **Valen** ("Vale with a new memory safety approach plus Rust interop"), created 2026-08-15 — [Vale README](https://github.com/ValeLang/Vale), [Valen](https://github.com/valen-lang/Valen).
  - *Lesson:* the relaunch explicitly adds interop with an existing ecosystem (Rust).
- **Austral (Fernando Borretti).**
  - Linear types plus capability-based security. Last push 2025-07-28, i.e. dormant for more than a year — [Austral README](https://github.com/austral/austral), [GitHub metadata](https://github.com/austral/austral).
- **Pony (ponylang).**
  - Actor model with reference capabilities. "Still pre-1.0 and as such, semi-regularly introduces breaking changes", though "used in production environments" — [ponyc README](https://github.com/ponylang/ponyc). Pony 0.72.1 is tagged 2026-09-13.
- **Roc (Richard Feldman).**
  - "Not ready for a 0.1 release yet" about 7 years after the repo was created (2019). Funded by the Roc Programming Language Foundation (Every.org, GitHub Sponsors, Liberapay) and corporate sponsors Lambda Class, ohne-makler and Decem — [Roc README](https://github.com/roc-lang/roc).
  - *Rust→Zig rewrite:* started early 2025, citing Rust's slow compile times — [Linked List](https://linkedlist.org/2025/02/06/roc-zig-rewrite), [HN discussion](https://news.ycombinator.com/item?id=42935516) [search snippets]. One secondary write-up reports 487 days and about 300k lines, with 35 ms incremental rebuilds in Zig vs 3.4 s in Rust — [Developers Digest](https://www.developersdigest.tech/blog/roc-rust-to-zig-rewrite-feldman) [search snippet; secondary source, unverified].
  - The repo now ships `AGENTS.md` coding rules for AI agents — [Roc AGENTS.md](https://github.com/roc-lang/roc/blob/main/AGENTS.md).

#### Functional / BEAM languages
- **Gleam (Louis Pilfold).**
  - Timeline: repo created 2016-06-30 ([repo](https://github.com/gleam-lang/gleam)); public "Hello, Gleam!" on 2019-04-15 ([gleam.run](https://gleam.run/news/hello-gleam/)); v1.0.0 on 2024-03-04, about 8 years in ([Gleam version 1](https://gleam.run/news/gleam-version-1/)).
  - *Funding:* "one full-time developer working on Gleam (me!)". The work is funded entirely by GitHub Sponsors, with Fly.io providing "approximately half the funding", and Pilfold earns "less than half" the median London lead-developer salary — [Gleam version 1](https://gleam.run/news/gleam-version-1/). Most sponsors give $5–$20/month, and Gleam is his "sole source of income" — [Convenient code actions, 2024-09](https://gleam.run/news/convenient-code-actions/).
  - *Survey and reception:* 70% "admired" in its first Stack Overflow survey appearance (2025), second only to Rust's 72% — [SO 2025](https://survey.stackoverflow.co/2025/technology), [byteiota](https://byteiota.com/gleam-hits-70-admiration-in-stack-overflow-survey-2025/) [search snippets].
  - *Community:* the first all-Gleam conference (Gleam Gathering, Bristol) was held in 2026, with a 2027 London edition announced — [Gleam Gathering 2027](https://gleam.run/news/gleam-gathering-2027-is-coming-to-london/).
  - *Why it succeeded:* it runs on two existing runtimes (the BEAM/OTP and JavaScript), it is small ("learn in an afternoon"), and it shipped the whole toolchain in one binary (see Q5).
- **Elixir (José Valim; originally at Plataformatec).**
  - Repo 2011; v1.0.0 tag dated 2014-09-10 ([elixir-lang/elixir](https://github.com/elixir-lang/elixir)).
  - *Type system:* v1.20 (2026-06-03) made Elixir "a gradually typed language" — every program is type-checked without annotations. The work came from a CNRS + Remote partnership and is sponsored by Fresha and Tidewave — [Elixir v1.20 released](https://elixir-lang.org/blog/2026/06/03/elixir-v1-20-0-released/).
  - *Interop push:* in 2025 the project promoted interoperability "beyond the Erlang VM" (Rust/Zig NIFs, Python, Swift; AtomVM/WASM via Popcorn) — [Interoperability in 2025](https://elixir-lang.org/blog/2025/08/18/interop-and-portability/).
  - 66% admired, SO 2025 — [SO 2025](https://survey.stackoverflow.co/2025/technology) [search snippet].
- **Elm (Evan Czaplicki).**
  - 0.19.1 was tagged 2019-10-20 and the next tag, 0.19.2, on 2026-04-28 — a 6.5-year gap between releases ([elm/compiler tags](https://github.com/elm/compiler/releases/tag/0.19.2)). The Gren fork (504★) exists partly because of that pace ([gren-lang/compiler](https://github.com/gren-lang/compiler)).
  - *Side effect:* Gleam cites Elm as a type-system inspiration, so Elm's stall fed its successors ([Gleam version 1](https://gleam.run/news/gleam-version-1/)).

#### Established reference cases (for timelines)
- **Kotlin (JetBrains).**
  - Started 2010, "open source from very early on"; 1.0 in February 2016 — [Kotlin FAQ](https://kotlinlang.org/docs/faq.html).
  - Android has been "Kotlin-first since Google I/O in 2019". Over 50% of professional Android developers use Kotlin as their primary language (vs 30% Java), and over 95% of the top thousand Android apps use it — [Kotlin for Android](https://kotlinlang.org/docs/android-overview.html).
- **Swift (Apple).** Still expanding beyond Apple platforms:
  - Swift SDK for Android previews, using `swift-java`'s `jextract` for JNI bindings — [Exploring the Swift SDK for Android, 2025-12-18](https://www.swift.org/blog/exploring-the-swift-sdk-for-android/).
  - A Windows workgroup (2026-01) — [swift.org blog](https://www.swift.org/blog/announcing-windows-workgroup/).
  - The Swift extension published on Open VSX so it works in Cursor, VSCodium, Kiro and Antigravity (2026-04-08) — [Expanding Swift IDE support](https://www.swift.org/blog/expanding-swift-ide-support/).
  - Swift 6.4 released 2026-09-15 — [Swift 6.4 Released](https://www.swift.org/blog/swift-6.4-released/).
- **TypeScript (Microsoft).** The native Go port is complete. The TypeScript 7 staging repo says "the native port process... is now completed", and the command name is `tsc` from TS 7.0 RC on — [microsoft/typescript-go README](https://github.com/microsoft/typescript-go).
- **Rust (Mozilla, then the Rust Foundation).**
  - "Road to Rust 1.0" was posted 2014-09-15; Rust 1.0 shipped 2015-05-15 — [Road to Rust 1.0](https://blog.rust-lang.org/2014/09/15/Rust-1.0/), [Announcing Rust 1.0](https://blog.rust-lang.org/2015/05/15/Rust-1.0/).
  - The 2025 State of Rust survey had 7,156 responses, slightly down year over year. It found fewer people attending communities to learn — "hints at some people moving their questions to LLM tooling" — and organisations still hiring Rust developers — [2025 State of Rust Survey Results](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/).
  - 72% admired, #1 in SO 2025 — [SO 2025](https://survey.stackoverflow.co/2025/technology) [search snippet].
- **Go (Google).**
  - Open-source release 2009-11-10 ([Go's Sweet 16](https://go.dev/blog/16years)); Go 1 blog post dated 2012-03-28 ([go.dev/blog/go1](https://go.dev/blog/go1)).
  - The 2025 survey (5,379 respondents) found 91% satisfied and 53% using AI tools daily; only 17% use agents as their primary mode — [Results from the 2025 Go Developer Survey](https://go.dev/blog/survey2025).
  - gopls now ships an experimental built-in MCP server exposing its functionality to AI assistants — [Go's Sweet 16](https://go.dev/blog/16years).
- **Julia.** v1.0.0 tag dated 2018-08-08 ([JuliaLang/julia](https://github.com/JuliaLang/julia)). Its Slack has over 15,000 members ([julialang.org community page source](https://github.com/JuliaLang/www.julialang.org)).
- **Python.** "Created in the early 1990s by Guido van Rossum at Stichting Mathematisch Centrum (CWI)... as a successor of a language called ABC" — [Python docs: History and License](https://docs.python.org/3/license.html).
- **Ruby.** Public release 1995; "In 2006, Ruby achieved mass acceptance", with "much of the growth... attributed to... Ruby on Rails" — [About Ruby](https://www.ruby-lang.org/en/about/).

#### Notable 2025–2026 newcomers (many aimed at AI agents)
- **Vercel Zero / "zerolang" (Vercel Labs).**
  - v0.1.2 launched 2026-05-15 as an experimental systems language. Features aimed at agents: sub-10 KiB native binaries, JSON diagnostics with stable error codes, typed repair plans (`zero fix --plan --json`), capability-based I/O through an explicit `World` parameter, and one CLI with `--json` on every subcommand — [MarkTechPost](https://www.marktechpost.com/2026/05/17/vercel-labs-introduces-zero-a-systems-programming-language-designed-so-ai-agents-can-read-repair-and-ship-native-programs/) [search snippet].
  - By v0.3.4 (Sept 2026) it had become "graph-native": `zero.graph` is the checked source of truth, `.0` text files are human-readable projections, and agents use `zero query` and `zero patch` with stale-hash rejection — [zerolang README](https://github.com/vercel-labs/zerolang). 5.4k★ in about 4 months.
- **Rue (Steve Klabnik + Claude).**
  - "Higher level than Rust but lower level than Go". Implemented in Rust, it emits native code without LLVM (x86-64 and AArch64) and comes with a spec plus fuzz and differential test suites — [Rue README](https://github.com/rue-language/rue).
  - "Rue is being developed by Steve Klabnik, but also by Claude" — [Rue blog: Hello, World! (2025-12-21)](https://github.com/rue-language/rue/blob/trunk/website/content/blog/hello-world.md). Klabnik reports a baby language going from zero to "core basics of a language + spec with two different codegen backends... in roughly a week" — [The story of Rue so far](https://github.com/rue-language/rue/blob/trunk/website/content/blog/the-story-of-rue-so-far.md).
  - *Autonomous work:* a 2026-06 post reports three days of largely autonomous agent work — 95 merged PRs and about 150 filed issues. It also found that the green test suite had been "lying": internal compiler errors counted as passing tests, and the ownership model was "sound on paper and unsound in the binary" — [An Agent Holds the Fort](https://github.com/rue-language/rue/blob/trunk/website/content/blog/an-agent-holds-the-fort.md).
- **Weft (WeaveMind).** "A programming language and framework for AI orchestration" (1,978★ since 2026-04). It compiles a graph of LLM, agent, human and API nodes into a Rust crate and bundles an AI assistant ("Tangle"): "You never have to learn this language" — [Weft README](https://github.com/WeaveMindAI/weft).
- **BAML (BoundaryML).** "The programming language for agents"; 9,280★; a DSL for structured LLM calls — [BoundaryML/baml](https://github.com/BoundaryML/baml).
- **Pel.** A research language for orchestrating AI agents, inspired by Lisp, Elixir, Gleam and Haskell — [arXiv 2505.13453](https://arxiv.org/pdf/2505.13453) [search snippet].
- **Others seen in 2025–2026:** Pkl (Apple config language, 11.5k★, [apple/pkl](https://github.com/apple/pkl)); Valen (above); C3 (5.8k★, v0.8.4, [c3lang/c3c](https://github.com/c3lang/c3c)); jank (native Clojure with C++ interop, 3.3k★, [jank-lang/jank](https://github.com/jank-lang/jank)); Borgo (a Rust-like language that compiles to Go, 4.6k★, [borgo-lang/borgo](https://github.com/borgo-lang/borgo)).

### Inferences
- Stars correlate poorly with shipped value. Carbon (33.9k★, no 0.1), V (37.9k★) and Bend (22.6k★) out-star Gleam (21.9k★), which has a v1, a conference and production users. Use stars as a measure of hype, not adoption.
- The "better X" languages that are working all live *inside* X's ecosystem: TypeScript/JS, Kotlin/JVM, Gleam/BEAM+JS, Mojo/Python packaging, Carbon/C++ interop, Borgo/Go, jank/C++, Valen/Rust interop. Standalone "better C" languages (Odin, Jai, V, C3, Hare) stay niche unless they have an anchor product (EmberGen, Blow's game) or a toolchain wedge (`zig cc`).
- Dormancy and pivots are the modal outcome for solo research-grade systems languages (Vale → archived → Valen; Austral dormant; Hylo reduced; Koka research-only). Pivots in 2025–2026 increasingly reframe the language as "for AI" (Bend 2's "blocks AI mistakes via proof"; Zero's graph-native agent workflow).
- For a small Rust-based interpreter project, the closest analogues are Gleam (one full-time developer, sponsor-funded, hosted on existing runtimes), Roc (a foundation plus a few corporate sponsors) and Rue (a solo developer with an AI agent, native codegen in Rust). Gleam took about 8 years to reach 1.0 and about 9 to reach wide admiration. Rue shows AI agents can compress the build-a-compiler phase to weeks, but not the ecosystem or trust phase.

### Gaps
- Could not verify Modular's funding amounts or the reported Qualcomm acquisition (only a Phoronix headline was seen). Mojo adoption numbers (users, downloads) were not found.
- MoonBit: whether 1.0 actually shipped in 2026 is not confirmed (no version tags in the public repos; only "targeting 2026" snippets). IDEA's team size and budget were not found.
- Verse: no 2025–2026 status or usage figures (e.g. number of UEFN creators using Verse) were found; the open-sourcing status is unclear.
- Carbon, Hylo, Koka, Austral, Pony, Odin: no team-size or funding figures found beyond the above.
- Julia (JuliaHub funding), Swift and TypeScript adoption statistics for 2025–2026 could not be fetched. GitHub Octoverse 2025, which I believe reported TypeScript becoming #1 by contributors in 2025, could not be verified — treat as unverified.
- An Anthropic acquisition of Bun (relevant to Zig's "killer app" funding story) could not be verified this session; the search budget ran out before it could be checked.
- The Gleam "~4 employees" and "$440K ARR" figures appeared only on getlatka.com, an unreliable auto-generated source, and are excluded.
- The Stack Overflow 2025 admiration percentages come from a search snippet of survey.stackoverflow.co; the page itself could not be opened to get usage percentages.

---

## Q2. Adoption patterns (killer apps, platforms, corporate backing, interop) and research on why developers adopt or switch

### Takeaway
The only large empirical study (Meyerovich & Rabkin, OOPSLA 2013) found that adoption is driven by **extrinsic** factors: open-source libraries, existing code, and existing team or personal expertise. Intrinsic features (safety, simplicity, even performance) rank low. Developers switch languages based on *domain*, not linguistic similarity. The historical winners fit this: each attached to a platform or killer app that supplied libraries and a reason to switch (Rails, the browser, iOS, Android, cloud infrastructure, the C toolchain), usually with corporate money behind it.

### Cited Findings
- **Meyerovich & Rabkin, "Empirical Analysis of Programming Language Adoption", OOPSLA 2013.**
  - *Data:* 213,471 SourceForge projects, 590,000 Ohloh projects, and surveys of 1,000–13,000 programmers — [paper PDF](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf), [ACM DL](https://dl.acm.org/doi/10.1145/2509136.2509515).
  - *Power law:* "language adoption follows a power law; a small number of languages account for most language use, but the programming market supports many languages with niche user bases" — [paper](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - *Libraries dominate:* "Open source libraries, existing code, and experience strongly influence developers when selecting a language... Language features such as performance, reliability, and simple semantics do not." Open-source libraries were the most influential factor ("strong" or "medium" for over 60% of respondents); simplicity was least influential (25%) — [paper, §4.1](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - *Social factors:* "Existing code or expertise with the language are four of the top five factors for adoption" — [paper](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - *Company size:* libraries are "the most influential factor for commercial projects at small companies". Larger companies weight legacy code and group experience more, and smaller companies "are likely more willing to adopt new languages that are not backward-compatible" — [paper](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - *Switching:* "developer movement between languages is driven more by external factors such as the developer's background or technical ecosystem than by similarity of the underlying languages. This implies that language advocates should focus on a domain and try to convince programmers in that domain" — [paper, §3](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - *Designer advice:* "language designers and advocates should emphasize libraries... such as numpy for numerical programming in Python, and Ruby on Rails for web applications." Developers "prioritize expressivity over correctness" and "see more value in unit tests than types" — [paper, §9 and abstract](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
- **Killer-app / platform wins:**
  - *Ruby / Rails:* Ruby went public in 1995 and reached "mass acceptance" in 2006, with growth "attributed to... Ruby on Rails" — [About Ruby](https://www.ruby-lang.org/en/about/).
  - *Kotlin / Android:* Kotlin-first since Google I/O 2019; over 95% of the top 1,000 Android apps; over 50% of professional Android developers use it as their primary language — [Kotlin for Android](https://kotlinlang.org/docs/android-overview.html).
  - *Zig / C toolchain and anchor products:* 2024 donors include Bun ($60k), TigerBeetle ($60k), ZML ($33k) and Mitchell Hashimoto ($150k) — [ZSF 2025 report](https://ziglang.org/news/2025-financials/). Roc chose Zig for its compiler rewrite — [Linked List](https://linkedlist.org/2025/02/06/roc-zig-rewrite) [search snippet].
  - *Odin / EmberGen:* JangaFX's products are the anchor application — [Wikipedia: Odin](https://en.wikipedia.org/wiki/Odin_(programming_language)) [search snippet].
  - *Verse / Fortnite:* Verse is the mandatory scripting language of UEFN — [Wikipedia: Verse](https://en.wikipedia.org/wiki/Verse_(programming_language)) [search snippet].
  - *Lean / formal mathematics:* Mathlib (1.56M lines) is the killer library, and AI-for-math funding followed — [Renaissance Philanthropy](https://www.renaissancephilanthropy.org/insights/lean-fro-and-mathlib-receive-10m-from-xtx-markets-founder-alex-gerko-to-further-advance-the-use-of-ai-for-mathematical-research) [search snippet].
- **Interop / host-ecosystem wins:**
  - *Gleam:* "runs on the Erlang virtual machine... [and] JavaScript runtimes". In the 2024 survey, 571 respondents used the Erlang target and 297 the JavaScript target; among production users the JS target is used even more (for frontends) — [Gleam v1](https://gleam.run/news/gleam-version-1/), [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
  - *Gleam draws from outside the BEAM:* respondents' other languages were TypeScript 385, JavaScript 338, Python 314, Rust 243, Go 230, Elixir 158. "Gleam folk overwhelmingly come from other ecosystems, often ones that already use static types" — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
  - *Dafny:* AWS compiles Dafny to Java and deploys it into an existing Java service — [Amazon Science](https://www.amazon.science/publications/formally-verified-cloud-scale-authorization) [search snippet].
  - *Carbon:* its whole 2025 plan is C++ interop — [Carbon roadmap](https://github.com/carbon-language/carbon-lang/blob/trunk/docs/project/roadmap.md).
  - *Swift:* reaching Android through Java interop tooling (`jextract`, `wrap-java`) — [swift.org](https://www.swift.org/blog/exploring-the-swift-sdk-for-android/).
  - *Valen:* relaunched with "Rust interop" — [Vale README](https://github.com/ValeLang/Vale).
  - *Elixir:* pushing interop with Rust, Zig, Python and Swift, plus WASM via Popcorn — [Elixir interop 2025](https://elixir-lang.org/blog/2025/08/18/interop-and-portability/).
- **Corporate backing:**
  - Carbon plans a foundation separate from its corporate funders only in 2027–2028 — [Carbon roadmap](https://github.com/carbon-language/carbon-lang/blob/trunk/docs/project/roadmap.md).
  - Mojo is company-built (Modular) and reached 1.0 about 3 years after its 2023 debut — [Mojo 1.0 notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.0.0.md), [Open Source For You](https://www.opensourceforu.com/2026/08/modular-launches-mojo-language/) [search snippet].
  - Kotlin is developed by JetBrains — [Kotlin FAQ](https://kotlinlang.org/docs/faq.html).
  - Elixir's type system is funded by CNRS and Remote plus sponsors Fresha and Tidewave — [Elixir v1.20](https://elixir-lang.org/blog/2026/06/03/elixir-v1-20-0-released/).
- **Who adopts first:** Gleam's production users are "more likely to be in smaller organisations... larger organisations being more risk-averse" — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/). This matches Meyerovich & Rabkin's finding on company size ([paper](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf)).
- **What keeps developers on a language:** Go survey respondents value Go for the domains it "does nicely support via stdlib and built-in tooling", and 91% were satisfied — [Go 2025 survey](https://go.dev/blog/survey2025).

### Inferences
- A solo or small team cannot out-library an incumbent, so the viable adoption moves are: (a) **borrow a library ecosystem** through a host runtime or seamless FFI (Gleam, Kotlin, TypeScript, Mojo); (b) **own a narrow domain** where incumbents are weak (Lean for mathematics, Verse for UEFN, Dafny for verified components, BAML/Weft for LLM orchestration); or (c) **become infrastructure** people use without "adopting the language" (the `zig cc` pattern).
- Meyerovich & Rabkin's "switch by domain, not syntax" result implies marketing to users of semantically similar languages (e.g. "Rust but easier") is weaker than targeting a domain's pain.
- Small companies and individuals are the realistic first adopters (Gleam, M&R). Large-company adoption tends to follow only with a sponsor-grade trust signal (Google, Apple, JetBrains, AWS, Epic).

### Gaps
- No post-2013 large-scale academic replication of Meyerovich & Rabkin was found in this session.
- No primary data (only historical knowledge) was gathered on JavaScript/browser, Go/Docker/Kubernetes, or TypeScript-superset adoption; web fetch was blocked and search was exhausted. These well-known narratives should be marked as uncited background in the final report.
- High-signal HN threads exist (e.g. the [Zig 2025 financial report thread](https://news.ycombinator.com/item?id=45111405), the [Roc rewrite thread](https://news.ycombinator.com/item?id=42935516), a [V criticism thread](https://news.ycombinator.com/item?id=29836360)) but could not be read.

---

## Q3. Solo and tiny-team successes: how long they took and how they were funded

### Takeaway
Nearly every language that started with one person took **5–12 years** to reach 1.0 or mass acceptance. Most funded the core developer through **donations and sponsorship at below-market pay**, usually with one or two anchor corporate donors providing a large share (Fly.io for Gleam, Hashimoto/Bun/TigerBeetle for Zig). A few took a company-sponsored route (Crystal via Manas; Odin effectively via JangaFX). Donation funding is fragile: Zig's donations declined in 2024–25, Vale's sponsorships paused and the project was archived, and Gleam's corporate users rarely pay.

### Cited Findings
- **Timelines (repo or first release → 1.0 or mass acceptance):**
  - *Gleam:* repo 2016 → v1.0 on 2024-03-04, about 8 years ([repo](https://github.com/gleam-lang/gleam), [Gleam v1](https://gleam.run/news/gleam-version-1/)).
  - *Zig:* `git init` about 2015 → 0.1.0 in 2017-10 → still 0.15.x in 2025–26, with no 1.0 after about 11 years ([Codeberg post](https://ziglang.org/news/migrating-from-github-to-codeberg/), [zig tags](https://github.com/ziglang/zig)).
  - *Elixir:* repo 2011 → v1.0.0 on 2014-09-10, about 3.5 years, with a company (Plataformatec) behind it ([elixir tags](https://github.com/elixir-lang/elixir)).
  - *Crystal:* repo 2012 → 1.0.0 on 2021-03-22, about 9 years ([crystal tags](https://github.com/crystal-lang/crystal)).
  - *Nim:* repo 2010 → v1.0.0 on 2019-09-23, about 9 years ([Nim tags](https://github.com/nim-lang/Nim)).
  - *Julia:* repo 2011 → v1.0.0 on 2018-08-08, about 7 years ([julia tags](https://github.com/JuliaLang/julia)).
  - *Kotlin:* started 2010 → 1.0 in February 2016, about 6 years, company-backed ([Kotlin FAQ](https://kotlinlang.org/docs/faq.html)).
  - *Unison:* repo 2015 → 1.0 on 2025-11-25, about 10 years ([Unison 1.0](https://www.unison-lang.org/unison-1-0/)).
  - *Ruby:* public 1995 → "mass acceptance" 2006, about 11 years ([About Ruby](https://www.ruby-lang.org/en/about/)).
  - *Python:* "created in the early 1990s" by one person at CWI ([Python docs](https://docs.python.org/3/license.html)).
  - *Odin:* 2016 → still pre-1.0 in 2026 ([Odin FAQ](https://odin-lang.org/docs/faq/)).
  - *Roc:* 2019 → no 0.1 in 2026 ([Roc README](https://github.com/roc-lang/roc)).
  - *Jai:* 2014 → closed beta in 2026 ([Wikipedia: Jai](https://en.wikipedia.org/wiki/Jai_(programming_language)) [search snippet]).
  - *Pony:* 2012 → still pre-1.0 in 2026 ([ponyc README](https://github.com/ponylang/ponyc)).
  - *Elm:* 2012 → 0.19.1 in 2019 → 0.19.2 in 2026 ([elm tags](https://github.com/elm/compiler/releases/tag/0.19.2)).
- **Gleam funding details:**
  - One full-time developer (Pilfold), funded via GitHub Sponsors with Fly.io providing about half. He earns under half the London median lead-developer salary and wants "to financially reward the regular contributors" — [Gleam v1](https://gleam.run/news/gleam-version-1/).
  - He returned to full-time Gleam work in May 2021 — [Gleam v0.15](https://gleam.run/news/gleam-v0.15-released/).
  - 2024 survey: 146 of 801 respondents sponsor Gleam, but only 1 of 38 production users' organisations does ("much less success getting sponsorship from corporate users") — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
  - Adoption signal in its first post-v1 year: 632 of 805 respondents use Gleam and about 8% (52) run it in production — [same](https://gleam.run/news/developer-survey-2024-results/).
- **Zig funding details:**
  - 2024 income $670,672, expenses $520,749, 92% to contributors at $60/hour, one employee. Monthly donations ran about $17k–$68k, apart from a $195k spike from the Hashimoto pledge. Cash on hand fell from about $288k (Nov 2024) to about $95k (Aug 2025) — [ZSF 2025 report](https://ziglang.org/news/2025-financials/).
  - "GitHub Sponsors... was key to Zig's early fundraising success" but is now considered "a liability" — [Codeberg post](https://ziglang.org/news/migrating-from-github-to-codeberg/).
- **Roc:** a 501(c)(3)-style foundation (donations via Every.org), GitHub Sponsors, Liberapay, and three corporate sponsors — [Roc README](https://github.com/roc-lang/roc).
- **Crystal:** company-backed (Manas.Tech), with customer-funded features (84codes multithreading) and a paid support subscription (Crystal Compass) — [84codes post](https://crystal-lang.org/2024/02/09/84codes-manas-mt/), [Crystal Compass](https://crystal-lang.org/2025/07/08/crystal-compass/).
- **Vale:** sponsor-funded → "donations and sponsorships... paused" → archived, then relaunched as Valen in 2026 — [vale.dev](https://vale.dev/) [search snippet], [Vale README](https://github.com/ValeLang/Vale).
- **Unison:** company-backed, monetised via Unison Cloud, including BYOC at 1.0 — [Unison 1.0](https://www.unison-lang.org/unison-1-0/), [InfoWorld](https://www.infoworld.com/article/4100673/futuristic-unison-functional-language-debuts.html) [search snippet].
- **Bend:** VC and crowdfunding ($5M seed, then a Wefunder raise of $4M at a $60M valuation) — [Wefunder](https://wefunder.com/higherorderco/) [search snippet].
- **AI-assisted solo development (new in 2025–26):** Rue got to a core language, a spec and two codegen backends in "roughly a week" of one person's spare time with Claude — [The story of Rue so far](https://github.com/rue-language/rue/blob/trunk/website/content/blog/the-story-of-rue-so-far.md). A later model then ran three days of multi-agent "hunt/fix" loops (95 PRs merged) — [An Agent Holds the Fort](https://github.com/rue-language/rue/blob/trunk/website/content/blog/an-agent-holds-the-fort.md).

### Inferences
- The realistic benchmark for a solo language reaching v1 plus a small production community is **about 5–9 years** (Gleam 8, Kotlin 6 with a company, Elixir 3.5 with a company, Crystal and Nim about 9). Company backing shortens this; donations alone do not.
- A realistic funding ceiling for a well-loved independent language is **about $0.5–0.7M/year** (Zig, the most successful donation-funded case) and **one modestly paid full-time developer** (Gleam). A single anchor corporate sponsor supplying about half the budget is the common pattern and a concentration risk.
- AI agents plausibly compress the *compiler-building* part of the timeline from years to weeks (Rue), but the survey data (Gleam, Nim) says the bottlenecks are libraries, docs, production guidance and trust — none of which an agent-built compiler supplies.
- Early production users skew toward small organisations that rarely pay (Gleam: 1 of 38). A plan that relies on corporate sponsorship should include a paid product (Unison Cloud, Crystal Compass) or an anchor product (JangaFX/Odin, Bun/Zig).

### Gaps
- Could not verify Elm's funding history (e.g. Evan Czaplicki's employment by NoRedInk and later Vendr), Python's and Ruby's early funding, or Odin's exact JangaFX funding relationship. These are commonly cited but unconfirmed here.
- No data found on Gleam's total annual sponsorship revenue (the getlatka figure is unreliable).
- No figures on Andrew Kelley's or Bill Hall's pre-foundation funding.

---

## Q4. Why most new languages fail: ecosystem gap, tooling expectations, no niche, "better X" without a 10× reason, funding, and the LLM low-resource problem

### Takeaway
New languages fail mainly for **extrinsic** reasons: no libraries, immature tooling, no learning material, perceived production risk, and funding that cannot sustain maintainers. In 2025–2026 there is an additional penalty: brand-new languages are effectively "no-resource" for LLMs, and most developers now code with AI assistance (53% daily among Go developers). Recent languages counter this with agent skills, LLM-friendly docs, machine-readable compiler output, and in-language verification.

### Cited Findings
- **Ecosystem and libraries:**
  - Libraries are the #1 adoption factor, and simplicity the last — [Meyerovich & Rabkin](https://lmeyerov.github.io/projects/socioplt/papers/oopsla2013.pdf).
  - Nim non-users cite "Nim seems immature, not ready for production", "doesn't have libraries I need", "doesn't have enough learning materials", and "seems too risky for production", in that order — [Nim 2024 survey](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
- **Tooling expectations:**
  - Nim users made "improve tooling" the top priority, rising every year — [Nim 2024 survey](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
  - Gleam's post-v1 priority was the language server, "immature compared to the rest of the Gleam tooling" — [Gleam v1](https://gleam.run/news/gleam-version-1/).
  - Rust users still list slow compile times and storage use among top productivity problems — [2025 State of Rust](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/). Roc left Rust for Zig over compile times — [Linked List](https://linkedlist.org/2025/02/06/roc-zig-rewrite) [search snippet].
- **Docs and production guidance:** Gleam users "like the current documentation but want more", especially help making applications and getting them into production; video content was in demand. JSON decoding was the top pain point — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
- **Failure to attract new users:** only 15% of Nim respondents were new users (under 1 year), and survey responses fell to 367 — [Nim 2024 survey](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
- **Never shipping or never stabilising:**
  - *Jai:* 12 years with no public release ([Wikipedia](https://en.wikipedia.org/wiki/Jai_(programming_language)) [search snippet]).
  - *Roc:* no 0.1 after 7 years, plus a full compiler rewrite ([Roc README](https://github.com/roc-lang/roc)).
  - *Carbon:* 0.1 slipped from 2022 to "end of 2026 at the soonest" ([Carbon roadmap](https://github.com/carbon-language/carbon-lang/blob/trunk/docs/project/roadmap.md)).
  - *Pony:* still "semi-regularly introduces breaking changes" ([ponyc README](https://github.com/ponylang/ponyc)).
  - *Elm:* a 6.5-year release gap ([elm tags](https://github.com/elm/compiler/releases/tag/0.19.2)).
- **Credibility loss from over-promising:** V's 2019 launch claims (compiler size, C-like speed) led to lasting "vaporware" labelling — [Xe](https://christine.website/blog/v-vaporware-2019-06-23), [Lobsters](https://lobste.rs/s/1ogeev/v_is_for_vaporware) [search snippets].
- **Funding:**
  - Zig warns it "will not be able to renew everyone's contracts" at current recurring income — [ZSF report](https://ziglang.org/news/2025-financials/).
  - Vale's sponsorships paused, then the project was archived — [Vale README](https://github.com/ValeLang/Vale).
  - Crystal's lead drifted away "due to Manas' own dynamics" — [Wind of change](https://crystal-lang.org/2025/09/29/wind-of-change/).
  - Hylo's lead designer retired — [hylo-lang.org](http://hylo-lang.org/) [search snippet].
- **LLMs and low- or no-resource languages:**
  - *Benchmarks:* MultiPL-E translates HumanEval and MBPP to 18 other languages — [MultiPL-E README](https://github.com/nuprl/MultiPL-E). Studies using it show "significant performance disparities between high-resource and low-resource programming languages", and most low-resource languages and DSLs "lack a benchmark for evaluation" — [Survey on LLM code generation for low-resource and domain-specific languages, arXiv 2410.03981](https://arxiv.org/abs/2410.03981) [search snippet].
  - *No-resource paper:* a June 2026 paper explicitly treats **Gleam and MoonBit as representative "no-resource" languages**, evaluating prompt-based techniques and pre-training/fine-tuning on the little data available — [arXiv 2606.16827, "No Resource, No Benchmarks, No Problem?"](https://arxiv.org/abs/2606.16827) [search snippet; numeric results not obtained].
  - *Cost of the data-synthesis fix:* MultiPL-T (training data for low-resource languages via LLM translation from Python) required an estimated 550 A100-days to reproduce, about 1,400 A100-hours per language for translation, and about 2,000 GPU-hours for the filtered Python source set. Fine-tuning a 1B model on the resulting data takes under an hour on a consumer GPU — [MultiPL-T artifact README](https://github.com/nuprl/MultiPL-T).
  - *AI use is mainstream:* 53% of Go developers use AI tools daily and 29% rarely or never; 17% use agents as their primary mode and 40% occasionally — [Go 2025 survey](https://go.dev/blog/survey2025). Rust's survey sees learners "moving their questions to LLM tooling" — [2025 State of Rust](https://blog.rust-lang.org/2026/03/02/2025-State-Of-Rust-Survey-results/).
  - *Mitigations by new languages:*
    - MoonBit ships an official Agent Skill for Codex, Claude Code and Copilot; agent-oriented CLI semantic navigation (`moon ide doc`, `outline`, `peek-def`, `find-references`, `rename`); and a script that concatenates its docs into one LLM-ingestible file — [moonbit-agent-guide](https://github.com/moonbitlang/moonbit-agent-guide), [SKILL.md](https://github.com/moonbitlang/moonbit-agent-guide/blob/main/moonbit-agent-guide/SKILL.md), [moonbit-docs llm.py](https://github.com/moonbitlang/moonbit-docs/blob/main/next/llm.py). Its design paper is titled "AI-Friendly Programming Language" — [dblp](https://dblp.org/rec/conf/llm4code/FeiZZWL24.html).
    - Zero exposes JSON diagnostics and repair plans and has agents patch a checked semantic graph rather than text — [zerolang README](https://github.com/vercel-labs/zerolang), [MarkTechPost](https://www.marktechpost.com/2026/05/17/vercel-labs-introduces-zero-a-systems-programming-language-designed-so-ai-agents-can-read-repair-and-ship-native-programs/) [search snippet].
    - Weft bundles its own assistant and targets 10 agent tools ("You never have to learn this language") — [Weft README](https://github.com/WeaveMindAI/weft).
    - Bend 2 is pitched as proofs that "block AI mistakes" — [bendlang/bend](https://github.com/bendlang/bend).
  - *Verification languages benefit from LLMs:* DafnyPro reaches 86% on DafnyBench — [POPL 2026](https://popl26.sigplan.org/details/dafny-2026-papers/12/DafnyPro-LLM-Assisted-Automated-Verification-for-Dafny-Programs) [search snippet]. The Lean FRO's new funding is explicitly "AI for mathematical research" — [Renaissance Philanthropy](https://www.renaissancephilanthropy.org/insights/lean-fro-and-mathlib-receive-10m-from-xtx-markets-founder-alex-gerko-to-further-advance-the-use-of-ai-for-mathematical-research) [search snippet].
  - *Counter-example:* Zig bans LLM use in contributions — [Zig CoC](https://ziglang.org/code-of-conduct/#strict-no-llm-no-ai-policy). It can afford to because it already has a large corpus and community.
- **Quality risk of AI-built languages:** Rue's agent audit found the test harness counted internal compiler errors as passes (98 of 216 compile-fail cases had no message assertion), ownership was unsound in the binary, and the optimizer deleted mandatory safety checks at `-O1` — [An Agent Holds the Fort](https://github.com/rue-language/rue/blob/trunk/website/content/blog/an-agent-holds-the-fort.md).

### Inferences
- A "better X" without a domain wedge fails the Meyerovich & Rabkin test: developers switch by domain and libraries, not by semantics. Examples: Nim ("Python-like but compiled"), V ("simple, fast, safe"), Crystal ("Ruby but fast"), and Hylo/Vale/Austral ("safer systems language").
- The LLM gap is now a first-order adoption barrier. A language whose users cannot get competent AI help is at a disadvantage against Python/TypeScript/Go, where AI use is already daily for most developers. The practical countermeasures are cheap: (1) syntax and semantics close to a high-resource language so transfer learning works (Gleam is Elm/Rust-like, Mojo is Python-like, MoonBit is Rust/Go-like); (2) small surface area; (3) compiler errors precise enough to drive an agent's fix loop; (4) shipped agent skills, llms.txt and single-file docs. Fine-tuning (MultiPL-T) is out of reach for a solo team at full scale, but small-model fine-tunes are cheap.
- Languages whose *output* LLMs are good at checking (Dafny, Lean, Bend 2's proofs, Zero's checked graph patches) have turned the AI era into a tailwind rather than a headwind.

### Gaps
- Pass@k numbers for Gleam and MoonBit from arXiv 2606.16827, and exact MultiPL-E/MultiPL-T gains, could not be read (arXiv was blocked). The final report should describe the direction of these effects without quoting numbers.
- No independent evidence was found on whether MoonBit's AI-first tooling actually increased adoption.
- No evidence was found in Gleam's own news posts of an llms.txt or LLM strategy (a grep of the site source found no LLM mentions), so treat claims that "Gleam addresses LLMs" as unsupported.

---

## Q5. What a minimum credible toolchain looks like in 2026

### Takeaway
In 2026, "credible" means one install that gives you a compiler/interpreter, **build tool plus package manager, formatter, language server** (with an editor extension published to *both* the VS Code Marketplace and Open VSX, so AI IDEs like Cursor get it), searchable API docs, and a **browser playground or tour (usually via WASM)**. The newer bar is **agent-readiness**: machine-readable (JSON) diagnostics, an `AGENTS.md`/skill file, llms.txt plus markdown docs, and optionally an MCP server or CLI semantic queries so coding agents can navigate code without guessing.

### Cited Findings
- **Gleam's v1 scope as the reference bar:** v1 covered "the Gleam language design... compiler... build tool... package manager... code formatter... language server... compiler WASM API and JavaScript bindings", all in the main repo — [Gleam version 1](https://gleam.run/news/gleam-version-1/). Its browser tour compiles "within the browser, rather [than] sending the code to a build server" — [Gleam's new interactive language tour, 2024-01-19](https://gleam.run/news/gleams-new-interactive-language-tour/).
- **The LSP is where small teams fall behind:** Gleam called its LSP "immature compared to the rest of the tooling" at v1 and made it the first post-v1 priority — [Gleam v1](https://gleam.run/news/gleam-version-1/). Later posts show sustained LSP investment: "A field day for Gleam's language server", "Global rename and find references", "Auto-imports and tolerant expressions" ([gleam.run/news](https://gleam.run/news/)).
- **Users rank tooling first:** Nim users made tooling the #1 priority — [Nim 2024 survey](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
- **Editor distribution in the AI-IDE era:** Swift published its extension on Open VSX so it works in Cursor, VSCodium, AWS Kiro and Google Antigravity — [Expanding Swift IDE support, 2026-04-08](https://www.swift.org/blog/expanding-swift-ide-support/).
- **MCP from the language server:** gopls ships an experimental built-in MCP server "that exposes a subset of gopls' functionality to AI assistants" — [Go's Sweet 16](https://go.dev/blog/16years).
- **Agent skills and CLI semantic navigation:** MoonBit's `moon` tool provides `moon ide doc/outline/peek-def/find-references/hover/rename`, `moon check`, `moon test --update` (snapshot tests), `moon fmt` and `moon info` (public-interface `.mbti` files), packaged as an installable Agent Skill — [MoonBit SKILL.md](https://github.com/moonbitlang/moonbit-agent-guide/blob/main/moonbit-agent-guide/SKILL.md).
- **Machine-readable diagnostics and repair:** Zero has `--json` on every subcommand, stable error codes, `zero fix --plan --json`, and graph patches that reject stale hashes — [MarkTechPost](https://www.marktechpost.com/2026/05/17/vercel-labs-introduces-zero-a-systems-programming-language-designed-so-ai-agents-can-read-repair-and-ship-native-programs/) [search snippet], [zerolang README](https://github.com/vercel-labs/zerolang). Mojo 1.0 ships "a compiler fix-it" with nearly every breaking change — [Mojo 1.0 notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.0.0.md).
- **llms.txt:** proposal v2 (updated 2026-08-10) says "thousands of sites publish an llms.txt file", documentation platforms generate one automatically, and Chrome Lighthouse audits for it. It recommends `.md` twins of every docs page plus `rel="alternate" type="text/markdown"` / `rel="describedby"` links — [llmstxt.org (source)](https://github.com/AnswerDotAI/llms-txt/blob/main/nbs/index.qmd).
- **Agent instruction files in language repos:** `AGENTS.md` in Roc ([link](https://github.com/roc-lang/roc/blob/main/AGENTS.md)); `AGENTS.md` and `CLAUDE.md` in Rue ([repo](https://github.com/rue-language/rue)); `CLAUDE.md` in Mojo ([modular/modular Mojo/](https://github.com/modular/modular)); `AGENTS.md` in MoonBit docs ([moonbit-docs](https://github.com/moonbitlang/moonbit-docs)) and in the Elixir website ([elixir-lang.github.com](https://github.com/elixir-lang/elixir-lang.github.com)).
- **Install and distribution:** Mojo ships through pip/conda/uv/pixi ([Mojo README](https://github.com/modular/modular/blob/main/Mojo/README.md)). Zero and Bend use one-line `curl | sh` installers ([zerolang](https://github.com/vercel-labs/zerolang), [bend](https://github.com/bendlang/bend)).
- **Stability and compatibility promises:** Gleam v1 committed to semver and avoiding "language bloat" — [Gleam v1](https://gleam.run/news/gleam-version-1/). Mojo 1.0 defined stability policies and marks stable stdlib APIs incrementally — [Mojo 1.0 notes](https://github.com/modular/modular/blob/main/Mojo/docs/site/releases/v1.0.0.md). Nim users credit "the mostly painless upgrade process" — [Nim 2024 survey](https://nim-lang.org/blog/2025/01/23/community-survey-results-2024.html).
- **Platform coverage users expect:** Gleam survey respondents develop on Linux (559), macOS (458) and Windows (213). Pilfold calls Windows support "vital", and notes that GitHub Actions' lack of BSD runners limits binary support — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
- **Community channels:** Gleam users get news from Discord (238), gleam.run (197), the Gleam Weekly newsletter (193), reddit (147), Twitter (124), Lobsters (82) and Hacker News (57) — [Gleam 2024 survey](https://gleam.run/news/developer-survey-2024-results/).
- **Testing and spec discipline for AI-built compilers:** Rue pairs a spec with paragraph-level test traceability, differential oracles, fuzzing and sanitizers, yet still needed adversarial agent "hunts" to find soundness bugs — [Rue README](https://github.com/rue-language/rue), [An Agent Holds the Fort](https://github.com/rue-language/rue/blob/trunk/website/content/blog/an-agent-holds-the-fort.md).

### Inferences
- **Minimum credible 2026 checklist** (synthesised from the above):
  1. A single binary or single install (compiler/interpreter, `fmt`, `test`, `doc`, package manager, LSP).
  2. A VS Code extension on both the Marketplace and Open VSX.
  3. A WASM browser playground or tour.
  4. Generated API docs with search, plus `.md` twins and an `llms.txt`.
  5. `--json` diagnostics with stable codes and fix-its.
  6. An `AGENTS.md` and a published agent skill; optionally an MCP server or CLI semantic queries (hover, definition, references, rename).
  7. A written stability/semver policy.
  8. Prebuilt binaries for Linux, macOS and Windows.
  9. A package registry (or reuse of an existing one, e.g. Hex, npm or PyPI, as Gleam and Mojo do).
- For a small team with a Rust interpreter, reusing an existing registry and runtime (PyPI wheels, npm/WASM) covers items 1, 3 and 9 cheaply and matches the interop pattern that worked for Gleam and Mojo.

### Gaps
- No survey data was found that quantifies how much developers weight llms.txt or MCP support when choosing a language; the evidence is only that leading toolchains (Go, MoonBit, Zero, Swift) are adding it.
- Could not confirm whether Gleam, Zig, Odin or Nim publish llms.txt files (their websites could not be fetched).
