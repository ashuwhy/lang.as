# Gap fill: industry-scale costs of software failures, verification round (as of 24 September 2026)

_Method note. This round checked the flagged claims in `industry_scale_pain.md` and looked for 2026 developments. Three limits shaped the results:_

- _**Blocked fetches.** Full-page reads worked only on cloud.google.com, www.microsoft.com, securitylabs.datadoghq.com and www.anthropic.com. Every other fetch attempted this round returned EGRESS_BLOCKED: blog.google, security.googleblog.com, helpnetsecurity.com, thehackernews.com, media.defense.gov, cpomagazine.com, cybersecuritynews.com, gbhackers.com, herbsutter.com, wrocpp.github.io, docs.carbon-lang.dev, ic3.gov, digital-strategy.ec.europa.eu, csa.gov.sg, blogs.microsoft.com, chromium.org, uber.com, files.gao.gov, ll.mit.edu, slashdata.co, blog.pypi.org, thousandeyes.com, infoq.com and red.anthropic.com._
- _**Search cap.** The session-wide web-search cap (200 searches, shared with other researchers) ran out after about 30 searches in this round. No searches could be run on the outage post-mortems, the Uber Go study, COBOL/SSA/GAO, SlashData/Evans Data, CISQ or slopsquatting. Those items are marked COULD NOT VERIFY. The earlier links in `industry_scale_pain.md` remain the best leads._
- _**How evidence was captured.** "Page opened" means the fetch tool read the page, and the quotes are the text it returned. "Search summary" means the search tool's generated summary. That tool returns a list of result URLs and one merged summary, not a separate snippet for each URL. SNIPPET-CONFIRMED is used only when at least two independent outlets appear in the results and their titles or summaries carry the claim._

## 1. Verification ledger for the flagged claims in industry_scale_pain.md

### Takeaway
- **Confirmed or corrected:** the core memory-safety numbers (Android 2024 and 2025, GTIG 2025), the C++ status, Linux-kernel Rust, the EU Cyber Resilience Act (CRA) dates and the Sonatype counts.
- **Two date or scope errors fixed:** the Android "Rust in Android" post dates from November 2025, not April 2026. The "Profiles in C++26" question is settled: Profiles are not in C++26 and are aimed at C++29.
- **Shai-Hulud conflict resolved in favor of Datadog.** Datadog's figures (796 packages, 1,092 versions, 20M+ weekly downloads) are the right ones for the November 2025 wave. The "1,300+ versions / 2B monthly downloads" figure comes from 2026 coverage of the campaign as a whole or of a later wave.
- **Not re-verified:** the outage post-mortems, Uber, COBOL/GAO and the slopsquatting sample size. The cause was the search cap and blocked hosts, not contrary evidence.

### Cited Findings
**A. Memory-safety share claims**

- **A1. Google Sept 2024 post: 76% of Android vulnerabilities (2019) → 24% (2024).** SNIPPET-CONFIRMED.
  - Search summary: "Memory safety issues, which accounted for 76% of Android vulnerabilities in 2019, are currently 24% in 2024, well below the 70% industry norm, and continuing to drop."
  - Results carrying the claim: [Google Security Blog, Sept 2024](https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html); [The Hacker News, "Google's Shift to Rust Programming Cuts Android Memory Vulnerabilities by 68%"](https://thehackernews.com/2024/09/googles-shift-to-rust-programming-cuts.html); [BleepingComputer, "Google sees 68% drop in Android memory safety flaws over 5 years"](https://www.bleepingcomputer.com/news/security/google-sees-68-percent-drop-in-android-memory-safety-flaws-over-5-years/); [Help Net Security, 2024-09-26](https://www.helpnetsecurity.com/2024/09/26/android-memory-safety-vulnerabilities/).
  - The "68%" headlines match 76% → 24%, a relative drop of 52/76 ≈ 68%.
  - Another summary adds that absolute counts fell "from 223 in 2019 to less than 50 in 2024" ([The Hacker News, Nov 2025](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html)).
  - The old note held this only as a "recollection". It can now be cited at snippet level.
- **A2. "Rust in Android: move fast and fix things": the date.** CORRECTED from "April 2026?" to November 2025.
  - Google's own URL carries the date: [security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html).
  - Coverage dates agree: [The Hacker News, 2025/11](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html), [Slashdot, 2025-11-17](https://developers.slashdot.org/story/25/11/17/012246/rust-in-android-more-memory-safety-fewer-revisions-fewer-rollbacks-shorter-reviews) and [SecurityPulse, 2025-11-17](https://securitypulse.tech/2025/11/17/android-memory-safety-rust-adoption/).
  - The exact day was not seen. The [blog.google copy](https://blog.google/security/rust-in-android-move-fast-fix-things/) has no date in its URL, which may explain the "April 2026" snippet. That explanation is an inference.
- **A3. The November 2025 figures.** SNIPPET-CONFIRMED.
  - Summary text:
    - "memory safety vulnerabilities falling below 20% of total vulnerabilities for the first time."
    - "With roughly 5 million lines of Rust in the Android platform and one potential memory safety vulnerability found (and fixed pre-release), Google's estimated vulnerability density for Rust is 0.2 vulnerabilities per 1 million lines (MLOC)."
    - "a 1000x reduction in memory safety vulnerability density compared to Android's C and C++ code."
    - "Rust changes have a 4x lower rollback rate and spend 25% less time in code review."
  - Independent carriers: The Hacker News's title "Rust Adoption Drives Android Memory Safety Bugs Below 20% for First Time"; Slashdot's title "…Fewer Rollbacks, Shorter Reviews"; two Medium titles citing "1000x" ([Sharda](https://medium.com/@ashishjsharda/googles-1000x-security-win-what-android-s-rust-migration-means-for-the-language-s-future-c613a1953a6b), [Cooper](https://medium.com/codeelevation/using-rust-googles-real-test-memory-vulnerability-rate-on-android-is-1000-lower-than-c-c-47c95351da22)); [LWN](https://lwn.net/Articles/1046397/).
  - Caveat: the snippets say "1000x lower density than C/C++". The "~1,000 per MLOC for C/C++" figure in the old notes is implied by that ratio, not quoted. Write "about 1,000x lower density".
- **A4. CVE-2025-48530 (CrabbyAVIF near-miss; Scudo made it non-exploitable).** COULD NOT VERIFY independently.
  - One search summary mentions "a near-miss vulnerability in CrabbyAVIF that was caught before release (CVE-2025-48530)".
  - The Scudo detail was not seen.
- **A5. GTIG 2025 zero-day review.** CONFIRMED ([page opened](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)).
  - Title "Look What You Made Us Patch: 2025 Zero-Days in Review", published 5 March 2026.
  - Counts: "GTIG tracked 90 zero-day vulnerabilities exploited in-the-wild in 2025", against 100 in 2023 and 78 in 2024, "within the 60–100 range established over the previous four years".
  - Memory safety: "memory safety issues (particularly use-after-free [UAF] and out-of-bounds write) accounting for roughly 35% of the vulnerabilities."
  - Enterprise: "43 (48%) zero-days in enterprise software and appliances in 2025, up from 36 (46%) in 2024."
  - Operating systems: "OSs … accounting for 44% (39) of all zero-days."
  - Browsers: "Browsers accounted for less than 10%."
  - Mobile OS: 15 zero-days in 2025 against 9 in 2024.
  - The page contains no mention of memory-safe languages or Rust.
  - Correction to the old notes: the text returned by the fetch tool did not include the words "all-time high" for the enterprise share. Say "48%, up from 46% in 2024".
- **A6. "Microsoft ~50%".** SNIPPET-CONFIRMED (weak). Source attribution CORRECTED.
  - Two separate search summaries state it, tied to coverage of the June 2025 CISA/NSA memory-safe-languages sheet, not to Wikipedia:
    - "Microsoft attributed nearly 70% of their CVEs to memory safety issues in 2016, but recent years have seen this percentage decline to approximately 50%."
    - "CVEs involving memory safety at Microsoft have dropped from about 70% in 2016 to 50% in more recent studies."
  - Results: [CybersecurityNews](https://cybersecuritynews.com/cisa-releases-guide-to-reduce-memory-safety-vulnerabilities/); [GBHackers](https://gbhackers.com/cisa-publishes-guide-to-address-memory-safety-vulnerabilities/); [CPO Magazine](https://www.cpomagazine.com/cyber-security/new-cisa-nsa-joint-report-reiterates-call-for-memory-safe-languages/); [NSA/CISA CSI PDF](https://media.defense.gov/2025/Jun/23/2003742198/-1/-1/0/CSI_MEMORY_SAFE_LANGUAGES_REDUCING_VULNERABILITIES_IN_MODERN_SOFTWARE_DEVELOPMENT.PDF).
  - No Microsoft primary source with the ~50% figure or its year was seen.
  - Recommended wording: "about 50% in recent years, per the June 2025 CISA/NSA guidance as reported; down from about 70%".
- **A7. A current (2024–2026) Chromium memory-safety share.** COULD NOT VERIFY.
  - Searches returned only the 2020 analysis: "Around 70% of Chromium's high severity security bugs are memory unsafety problems … based on analysis of 912 high or critical severity security bugs since 2015" ([Chromium memory-safety page](https://www.chromium.org/Home/chromium-security/memory-safety/); [Slashdot 2020](https://developers.slashdot.org/story/20/05/24/1349204/chromium-project-finds-70-of-its-serious-security-bugs-are-memory-safety-problems)).
  - chromium.org was blocked for fetching.

**B. C/C++ ecosystem**

- **B1. Safe C++ dropped in favor of Profiles.** SNIPPET-CONFIRMED.
  - Titles: [The Register, "Safe C++ proposal all but abandoned in favor of profiles", 2025-09-16](https://www.theregister.com/2025/09/16/safe_c_proposal_ditched/); [Slashdot, "C++ Committee Prioritizes 'Profiles' Over Rust-Style Safety Model Proposal"](https://developers.slashdot.org/story/25/09/20/0449252/c-committee-prioritizes-profiles-over-rust-style-safety-model-proposal); [Lobsters, "Safe C++ proposal is not being continued"](https://lobste.rs/s/xinll3/safe_c_proposal_is_not_being_continued).
  - Summary quotes:
    - Baxter: "Safe C++ is not being continued", "The Rust safety model is unpopular with the committee" and "Profiles won the argument".
    - Erich Keane (co-chair of the Evolution Working Group, EWG): "roughly 1/2 (20/45) of the people encouraged Sean's paper, and 30/45 encouraged work on profiles (with 6 neutral)."
  - Vote-count conflict: another summary gives "19 for profiles, 9 for Safe C++, and 11 for both" ([wro.cpp](https://wrocpp.github.io/posts/cpp-safety-state-of-the-union-may-2026/)). These are probably different polls; report both, or neither.
- **B2. Profiles in or out of C++26.** CORRECTED and resolved: out.
  - Summary: "When the standard was finalized at the March 2026 meeting, profiles were left out, as Herb Sutter's trip report confirms. Stroustrup's type-safety profile paper (P3984) and a general profiles framework are both targeted at C++29, with no final ship date promised" ([Whole Tomato, July 2026](https://www.wholetomato.com/blog/c26-memory-safety-what-actually-ships-for-existing-code/); [wro.cpp toolset page](https://wrocpp.github.io/toolset/memory-safety-cpp26-and-beyond/)).
  - The earlier Techzine/InfoWorld line, that C++26 would bring safety "primarily through profiles", is wrong for the final standard.
- **B3. Carbon 0.1.** COULD NOT VERIFY independently; only a single-source snippet.
  - Carbon's own roadmap, via snippet: "Shipping 0.1 in 2026 will be a very ambitious goal and may not be possible, but the end of 2026 is now the soonest that 0.1 could realistically be ready to ship" ([Carbon roadmap](https://docs.carbon-lang.dev/docs/project/roadmap.html)).
  - A 2026 talk exists: [Carbon memory safety: a first deep dive (v3)](https://chandlerc.blog/slides/2026-memory-safety-deep-3/).
  - No 0.1 release was seen in any result.
- **B4. TrapC.**
  - Missed target and code-complete status: SNIPPET-CONFIRMED.
    - "Robin Rowe wanted to make a software release by January 1, 2026, but encountered too many bugs, reaching code complete only in January 2026 and aiming for Q1 2026 … Rowe stated 'We're almost there' and 'It almost works'."
    - It is built along two paths: an interpreter (`itrapc`) and a compiler (`trapc`).
    - Sources: [The Register, 2026-01-26](https://www.theregister.com/2026/01/26/trapc_claude_c_memory_safe_robin_rowe/); [The Register (alternate URL)](https://www.theregister.com/software/2026/01/26/dev-used-claude-to-build-trapc-memory-safe-extension-of-c/4132586); [trapc.org, "TrapC, a Year Later"](https://trapc.org/trapc-a-year-later/).
  - Whether it shipped in 2026: COULD NOT VERIFY. No release was seen in any result, and no follow-up search was possible.
- **B5. Fil-C slowdown "1.5x–5x".** CORRECTED.
  - The sources seen say "only a few times slower than Clang-generated code, although the exact slowdown depends heavily on the structure of the benchmarked program" ([LWN](https://lwn.net/Articles/1042938/)).
  - Pizlo's own summary: "raw benchmark numbers suggest a 4x slowdown (and sometimes even more)". Earlier capability models were "40x" and then "10x or less" ([Pizlo on X](https://x.com/filpizlo/status/1936202225468297367); [Pizlo on X](https://x.com/filpizlo/status/1920929346623017359)).
  - Pizlo's post title says user-facing ported programs show no usability-affecting slowdown.
  - No primary benchmark table was seen. Use "roughly 4x typical, sometimes more; small for I/O-bound programs".
- **B6. DARPA TRACTOR.**
  - Two batteries and three milestone projects released as of September 2026, with batteries every six months: SNIPPET-CONFIRMED ([TRACTOR benchmark paper, arXiv 2609.25121](https://arxiv.org/abs/2609.25121); [MIT LL](https://www.ll.mit.edu/r-d/projects/translating-all-c-rust-tractor-benchmarks)).
  - Six performer systems are scored per test case: SNIPPET-CONFIRMED ([arXiv 2609.25121](https://arxiv.org/html/2609.25121)).
  - The $5M ForCLift award (UW–Madison, UC Berkeley, Edinburgh, UIUC): SNIPPET-CONFIRMED ([UW–Madison, 2025-07-15](https://www.cs.wisc.edu/2025/07/15/translating-legacy-code-for-a-safer-future)).
  - Aarno Labs "Tenjin": COULD NOT VERIFY this round.
  - A snippet saying "7 teams were awarded contracts … (14 million USD)" had no clear source: COULD NOT VERIFY.
  - No official DARPA 2026 results were found.
- **B7. Microsoft "eliminate every line of C and C++ by 2030" and the clarification.** SNIPPET-CONFIRMED.
  - Hunt's update: "Windows is *NOT* being rewritten in Rust with AI [...] My team's project is a research project." Its intent was "not to set a new strategy for Windows 11+ or to imply that Rust is an endpoint."
  - Sources: [InfoWorld, "Microsoft is not rewriting Windows in Rust"](https://www.infoworld.com/article/4111553/microsoft-is-not-rewriting-windows-in-rust.html); [Windows Central](https://www.windowscentral.com/microsoft/windows-11/my-goal-is-to-eliminate-every-line-of-c-and-c-from-microsoft-by-2030-microsoft-bets-on-ai-to-finally-modernize-windows); [IT Pro](https://www.itpro.com/software/development/microsoft-rust-programming-language-modernization-ai); [Slashdot, 2025-12-23](https://developers.slashdot.org/story/25/12/23/010200/microsoft-to-replace-all-cc-code-with-rust-by-2030).
- **B8. "Microsoft has rewritten parts of the Windows kernel in Rust".** Partly CONFIRMED.
  - Microsoft's July 2024 post says Windows has "recently expanded the Windows kernel to support Rust". It also says "Microsoft has announced a commitment around the Rust programming language as part of Microsoft's Secure Future Initiative" ([Microsoft Security Blog, 2024-07-27, page opened](https://www.microsoft.com/en-us/security/blog/2024/07/27/windows-security-best-practices-for-integrating-and-managing-security-tools/)).
  - The claim that Russinovich told teams to stop new C/C++ projects: COULD NOT VERIFY.

**C. Policy**

- **C1. CISA/FBI: a memory-safety roadmap by 1 January 2026, voluntary.** SNIPPET-CONFIRMED.
  - "Software manufacturers should publish a memory safety roadmap by January 1, 2026" ([Product Security Bad Practices, ic3.gov PDF](https://www.ic3.gov/CSA/2024/241016-2.pdf); [TechRepublic](https://www.techrepublic.com/article/cisa-fbi-memory-safety-recommendations/); [RunSafe](https://runsafesecurity.com/blog/cisa-memory-safety-ot-leaders/)).
  - The related Secure by Design pledge "is a voluntary pledge. CISA does not enforce nor verify adherence" ([CISA pledge page](https://www.cisa.gov/securebydesign/pledge)).
  - How many vendors published roadmaps: COULD NOT VERIFY. Two searches found no count.
- **C2. CISA/NSA sheet "Memory Safe Languages: Reducing Vulnerabilities in Modern Software Development" (June 2025).** SNIPPET-CONFIRMED ([CISA alert, 2025-06-24](https://www.cisa.gov/news-events/alerts/2025/06/24/new-guidance-released-reducing-memory-related-vulnerabilities); [NSA press release](https://www.nsa.gov/Press-Room/Press-Releases-Statements/Press-Release-View/Article/4223298/nsa-and-cisa-release-csi-highlighting-importance-of-memory-safe-languages-in-so/); [Infosecurity Magazine](https://www.infosecurity-magazine.com/news/nsa-cisa-urge-memory-safe-languages/)).
  - The list of named MSLs was not re-seen this round.
- **C3. EU CRA: reporting from 11 September 2026, full application 11 December 2027.** SNIPPET-CONFIRMED ([Goodwin, Sept 2026](https://www.goodwinlaw.com/en/insights/publications/2026/09/alerts-lifesciences-technology-preparing-for-eu-cyber-resilience-act); [Mondaq](https://www.mondaq.com/uk/strategy/1845122/preparing-for-the-eu-cyber-resilience-act-key-reporting-obligations-from-11-september-2026); [Pearl Cohen](https://www.pearlcohen.com/eu-cyber-reporting-obligations-take-effect-on-september-11/); [European Commission news, 2026-09-11](https://commission.europa.eu/news-and-media/news/safer-and-more-secure-digital-products-2026-09-11_en)). Details are in Section 4.
- **C4. Linux kernel Rust declared no longer experimental (December 2025).** SNIPPET-CONFIRMED ([LWN, "The (successful) end of the kernel Rust experiment"](https://lwn.net/Articles/1049831/); [Slashdot, 2025-12-13](https://linux.slashdot.org/story/25/12/13/0347245/rust-in-linuxs-kernel-is-no-longer-experimental); [DevClass, 2025-12-15](https://devclass.com/2025/12/15/rust-boosted-by-permanent-adoption-for-linux-kernel-code/); [Phoronix](https://www.phoronix.com/news/Rust-To-Stay-Linux-Kernel)). Details are in Section 3.

**D. Supply chain**

- **D1. Sonatype 2026 report: 454,600 new malicious packages in 2025; 1.233M cumulative.** SNIPPET-CONFIRMED.
  - "Throughout 2025, Sonatype identified more than 454,600 new malicious packages, bringing the cumulative total of known and blocked malware to over 1.233 million packages across npm, PyPI, Maven Central, NuGet, and Hugging Face."
  - The report was released on 28 January 2026 ([Sonatype 2026 malware chapter](https://www.sonatype.com/state-of-the-software-supply-chain/2026/open-source-malware); [Infosecurity Magazine, "Researchers Uncover 454,000+ Malicious Open Source Packages"](https://www.infosecurity-magazine.com/news/454000-malicious-open-source/); [GlobeNewswire, 2026-01-28](https://www.globenewswire.com/news-release/2026/01/28/3227372/0/en/Sonatype-Research-Reveals-OSS-Malware-Grows-75-as-Yearly-Open-Source-Downloads-Surpass-9-8-Trillion.html)).
  - "Repository abuse shows up in 55.9% of all logged malicious packages." "Over 99% of open source malware occurred on npm." Downloads reached 9.8 trillion, up 67% year on year.
  - Consistency check: Sonatype's December 2024 cumulative was "More Than 778,500 Packages" ([GlobeNewswire, 2024-12-10](https://www.globenewswire.com/news-release/2024/12/10/2994607/0/en/Open-Source-Malware-Reaches-More-Than-778-500-Packages-According-to-Sonatype-Researchers.html)). 778,500 + 454,600 = 1,233,100, so the figures add up.
  - The "800+ Lazarus packages" claim was not re-seen: COULD NOT VERIFY.
  - The press-release headline "Malware Grows 75%" has no stated basis in the snippets. Do not use it without the base.
- **D2. "512,847 malicious packages logged in 2024".** CORRECTED (time window).
  - Sonatype's 10th annual report (10 October 2024) "identified 512,847 malicious packages discovered since November 2023, representing a 156% year-over-year surge" ([Sonatype press release](https://www.sonatype.com/press-releases/sonatypes-10th-annual-state-of-the-software-supply-chain-report); [Infosecurity Magazine](https://www.infosecurity-magazine.com/news/156-increase-in-oss-malicious/); [SiliconANGLE](https://siliconangle.com/2024/10/10/sonatype-report-open-source-software-reaches-6-6t-requests-security-risks-escalate/)).
  - It covers roughly November 2023 to October 2024, not calendar 2024. It is not directly comparable with the calendar-2025 figure of 454,600.
- **D3. Shai-Hulud v1 (September 2025): "500+ packages".** COULD NOT VERIFY.
  - Only early counts were seen, in headlines: "infects 147 npm packages with over 2 million downloads per week" ([TechRadar](https://www.techradar.com/pro/security/self-replicating-shai-hulud-infects-147-npm-packages-with-over-2-million-downloads-per-week)) and "Self-Replicating Worm Hits 180+ Software Packages" ([Krebs, via Harvard feed](https://tagteam.harvard.edu/hub_feeds/4281/feed_items/15925827/content)).
- **D4. Shai-Hulud 2.0, Datadog count.** CONFIRMED ([Datadog Security Labs, page opened](https://securitylabs.datadoghq.com/articles/shai-hulud-2.0-npm-worm/)).
  - Published 25 November 2025 and updated 4 December 2025.
  - "796 unique npm packages", "1,092 unique package versions", "over 20 million weekly downloads".
  - "over 500 unique GitHub users" (a lower bound), "over 150 unique GitHub organizations", 14,000+ exfiltration repositories.
  - Mechanism: it "adds a new preinstall script", installs "the Bun JavaScript runtime, likely to evade standard Node.js monitoring", and backdoors "the first 100 entries" per stolen token.
  - Wiz counted "over 25,000 malicious repositories across about 350 unique users" ([Wiz](https://www.wiz.io/blog/shai-hulud-2-0-ongoing-supply-chain-attack)), which is a different metric.
- **D5. The conflicting "1,300+ package versions … 2 billion monthly downloads".** CORRECTED (scope).
  - The search that surfaced this figure returned 2026 items: the [CSA Singapore advisory AD-2026-009 on the keyv wave](https://www.csa.gov.sg/alerts-and-advisories/advisories/ad-2026-009/), [OPSWAT, "Shai-Hulud Returns: ChainDrop Worm"](https://www.opswat.com/blog/shai-hulud-returns-chaindrop-worm-hits-npm-infecting-hundreds-of-packages), and an August 2026 blog ([Neupane](https://www.rijanneupane.com.np/2026/08/the-npm-worm-that-compromised-2-billion.html)).
  - The summary itself mixes "1,300 package versions" and "above 1,300 packages". It also lists the waves as "Shai-Hulud 1.0 (September 2025), 2.0 (November 2025), and Mini Shai-Hulud, Wave Four (May 2026)".
  - Settle it this way: use Datadog's figures for the November 2025 wave. Treat 1,300+/2B-monthly as a later or cumulative, secondary figure.
  - The metrics also differ: 20M weekly is about 87M monthly, far from 2B.
- **D6. chalk/debug compromise (8 September 2025; 18 packages; ~2.6B weekly downloads).** COULD NOT VERIFY the counts or the date.
  - The incident itself is referenced by Google as "the historical chalk and debug hijackings" ([GTIG, 2026-07-30, page opened](https://cloud.google.com/blog/topics/threat-intelligence/mitigation-guidance-for-supply-chain-compromise)).
- **D7. "Mini Shai-Hulud" (May 2026): 170+ npm packages, 2 PyPI packages, 404 versions.** CONFIRMED ([Microsoft Security Blog, originally 2025-12-09, updated 13 May 2026, page opened](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)).
  - Quotes: "170+ npm packages and 2 PyPI packages across 404 malicious versions"; "the first supply chain attack to simultaneously span both the npm and PyPI registries in a single coordinated operation".
  - The exact date of 11 May was not in the returned text. Use "May 2026".
- **D8. CSA Singapore AD-2026-009 (keyv, cacheable, flat-cache, file-entry-cache).** SNIPPET-CONFIRMED for existence and title only; the date was not seen. Its title, "Ongoing npm Supply Chain Attack Affecting Keyv and Related Packages ('Shai-Hulud' Worm)", appeared in two separate searches ([CSA](https://www.csa.gov.sg/alerts-and-advisories/advisories/ad-2026-009/)).
- **D9. Slopsquatting sample size (576K vs 2.23M).** COULD NOT VERIFY. The search cap was hit before this query, and arxiv.org and usenix.org are blocked.
  - An unverified guess: the two numbers may count different things (code samples vs package references).

**E. Outages**

- **E1. CrowdStrike, 19 July 2024.**
  - Out-of-bounds read: CONFIRMED by Microsoft's crash-dump analysis: "a read out-of-bounds access violation in the CSagent driver", with csagent.sys as the faulting module ([Microsoft Security Blog, 2024-07-27, page opened](https://www.microsoft.com/en-us/security/blog/2024/07/27/windows-security-best-practices-for-integrating-and-managing-security-tools/)).
  - "8.5M machines": COULD NOT VERIFY this round. The Microsoft post refers to a figure "previously shared" but does not state it; blogs.microsoft.com is blocked.
  - The 21-vs-20 input-fields detail from CrowdStrike's own root-cause analysis (RCA): COULD NOT VERIFY (crowdstrike.com blocked).
  - Parametrix's $5.4B estimate: COULD NOT VERIFY.
- **E2. Google Cloud, 12 June 2025 (null pointer in Service Control).** COULD NOT VERIFY. status.cloud.google.com is blocked and the search cap was exhausted.
- **E3. Cloudflare, 18 November 2025 (Rust `unwrap()` in FL2).** COULD NOT VERIFY (blog.cloudflare.com blocked; no searches left).
- **E4. AWS us-east-1, 19–20 October 2025 (DynamoDB DNS race; CyberCube up to $581M).** COULD NOT VERIFY (aws.amazon.com, thousandeyes.com and infoq.com blocked; no searches left).
- **E5. A major 2026 outage with a published root cause.** COULD NOT VERIFY; none was searched for.

**F. Concurrency**

- **F1. Uber Go data-race study (PLDI 2022) and DR.FIX (2025).** COULD NOT VERIFY this round (uber.com, arxiv.org and dl.acm.org blocked; no searches left). The earlier notes' links still stand as leads.

**G. Legacy**

- **G1. COBOL line counts (220B vs 800B), the DOGE/SSA rewrite outcome through 2026, and the GAO-25-107795 figures.** COULD NOT VERIFY (gao.gov and files.gao.gov blocked; no searches left).

**H. Sizing**

- **H1. GitHub "180M+" developers.** CONFIRMED, with a 2026 update.
  - Microsoft FY26 Q1 earnings call, 29 October 2025: "GitHub is now home to over 180 million developers, and the platform is growing at the fastest rate in its history – adding a developer every second" ([Microsoft Investor Relations, page opened](https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q1)).
  - FY26 Q4 earnings call, 29 July 2026: "GitHub now has 225 million users" ([Microsoft Investor Relations, page opened](https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q4)).
- **H2. SlashData / Evans Data developer counts.** COULD NOT VERIFY (slashdata.co blocked; no searches left).
- **H3. A CISQ estimate newer than the 2022 figure of $2.41T.** COULD NOT VERIFY; none was found, and it-cisq.org is blocked.

### Inferences
- The memory-safety figures the report leans on hardest are sound: Android 76% → 24% → under 20%, about 1,000x lower Rust density, and GTIG's ~35% of 90 zero-days. They can be cited with the dates corrected.
- The outage section of the final report should carry a visible caveat. None of the four post-mortems was re-read in either round. CrowdStrike's out-of-bounds read is the only root cause confirmed from a primary-adjacent source (Microsoft's crash-dump analysis).
- The supply-chain numbers need scoping language:
  - Sonatype's "new in calendar 2025" and "since November 2023" windows are different.
  - Shai-Hulud figures differ by wave and by metric (weekly vs monthly downloads; packages vs versions vs repositories).

### Gaps
- Every COULD NOT VERIFY item above. The main cause was the exhausted shared search cap, not contrary evidence.
- A future pass with working search should prioritize, in order: (1) the four outage post-mortems; (2) the slopsquatting sample size; (3) the chalk/debug counts; (4) the DOGE/SSA outcome in 2026; (5) SlashData 2025/2026; (6) the Uber and DR.FIX numbers.

## 2. Memory safety in 2025–2026: how much of the problem it still is (language or process?)

### Takeaway
Memory safety remains the pain point most clearly rooted in the language:
- Android's measured share fell from 76% (2019) to 24% (2024) and then below 20% (2025) as new code moved to Rust.
- About 35% of 2025's 90 in-the-wild zero-days were still memory-safety bugs.

The big 2026 change is that AI-driven bug finding is now aimed at memory-unsafe code at scale. Google reports "a surge in AI-discovered vulnerabilities across memory-unsafe targets like web browsers and operating systems". An April 2026 industry initiative (a vendor claim) reports "thousands of zero-day vulnerabilities" across every major OS and browser. This raises the cost of keeping C/C++ and strengthens the case for memory-safe code. The language answer (Rust plus garbage-collected MSLs) already exists.

### Cited Findings
- **Android.**
  - 76% of Android vulnerabilities were memory-safety issues in 2019 and 24% in 2024. Counts fell from 223 (2019) to fewer than 50 (2024), per search summaries ([Google Security Blog, Sept 2024](https://security.googleblog.com/2024/09/eliminating-memory-safety-vulnerabilities-Android.html); [BleepingComputer](https://www.bleepingcomputer.com/news/security/google-sees-68-percent-drop-in-android-memory-safety-flaws-over-5-years/)).
  - The share fell below 20% for the first time in 2025. About 5M lines of Rust, with 0.2 memory-safety vulnerabilities per MLOC, give a 1000x lower density than Android C/C++.
  - Rust changes have a 4x lower rollback rate and spend 25% less time in review, per search summaries ([Google Security Blog, Nov 2025](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html); [The Hacker News, Nov 2025](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html)).
  - A search summary adds that Google notes "even unsafe Rust has significantly lower vulnerability density compared to C and C++" ([The Hacker News, Nov 2025](https://thehackernews.com/2025/11/rust-adoption-drives-android-memory.html)).
- **GTIG 2025 zero-days** (page opened; published 5 March 2026) ([GTIG](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)):
  - 90 zero-days; ~35% memory safety (UAF and OOB write); enterprise 43 (48%); OS 39 (44%); browsers under 10%; mobile OS 15 (against 9 in 2024).
  - "For the first time since we started tracking zero-day exploitation, we attributed more exploitation to CSVs [commercial surveillance vendors] than to traditional state-sponsored cyber espionage groups."
- **Mandiant M-Trends 2026** (published 23 March 2026, page opened) ([Google Cloud](https://cloud.google.com/blog/topics/threat-intelligence/m-trends-2026)):
  - "Exploits remained the most common initial infection vector for the sixth consecutive year, accounting for 32% of intrusions."
  - "The mean time to exploit vulnerabilities dropped to an estimated -7 days, meaning exploitation is routinely occurring before a patch is even released."
  - "Global median dwell time rose to 14 days from 11 days."
- **Google on AI and memory-unsafe code** (Mandiant, 16 July 2026, page opened) ([Google Cloud](https://cloud.google.com/blog/topics/threat-intelligence/ai-assisted-vulnerability-management)):
  - "the industry is currently seeing a surge in AI-discovered vulnerabilities across memory-unsafe targets like web browsers and operating systems."
  - It recommends: "organizations need to phase memory-safe languages into new internal development," and "using LLMs to assist engineers with the bulk of the conversion can make these long-term migrations operationally viable."
- **Project Glasswing** (7 April 2026; vendor source, page opened) ([Anthropic](https://www.anthropic.com/glasswing)):
  - An initiative with AWS, Anthropic, Apple, Broadcom, Cisco, CrowdStrike, Google, JPMorganChase, the Linux Foundation, Microsoft, NVIDIA and Palo Alto Networks, plus 40+ further organizations.
  - It says an unreleased model found "thousands of zero-day vulnerabilities" across every major operating system and web browser, including a 27-year-old OpenBSD bug and a 16-year-old FFmpeg bug.
  - No memory-safety share is given. This is a vendor claim with no independent count.
- **Microsoft.** About 70% of CVEs were memory safety in 2016, falling to about 50% in "more recent studies", as reported from the June 2025 CISA/NSA sheet. This is a search-summary level source with no Microsoft primary seen ([CybersecurityNews](https://cybersecuritynews.com/cisa-releases-guide-to-reduce-memory-safety-vulnerabilities/); [NSA/CISA CSI](https://media.defense.gov/2025/Jun/23/2003742198/-1/-1/0/CSI_MEMORY_SAFE_LANGUAGES_REDUCING_VULNERABILITIES_IN_MODERN_SOFTWARE_DEVELOPMENT.PDF)).
- **Project Zero figures, as cited in the June 2025 sheet** (search summary): "75% of CVEs used in real-world exploits were memory safety vulnerabilities, with 67% of the 58 in-the-wild zero-day vulnerabilities discovered in 2021 falling into this category" ([NSA/CISA CSI](https://media.defense.gov/2025/Jun/23/2003742198/-1/-1/0/CSI_MEMORY_SAFE_LANGUAGES_REDUCING_VULNERABILITIES_IN_MODERN_SOFTWARE_DEVELOPMENT.PDF)).
- **CrowdStrike.** Microsoft's analysis found the July 2024 outage was "a read out-of-bounds access violation in the CSagent driver". The same post points to Rust support in the Windows kernel as a mitigation direction ([Microsoft Security Blog, 2024-07-27](https://www.microsoft.com/en-us/security/blog/2024/07/27/windows-security-best-practices-for-integrating-and-managing-security-tools/)).

### Inferences
- **Root cause: language.** Android's result came from a process choice (write new code in an MSL; leave old code alone), but the mechanism is the language guarantee.
- **The ~65% of zero-days that are not memory-safety bugs are mostly design and process failures** in enterprise edge appliances: injection, auth bypass, logic. Some are language-addressable (typed query APIs against injection), most are not.
- **AI-scale bug finding makes the stock of C/C++ more expensive to hold**, because unfound bugs get found faster. With time-to-exploit at −7 days, patching cannot keep up. This favors eliminating bug classes (a language lever) over finding bugs (a process lever).
- **The gap a new language could fill is not greenfield memory safety.** Rust and the GC MSLs cover that. The gap is migrating and interoperating with existing C/C++.

### Gaps
- No 2024–2026 Chromium or Microsoft primary figure was found.
- The Glasswing counts are self-reported, with no breakdown by bug class.
- CHERI, Arm MTE and Apple's Memory Integrity Enforcement were not researched.

## 3. C/C++ response and migration tooling in 2026 (language or process?)

### Takeaway
The C++ committee finished C++26 on 28 March 2026 without Profiles and without a borrow checker. C++26's safety gains are runtime checks and UB removal:
- a hardened standard library, which Google reports fixed 1,000+ bugs and cut fleet segfaults by 30%;
- "erroneous behavior" for uninitialized locals;
- contracts.

Profiles and a full UB catalog are aimed at C++29, so no language-level memory-safety guarantee arrives in standard C++ before about 2029.

Rust became a permanent part of the Linux kernel in December 2025. The successor and translation efforts are all still pre-1.0 or research: Carbon (0.1 at the earliest in late 2026), TrapC (no release seen), Fil-C (about 4x slowdown), TRACTOR and Microsoft's research.

### Cited Findings
- **C++26 completed.**
  - "On Saturday, the ISO C++ committee completed technical work on C++26 in (partly) sunny London Croydon, UK." Headline features: "compile-time reflection, memory safety improvements including elimination of UB for uninitialized local variables, contracts … and std::execution". Trivial relocatability was removed. These are search summaries of [Herb Sutter, 2026-03-29](https://herbsutter.com/2026/03/29/c26-is-done-trip-report-march-2026-iso-c-standards-meeting-london-croydon-uk/).
  - "The ISO C++ committee (WG21) approved the C++26 standard on March 28 … The next stage is that the final document will be prepared and sent for international approval" ([The Register, 2026-03-31](https://www.theregister.com/2026/03/31/cplusplus26_approved/); [DevClass, 2026-04-01](https://www.devclass.com/development/2026/04/01/contracts-are-in-c26-despite-disquiet-over-their-value/5213555)).
  - Formal ISO publication was not confirmed.
- **Hardened standard library.** When implemented at Google, it "fixed more than a thousand bugs and 'reduced the segfault rate across the production fleet by 30 percent'" (search summary of [The Register, 2026-03-31](https://www.theregister.com/2026/03/31/cplusplus26_approved/)). See also [C++26: Standard library hardening, isocpp.org, July 2026](https://isocpp.org/blog/2026/07/cpp26-standard-library-hardening-sandor-dargo).
- **Profiles deferred.**
  - "the standard-library hardening + contracts + reflection trio shipped, but the [[profiles::enforce]] attribute did not – it moves to C++29 via SG23's continued work on Stroustrup's P3984 type-safety profile, built on Dos Reis's P3589 general framework" ([wro.cpp, May 2026](https://wrocpp.github.io/posts/cpp-safety-state-of-the-union-may-2026/); [Whole Tomato, July 2026](https://www.wholetomato.com/blog/c26-memory-safety-what-actually-ships-for-existing-code/)).
  - A 2026 WG21 paper, [P4186R0 "A proposed plan for profiles in C++"](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2026/p4186r0.html), exists (title seen only).
- **Brno, 8–13 June 2026, the first C++29 meeting** (search summary of the trip report) ([isocpp.org](https://isocpp.org/blog/2026/06/trip-report-june-2026-iso-cpp-standards-meeting-brno-czechia-herb-sutter); [Herb Sutter, 2026-06-13](https://herbsutter.com/2026/06/13/brno-trip-report/); [WG21 agenda N5045](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2026/n5045.html)):
  - Adopted into draft C++29: "a complete catalog of all undefined behavior (UB) in C++, contract pre/post support for virtual functions".
  - For the next six months: review of a proposal "to systematically address all undefined behavior in C++, and progress on adding C++ memory safety subsetting profiles, with both aiming for inclusion in C++29."
- **Safe C++.** Discontinued in September 2025 after Profiles were prioritized ([The Register, 2025-09-16](https://www.theregister.com/2025/09/16/safe_c_proposal_ditched/)). The vote counts conflict (20/45 vs 30/45; or 19/9/11); see ledger item B1.
- **Linux kernel.**
  - At the 2025 Maintainers Summit in Tokyo, maintainers decided "Rust in the kernel is no longer experimental — it is now a core part of the kernel and is here to stay". Steven Rostedt: "There was zero pushback." Miguel Ojeda: "the experiment is done, i.e. Rust is here to stay" ([The New Stack](https://thenewstack.io/rust-goes-mainstream-in-the-linux-kernel/); [LWN](https://lwn.net/Articles/1049831/); [Slashdot, 2025-12-13](https://linux.slashdot.org/story/25/12/13/0347245/rust-in-linuxs-kernel-is-no-longer-experimental)).
  - Scale: "By April 2025, the Linux kernel contained about 34 million lines of C code, with only 25 thousand lines written in Rust." This comes from a single summary; treat it as approximate ([The New Stack](https://thenewstack.io/rust-goes-mainstream-in-the-linux-kernel/)).
- **Carbon.**
  - Its roadmap says "the end of 2026 is now the soonest that 0.1 could realistically be ready to ship", after memory-safety design was added to 0.1 ([Carbon roadmap](https://docs.carbon-lang.dev/docs/project/roadmap.html)).
  - "An early preview of Carbon's memory safety model has been developed and is ready for community feedback" ([Chandler Carruth, 2026 slides](https://chandlerc.blog/slides/2026-memory-safety-deep-3/)).
- **TrapC.** Code complete in January 2026 after missing its 1 January target; aiming for Q1 2026; "It almost works" ([The Register, 2026-01-26](https://www.theregister.com/2026/01/26/trapc_claude_c_memory_safe_robin_rowe/); [trapc.org](https://trapc.org/trapc-a-year-later/)). No release was seen.
- **Fil-C.** "a few times slower than Clang-generated code" ([LWN](https://lwn.net/Articles/1042938/)); Pizlo: "4x slowdown (and sometimes even more)" ([X](https://x.com/filpizlo/status/1936202225468297367)).
- **DARPA TRACTOR.**
  - Two benchmark batteries and three milestone projects as of September 2026; six performer systems scored ([arXiv 2609.25121](https://arxiv.org/abs/2609.25121)).
  - An academic (non-DARPA) system, ORBIT, reports about 70% (9/13) on the battery's hardest programs, "competitively within the range of six performer systems", and 100% compilation and 91.7% test success on its own evaluation ([arXiv 2604.12048](https://arxiv.org/pdf/2604.12048)).
- **Microsoft.** The "eliminate every line of C and C++ … by 2030" post (December 2025) was clarified as a research project: "Windows is *NOT* being rewritten in Rust with AI" ([InfoWorld](https://www.infoworld.com/article/4111553/microsoft-is-not-rewriting-windows-in-rust.html)).

### Inferences
- **Root cause: language, with the migration a process and tooling problem.** C++ has chosen runtime hardening and UB reduction over compile-time memory safety for at least one more cycle.
- **The measured wins in 2026 C++ are runtime checks** (Google's 30% segfault reduction), not static guarantees.
- **The C/C++ migration space is crowded with well-funded efforts** (Google/Carbon, DARPA, Microsoft, Linux/Rust) and increasingly LLM-assisted. A new language entering it competes with Rust as the translation target and with Carbon on interop. That is weak ground for a small team.

### Gaps
- The formal ISO publication date of C++26 and the full Brno minutes were not seen.
- Whether Carbon 0.1 or TrapC shipped by September 2026 was not seen.
- The official TRACTOR performer list and 2026 scores were not seen.

## 4. Policy and regulation in 2026 (language or process?)

### Takeaway
US federal pressure on memory safety has softened in form:
- The 1 January 2026 CISA roadmap date passed as voluntary guidance, and no public count of vendors who met it was found.
- The March 2026 US strategy moved away from the 2023 idea of shifting liability to software makers.

The EU went the other way. The CRA's vulnerability- and incident-reporting duties became binding on 11 September 2026, and full obligations follow on 11 December 2027. Regulation is pushing process (reporting, patching, SBOMs), not a choice of language.

### Cited Findings
- **CISA/FBI Product Security Bad Practices.**
  - "Software manufacturers should publish a memory safety roadmap by January 1, 2026" ([ic3.gov PDF, October 2024 publication](https://www.ic3.gov/CSA/2024/241016-2.pdf); [CISA](https://www.cisa.gov/resources-tools/resources/product-security-bad-practices)).
  - A commentary summary says the guidance "was never legally binding, but it has migrated into procurement questionnaires, customer security questionnaires, EU Cyber Resilience Act readiness reviews, and acquisition diligence packs." This is a single unattributed summary, most likely [StepTo](https://stepto.net/blog/memory-safety-roadmap-c-rust-migration-2026) or [corrode](https://corrode.dev/blog/memory-safety/); treat it as opinion.
- **Secure by Design pledge.** "a voluntary pledge. CISA does not enforce nor verify adherence to the pledge" ([CISA](https://www.cisa.gov/securebydesign/pledge)).
- **US strategy, March 2026.**
  - "On March 6, 2026, the Trump administration released … President Trump's Cyber Strategy for America, alongside an executive order on combating cybercrime and fraud … only five pages of text."
  - It "departs from key portions of the Biden Administration's 2023 National Cybersecurity Strategy, which … sought to shift liability toward software developers for insecure products" ([Inside Privacy / Covington](https://www.insideprivacy.com/u-s-national-cybersecurity-strategy/white-house-releases-new-national-cyber-strategy-and-executive-order/); [Mayer Brown](https://www.mayerbrown.com/en/insights/publications/2026/03/trump-administration-releases-cyber-strategy-for-america-and-related-executive-order-on-combatting-cybercrime); [Davis Wright Tremaine](https://www.dwt.com/blogs/privacy--security-law-blog/2026/03/president-trump-cyber-strategy)).
  - Whether the strategy mentions memory safety was not found.
- **Other 2026 White House actions (titles only; content not read):**
  - A June 2026 fact sheet on "Advanced Cryptographic Attacks" ([White House](https://www.whitehouse.gov/fact-sheets/2026/06/fact-sheet-president-donald-j-trump-secures-the-nation-against-advanced-cryptographic-attacks/)).
  - An August 2026 order on "Transnational Cyber-Enabled Crime" ([White House](https://www.whitehouse.gov/presidential-actions/2026/08/expanding-capabilities-to-combat-transnational-cyber-enabled-crime/)).
- **EU CRA.**
  - "The main obligations introduced by the Act will apply from 11 December 2027, while the reporting obligations apply from 11 September 2026."
  - Manufacturers must give "an early warning within 24 hours … and a full notification within 72 hours." The Article 14 reporting duties apply "to all in-scope products already on the EU market, as well as new products."
  - Sources: [Goodwin, Sept 2026](https://www.goodwinlaw.com/en/insights/publications/2026/09/alerts-lifesciences-technology-preparing-for-eu-cyber-resilience-act); [Pearl Cohen](https://www.pearlcohen.com/eu-cyber-reporting-obligations-take-effect-on-september-11/); [European Commission, 2026-09-11](https://commission.europa.eu/news-and-media/news/safer-and-more-secure-digital-products-2026-09-11_en).
  - The same summary cites fines "up to €15 million or … 2.5% of … worldwide annual turnover" for breaches of reporting duties. Check this penalty tier against the regulation text before using it.

### Inferences
- **Root cause of the policy pressure: process.** Neither the US nor the EU mandates a language. The CRA's reporting clock (24h/72h) and liability exposure raise the cost of every exploitable bug. Memory-safety bugs are the class a language can remove, so the CRA indirectly rewards MSL adoption without requiring it.
- **The US retreat from the liability shift weakens the "regulation will force a new language" argument** for the US market.

### Gaps
- No count of vendors who published memory-safety roadmaps was found.
- The CRA penalty tiers and the first weeks of CRA reporting were not verified.
- It is not known whether the 2026 US strategy mentions memory safety.

## 5. Software supply chain, 2025–2026: new incidents and the platform response (language or process?)

### Takeaway
Supply-chain compromise kept growing into 2026:
- Sonatype counted 454,600 new malicious packages in 2025.
- Google cites OpenSSF data showing a 1,444% rise in identified malicious packages from 2024 to 2025. This is a different dataset, and it conflicts in scale with Sonatype.
- 2026 brought the axios compromise (March, 100M+ weekly downloads, attributed to North Korea), a months-long npm/PyPI/Docker Hub campaign (TeamPCP), and Mini Shai-Hulud (May).

The biggest 2026 change is the platform response in July 2026, reported by Google:
- npm v12 disables lifecycle scripts by default.
- Dependabot waits three days before proposing updates.
- PyPI rejects new files on releases older than 14 days.

These fix the install-hook and publishing parts of the problem at the package-manager level. What is left for a language is runtime authority: what an imported dependency's code can do once it is running.

### Cited Findings
- **Sonatype 2026 report.** More than 454,600 new malicious packages in 2025; 1.233M+ cumulative; over 99% of malware on npm; 9.8T downloads (+67%); published 28 January 2026 ([Sonatype](https://www.sonatype.com/state-of-the-software-supply-chain/2026/open-source-malware); [Infosecurity Magazine](https://www.infosecurity-magazine.com/news/454000-malicious-open-source/)).
- **GTIG supply-chain guidance** (Kelli Vanderlee and Stuart Carrera, 30 July 2026, page opened) ([Google Cloud](https://cloud.google.com/blog/topics/threat-intelligence/mitigation-guidance-for-supply-chain-compromise)):
  - **OpenSSF data:** "Statistics compiled by the Open Source Security Foundation (OpenSSF) … indicate that the number of malicious open source software packages identified increased exponentially, or 1,444% from 2024 to 2025."
  - **TeamPCP:** "from February to May 2026, UNC6780 (aka 'TeamPCP') conducted extensive open source supply chain compromises targeting ecosystems like PyPI, npm, and Docker Hub." It abused the GitHub Actions `pull_request_target` trigger and deployed a credential stealer.
  - **axios:**
    - "In March 2026, GTIG observed the introduction of a malicious dependency in the legitimate `axios` package … a dropper that deploys the WAVESHAPER.V2 backdoor", attributed to the North Korean actor MIDNIGHT NEPTUNE.
    - The maintainer account was compromised through social engineering.
    - The package has "Over 100 million weekly downloads". The malicious versions were "Removed from the npm registry within three hours". GTIG supported customers "in at least 15 industry verticals and 13 different countries".
  - **Other incidents:**
    - The Notepad++ update infrastructure was compromised from June to December 2025 (UNC6688).
    - DAEMON Tools installers were compromised in early 2026 (UNC6863).
    - LiteLLM and Telnyx releases on PyPI were retroactively poisoned (cited as the reason for PyPI's new policy).
  - **Assessment:** "GTIG assesses with high confidence that the growth in very large-scale, open-source supply chain compromise campaigns, including use of worms and iterative compromises in 2025 and early 2026, represent a significant expansion".
  - **Platform changes, as reported by GTIG (the linked vendor pages were not opened):**
    - "npm v12 disables all lifecycle scripts by default (`allowScripts: off`)".
    - "Dependabot now enforces a default three-day cooldown on version updates … (such as the historical chalk and debug hijackings)".
    - "PyPI now natively rejects new file uploads to any release older than 14 days".
  - **Recommendations:** `ignore-scripts=true`, `minimumReleaseAge` of at least 24 hours, trusted publishing through OIDC instead of long-lived tokens, and restricting `pull_request_target`.
- **Mandiant CI/CD hardening post** (24 September 2026, page opened). It names three active tactics: attacks on trusted scanners and AI developer tools in pipelines; attacks on developer workstations and IDEs; and "GitHub Actions cache poisoning, OIDC token extraction, and mutable action tag subversion" ([Google Cloud](https://cloud.google.com/blog/topics/threat-intelligence/hardening-code-pipelines-and-ci-cd-infrastructure)).
- **Mini Shai-Hulud (May 2026).** "170+ npm packages and 2 PyPI packages across 404 malicious versions" ([Microsoft, updated 2026-05-13](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)).
  - Microsoft calls it the first attack to span npm and PyPI "in a single coordinated operation". GTIG's account of TeamPCP (PyPI + npm + Docker Hub, February–May 2026) makes that "first" claim contestable.
- **Shai-Hulud 2.0 (November 2025).** 796 packages, 1,092 versions, 20M+ weekly downloads, 500+ GitHub users, 150+ organizations; the payload ran from a `preinstall` hook and installed Bun ([Datadog](https://securitylabs.datadoghq.com/articles/shai-hulud-2.0-npm-worm/)). The Shai-Hulud 2.0 guidance names Zapier, PostHog and Postman maintainer accounts as compromised ([Microsoft](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)).

### Inferences
- **Root cause: mostly process, partly language.** Account takeover through social engineering (axios), CI trigger abuse (TeamPCP), stolen long-lived tokens and install hooks (Shai-Hulud) are all process and package-manager failures. Registries and tools began fixing them in July 2026.
- **What the platform fixes leave open.** Once malicious code runs as an imported library, for example the axios dropper at runtime, nothing in JavaScript, Python or Java limits what it can reach.
  - A capability-safe language would still block that part.
  - But the npm v12 default removes the cheapest attack path (install-time execution). That shrinks, somewhat, the share of attacks a language-level fix is uniquely placed to stop. The report should weigh this when ranking supply chain as the top language opportunity.
- **The two growth figures (OpenSSF 1,444% vs Sonatype) are not comparable.** They come from different datasets with different inclusion rules. Cite both with their sources, or cite neither as "the" growth rate.

### Gaps
- The chalk/debug counts, the full count for Shai-Hulud v1, the date of the keyv wave (CSA AD-2026-009) and the slopsquatting sample size were not verified.
- The npm v12 and Dependabot changes were seen only as GTIG reported them (github.blog is blocked).
- No measured dollar cost of supply-chain attacks was found.

## 6. Outages and the cost of poor software quality (language or process?)

### Takeaway
This round confirmed one root cause from a primary-adjacent source: Microsoft's crash-dump analysis identifies CrowdStrike's July 2024 outage as an out-of-bounds read in the kernel driver. That is a bug class a bounds-checked language would turn into a handled error. The other post-mortems and all loss estimates are unverified this round:
- Google Cloud, June 2025 (null pointer);
- Cloudflare, November 2025 (Rust `unwrap()`);
- AWS, October 2025 (DNS race);
- the CrowdStrike machine count and dollar losses.

No 2026 outage was researched. No CISQ estimate newer than 2022 was found.

### Cited Findings
- CrowdStrike root cause: "a read out-of-bounds access violation in the CSagent driver", faulting module csagent.sys ([Microsoft Security Blog, 2024-07-27](https://www.microsoft.com/en-us/security/blog/2024/07/27/windows-security-best-practices-for-integrating-and-managing-security-tools/)).

### Inferences
- **Root cause: mixed.** The trigger bugs are language-addressable: OOB read (CrowdStrike, confirmed), null field (Google, unverified) and unhandled error (Cloudflare, unverified). The blast radius comes from process: global, unstaged pushes of content or configuration.
- **The recommendation should not rest on outage dollar figures** until the post-mortems and loss estimates are re-verified.

### Gaps
- CrowdStrike's 8.5M machines and Parametrix's $5.4B; the Google Cloud, Cloudflare and AWS post-mortems; CyberCube's $581M; any 2026 outage; and a CISQ update were all not verified. Blocked hosts and the search cap are the reason.

## 7. Concurrency (language or process?)

### Takeaway
Nothing new could be verified this round. The Uber PLDI 2022 figures (46M lines of Go, 2,000+ races, 1,000+ fixed) and DR.FIX (2025) remain as cited in `industry_scale_pain.md` from the first round, unverified.

### Cited Findings
- None re-verified this round. Leads from the first round: [ACM DL](https://dl.acm.org/doi/10.1145/3519939.3523720); [arXiv 2204.00764](https://arxiv.org/abs/2204.00764); [DR.FIX, arXiv 2504.15637](https://arxiv.org/pdf/2504.15637).

### Inferences
- **Root cause, as framed in the first round:** in-process races are a language issue (Rust's Send/Sync prevents them in safe code). Distributed races are a design and process issue.

### Gaps
- All figures in this section are unverified in this round.

## 8. Legacy code: COBOL, SSA, GAO (language or process?)

### Takeaway
Nothing new could be verified this round. The COBOL line-count range (220B to 800B+), the SSA's 60M+ lines, the DOGE rewrite plan, and the GAO-25-107795 figures (about 79–80% of federal IT spending on operations and maintenance; 3 of 10 critical legacy systems modernized by February 2025) remain first-round, snippet-level claims. The DOGE/SSA outcome through 2026 is unknown.

### Cited Findings
- None re-verified this round. Leads: [GAO-25-107795](https://www.gao.gov/products/gao-25-107795); [The Stack](https://www.thestack.technology/cobol-in-daily-use/); [Gizmodo](https://gizmodo.com/doge-plans-to-rewrite-entire-social-security-codebase-in-just-a-few-months-report-2000582062).

### Inferences
- **Root cause: process.** The problems are knowledge loss, verification of translations, and mainframe coupling. COBOL itself is not memory-unsafe.
- **LLM-assisted migration favors existing target languages.** Google's July 2026 guidance on LLM-assisted C/C++ → Rust conversion is an example ([Google Cloud](https://cloud.google.com/blog/topics/threat-intelligence/ai-assisted-vulnerability-management)).

### Gaps
- The 2026 outcome of the SSA rewrite, a primary COBOL line count and the GAO figures were not verified.

## 9. Sizing the developer population, 2025–2026

### Takeaway
The best verified size indicator is GitHub, as reported in Microsoft earnings calls:
- "over 180 million developers" in October 2025;
- "225 million users" in July 2026.

These count accounts, not unique professional developers. They set an upper bound, not a direct measure, for "millions of developers". AI-assisted coding is now the norm on the platform: 50 million Copilot users, and "one in three pull requests … involves an agent".

### Cited Findings
- **29 October 2025:** "GitHub is now home to over 180 million developers … adding a developer every second"; "80% of new developers on GitHub start with Copilot within their first week"; Copilot has "over 26 million users"; "over 500 million pull requests merged over the past year" ([Microsoft FY26 Q1 earnings](https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q1)).
- **29 July 2026:** "GitHub now has 225 million users"; "GitHub Copilot now has 50 million users"; "one in three pull requests on GitHub now involves an agent" ([Microsoft FY26 Q4 earnings](https://www.microsoft.com/en-us/investor/events/fy-2026/earnings-fy-2026-q4)).
- **Microsoft 2025 Annual Report (15 October 2025):** "GitHub Copilot now has more than 20 million users" ([Microsoft Annual Report 2025](https://www.microsoft.com/investor/reports/ar25/index.html)).

### Inferences
- **Microsoft's wording changed from "developers" (October 2025) to "users" (July 2026).** Cite the July 2026 figure as "GitHub users".
- **How to size a language's audience:** "hundreds of millions of accounts, tens of millions of professional developers" is defensible only if a professional-developer estimate (SlashData or Evans Data) is added. None was verified here.
- **Agent-written pull requests (one in three) raise the value of guarantees that are checked mechanically** over those that rest on review.

### Gaps
- SlashData and Evans Data 2025–2026 totals were not verified (host blocked; search cap reached).
- No per-language developer counts were found (C/C++, npm, PyPI).

## 10. Summary for the report writer: language vs process, per pain point (with this round's evidence status)

### Takeaway
After this round, memory safety is the best-evidenced pain point whose root cause is the language. That evidence confirms that existing languages already solve it for new code; the open part is migration. Supply chain remains the largest open problem with a language-level component. But 2026 package-manager changes are closing its install-time path, so any pitch must centre on runtime least authority for dependencies. Outages, concurrency and legacy could not be re-verified this round, so the report should cite them with caveats.

### Cited Findings

| Pain point | Best verified 2025–2026 evidence | Root cause | Evidence status this round |
|---|---|---|---|
| Memory safety | Android under 20% memory-safety share (2025); ~1000x lower Rust density ([Google, Nov 2025](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)); ~35% of 90 zero-days ([GTIG](https://cloud.google.com/blog/topics/threat-intelligence/2025-zero-day-review)); AI-found bugs surging in memory-unsafe targets ([Google, July 2026](https://cloud.google.com/blog/topics/threat-intelligence/ai-assisted-vulnerability-management)) | Language (the fix is delivered through a process choice: new code in an MSL) | GTIG confirmed; Android snippet-confirmed |
| C/C++ successor path | C++26 without Profiles; Profiles and a UB catalog aimed at C++29 ([Sutter](https://herbsutter.com/2026/06/13/brno-trip-report/)); Linux Rust permanent ([LWN](https://lwn.net/Articles/1049831/)) | Language plus migration tooling | Snippet-confirmed |
| Supply chain | 454,600 new malicious packages in 2025 ([Sonatype](https://www.sonatype.com/state-of-the-software-supply-chain/2026/open-source-malware)); axios March 2026, TeamPCP, npm v12 script defaults ([GTIG](https://cloud.google.com/blog/topics/threat-intelligence/mitigation-guidance-for-supply-chain-compromise)); Mini Shai-Hulud ([Microsoft](https://www.microsoft.com/en-us/security/blog/2025/12/09/shai-hulud-2-0-guidance-for-detecting-investigating-and-defending-against-the-supply-chain-attack/)) | Mostly process; runtime ambient authority is the language part | GTIG, Microsoft and Datadog confirmed; Sonatype snippet-confirmed |
| Outages | CrowdStrike OOB read ([Microsoft](https://www.microsoft.com/en-us/security/blog/2024/07/27/windows-security-best-practices-for-integrating-and-managing-security-tools/)) | Trigger bug is language-addressable; blast radius is process | Only the CrowdStrike root cause confirmed |
| Concurrency | none re-verified | In-process: language; distributed: process | Could not verify |
| Legacy/COBOL | none re-verified | Process (translation and verification) | Could not verify |
| Regulation | CRA reporting live 11 September 2026 ([Goodwin](https://www.goodwinlaw.com/en/insights/publications/2026/09/alerts-lifesciences-technology-preparing-for-eu-cyber-resilience-act)); US liability shift dropped ([Inside Privacy](https://www.insideprivacy.com/u-s-national-cybersecurity-strategy/white-house-releases-new-national-cyber-strategy-and-executive-order/)) | Process | Snippet-confirmed |

### Inferences
- **The two strongest 2026 forces point toward eliminating bug classes by construction:** AI-scale vulnerability discovery and agent-written code. That favors languages whose guarantees are checked mechanically, not by reviewers.
- **For supply chain, the language-level opening is now narrower and sharper:** runtime least authority for imported code. Install scripts are being handled by npm v12, PyPI and Dependabot defaults.

### Gaps
- See Section 1's ledger. The outage, concurrency, legacy and slopsquatting items should be re-checked before the final report cites their numbers as verified.
