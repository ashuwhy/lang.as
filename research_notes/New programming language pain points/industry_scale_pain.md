# Industry-Scale Software Pain Points (2023–2026) and How Much a New Programming Language Could Fix

_Method note: researched 2026-09-24. The network proxy blocked direct page fetches for most domains (blog.google, security.googleblog.com, cisa.gov, darpa.mil, theregister.com, sonatype.com, it-cisq.org, gao.gov, crowdstrike.com, cloudflare.com and others). Full-page reads succeeded only for cloud.google.com (GTIG 2025 zero-day review), github.com (Carbon roadmap, Fil-C, Austral, LavaMoat READMEs), microsoft.com (Shai-Hulud 2.0 guidance; MSR incident paper), and securitylabs.datadoghq.com (Shai-Hulud 2.0). Every other figure comes from search-engine result summaries of the linked pages. The links are the correct primary or secondary pages, but the writer should treat single-number details from them as "reported by", not as independently re-read. The session's web-search budget ran out before the concurrency, two-language/CUDA, SBOM-mandate and CHERI questions were fully covered; those gaps are listed explicitly._

## 1. Memory safety: how much of the vulnerability problem it is, the government push, C++'s response, and migration tooling

### Takeaway
Memory unsafety is the one large pain point whose root cause is the language itself, and the evidence that changing language fixes it is very strong. Android's Rust code has about 0.2 memory-safety vulnerabilities per million lines against about 1,000 for its C/C++, and memory-safety bugs fell below 20% of Android vulnerabilities in 2025. Memory corruption also still made up about 35% of in-the-wild zero-days in 2025. But the language answer already exists: Rust, plus the garbage-collected memory-safe languages (MSLs). What remains is the migration and interop problem for billions of existing lines of C/C++. The C++ committee has effectively declined to add a borrow-checked safe subset (Safe C++ was dropped in favor of Profiles). That leaves room only for a C/C++-compatible successor or a translation target (Carbon, TrapC, Fil-C, DARPA TRACTOR), not for yet another greenfield safe language.

### Cited Findings
**Share of vulnerabilities caused by memory unsafety**
- Microsoft (2019, older figure): about 70% of the vulnerabilities Microsoft fixes and assigns a CVE are memory-safety issues, a share that held for at least 12 years. They are mostly memory-corruption bugs that developers put into C and C++ code (Matt Miller, BlueHat IL 2019). — [MSRC blog, 2019](https://www.microsoft.com/en-us/msrc/blog/2019/07/we-need-a-safer-systems-programming-language); [MSRC "proactive approach" 2019](https://msrc.microsoft.com/blog/2019/07/a-proactive-approach-to-more-secure-code/)
- One search summary said Microsoft's share "declined to approximately 50%" in recent years. The claim traces to the Wikipedia "Memory safety" article, not to a Microsoft primary source, so treat it as unverified. — [Wikipedia: Memory safety](https://en.wikipedia.org/wiki/Memory_safety)
- Chromium (2020 analysis, older figure): about 70% of Chrome's serious (high-severity) security bugs are memory-safety problems, and about half of those are use-after-free. Chrome's MiraclePtr aims to deterministically stop exploitation of use-after-free bugs and "stands a chance of eliminating over 50%" of UAF bugs in the browser process. — [Chromium memory safety page](https://www.chromium.org/Home/chromium-security/memory-safety/); [Slashdot, May 2020](https://developers.slashdot.org/story/20/05/24/1349204/chromium-project-finds-70-of-its-serious-security-bugs-are-memory-safety-problems)
- Android (Google, November 2025, "Rust in Android: move fast and fix things"):
  - Memory-safety bugs fell below 20% of total Android vulnerabilities for the first time in 2025.
  - Android has about 5 million lines of Rust. Only one potential memory-safety vulnerability was found in it, and it was fixed before release. That gives about 0.2 vulnerabilities per MLOC, against about 1,000 per MLOC historically for Android C/C++: a claimed 1000x reduction in density.
  - Rust changes have a 4x lower rollback rate and spend 25% less time in code review. — [Google blog](https://blog.google/security/rust-in-android-move-fast-fix-things/); [The Hacker News, Nov 2025](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html); [LWN mirror](https://lwn.net/Articles/1046397/); [Slashdot, 2025-11-17](https://developers.slashdot.org/story/25/11/17/012246/rust-in-android-more-memory-safety-fewer-revisions-fewer-rollbacks-shorter-reviews)
  - Date conflict: one search summary dated this post April 2026. The URLs of the Hacker News and Slashdot coverage show November 2025.
- First near-miss memory-safety bug in Android Rust: CVE-2025-48530, a linear buffer overflow in CrabbyAVIF, an `unsafe` Rust AVIF parser. The Scudo hardened allocator made it non-exploitable. — [The Hacker News, Nov 2025](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html)
- Google's September 2024 post, "Eliminating Memory Safety Vulnerabilities at the Source", makes the "safe coding" argument. Vulnerabilities decay with code age, so writing new code in MSLs cuts memory-safety bugs without rewriting old code. — [Google Security Blog, Sept 2024](https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html) (the page's figures could not be fetched; see Gaps)

**Share of exploited zero-days (in the wild)**
- GTIG counted 90 zero-days exploited in the wild in 2025, against 78 in 2024 and a peak of 100 in 2023. The count has sat in a 60–100 band since 2021. — [GTIG 2025 Zero-Day Review (full page read)](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)
- Breakdown of the 2025 zero-days (same GTIG review):
  - About 35% were memory-safety issues. Use-after-free was the main memory-corruption vector (e.g., CVE-2025-48543 in Android ART, CVE-2025-27038 in the Qualcomm GPU driver), then out-of-bounds write (CVE-2025-6558 in Mali, CVE-2025-21042 in the Samsung DNG/Quram library).
  - The rest were injection and deserialization, access-control/authentication bypass, and logic/design flaws, concentrated in enterprise edge appliances.
  - Browsers accounted for under 10% of 2025 zero-days, a historic low. Operating systems accounted for 39 (44%). Enterprise products accounted for 43 (48%), an all-time high.
  - The review does not mention memory-safe-language adoption as a defense. — [GTIG 2025 Zero-Day Review](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)
- Project Zero's long-running in-the-wild 0-day root-cause tracking (from 2019) attributes 68% of listed CVEs to memory corruption. The period is cumulative and not specific to 2024–2025. — [Project Zero 0days-in-the-wild RCA](https://googleprojectzero.github.io/0days-in-the-wild/rca.html)

**Government guidance**
- White House ONCD, 26 February 2024, "Back to the Building Blocks: A Path Toward Secure and Measurable Software". It argues manufacturers can remove whole vulnerability classes by adopting MSLs, backed by formal methods and memory-protecting hardware, and asks researchers to work on software measurability. — [ONCD report PDF](https://bidenwhitehouse.archives.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf); [White House press release via UCSB](https://www.presidency.ucsb.edu/documents/white-house-press-release-future-software-should-be-memory-safe)
- CISA/FBI "Product Security Bad Practices" guidance:
  - Manufacturers should publish a memory-safety roadmap by 1 January 2026 covering how they will remove memory-safety vulnerabilities in priority code, either with MSLs or with hardware capabilities.
  - It is voluntary guidance, not a mandatory rule, aimed at software for critical infrastructure and national critical functions.
  - The same document lists as bad practices: SQL injection, OS command injection, default passwords, and shipping with a Known Exploited Vulnerabilities (KEV) catalog vulnerability. — [TechRepublic](https://www.techrepublic.com/article/cisa-fbi-memory-safety-recommendations/); [Industrial Cyber](https://industrialcyber.co/cisa/cisa-and-fbi-release-draft-guidance-on-product-security-bad-practices-for-software-manufacturers/)
- CISA/NSA Cybersecurity Information Sheet, 24 June 2025, "Memory Safe Languages: Reducing Vulnerabilities in Modern Software Development". It covers obstacles to MSL adoption and gives recommendations: language-level protections, libraries, tooling, training. It lists as MSLs: Ada, C#, Delphi/Object Pascal, Go, Java, Python, Ruby, Rust, Swift. — [CISA resource page](https://www.cisa.gov/resources-tools/resources/memory-safe-languages-reducing-vulnerabilities-modern-software-development); [NSA PDF](https://media.defense.gov/2025/Jun/23/2003742198/-1/-1/0/CSI_MEMORY_SAFE_LANGUAGES_REDUCING_VULNERABILITIES_IN_MODERN_SOFTWARE_DEVELOPMENT.PDF); [CISA alert 2025-06-24](https://www.cisa.gov/news-events/alerts/2025/06/24/new-guidance-released-reducing-memory-related-vulnerabilities)

**C/C++ ecosystem responses**
- Safe C++ (Sean Baxter's borrow-checked C++ extension):
  - The C++ Safety and Security study group voted to prioritize Profiles over Safe C++. Baxter said in June 2025 that "Safe C++ is not being continued".
  - About twenty committee members had supported Safe C++; a majority preferred Profiles.
  - Critics say Profiles "don't look like any established working solution, don't have an implementation, and also failed to get into the C++ 26 standard." — [The Register, 2025-09-16](https://www.theregister.com/2025/09/16/safe_c_proposal_ditched/); [InfoWorld](https://www.infoworld.com/article/4065702/safe-c-proposal-for-memory-safety-flames-out.html); [Techzine](https://www.techzine.eu/news/devops/134679/c-opts-for-profiles-and-leaves-safe-c-behind/)
  - Conflict: Techzine/InfoWorld summaries say C++26 will bring safety "primarily through profiles and standard-library refinements". The Register quotes critics saying Profiles failed to get into C++26. The writer should say Profiles were not standardized in C++26 and remain in development.
- Carbon (Google's experimental C++ successor):
  - The 2025 roadmap goals were C++ interop for non-template APIs and a design for memory safety.
  - It states: "the end of 2026 is now the soonest that 0.1 could realistically be ready to ship". The 0.1 milestone slipped by at least a year because of the added memory-safety design work.
  - The plan is to follow "the direction of Rust, using the type system to ensure compile-time guarantees of safety". — [Carbon roadmap on GitHub (full page read)](https://github.com/carbon-language/carbon-lang/blob/trunk/docs/project/roadmap.md)
- TrapC (Robin Rowe/Trasec): a memory-safe C extension whose pointers "cannot produce segfaults, buffer overruns, or memory leaks".
  - It missed the 1 January 2026 release target and was "code complete" in January 2026. The site's estimate was Q1 2026.
  - The Register reported that Rowe used Claude to build it. — [trapc.org](https://trapc.org/about/); [The Register, 2026-01-26](https://www.theregister.com/software/2026/01/26/dev-used-claude-to-build-trapc-memory-safe-extension-of-c/4132586); [The New Stack](https://thenewstack.io/memory-safe-c-trapcs-pitch-to-the-c-iso-working-group/)
- Fil-C (Filip Pizlo): a memory-safe C/C++ implementation built on clang 20.1.8 that supports C17 and C++20.
  - "All memory safety errors are caught as Fil-C panics", including OOB, UAF, type confusion and pointer races, and there is no `unsafe` escape hatch.
  - It uses concurrent GC and "invisible capabilities".
  - OpenSSL, CPython and SQLite run on it, and there is a memory-safe Linux userland (Pizlix). — [Fil-C GitHub (full page read)](https://github.com/pizlonator/fil-c/)
  - Reported slowdown is about 1.5x–5x against normal clang output, depending on the program. — [LWN](https://lwn.net/Articles/1042938/); [The Register, 2024-11-16](https://www.theregister.com/2024/11/16/rusthaters_unite_filc/)
- Microsoft:
  - In December 2025, Distinguished Engineer Galen Hunt posted a goal "to eliminate every line of C and C++ from Microsoft by 2030" using AI plus algorithmic tooling, with a North Star of "1 engineer, 1 month, 1 million lines of code".
  - He then clarified that Windows is not being rewritten in Rust with AI and that this is a research project on language-migration technology.
  - Since 2023 Microsoft has rewritten parts of the Windows kernel in Rust, after Azure CTO Mark Russinovich told teams to stop starting new C/C++ projects. — [Windows Central](https://www.windowscentral.com/microsoft/windows-11/my-goal-is-to-eliminate-every-line-of-c-and-c-from-microsoft-by-2030-microsoft-bets-on-ai-to-finally-modernize-windows); [The New Stack](https://thenewstack.io/microsofts-bold-goal-replace-1b-lines-of-c-c-with-rust/); [IT Pro](https://www.itpro.com/software/development/microsoft-rust-programming-language-modernization-ai)
- DARPA TRACTOR (Translating All C to Rust):
  - Goal: automated translation of legacy C into safe, idiomatic Rust "of the same quality and style" a skilled Rust developer would write.
  - MIT Lincoln Laboratory runs test and evaluation and releases a benchmark every six months. As of September 2026, two batteries and three milestone projects have been released.
  - Performers include a UW–Madison / UC Berkeley / Edinburgh / UIUC team, which won a $5M award in July 2025 for "ForCLift" (verified lifting combining formal methods and LLMs), and Aarno Labs ("Tenjin"). — [DARPA TRACTOR](https://www.darpa.mil/research/programs/translating-all-c-to-rust); [MIT LL](https://www.ll.mit.edu/r-d/projects/translating-all-c-rust-tractor-benchmarks); [TRACTOR benchmark arXiv](https://arxiv.org/html/2609.25121); [UW–Madison news, 2025-07-15](https://www.cs.wisc.edu/2025/07/15/translating-legacy-code-for-a-safer-future); [UIUC CSL](https://csl.illinois.edu/news-and-media/translating-legacy-code-for-a-safer-future-darpa-backs-effort-to-convert-c-to-rust)

### Inferences
- **Root cause is the language: yes.** Memory safety is the clearest "language is the lever" case. Android's figure, about 0.2 vs about 1,000 vulnerabilities/MLOC, is the strongest natural experiment in the industry.
- **The greenfield language slot is already filled.** Rust has won systems-level greenfield work, and governments name ten existing MSLs. A new language aimed at "memory safety for new code" would bring little.
- **What is still open is the brownfield problem: C/C++ that cannot easily be rewritten.** The candidate levers:
  - A successor language with seamless C++ interop (Carbon, still pre-0.1 in 2026).
  - Compatible safe dialects or runtimes (Fil-C, TrapC; performance costs).
  - Automated translation (TRACTOR, Microsoft's research).
  - Hardware (CHERI, MTE).

  A new language here competes mostly on interop and migration, not on its safety model.
- **The memory-safety share of real exploitation is falling, from about 70% of CVEs (2019-era) to about 35% of exploited zero-days (2025).** Attackers are moving to logic, auth-bypass and injection bugs in enterprise edge appliances. The problem is being addressed with some success, so the marginal value of one more language on this axis is shrinking.
- **The dollar size of memory safety is not measured directly.** It can only be approximated as a large share of the cost of cybercrime and vulnerability management, e.g., within CISQ's $2.41T (see Section 3).

### Gaps
- Could not fetch Google's September 2024 Android post. My recollection is that it reported memory-safety bugs falling from 76% of Android vulnerabilities (2019) to 24% (2024), but this was not verified in this session.
- No current (2024–2026) Microsoft or Chromium memory-safety share was found. The 70% figures date from 2019–2020.
- CHERI and Arm MTE/Apple Memory Integrity Enforcement status, and any published count of vendors who met CISA's 1 January 2026 roadmap target: not researched, because the search budget ran out.
- Whether TrapC actually shipped in 2026, and current Fil-C performance numbers from a primary benchmark: not verified.
- Linux kernel Rust status (reportedly made non-experimental in late 2025): not verified in this session.

## 2. Software supply-chain security: incidents, scale, costs, mandates, and whether language-level capabilities help

### Takeaway
Open-source malware is growing fast. Sonatype counted about 454,600 new malicious packages in 2025 (1.233M cumulative), and 2025 brought the first self-replicating npm worm (Shai-Hulud), which recurred in November 2025 and May 2026. Dollar estimates ($60B in 2025 to about $81B in 2026) are vendor projections, not measurements. Most attack payloads rely on the ambient authority of install scripts and processes: any dependency can read environment variables, credentials and the network. A language with object capabilities (no ambient authority, so a dependency can only touch what it is passed) would address the runtime-payload part of the problem. It would not address stolen maintainer accounts, install-time hooks run by the package manager, or build-system tampering like xz-utils. No mainstream language does this today. The only examples are Austral (pre-release), Hardened JavaScript/LavaMoat (a library/runtime approach) and WASI (capability-based but coarse-grained).

### Cited Findings
**Scale**
- Sonatype 2026 State of the Software Supply Chain report:
  - More than 454,600 new malicious packages were found in 2025, for a cumulative 1.233M+ across npm, PyPI, Maven Central, NuGet and Hugging Face.
  - Repository abuse appears in 55.9% of logged malicious packages.
  - Sonatype found 800+ Lazarus-associated packages in 2025, 97% of them on npm.
  - It calls the September 2025 Shai-Hulud worm "the first known self-replicating npm malware". — [Sonatype 2026 SSSC: open-source malware](https://www.sonatype.com/state-of-the-software-supply-chain/2026/open-source-malware); [Infosecurity Magazine](https://www.infosecurity-magazine.com/news/454000-malicious-open-source/); [Sonatype Q4 2025 malware index](https://www.sonatype.com/blog/open-source-malware-index-q4-2025-automation-overwhelms-ecosystems)
- A search summary reported "512,847 malicious packages logged" in 2024, up 156% year on year. This is probably Sonatype's 10th annual (2024) report, but the summary did not name its source clearly. The difference from the 2025 figure of 454,600 "new" packages probably reflects different counting (logged vs new); flag as not comparable. — [Sonatype 10th annual report press release](https://www.sonatype.com/press-releases/sonatypes-10th-annual-state-of-the-software-supply-chain-report); [OWASP Top 10:2025 A03 Software Supply Chain Failures](https://top10.owasp.org/2025/A03_2025-Software_Supply_Chain_Failures/)
- A 2020 study of more than 50,000 npm projects found a median of 408 transitive dependencies (reported via research summaries). — [OWASP A03:2025](https://top10.owasp.org/2025/A03_2025-Software_Supply_Chain_Failures/)
- OWASP's 2025 Top 10 includes "A03 Software Supply Chain Failures" as a category. — [OWASP](https://top10.owasp.org/2025/A03_2025-Software_Supply_Chain_Failures/)

**Incidents**
- **xz-utils (CVE-2024-3094, CVSS 10), discovered 29 March 2024.**
  - "Jia Tan" made a first contribution on 29 October 2021 and a first commit on 21 January 2022. Sockpuppet accounts ("Jigar Kumar", "Dennis Ens") pressured maintainer Lasse Collin to add a co-maintainer.
  - The backdoor shipped in release 5.6.0 (February 2024). It used IFUNC resolvers, added in June 2023, to replace OpenSSH's `RSA_public_decrypt` at runtime, giving remote code execution to whoever held a private key. — [Akamai](https://www.akamai.com/blog/security-research/critical-linux-backdoor-xz-utils-discovered-what-to-know); [Sonatype](https://www.sonatype.com/blog/cve-2024-3094-the-targeted-backdoor-supply-chain-attack-against-xz-and-liblzma); [Logpoint](https://logpoint.com/en/blog/emerging-threats/xz-utils-backdoor)
- **Shai-Hulud v1 (npm), discovered 16 September 2025.** It started with about 40 packages, including some CrowdStrike packages, and grew to 500+ (e.g., @ctrl/tinycolor). It harvested CI/CD and cloud-metadata secrets, exfiltrated them through GitHub repositories and workflows, and republished packages under stolen maintainer accounts. CISA issued an alert on 23 September 2025. — [Upwind](https://www.upwind.io/feed/npm-supply-chain-attack-shai-hulud-worm-escalates-august-nx-compromise); [StepSecurity](https://www.stepsecurity.io/blog/ctrl-tinycolor-and-40-npm-packages-compromised); [CISA alert 2025-09-23](https://www.cisa.gov/news-events/alerts/2025/09/23/widespread-supply-chain-compromise-impacting-npm-ecosystem)
- **Shai-Hulud 2.0, 24 November 2025** (Datadog count):
  - 796 unique npm packages and 1,092 unique versions backdoored, with more than 20 million weekly downloads.
  - Credentials exfiltrated for 500+ GitHub users; 150+ GitHub organizations affected; 14,000+ exfiltration repositories created.
  - Mechanism: a new `preinstall` script installs the Bun runtime to evade Node monitoring and runs an obfuscated payload. The payload can backdoor up to 100 more packages per stolen npm token. — [Datadog Security Labs (full page read)](https://securitylabs.datadoghq.com/articles/shai-hulud-2.0-npm-worm/)
- Microsoft's guidance on Shai-Hulud 2.0:
  - It names compromised maintainer accounts at Zapier, PostHog and Postman, and describes TruffleHog-based credential harvesting and persistence through GitHub runners.
  - It reports a "Mini Shai-Hulud" wave on 11 May 2026: 170+ npm packages, 2 PyPI packages and 404 malicious versions, described as "the first supply chain attack to simultaneously span both the npm and PyPI registries". — [Microsoft Security Blog, 2025-12-09 (full page read)](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)
- Conflicting scale figure: one search summary said the campaign compromised "over 1,300 package versions ... combined 2 billion monthly downloads" (attributed among Unit 42 and others). That does not match Datadog's 1,092 versions and 20M+ weekly downloads, probably because of different counting windows or popularity metrics. — [Unit 42](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/) vs [Datadog](https://securitylabs.datadoghq.com/articles/shai-hulud-2.0-npm-worm/)
- Singapore's Cyber Security Agency issued advisory AD-2026-009 on an "ongoing npm supply chain attack affecting keyv and related packages (Shai-Hulud worm)", including cacheable, flat-cache and file-entry-cache, so the worm family was still active in 2026. — [CSA Singapore AD-2026-009](https://www.csa.gov.sg/alerts-and-advisories/advisories/ad-2026-009/)
- **Slopsquatting / AI package hallucination** (USENIX Security 2025, UTSA / Oklahoma / Virginia Tech):
  - 16 code-generating LLMs were tested across Python and JavaScript. 19.7% of recommended packages did not exist, giving 205,474 unique hallucinated names.
  - On 10 re-runs of the same prompt, 43% of hallucinated names came back on every run and 58% on more than one, so they are predictable and attackers can register them.
  - Sample-count conflict: one summary says 2.23M samples, another 576,000.
  - One secondary breakdown: 38% conflations, 13% typo variants, 51% pure fabrications. — [TechTarget](https://www.techtarget.com/it-strategy/feature/Slopsquatting-explained-When-AI-code-turns-malicious); [CSA research note, April 2026](https://labs.cloudsecurityalliance.org/research/csa-research-note-slopsquatting-ai-supply-chain-20260419-csa/); [Aikido](https://www.aikido.dev/blog/slopsquatting-ai-package-hallucination-attacks)

**Cost estimates (projections, not measurements)**
- Cybersecurity Ventures projects the global annual cost of software supply-chain attacks at $60B in 2025 and $138B by 2031, assuming 15% annual growth. — [Cybersecurity Ventures](https://cybersecurityventures.com/software-supply-chain-attacks-to-cost-the-world-60-billion-by-2025/)
- Juniper Research projects losses rising 76% to about $80.6B by 2026. — [Cybersecurity Dive](https://www.cybersecuritydive.com/news/software-supply-chain-attacks/650148/)
- CISQ's 2022 report treats supply-chain and open-source component problems as one of three focus areas inside the $2.41T US cost of poor software quality. — [CISQ 2022](https://www.it-cisq.org/the-cost-of-poor-quality-software-in-the-us-a-2022-report/)

**Language- and runtime-level capability approaches**
- **Austral:** "linear capabilities enable fine-grained permissioned access to low-level facilities. Third-party dependencies can be constrained in what types of resources they can access". Code needs an explicit capability value to reach the filesystem or network.
  - Status: pre-release. The OCaml compiler implements the full spec but has no separate compilation, and the capability-based standard library is "being designed". — [Austral GitHub (full page read)](https://github.com/austral/austral)
- **LavaMoat (MetaMask/ConsenSys):**
  - Disables dependency lifecycle scripts by default and keeps an allowlist (`@lavamoat/allow-scripts`).
  - Runs each package in a SES (Hardened JavaScript) compartment with per-package policy files that limit access to platform APIs and globals.
  - This is a runtime and library approach to object capabilities in an existing language. — [LavaMoat GitHub (full page read)](https://github.com/LavaMoat/LavaMoat)
- **WebAssembly component model / WASI:** research published at the ‹Programming› 2026 companion notes these limits:
  - WASI grants some capabilities statically and lets components discover capabilities at run time through ambient authority.
  - There is no native dynamic revocation, which "violates the principle of least authority and exacerbates the risk of software supply chain attacks".
  - Reusing third-party components from OCI registries recreates npm-style risk.

  The paper proposes transparent interposition at component boundaries. — [ACM DOI 10.1145/3801119.3801125](https://doi.org/10.1145/3801119.3801125)
- Related research:
  - ZTD_JAVA: zero-trust dependencies that enforce per-dependency permissions in Java. — [arXiv 2310.14117](https://arxiv.org/pdf/2310.14117)
  - Runtime protection against malicious package updates for Node.js, which infers and enforces least-privilege capabilities per package. — [arXiv 2305.19760](https://arxiv.org/pdf/2305.19760)

### Inferences
- **Root cause is only partly the language.** The Shai-Hulud payloads need a `preinstall` hook (a package-manager policy) plus a process that can read `~/.npmrc`, environment variables and cloud metadata, and reach the network (ambient authority, which is a language and runtime property).
  - In an object-capability language whose module system forbids ambient I/O, a transitive dependency such as a color-parsing library could not exfiltrate secrets. It would need a network or filesystem capability in its signature, which would show up in review and could be checked mechanically.
  - It would not stop install-time scripts (package-manager design), phished maintainer tokens, or xz-style attacks. The xz backdoor lived in build scripts and binary test fixtures and hooked `sshd` through the dynamic loader (IFUNC), below the language.
- **This is the most under-served language-level opportunity.** It fits "one big pain point a new language could solve" better than memory safety:
  - The problem is growing (malware counts rising, worms recurring into 2026).
  - Current mitigations are mostly process-level (2FA, trusted publishing, SBOMs, scanners).
  - No mainstream language offers per-dependency least privilege; Java's SecurityManager-style approaches were abandoned.
  - The Hardened JavaScript/LavaMoat and WASI work shows the idea works but is bolted on and coarse-grained.
- **Adoption barrier:** a capability language needs its own package ecosystem, or capability-safe wrappers around foreign libraries (FFI is ambient authority). Value grows with the share of the dependency graph written in it.
- **AI coding assistants make it worse.** Hallucinated packages (19.7%) and AI-generated code pulling in dependencies raise the value of a language where adding a dependency cannot silently widen authority.

### Gaps
- SBOM mandates were not verified in this session: EU Cyber Resilience Act timelines (reporting obligations are reportedly applicable from September 2026 and full obligations from December 2027), CISA's 2025 SBOM minimum-elements update, and changes to US EO 14028 attestation in 2025. The search budget ran out before these queries.
- Details of the 8 September 2025 npm compromise (chalk, debug; reportedly about 18 packages and about 2.6B weekly downloads), the tj-actions/changed-files GitHub Action compromise (March 2025), and the Nx "s1ngularity" compromise (August 2025): not verified.
- Deno's permission model is known to be process-wide rather than per-dependency, but this was not verified from Deno documentation (fetch blocked).
- No measured (as opposed to projected) dollar cost of supply-chain attacks was found. ReversingLabs, Socket and Black Duck OSSRA 2025/2026 figures were not retrieved.

## 3. Cost of poor software quality, technical debt, and major outages caused by bugs

### Takeaway
Poor software quality is the largest dollar figure: CISQ put the US cost at $2.41T in 2022 (a 2022 estimate), plus about $1.52T of accumulated technical debt, which is not additive. The biggest single-incident losses of 2024–2025 came from ordinary input-validation, null-handling and error-handling bugs amplified by global instant rollout:
- CrowdStrike: about $5.4B direct loss to the US Fortune 500 alone.
- AWS us-east-1 in October 2025: up to $581M insured losses.
- Google Cloud in June 2025.
- Cloudflare in November 2025.

A language could have turned some of these into handled errors: bounds checks for CrowdStrike, null safety for Google. But Cloudflare's outage came from a Rust `.unwrap()`, which shows a safe language does not stop developers from choosing to crash. The dominant fixes are process fixes: staged rollouts, config validation, feature flags.

### Cited Findings
**Aggregate cost**
- CISQ, "Cost of Poor Software Quality in the US: A 2022 Report" (December 2022):
  - The cost of poor software quality in the US was $2.41T.
  - Accumulated technical debt was about $1.52T. It is not included in the $2.41T and should not be added to it.
  - Focus areas were cybercrime from software vulnerabilities, supply-chain and open-source component problems, and technical debt. — [CISQ report page](https://www.it-cisq.org/the-cost-of-poor-quality-software-in-the-us-a-2022-report/); [CISQ press release 2022-12-06](https://www.it-cisq.org/press-releases/12-06-22/); [Synopsys IR](https://investor.synopsys.com/news/news-details/2022/Software-Quality-Issues-in-the-U.S.-Cost-an-Estimated-2.41-Trillion-in-2022/default.aspx); [PDF](https://www.it-cisq.org/wp-content/uploads/sites/6/2022/11/CPSQ-Report-Nov-22-2.pdf)
- Stripe "Developer Coefficient" (September 2018, older figure):
  - Developers spend 42% of the working week on technical debt (13.5h) and bad code (3.8h), i.e., 17.3h of a 41.1h week.
  - That is about $85B a year worldwide in lost opportunity cost.
  - The survey covered 1,000+ developers and 1,000+ C-level executives in five countries. — [Stripe PDF](https://stripe.com/files/reports/the-developer-coefficient.pdf)

**CrowdStrike Falcon outage, 19 July 2024**
- About 8.5 million Windows machines crashed into boot loops (BSOD). — [TechTarget](https://www.techtarget.com/whatis/feature/Explaining-the-largest-IT-outage-in-history-and-whats-next); [CISA alert](https://www.cisa.gov/news-events/alerts/2024/07/19/widespread-it-outage-due-crowdstrike-update)
- Root cause (CrowdStrike RCA, 6 August 2024):
  - The IPC template type defined 21 input fields, but the sensor code supplied only 20 inputs to the Content Interpreter.
  - A channel file (291) with a non-wildcard criterion in the 21st field caused an out-of-bounds memory read past the end of the input array in the kernel driver.
  - A runtime array bounds check was missing, the Content Validator had a logic error, and there was no test for non-wildcard matching in the 21st field. — [CrowdStrike RCA PDF](https://www.crowdstrike.com/wp-content/uploads/2024/08/Channel-File-291-Incident-Root-Cause-Analysis-08.06.2024.pdf); [SC Media](https://www.scworld.com/news/massive-crowdstrike-outage-caused-by-an-out-of-bounds-memory-error); [The Hacker News](https://thehackernews.com/2024/08/crowdstrike-reveals-root-cause-of.html)
- Parametrix estimates:
  - US Fortune 500 companies, excluding Microsoft, faced $5.4B in direct losses. Only 10–20% ($540M–$1.08B) was likely insured, because of large retentions and low limits.
  - The weighted average loss was $44M per Fortune 500 company, ranging from $6M (manufacturing) to $143M (airlines).
  - Healthcare lost the most ($1.938B), then banking ($1.149B). — [Parametrix](https://www.parametrixinsurance.com/in-the-news/crowdstrike-to-cost-fortune-500-5-4-billion-insured-loss-range-of-540-million-to-1-08-billion); [Cybersecurity Dive](https://www.cybersecuritydive.com/news/crowdstrike-cost-fortune-500-losses-cyber-insurance/722396/); [Insurance Journal](https://www.insurancejournal.com/news/international/2024/07/24/785285.htm)
- Other insured-loss estimates: Guy Carpenter $300M–$1B; CyberCube $400M–$1.5B. — [Claims Journal](https://www.claimsjournal.com/news/national/2024/08/05/325496.htm)
- Delta Air Lines: about $500M in lost revenue and extra costs; 7,000 flights canceled; 1.3M passengers affected over five days; Delta sued CrowdStrike. — [Insurance Journal](https://www.insurancejournal.com/news/national/2024/08/09/787765.htm); [Newsweek](https://www.newsweek.com/delta-sues-crowdstrike-outage-causing-500-million-losses-1975432)
- One search summary mentioned "at least $10 billion" in global losses. Its primary source was unclear (probably the Wikipedia or TechTarget roll-up), so treat it as a loose upper-level estimate. — [Wikipedia](https://en.wikipedia.org/wiki/2024_CrowdStrike-related_IT_outages)

**Google Cloud outage, 12 June 2025**
- A Service Control code change without error handling or feature-flag protection was rolled out on 29 May 2025.
- On 12 June, a policy change put "unintended blank fields" into regional Spanner tables. Spanner replicated them worldwide within seconds, and every Service Control instance hit a null-pointer path and crash-looped at about the same moment.
- SRE identified the root cause in about 10 minutes and started the "red-button" within 25 minutes; most regions recovered after about 40 minutes of rollout. — [ThousandEyes](https://www.thousandeyes.com/blog/google-cloud-outage-analysis-june-12-2025); [BigGo summary](https://biggo.com/news/202506141923_Google_Cloud_Major_Outage_Null_Pointer_Bug)

**Cloudflare outage, 18 November 2025**
- A database permission change caused the Bot Management feature file to include duplicate rows, growing from about 60 to more than 200 features.
- The FL2 proxy, written in Rust, preallocates for a fixed limit. Its limit check returned an error that the code `unwrap()`ped, causing a panic and HTTP 5xx errors across the network.
- Workers KV and Access were affected until a bypass at 13:04. — [Cloudflare postmortem](https://blog.cloudflare.com/18-november-2025-outage/); [Hackaday](https://hackaday.com/2025/11/20/how-one-uncaught-rust-exception-took-out-cloudflare/)

**AWS us-east-1 outage, 19–20 October 2025** (a concurrency root cause; see Section 4)
- Disruption lasted 14+ hours. Downdetector received over 6.5 million reports covering 1,000+ services.
- CyberCube estimates insured losses up to $581M. — [ThousandEyes](https://www.thousandeyes.com/blog/aws-outage-analysis-october-20-2025); [The Register](https://www.theregister.com/2025/10/23/amazon_outage_postmortem/); [InfoQ](https://www.infoq.com/news/2025/11/aws-dynamodb-outage-postmortem)

### Inferences
- **"Poor software quality" is too broad for one language to own.** The $2.41T covers cybercrime, failed projects, operational failures and legacy costs. It is useful as a ceiling, not as the addressable market for a language.
- **In all four mega-outages the proximate bug was of a kind a stricter type system could flag, but the blast radius came from process** (global, instant, unstaged propagation of config or content):
  - CrowdStrike: out-of-bounds read.
  - Google: null or absent field.
  - Cloudflare: unhandled error.
  - AWS: race on shared state.
- **A language that makes "config/content is untrusted input" a type-level concept could help.** It would force total parsing and validation, exhaustive error handling with no `unwrap`/panic in production paths, and bounded resource use. That is a narrower pitch than "memory safety".
- **Rust already covers bounds and null.** Cloudflare's incident shows developers opt out of error handling unless the language or tooling forbids partial functions.
- **CrowdStrike root cause and language:** yes in part. A bounds-checked kernel language would have turned the OOB read into a handled error. But a kernel-mode panic still crashes the machine unless the error path degrades gracefully, and the systemic cause was pushing unvalidated content to 8.5M hosts at once.

### Gaps
- No updated CISQ figure after 2022 was found; the 2022 report is the latest known.
- McKinsey's technical-debt estimates (e.g., "tech debt is 20–40% of the value of the technology estate") were not retrieved.
- IBM's Cost of a Data Breach 2025 average was not retrieved.
- No study was found that measures what fraction of large outages come from bug classes a type system could catch, as opposed to config and process errors.

## 4. Concurrency and distributed-systems bugs: frequency, cost, and language approaches

### Takeaway
Concurrency bugs are common and expensive. Uber found more than 2,000 data races in 46M lines of Go. The costliest single incident of 2025, the AWS us-east-1 outage (up to $581M insured losses), came from a race condition in distributed automation. But the expensive failures are mostly cross-process and cross-service races (stale reads, TOCTOU across machines), which in-process type systems like Rust's Send/Sync do not prevent. They need protocol-level verification, deterministic simulation or formal methods. A language that builds in distributed-protocol correctness is a research-grade bet with weak adoption precedent, based on the evidence gathered.

### Cited Findings
- Uber (PLDI 2022, older data):
  - A dynamic race detector deployed across Uber's 46 million lines of Go (2,100 microservices) found more than 2,000 data races.
  - More than 1,000 were fixed in six months, through 790 patches from 210 developers.
  - The authors conclude that Go's concurrency idioms and nuances "make Go programs highly susceptible to data races". — [ACM DL](https://dl.acm.org/doi/10.1145/3519939.3523720); [arXiv 2204.00764](https://arxiv.org/abs/2204.00764); [Uber blog](https://www.uber.com/ch/fr/blog/dynamic-data-race-detection-in-go-code)
- A follow-up, "DR.FIX: Automatically Fixing Data Races at Industry Scale" (2025), shows automated repair of races is now its own research line. Details were not retrieved. — [arXiv 2504.15637](https://arxiv.org/pdf/2504.15637)
- AWS, 19–20 October 2025, us-east-1 root cause:
  - A race condition in DynamoDB's automated DNS management. The DNS Enactor checked that its plan was fresh only at the start of processing.
  - After unusual delays, an old plan overwrote a newer one, and cleanup automation then deleted the old plan. That removed all DNS records for the DynamoDB regional endpoint.
  - The failure cascaded to EC2, Lambda, NLB and ECS/EKS; full recovery came at 2:20 PM PDT on 20 October.
  - CyberCube estimated insured losses up to $581M. — [The Register, 2025-10-23](https://www.theregister.com/2025/10/23/amazon_outage_postmortem/); [InfoQ](https://www.infoq.com/news/2025/11/aws-dynamodb-outage-postmortem); [ThousandEyes](https://www.thousandeyes.com/blog/aws-outage-analysis-october-20-2025)
- Microsoft Research (HotOS 2019, older data) studied "hundreds of high severity incidents" in production Azure services and found software bugs to be "a major cause". The per-category percentages were not available on the accessible page. — [MSR publication page (full page read)](https://www.microsoft.com/en-us/research/publication/what-bugs-cause-production-cloud-incidents/)
- Fil-C catches "pointer races" as a memory-safety violation, i.e., race-induced memory corruption in C/C++ becomes a runtime panic. — [Fil-C GitHub](https://github.com/pizlonator/fil-c/)

### Inferences
- **Two distinct problems:**
  - (a) In-process data races, which Rust's Send/Sync solves statically for safe code and Go and Java do not.
  - (b) Distributed races and protocol bugs (AWS DynamoDB DNS), which no mainstream type system addresses.

  The dollar-heavy incidents are type (b).
- **Root cause is only partly the language.** For (a), yes: language choice decides whether data races are possible. For (b), the root cause is unverified protocol design plus automation with global blast radius. Candidate language levers are session or choreographic types, actor isolation (Erlang/Pony-style), or built-in deterministic simulation. The evidence here does not show adoption at scale.

### Gaps
- The search budget ran out before sourcing:
  - Rust Send/Sync documentation and empirical studies of Rust concurrency or async bugs.
  - Pony, Erlang/Elixir reliability evidence.
  - Structured concurrency (Java 21/25, Kotlin, Swift).
  - Deterministic simulation testing (FoundationDB, TigerBeetle, Antithesis and its funding).
  - Quantitative studies of production incidents by root cause (share due to concurrency or timing).

  None of these could be cited.
- No aggregate dollar estimate of concurrency-bug cost was found.

## 5. Legacy migration: COBOL and C/C++ modernization

### Takeaway
Legacy code is a very large stock problem:
- Estimates of COBOL in use range from 220B lines (Reuters, 2017) to more than 800B lines (a Micro Focus-commissioned survey).
- The SSA alone runs more than 60M lines of COBOL.
- The US federal government spends about 79–80% of its roughly $100B+ IT budget on operations and maintenance.

The 2025 DOGE plan to rewrite SSA's COBOL "in months" showed how politically salient the issue is. Modernization is a translation, verification and tooling problem (TRACTOR, Microsoft's AI migration research, LLM-assisted rewrites), not a problem of missing target languages. Java, C# and Rust already serve as targets. A new language helps only if it is designed as a provably equivalent migration target.

### Cited Findings
- More than 800 billion lines of COBOL are in daily production use, according to a Micro Focus-commissioned survey of 1,104 architects, engineers, developers, managers and IT executives in 49 countries. Reuters' 2017 estimate was 220 billion lines. — [The Stack](https://www.thestack.technology/cobol-in-daily-use/)
- Reuters (2017) estimated that COBOL runs 43% of banking systems and 95% of ATM swipes; these older figures are widely re-quoted. — [The Stack](https://www.thestack.technology/cobol-in-daily-use/); [BMC](https://www.bmc.com/blogs/cobol-trends/)
- Weak-source figures: about 240B lines of COBOL in the US; the average COBOL programmer is 58, with about 10% retiring each year. These come from a Substack roll-up, not primary data. — [Substack roll-up](https://abhaskjha.substack.com/p/cobol-and-cobwebs-why-yesterdays)
- DOGE and SSA (March 2025):
  - DOGE planned to move SSA systems off COBOL (to Java or similar) within "a few months", in a project organized by Steve Davis.
  - SSA systems contain more than 60 million lines of COBOL.
  - Experts, including a former senior SSA technologist, warned that a rushed migration could miscalculate or stop payments.
  - Senators Warren and Wyden wrote to the SSA Commissioner about the modernization. — [Gizmodo](https://gizmodo.com/doge-plans-to-rewrite-entire-social-security-codebase-in-just-a-few-months-report-2000582062); [SiliconANGLE](https://siliconangle.com/2025/03/28/doge-reportedly-planning-rewrite-social-security-administrations-software/); [Warren–Wyden letter PDF](https://www.warren.senate.gov/imo/media/doc/warren_wyden_letter_to_bisignano_on_ssa_it_modernization.pdf)
- GAO-25-107795 (2025):
  - The federal government spends more than $100B a year on IT and cyber, about 80% of it on operations and maintenance. About $83B (79%) of planned FY2025 IT spending was for O&M.
  - Of the 10 critical legacy systems GAO flagged in June 2019, only 3 had been modernized by February 2025. Of the remaining seven, four were planned within a few years, two in 5+ years, and one had no planned date. — [GAO-25-107795](https://www.gao.gov/products/gao-25-107795); [GAO full report](https://files.gao.gov/reports/GAO-25-107795/index.html)
- The IRS Individual Master File, which processes every individual tax return, runs 1960s-era COBOL (per secondary summaries of GAO work). — [mLogica summary of GAO-23-106821](https://www.mlogica.com/resources/blogs/agencies-need-to-continue-addressing-critical-legacy-systems); [GAO-23-106821](https://www.gao.gov/products/gao-23-106821)
- For C/C++ migration efforts (TRACTOR; Microsoft's 2030 C/C++ elimination research goal), see Section 1. — [DARPA TRACTOR](https://www.darpa.mil/research/programs/translating-all-c-to-rust); [The New Stack](https://thenewstack.io/microsofts-bold-goal-replace-1b-lines-of-c-c-with-rust/)
- A Mechanical Orchard analysis is titled "$1.14 Trillion to Keep the Lights On: Legacy's Drag on Productivity". Methodology not retrieved. — [Mechanical Orchard](https://www.mechanical-orchard.com/insights/1-14-trillion-to-keep-the-lights-on-legacys-drag-on-productivity)

### Inferences
- **Root cause is only indirectly the language.** COBOL is not unsafe; the pain is knowledge loss, undocumented business rules, mainframe coupling, and the risk of changing semantics (fixed-point decimals, record layouts, batch/JCL control flow).
- **The binding constraint is proving the translation preserves behavior.** Target languages are plentiful.
- **A new language could add value only as a verification-oriented migration target:** semantic-preserving types for decimal and record data, built-in equivalence testing against the legacy system. That is a niche, enterprise-sales-driven market.
- **LLM-assisted translation is changing the economics fast** (TRACTOR's LLM-plus-formal-methods teams, Microsoft's "1 engineer, 1 month, 1M lines" goal). This favors translating into existing popular languages, not a new one.

### Gaps
- The outcome of the DOGE/SSA COBOL rewrite through 2026 was not found.
- No reliable, recent (2024–2026) primary count of COBOL lines exists. The 800B figure comes from a vendor-commissioned survey, and estimates differ almost 4x.
- The worldwide cost of legacy maintenance was not sourced beyond the Mechanical Orchard title and GAO federal figures.

## 6. Two-language problem and heterogeneous-computing economics (brief; covered in depth by another researcher)

### Takeaway
Not researched in this session: the web-search budget ran out before any query on this topic, and fetches of candidate sources (julialang.org) were blocked. No cited findings are available here. The writer should rely on the dedicated GPU-language researcher's notes.

### Cited Findings
- None retrieved in this session.

### Inferences
- The two-language problem is a real language-level root cause: prototyping in Python and rewriting hot paths in C++/CUDA exists because no single language combines Python's ergonomics with native and GPU performance. It is one of the few pain points where a new language (the Julia and Mojo pitch) is plausibly the right lever. Its dollar size (GPU underutilization, CUDA lock-in) was not quantified here. _(Inference from domain knowledge; unsourced.)_

### Gaps
- Nothing is sourced in this session on: NVIDIA CUDA market share or lock-in, GPU utilization rates in AI training and inference, the size of the Python+CUDA developer population, or the Mojo/Julia/Triton adoption data.

## 7. Cross-cutting sizing: people, organizations, dollars, and whether the language is the root cause

### Takeaway
Measured on "root cause is the language" × "existing languages do not already solve it" × "growing problem", supply-chain least-privilege (capability-safe dependencies) scores best among the problems researched. Memory safety has the strongest language root cause, but Rust and the MSLs have largely taken it, and its share of exploitation is falling. Outages and quality problems are the largest in dollars but mostly process-driven. Legacy migration is a translation and verification problem. Distributed concurrency is costly but beyond mainstream type systems.

### Cited Findings (summary table; each figure is sourced in Sections 1–5)

| Pain point | Scale indicators (2023–2026 unless noted) | Dollar indicators | Language the root cause? | Already addressed by existing languages? |
|---|---|---|---|---|
| Memory safety | ~35% of 90 exploited zero-days in 2025 ([GTIG](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)); 70% of MS CVEs (2019) ([MSRC](https://www.microsoft.com/en-us/msrc/blog/2019/07/we-need-a-safer-systems-programming-language)); Android C/C++ ~1,000 vs Rust ~0.2 vulns/MLOC ([Google](https://blog.google/security/rust-in-android-move-fast-fix-things/)) | Not directly measured; part of CISQ $2.41T ([CISQ](https://www.it-cisq.org/the-cost-of-poor-quality-software-in-the-us-a-2022-report/)) | Yes | Largely yes (Rust + GC MSLs); the gap is legacy C/C++ interop and translation |
| Supply chain | 454,600 new malicious packages in 2025; 1.233M cumulative ([Sonatype](https://www.sonatype.com/state-of-the-software-supply-chain/2026/open-source-malware)); Shai-Hulud 2.0: 796 packages, 20M+ weekly downloads ([Datadog](https://securitylabs.datadoghq.com/articles/shai-hulud-2.0-npm-worm/)); worm waves continued into May 2026 ([Microsoft](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)) | $60B (2025) projected, $138B by 2031 ([CSV](https://cybersecurityventures.com/software-supply-chain-attacks-to-cost-the-world-60-billion-by-2025/)); ~$80.6B by 2026 projected ([Juniper via Cybersecurity Dive](https://www.cybersecuritydive.com/news/software-supply-chain-attacks/650148/)) | Partly (ambient authority is a language/runtime property; account takeover and install hooks are not) | No mainstream language; only Austral (pre-release), LavaMoat/SES (JS add-on), WASI (coarse) |
| Outages / poor quality | CrowdStrike: 8.5M machines; AWS: 6.5M Downdetector reports, 1,000+ services | CISQ $2.41T US (2022); CrowdStrike $5.4B for the Fortune 500 ([Parametrix](https://www.parametrixinsurance.com/in-the-news/crowdstrike-to-cost-fortune-500-5-4-billion-insured-loss-range-of-540-million-to-1-08-billion)); AWS up to $581M insured ([The Register](https://www.theregister.com/2025/10/23/amazon_outage_postmortem/)); tech debt $1.52T ([CISQ](https://www.it-cisq.org/press-releases/12-06-22/)); 42% of developer time, $85B/yr (2018) ([Stripe](https://stripe.com/files/reports/the-developer-coefficient.pdf)) | Partly (bounds, null, error handling); mainly rollout and validation process | Partly (Rust/Kotlin/Swift null and bounds safety); Cloudflare shows the opt-out gap |
| Concurrency / distributed | Uber: 2,000+ races in 46M LOC Go ([PLDI'22](https://dl.acm.org/doi/10.1145/3519939.3523720)) | AWS Oct 2025 up to $581M insured | In-process: yes; distributed: no | In-process: Rust; distributed: no language |
| Legacy (COBOL, C/C++) | 220B–800B+ lines of COBOL ([The Stack](https://www.thestack.technology/cobol-in-daily-use/)); SSA 60M+ lines ([Gizmodo](https://gizmodo.com/doge-plans-to-rewrite-entire-social-security-codebase-in-just-a-few-months-report-2000582062)) | US federal ~$83B/yr O&M (79% of IT, FY2025) ([GAO](https://www.gao.gov/products/gao-25-107795)) | Indirectly | Target languages exist; the gap is verified translation |

### Inferences
- **Best fit for "one pain point a new language could solve": software supply-chain attacks through capability-safe (least-authority) dependencies.**
  - Growing threat with recurring worms into 2026.
  - A demonstrable language-level root cause: ambient authority.
  - No incumbent language solves it.
  - Prior art shows feasibility: object-capability research, SES/LavaMoat, Austral, WASI.
  - The pitch also covers AI-era risks: hallucinated and AI-added dependencies.
  - Main risks: ecosystem bootstrapping, and the FFI/escape-hatch problem (any `unsafe` or foreign call is ambient authority).
- **Runner-up: memory safety for legacy C/C++ through a compatible successor.** Strong root cause and strong government pressure (CISA roadmaps, ONCD), but crowded (Rust, Carbon, Fil-C, TrapC, TRACTOR, Microsoft) and increasingly handled by AI-assisted translation.
- **A combined pitch may be strongest:** a memory-safe, capability-safe systems language where dependencies get no ambient authority and errors must be handled (no partial `unwrap`). It addresses supply chain plus CrowdStrike, Cloudflare and Google-style outage classes at once.

### Gaps
- Developer-population counts per affected ecosystem (npm/PyPI users, C/C++ developer counts, e.g., SlashData or Evans Data 2025) were not retrieved; the search budget was exhausted.
- No source quantifies the fraction of supply-chain malware payloads that a capability model would have blocked. Estimating it would need a study of payload behavior (install-time vs runtime; which APIs the payloads use).
- Section 6 (two-language/GPU economics) has no sourced findings in these notes.
