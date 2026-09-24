# Industry follow-up: settling the remaining unverified items (as of 24 September 2026)

_Status: complete for this round. 40 web searches used._

_Method note. Inputs read first: `gap_fill_industry.md` and `primary_reads_industry_partial.md` (both in full) and a skim of `industry_scale_pain.md`. Items already CONFIRMED there were not rechecked. Status labels: CONFIRMED (page or data opened and read in this session), SNIPPET-CONFIRMED (page not opened, but two or more independent search results agree), CORRECTED (old value, new value, source), COULD NOT VERIFY (what was tried)._

_Fetch attempts this round that returned EGRESS_BLOCKED (one try per domain, then stopped): blog.cloudflare.com, status.cloud.google.com, aws.amazon.com, www.cybcube.com, www.parametrixinsurance.com, nebius.com, www.cisa.gov, www.ic3.gov, www.aarno-labs.com, fil-c.org, sprite.utsa.edu, api.npmjs.org, evansdata.com, www.cncf.io. Reachable and used: www.microsoft.com (web fetch) and registry.npmjs.org (package metadata and the npm 12.0.0 tarball, read with curl)._

_The search tool returns a list of result titles and URLs plus one merged summary. It does not give a separate snippet per URL. "Search summary" below means that merged text. SNIPPET-CONFIRMED is used only when two or more independent outlets appear in the result list and their titles or the summary carry the claim. GitHub repositories other than the project's own were not read (session rule), so the Carbon roadmap and release pages on github.com were not opened._

**Ledger at a glance** (details and sources in the sections below)

| Item | Status now | Note |
|---|---|---|
| Google Cloud 12 Jun 2025: Service Control null pointer, no feature flag | SNIPPET-CONFIRMED | Matches old note |
| Cloudflare 18 Nov 2025: FL2 Rust `unwrap()` on oversized feature file | SNIPPET-CONFIRMED | Limit 200, normal ~60 |
| AWS 19–20 Oct 2025: DynamoDB DNS Enactor race | SNIPPET-CONFIRMED | |
| CyberCube AWS "up to $581M" | CORRECTED | $38M–$581M range; likely ~$40M |
| CrowdStrike 8.5M devices | SNIPPET-CONFIRMED | Microsoft estimate, "less than one percent" |
| Parametrix $5.4B (Fortune 500 ex-Microsoft) | SNIPPET-CONFIRMED | Per-company average conflicts; avoid it |
| 2026 outages | NEW | Microsoft 365 22 Jan (capacity); Cloudflare 20 Feb (empty filter matched all 1,100 BYOIP prefixes) |
| CISA Bad Practices v2 dropped the roadmap item | CORRECTED (reverses the tentative correction) | v2 keeps it: "by the end of 2025", exempts products leaving support before 1 Jan 2030 |
| 2026 federal memory-safety action | COULD NOT VERIFY | None found |
| Carbon 0.1 shipped | COULD NOT VERIFY | Not seen; roadmap says end of 2026 at the earliest |
| Fil-C performance | No change | ~4x on raw benchmarks; no newer figure |
| TRACTOR performers | Aarno Labs Tenjin SNIPPET-CONFIRMED | No full list or 2026 scores |
| chalk/debug: 8 Sep 2025, 18 packages, ~2.6B weekly | SNIPPET-CONFIRMED; date CONFIRMED (npm registry) | |
| Shai-Hulud v1 "500+ packages" | SNIPPET-CONFIRMED as CISA's figure | Others: ~200 packages, 500+ versions |
| npm v12 install scripts off by default | CONFIRMED (npm registry and bundled docs) | 12.0.0 on 8 Jul 2026; dependency install scripts skipped unless in `allowScripts`; git/remote deps off |
| axios, March 2026 | CONFIRMED (Microsoft page; registry) | 1.14.1 and 0.30.4, 31 Mar UTC; 70M vs 100M weekly conflict |
| OpenSSF "1,444%" | COULD NOT VERIFY at source | GTIG only; Endor Labs "14x in two years" is the nearest independent figure |
| Slopsquatting 19.7%, 205,474, 43%/58% | SNIPPET-CONFIRMED | Sample size is 2.23M; unit of 19.7% unsettled |
| "The Range Shrinks, the Threat Remains" | SNIPPET-CONFIRMED | 4.62%–6.10%; 53 shared names still registrable |
| SlashData 47.2M (2025) | SNIPPET-CONFIRMED | 36.5M professional; Q1 2026 cloud-native 19.9M |
| Evans Data | Single-source snippet | 27M |
| JetBrains population | COULD NOT VERIFY | Not searched |
| Uber PLDI 2022 figures | SNIPPET-CONFIRMED | |
| DR.FIX | SNIPPET-CONFIRMED (weak) | PLDI 2025; 193 races, 86% accepted; ~4,000 races fixed at Uber |
| COBOL line counts; DOGE/SSA 2026 outcome | COULD NOT VERIFY | No 2026 status found |
| CISQ newer than 2022 | None found | |

## 1. Outage root causes (Google Cloud, Cloudflare, AWS, CrowdStrike, and 2026)

### Takeaway
All four post-mortem root causes are now at least SNIPPET-CONFIRMED; the primary pages themselves stay blocked. One loss figure needs rewording. CyberCube's AWS estimate is a range of **$38M to $581M, and CyberCube expected the outcome near the low end (about $40M)**, so "up to $581M" should not be quoted on its own. Two 2026 outages have published causes: Microsoft 365 on 22 January 2026 (capacity shortfall during maintenance) and Cloudflare on 20 February 2026 (an automation bug where an empty query parameter matched every BYOIP prefix). Neither is a memory-safety bug. The Cloudflare one is a "missing value treated as match-all" logic bug that shipped as part of Cloudflare's own post-November safety programme.

### Cited Findings

**E2. Google Cloud, 12 June 2025: SNIPPET-CONFIRMED.**
- Search summary: "On May 29, 2025, engineers had deployed a new feature for additional quota policy checks, but the code lacked proper error handling and feature flag protection." On 12 June "a policy change was made to the regional Spanner tables that Service Control depends on", which "inadvertently introduced 'unintended blank fields'". "Service Control instances crashed whenever encountering null pointer exceptions caused by corrupted policy data, entering repeated crash loops."
- Timeline in the same summary: triage "within 2 minutes", root cause "within 10 minutes", "red-button" rollout started "within 25 minutes", fix rollout "completed within 40 minutes, leading to recovery in most regions". Start 10:51 PDT; the whole incident ran "over seven hours".
- Results carrying it: [Google Cloud Service Health incident page](https://status.cloud.google.com/incidents/ow5i3PPK96RduMcb1SsW) (blocked for fetch); [ThousandEyes](https://www.thousandeyes.com/blog/google-cloud-outage-analysis-june-12-2025); [CybersecurityNews](https://cybersecuritynews.com/google-massive-cloud-outage-linked/); [Panto](https://www.getpanto.ai/blog/how-a-null-pointer-exception-brought-down-mighty-google-7-hours-of-downtime-explained); [OutageCost.com](https://outagecost.com/case-studies/gcp-june-2025).
- The earlier note matches this. No correction.

**E3. Cloudflare, 18 November 2025: SNIPPET-CONFIRMED.**
- Search summary: "a ClickHouse permission update at 11:05 UTC caused the Bot Management feature-file query to double its output via duplicate rows. The Bot Management system had a hardcoded limit of 200 features (normal usage was approximately 60 features)." "When the oversized file with more than 200 features was propagated, the FL2 Rust code called Result::unwrap() on an error value, causing an unhandled panic."
- Only FL2 customers saw 5xx errors: "Customers on the old proxy engine, known as FL, did not see errors, but bot scores were not generated correctly, resulting in all traffic receiving a bot score of zero." Workers KV and Access were hit; impact was reduced "at 13:04 when a patch was made to Workers KV to bypass the core proxy."
- An exact-phrase search for the panic text (`fl2_worker_thread panicked` plus `called Result::unwrap() on an Err value`) returned [Simon Willison quoting Matthew Prince](https://simonwillison.net/2025/Nov/19/matthew-prince/), [Tech Scoop](https://techscoop.substack.com/p/what-happened-cloudflare-outage-november) and [Hackaday](https://hackaday.com/2025/11/20/how-one-uncaught-rust-exception-took-out-cloudflare/). Hackaday's summary adds that "in the old FL proxy code this situation was apparently cleanly detected and handled".
- Other results: [Cloudflare blog](https://blog.cloudflare.com/18-november-2025-outage/) (blocked); [Pinggy](https://pinggy.io/blog/cloudflare_outage_november_18_2025/); [SoftwareSeni](https://www.softwareseni.com/the-2025-aws-and-cloudflare-outages-explained/); [Panto](https://www.getpanto.ai/blog/cloudflare-outage); [InfoQ](https://www.infoq.com/news/2025/11/cloudflare-global-outage-cause).
- Refinement to the old note: the "about 60 features" figure is the normal count and "200" is the preallocated limit. The old note's "grew from about 60 to more than 200" is consistent.

**E4. AWS us-east-1, 19 to 20 October 2025.**
- Root cause: SNIPPET-CONFIRMED. Search summary: "a race condition in DynamoDB's automated DNS management system that left an empty DNS record for the service's regional endpoint." It began "at 11:48 PM PDT on October 19". "Enactor 2 initiated the cleanup process … Simultaneously, the delayed Enactor 1 completed its work, applying the older plan and overwriting Enactor 2's recent updates. The cleanup process then removed the 'outdated' plan that had just been applied, resulting in empty DNS records." AWS "disabled the DynamoDB DNS Planner and DNS Enactor automation worldwide" until the race is fixed. Results: [The Register, "A single DNS race condition brought AWS to its knees"](https://www.theregister.com/2025/10/23/amazon_outage_postmortem/); [InfoQ](https://www.infoq.com/news/2025/11/aws-dynamodb-outage-postmortem/); [Pragmatic Engineer](https://blog.pragmaticengineer.com/aws-outage-us-east-1/); [Computerworld](https://www.computerworld.com/article/4082890/the-aws-outage-post-mortem-is-more-revealing-in-what-it-doesnt-say.html); [AWS post-event summary](https://aws.amazon.com/message/101925/) (blocked).
- CyberCube loss estimate: **CORRECTED (scope)**.
  - Old: "CyberCube estimates insured losses up to $581M."
  - New: a preliminary insured-loss range of **$38M to $581M**, with losses "most likely" toward "the lower end of the range". Two trade outlets headline the likely value as about $40M.
  - Search summary: "projecting a range between $38 million and $581 million … Most likely, losses for the insurance industry will fall toward the lower end of the range." The outage "affected more than 2,000 large organizations and nearly 70,000 organizations in total."
  - Results: [CyberCube news release](https://www.cybcube.com/news/insurance-loss-estimate-for-aws-amazonk-outage) (blocked); [Business Wire, 2025-10-23](https://www.businesswire.com/news/home/20251023532923/en/CyberCube-Reveals-Insurance-Loss-Estimate-for-AWS-Amazonk-Outage); [Insurance Journal, "CyberCube: Insured Loss Estimate From AWS Outage Likely About $40M", 2025-10-27](https://www.insurancejournal.com/news/national/2025/10/27/845197.htm); [Claims Journal, "…Around $40M"](https://www.claimsjournal.com/news/national/2025/10/27/333727.htm); [Reinsurance News, "$38-581m"](https://www.reinsurancene.ws/cybercube-estimates-preliminary-aws-outage-loss-range-of-38-581m/); [Insurance Times](https://www.insurancetimes.co.uk/news/loss-estimate-for-aws-outage-between-38m-and-581m/1456730.article).

**E1. CrowdStrike, 19 July 2024.**
- Out-of-bounds read: already CONFIRMED last round (Microsoft crash-dump analysis). Not rechecked.
- 8.5M machines: SNIPPET-CONFIRMED. Search summary: "Microsoft estimated that CrowdStrike's update affected 8.5 million Windows devices, or less than one percent of all Windows machines." Results: [Official Microsoft Blog, 2024-07-20](https://blogs.microsoft.com/blog/2024/07/20/helping-our-customers-through-the-crowdstrike-outage/) (blogs.microsoft.com blocked last round, not retried); [CNBC](https://www.cnbc.com/2024/07/20/microsoft-says-about-8point5-million-of-its-devices-affected-by-crowdstrike-related-outage.html); [TechCrunch](https://techcrunch.com/2024/07/20/microsoft-says-8-5m-windows-devices-were-affected-by-crowdstrike-outage/); [PC Gamer](https://www.pcgamer.com/software/windows/microsoft-says-85-million-devices-were-affected-by-the-crowdstrike-bug-or-less-than-one-percent-of-all-windows-devices-as-new-details-emerge-on-fridays-tech-meltdown/); [TechTarget](https://www.techtarget.com/searchsecurity/news/366596532/Microsoft-Faulty-CrowdStrike-update-affected-85M-devices). Wording: Microsoft's figure is an estimate of devices affected ("about 8.5 million"), not a count of machines that crashed.
- Parametrix $5.4B: SNIPPET-CONFIRMED. Search summary: the outage "will likely cost the Fortune 500, excluding Microsoft, at least $5.4 billion in direct financial losses"; healthcare "$1.94 billion"; banking "$1.15 billion"; "Cyber insurance will only cover 10% to 20% of the losses"; it "directly impacted about one-quarter of the Fortune 500, which includes 124 companies". Results: [Parametrix](https://www.parametrixinsurance.com/in-the-news/crowdstrike-to-cost-fortune-500-5-4-billion-insured-loss-range-of-540-million-to-1-08-billion) (blocked); [Cybersecurity Dive](https://www.cybersecuritydive.com/news/crowdstrike-cost-fortune-500-losses-cyber-insurance/722396/); [Axios, 2024-07-24](https://www.axios.com/2024/07/24/fortune-500-crowdstrike-outage-impact); [Computer Weekly](https://www.computerweekly.com/news/366598426/Fortune-500-stands-to-lost-5bn-plus-from-CrowdStrike-incident); [Beinsure](https://beinsure.com/news/us-fortune-500-loss-crowdstrike/).
- Small conflict on the per-company figure: this round's summary says Parametrix expected "the average per company loss to exceed $143 million", while `industry_scale_pain.md` gives a $44M weighted average and $143M for airlines. The $143M most likely belongs to airlines only. Use the $5.4B total and the sector totals; avoid the per-company average unless the Parametrix page is read.
- The 21-vs-20 input fields detail from CrowdStrike's own RCA was not searched again this round (crowdstrike.com blocked). It stays at the level of the earlier note.

**E5. Major 2026 outages with a published cause.**
- **Cloudflare, 20 February 2026 (BYOIP): SNIPPET-CONFIRMED.**
  - Search summary: "at 17:48 UTC, Cloudflare experienced a service outage when a subset of customers who use Cloudflare's Bring Your Own IP (BYOIP) service saw their routes to the Internet withdrawn via Border Gateway Protocol (BGP)." Total duration "6 hours and 7 minutes".
  - Cause: "A cleanup sub-task queried API with pending_delete parameter missing value, interpreted as request for all BYOIP prefixes instead of deletion candidates … the API query used to filter those prefixes contained an empty string, which matched everything."
  - Scale: "25% of Cloudflare's 4,306 total BYOIP prefixes (1,100) were withdrawn".
  - Irony worth noting: the change was part of Cloudflare's "Code Orange: Fail Small work to push all changes toward safe, automated, health-mediated deployment" (quoted from Cloudflare's post via the summary).
  - Results: [Cloudflare blog, "Cloudflare outage on February 20, 2026"](https://blog.cloudflare.com/cloudflare-outage-february-20-2026/) (blocked); [365i, "Automation Script Deleted 1,100 BGP Prefixes"](https://www.365i.co.uk/news/2026/02/24/cloudflare-outage-automation-deleted-1100-prefixes/); [ASCII News, "BGP Withdrawal Bug Cascades Across 1,100 Prefixes"](https://ascii.co.uk/news/article/news-20260223-364f287d/cloudflare-byoip-outage-bgp-withdrawal-bug-cascades-across-1); [Surfing Complexity, 2026-02-21](https://surfingcomplexity.blog/2026/02/21/quick-takes-on-feb-20-cloudflare-outage/); [CybersecurityNews](https://cybersecuritynews.com/cloudflare-down-6-hour-of-massive-global-service-outage/); [IT Pro](https://www.itpro.com/infrastructure/networking/cloudflare-outage-explained-what-happened-who-was-impacted-and-how-was-it-resolved).
  - The summary's list of affected services (Uber Eats, Bet365, Wikipedia, Workday and others) came from one unattributed line; do not use it without a check.
- **Microsoft 365, 22 January 2026: SNIPPET-CONFIRMED (preliminary cause only).**
  - Search summary: Microsoft's post-incident report attributed it to "elevated service load resulting from reduced capacity during maintenance for a subset of North America hosted infrastructure", plus "a third-party networking issue in North America". It started about 2:37 p.m. ET and lasted 8 to 9 hours; Outlook, Teams and Defender were affected.
  - The summary also says "Microsoft has not released a detailed Root Cause Analysis (RCA)". Results: [National CIO Review](https://nationalcioreview.com/articles-insights/extra-bytes/microsoft-restores-365-services-after-hours-of-disruption/); [Boston University TechWeb, 2026-01-22](https://www.bu.edu/tech/2026/01/22/160781/); [WebProNews](https://www.webpronews.com/microsoft-365-outage-halts-global-operations-on-jan-22-2026/); [Windows Forum](https://windowsforum.com/windows-news.4/microsoft-365-outage-jan-22-2026-outlook-defender-purview-teams-impact.398262/).
- **Not usable:**
  - A search summary said "Google Cloud us-central1 Disruption (August 19, 2026)" was a storm that disabled cooling. The only link was a **Nebius** post-mortem titled "Incident post-mortem analysis: us-central1 service disruption on August 19, 2026" ([Nebius](https://nebius.com/blog/posts/incident-post-mortem-analysis-us-central1-service-disruption-on-august-19)), and nebius.com is blocked. The attribution to Google in the summary looks wrong. It is a facility failure, not a software bug, in any case.
  - A March 2026 AI-service outage appeared only in a Medium opinion post; no provider post-mortem was seen.
  - A "Cloudflare Logs 13 Outages in 8 Days as R2 Falters" item from August 2026 ([shattered.io](https://shattered.io/cloudflare-outage-august-2026/)) appeared once; not checked.

### Inferences
- **The 2025 pattern holds in 2026: the trigger is a small logic or input-handling bug, and the blast radius comes from automated global propagation.** Cloudflare February 2026 is a clean example of "absent value treated as wildcard". A language or API design where an empty filter cannot silently mean "all" (for example, an explicit `All` vs `Only(set)` type) would have blocked it. That is a type-design point, not a memory-safety one.
- **Microsoft 365 January 2026 is a capacity and process failure.** No language angle.
- **Dollar figures for outages are soft.** Parametrix's $5.4B is an uninsured direct-loss model for 124 companies. CyberCube's AWS figure is an insured-loss range whose likely value is about 7% of its top end. The report should quote ranges, not maxima.

### Gaps
- None of the primary post-mortem pages (Cloudflare, Google status, AWS message, CrowdStrike RCA) could be opened; all stay at snippet level.
- The exact-day publication date of Microsoft 365's final RCA, if one exists, was not found.
- No 2026 outage was found whose published root cause is a memory-safety bug.

## 2. CISA "Product Security Bad Practices" and 2026 US federal changes

### Takeaway
**The tentative correction in `primary_reads_industry_partial.md` should be revised.** Search results for Version 2 (January 2025) show the memory-safety roadmap item was kept, with its wording changed: manufacturers "should publish a memory safety roadmap **by the end of 2025**", and the item "does not apply to products that have an announced end-of-support date that is prior to Jan. 1, 2030." So the literal "by January 1, 2026" wording from the October 2024 version is gone, but the deadline itself is the same point in time. It was always voluntary guidance. No 2026 federal action on memory safety was found.

### Cited Findings
- **Version 2 still has the roadmap item: SNIPPET-CONFIRMED; CORRECTS the tentative correction.**
  - Search summary (query on "Version 2 January 2025 … roadmap removed"): "for existing products, software manufacturers should publish a memory safety roadmap by the end of 2025, outlining their prioritized approach to eliminating memory safety vulnerabilities in priority code components written in memory unsafe languages. However, publication of a memory safety roadmap does not apply to products that have an announced end-of-support date that is prior to Jan. 1, 2030." The summary adds that the January 2025 version "does include memory safety roadmap requirements".
  - An exact-phrase search for "memory safety roadmap by the end of 2025" returned the Version 2 PDF on ic3.gov ([ic3.gov/CSA/2025/250117.pdf](https://www.ic3.gov/CSA/2025/250117.pdf)), [Cyble](https://cyble.com/blog/fbi-cisa-push-for-memory-safe-software-practices/), [corrode](https://corrode.dev/blog/memory-safety/) and [DEVOPSdigest](https://www.devopsdigest.com/fix-it-or-face-the-consequences-cisas-memory-safe-muster).
  - An exact-phrase search for "end-of-support date" plus "January 1, 2030" returned the Version 2 PDF on both hosts ([ic3.gov](https://www.ic3.gov/CSA/2025/250117.pdf); [cisa.gov, 2025-01](https://www.cisa.gov/sites/default/files/2025-01/joint-guidance-product-security-bad-practices-508c_0.pdf)) and [RunSafe](https://runsafesecurity.com/blog/cisa-memory-safety-ot-leaders/).
  - What v2 changed, per the summaries: it incorporated RFI comments, "adding additional bad practices, context regarding memory-safe languages, clarifying timelines for patching Known Exploited Vulnerabilities (KEVs)". Three bad practices were added: known insecure or outdated cryptographic functions, hardcoded credentials, and product support periods ([CISA alert, 2025-01-17](https://www.cisa.gov/news-events/alerts/2025/01/17/cisa-and-fbi-release-updated-guidance-product-security-bad-practices)).
  - Neither PDF could be opened (cisa.gov and ic3.gov blocked). The v2 change record was not seen.
  - How this fits with the partial primary read: that session saw that v2 no longer carries the words "by January 1, 2026" and was checking the change record when it stopped. The search results suggest the date was reworded to "by the end of 2025", not dropped. The two readings agree on the text change; they differ only on whether the item survived, and the search evidence says it did.
- **Other items still in the guidance, per the summary:** new product lines for critical infrastructure in a memory-unsafe language "where readily available alternative memory-safe languages could be used is dangerous"; for existing products, "not having a published memory safety roadmap is itself a bad practice".
- **Status of the guidance:** still voluntary. One summary claimed it is "increasingly becoming mandatory for federal contractors through acquisition rules"; no rule or source was named, so treat that as unsupported.
- **2026 US federal changes on memory safety or secure-by-design: COULD NOT VERIFY any.** One search ("CISA secure by design 2026 memory safety federal policy change") returned only 2023 to 2025 material (the roadmap guide, the June 2025 CISA/NSA sheet, the pledge page). The March 2026 national cyber strategy and its move away from software liability are already in `gap_fill_industry.md` §4.

### Inferences
- The report can say: "CISA/FBI guidance (October 2024, revised January 2025) asks manufacturers to publish memory-safety roadmaps by the end of 2025, with an exemption for products leaving support before 2030. It is voluntary, and no count of vendors who did so was found."
- Do not describe it as a removed item, and do not describe it as a binding federal deadline.

### Gaps
- The v2 PDF text and its change record were not opened.
- No public count of published vendor roadmaps.
- No 2026 federal memory-safety action found in one search.

## 3. Carbon 0.1, Fil-C performance, DARPA TRACTOR

### Takeaway
No Carbon 0.1 release was found as of September 2026, and the roadmap still says the end of 2026 is the earliest possible date. No newer Fil-C performance claim than "a few times slower; about 4x on raw benchmarks" was found. For TRACTOR, Aarno Labs' Tenjin is now SNIPPET-CONFIRMED as a performer project. No official list of all performers and no DARPA-published 2026 scores were found.

### Cited Findings
- **Carbon 0.1: COULD NOT VERIFY a release; roadmap position re-seen at snippet level.**
  - Search summary: "Shipping 0.1 in 2026 will be a very ambitious goal and may not be possible, but the end of 2026 is now the soonest that 0.1 could realistically be ready to ship. … because they are adding a design for memory safety to the 0.1 milestone, they are also expecting to push it out by at least a year." The "potential 2026 goal is to ship a working 0.1 language for evaluation" ([Carbon roadmap](https://docs.carbon-lang.dev/docs/project/roadmap.html); [Innowise](https://innowise.com/blog/googles-carbon-language/); [Programming Helper, 2026](https://www.programming-helper.com/tech/carbon-programming-language-2026-google-experimental-cpp-successor)).
  - No search result mentioned a 0.1 release. The GitHub releases page appeared in results but was not opened (session rule). Absence of evidence in one search is weak; treat as "not seen shipped as of late September 2026".
- **Fil-C: no newer figure found.** The same sources as last round came back: "only a few times slower than Clang-generated code, although the exact slowdown depends heavily on the structure of the benchmarked program" ([LWN](https://lwn.net/Articles/1042938/)); "Raw benchmark numbers suggest a 4x slowdown (and sometimes even more)"; MonoCaps went from "40x" to "10x or less" ([Pizlo on X](https://x.com/filpizlo/status/1936202225468297367)); Bash showed "no noticeable performance difference" because it mostly runs external programs. fil-c.org was blocked. Keep last round's wording: "roughly 4x typical, sometimes more; small for I/O-bound programs".
- **DARPA TRACTOR.**
  - Aarno Labs "Tenjin": **SNIPPET-CONFIRMED** (was COULD NOT VERIFY). Search summary: "Aarno Labs has been selected by DARPA to develop advanced tools for translating legacy C code into safe, idiomatic Rust as part of DARPA's newly launched TRACTOR program … The Aarno-led project, named Tenjin, will build a modular framework that combines source-level refactoring with multi-stage translation driven by comprehensive program understanding" ([Aarno Labs](https://www.aarno-labs.com/blog/post/news-aarno-labs-awarded-darpa-grant-to-develop-tools-for-translating-c-to-safe-rust) (blocked); [AccessNewswire press release](https://www.accessnewswire.com/newsroom/en/computers-technology-and-internet/aarno-labs-awarded-darpa-grant-to-develop-tools-for-translating-c-1037136); [Yahoo Finance](https://finance.yahoo.com/news/aarno-labs-awarded-darpa-grant-230500990.html)). Award date and amount were not in the summary.
  - ForCLift (Illinois, UW–Madison, UC Berkeley, Edinburgh): re-seen, consistent with last round ([UIUC CSL](https://csl.illinois.edu/news-and-media/translating-legacy-code-for-a-safer-future-darpa-backs-effort-to-convert-c-to-rust)).
  - Immunant: the summary says it maintains "a C-to-Rust translation tool, which has been developed with support from DARPA". That does not establish it as a TRACTOR performer; do not list it as one.
  - Benchmark status: "As of September 2026, two batteries (Battery 01 and Battery 02) and three milestone projects (Project 00, Project 01, and Project 02) have been released", and Lincoln Laboratory releases "a benchmark every six months" ([arXiv 2609.25121](https://arxiv.org/html/2609.25121); [MIT LL](https://www.ll.mit.edu/r-d/projects/translating-all-c-rust-tractor-benchmarks)). Consistent with last round.
  - A search aimed at the "six performer systems" scores returned only the benchmark paper's abstract, which describes the framework and metrics, not per-performer scores. The six-performer detail stays at last round's level.
  - Official performer list and 2026 results: COULD NOT VERIFY (darpa.mil blocked; two searches found no list).

### Inferences
- The C/C++ successor landscape in September 2026 is unchanged from last round: Carbon pre-0.1, TrapC unreleased, Fil-C usable with a large CPU cost, TRACTOR still in benchmark rounds.

### Gaps
- Carbon's GitHub releases page, fil-c.org and darpa.mil were not read.
- TRACTOR per-performer scores and the "7 teams, $14M" claim remain unverified.

## 4. Supply chain

### Takeaway
Most supply-chain items are now settled:
- **chalk/debug (8 September 2025):** 18 packages and about 2.6B weekly downloads are SNIPPET-CONFIRMED. The npm registry's own publish times, read directly, confirm that the compromised chalk 5.6.1 and debug 4.4.2 were published at about 13:12 to 13:13 UTC that day and later removed.
- **Shai-Hulud v1:** "500+ packages" is SNIPPET-CONFIRMED as CISA's figure, but other counts say about 200 packages and 500+ versions. The report should say "hundreds of packages (CISA: more than 500)".
- **npm v12:** CONFIRMED from npm's own registry data and bundled docs. Version 12.0.0 was published on **8 July 2026**. Dependency install scripts are blocked by default: they are skipped with a warning list unless approved in `package.json` `allowScripts`. Git and remote-URL dependencies are also off by default.
- **axios (March 2026):** CONFIRMED from Microsoft's page and the registry. Versions 1.14.1 and 0.30.4, 31 March 2026 UTC, attributed to North Korea. The weekly-download figure conflicts (Microsoft "over 70 million", GTIG and Trend Micro "100 million+").
- **OpenSSF "1,444%":** still single-source (GTIG). No OpenSSF page with that number was found. Endor Labs' separate OSV-based "14x in two years" is the nearest independent figure.

### Cited Findings
- **D6. chalk/debug, 8 September 2025: SNIPPET-CONFIRMED (counts) and CONFIRMED (date, from registry data).**
  - Search summary: "On September 8, 2025 … Aikido's intel system flagged 18 heavily downloaded packages, including chalk, debug, ansi-styles, supports-color, and more." "affecting an estimated 2.6 billion weekly downloads". The payload "intercepted wallet interactions via window.ethereum, mutated transaction destinations, and redirected funds". Entry was "a phishing campaign targeting npm package maintainers" using "the domain npmjs.help". "Malicious versions first published to npm at 13:16 UTC"; clean versions restored "Within 2 hours".
  - Results: [Aikido](https://www.aikido.dev/blog/npm-debug-and-chalk-packages-compromised); [Upwind, "debug, chalk, and 16 Other Packages"](https://www.upwind.io/feed/npm-supply-chain-attack-massive-compromise-of-debug-chalk-and-16-other-packages); [Wiz](https://www.wiz.io/blog/widespread-npm-supply-chain-attack-breaking-down-impact-scope-across-debug-chalk); [Vercel, 2025-09-08](https://vercel.com/blog/critical-npm-supply-chain-attack-response-september-8-2025); [Sonatype](https://www.sonatype.com/blog/npm-chalk-and-debug-packages-hit-in-software-supply-chain-attack); [Semgrep](https://semgrep.dev/blog/2025/chalk-debug-and-color-on-npm-compromised-in-new-supply-chain-attack/).
  - Registry data (read directly from [registry.npmjs.org/chalk](https://registry.npmjs.org/chalk) and [registry.npmjs.org/debug](https://registry.npmjs.org/debug)): chalk `5.6.1` has publish time `2025-09-08T13:13:05Z` and is no longer in the version list; `5.6.2` followed at `14:47:54Z`. debug `4.4.2` has `2025-09-08T13:12:39Z` and is no longer in the version list. The registry does not label versions as malicious; "removed" is what the data shows. The registry times are about 3 minutes earlier than the "13:16 UTC" in the summary.
  - The 2.6B weekly figure is a sum of the packages' normal weekly downloads, not a count of malicious installs.
- **D3. Shai-Hulud v1 (September 2025): SNIPPET-CONFIRMED for "500+" as CISA's figure, with a scope caveat.**
  - Search summary: "CISA published a September 23 alert about a 'widespread supply chain compromise' affecting more than 500 npm packages"; the alert advised checking lockfiles "to identify packages released after September 16, 2025". Results: [CISA alert, 2025-09-23](https://www.cisa.gov/news-events/alerts/2025/09/23/widespread-supply-chain-compromise-impacting-npm-ecosystem); [CybersecurityNews, "CISA Warns of Shai-Hulud … Compromised 500+ Packages"](https://cybersecuritynews.com/cisa-shai-hulud-npm-attack/); [Cybersecurity Dive](https://www.cybersecuritydive.com/news/cisa-dependency-checks--shai-hulud-compromise/761018/); [CERT/CC VU#534320](https://www.kb.cert.org/vuls/id/534320).
  - Conflicting count, from a separate search summary: "Over 200 npm packages and more than 500 versions were compromised between September 14th and 18th", and "different security researchers reporting figures ranging from over 180 packages to over 200 packages" ([Zscaler](https://www.zscaler.com/blogs/security-research/mitigating-risks-shai-hulud-npm-worm); [Wiz](https://www.wiz.io/blog/shai-hulud-npm-supply-chain-attack); [Unit 42](https://unit42.paloaltonetworks.com/npm-supply-chain-attack/)). Discovery: "On September 15th 2025, ReversingLabs researchers discovered" it.
  - Likely reconciliation (inference): "500+" may count package versions, not distinct packages. Recommended wording: "about 200 packages and 500+ versions (CISA's alert said more than 500 packages)".
- **npm v12: CONFIRMED (date and behaviour) from primary data.**
  - Release date: registry metadata ([registry.npmjs.org/npm](https://registry.npmjs.org/npm)) gives `12.0.0` at `2026-07-08T21:06:05Z`, after pre-releases from 20 May 2026 (`12.0.0-pre.0.0`). Later: `12.0.1` (10 July), `12.0.2` (29 July), `12.1.0` (22 September 2026, now `latest`). The 11.x line is still maintained (`11.19.1`, 26 August 2026).
  - Behaviour, from the docs inside the npm 12.0.0 tarball downloaded from the registry ([npm-12.0.0.tgz](https://registry.npmjs.org/npm/-/npm-12.0.0.tgz), files `docs/content/commands/npm-approve-scripts.md` and `docs/content/using-npm/config.md`):
    - "Dependency install scripts are blocked by default. Install commands silently skip lifecycle scripts for any dependency that does not have a matching entry in `allowScripts`, and end with a list of the packages whose scripts were skipped".
    - The scripts covered are "`preinstall`, `install`, `postinstall`, and `prepare` for non-registry dependencies".
    - `allowScripts` is a field in the project's `package.json` ("which also supports explicit denials"); `npm approve-scripts` maintains it.
    - `strict-allow-scripts` (default false): "turn the install-script policy from a warning into a hard error".
    - `dangerously-allow-all-scripts` (default false): "bypass the `allowScripts` policy entirely … Intended as a migration escape hatch only".
    - `allow-git`: "As of npm 12 the default is `none`." `allow-remote`: "As of npm 12 the default is `none`."
  - Secondary coverage agrees: "npm v12 shipped July 8, 2026 and turns off install scripts, Git dependencies, and remote-URL dependencies by default"; "an unapproved script is skipped, npm prints a warning, and the install still succeeds" ([Socket](https://socket.dev/blog/npm-12); [The Hacker News, 2026/07](https://thehackernews.com/2026/07/npm-12-disables-install-scripts-by.html); [InfoQ, 2026/08](https://www.infoq.com/news/2026/08/npm-12-released/)). Socket's title adds that npm 12 "Begins Deprecating 2FA-Bypass Tokens" (title only; not checked).
  - Correction to GTIG's shorthand (in `gap_fill_industry.md` §5): GTIG wrote "npm v12 disables all lifecycle scripts by default (`allowScripts: off`)". The npm docs scope the block to **dependency install scripts**. Scripts a developer runs explicitly (`npm run`, `npm test`) are not what `allowScripts` governs. Say "dependency install scripts", not "all lifecycle scripts".
- **axios, March 2026: CONFIRMED (Microsoft page opened) with registry corroboration.**
  - [Microsoft Security Blog, 2026-04-01](https://www.microsoft.com/en-us/security/blog/2026/04/01/mitigating-the-axios-npm-supply-chain-compromise/) (page opened): "versions (1.14.1 and 0.30.4) were injected with a malicious dependency", "plain-crypto-js@4.2.1"; the payload ran because "a dependency's lifecycle script automatically launches node setup.js" (postinstall); attributed to "Sapphire Sleet, a North Korean state actor"; axios had "over 70 million weekly downloads". The fetch tool's reading gives the date as "March 31, 2026".
  - Registry data ([registry.npmjs.org/axios](https://registry.npmjs.org/axios)): `1.14.1` at `2026-03-31T00:21:58Z` and `0.30.4` at `2026-03-31T01:00:57Z`, both no longer in the version list.
  - Search summary (Sophos, SANS, StepSecurity, Datadog, Trend Micro, Elastic, CISA): "live approximately from 2026-03-31 00:21 UTC to 03:15 UTC", "about 3 hours"; US-time reports date it 30 March 2026 ([StepSecurity](https://www.stepsecurity.io/blog/axios-compromised-on-npm-malicious-versions-drop-remote-access-trojan); [Datadog](https://securitylabs.datadoghq.com/articles/axios-npm-supply-chain-compromise/); [CISA alert, 2026-04-20](https://www.cisa.gov/news-events/alerts/2026/04/20/supply-chain-compromise-impacts-axios-node-package-manager); [axios post-mortem issue #10636](https://github.com/axios/axios/issues/10636), title only, not opened).
  - Conflicts to report: (a) weekly downloads: Microsoft "over 70 million" vs GTIG "Over 100 million" and Trend Micro "100M+ Weekly Downloads" ([Trend Micro](https://www.trendmicro.com/en_us/research/26/c/axios-npm-package-compromised.html)); (b) actor names: Microsoft "Sapphire Sleet", GTIG "MIDNIGHT NEPTUNE". Both say North Korea; vendors use different naming schemes, so these may be the same actor.
  - Language angle: the payload ran through a dependency's install-time lifecycle script, which is exactly what npm 12's default now blocks.
- **OpenSSF "1,444%" vs Sonatype: still single-source.**
  - A search for the OpenSSF figure found no OpenSSF page stating it. The only source remains GTIG's July 2026 post (page opened last round).
  - Nearest independent figure: Endor Labs' 2026 report, based on OSV data (which includes OpenSSF's malicious-packages feed) plus a survey of 600+ IT staff. "In 2025 alone, more than 90% of open source vulnerability (OSV) malware advisories were reported, a 14x increase over the past two years"; "Of the 1,011 npm ATO advisories recorded in the OSV database over all time, 930 were filed in 2025" ([Endor Labs](https://www.endorlabs.com/learn/new-research-malware-in-open-source-ecosystems-surges-14x-as-attackers-hijack-trusted-packages); [PR Newswire](https://www.prnewswire.com/news-releases/endor-labs-finds-malware-in-open-source-ecosystems-surges-14x-in-two-years-as-organizations-struggle-to-respond-302730716.html); [Crowdfund Insider, 2026-04](https://www.crowdfundinsider.com/2026/04/271319-open-source-software-malware-surging-endor-labs/)). Note the summary's first sentence is garbled ("more than 90% … were reported, a 14x increase"); the "14x" headline is clear from several titles.
  - Possible reason the OSV/OpenSSF counts jumped: Amazon Inspector reported "over 150,000 malicious packages linked to token farming campaign" and submitted findings to the OpenSSF repository ([AWS Security Blog](https://aws.amazon.com/blogs/security/amazon-inspector-detects-over-150000-malicious-packages-linked-to-token-farming-campaign/), title and summary only). This is an inference about the cause, not a sourced explanation.
  - Sonatype for comparison: 454,600 new malicious packages in 2025 (already SNIPPET-CONFIRMED last round); Sonatype's own Q2 2025 index said "188%" year on year ([Infosecurity Magazine](https://www.infosecurity-magazine.com/news/malicious-open-source-surge-188/)).

### Inferences
- **npm 12 changes the supply-chain argument for a new language.** Two of the three big npm incidents here ran at install time: Shai-Hulud 2.0 through a `preinstall` hook (Datadog, confirmed last round) and axios through a dependency's `postinstall` (Microsoft, above). npm 12 skips both by default since 8 July 2026. The chalk/debug payload was different: code inside the package that ran in the end user's browser and rewrote wallet transactions. No install-script policy stops that. Runtime authority for imported code is what remains for a language.
- **The new default does not reach everyone.** Projects that stay on npm 11 keep the old behaviour, and the 11.x line was still getting releases in August 2026. Projects that set `dangerously-allow-all-scripts` or approve scripts broadly also get the old behaviour back. The default narrows the attack surface; it does not remove it.
- **Growth rates are not comparable across vendors.** OpenSSF/GTIG 1,444% (2024 to 2025), Endor 14x (two years to 2025), Sonatype 188% (Q2 2025 year on year) and 454,600 new in 2025 use different datasets and windows. Report counts with their source, not "the" growth rate.

### Gaps
- The OpenSSF primary for 1,444% was not found.
- npm weekly-download history (api.npmjs.org) was blocked, so the axios 70M vs 100M conflict could not be settled from primary data.
- The CISA Shai-Hulud alert text was not opened, so whether "500 packages" means packages or versions is unresolved.

## 5. Slopsquatting

### Takeaway
The USENIX Security 2025 figures are now SNIPPET-CONFIRMED: **19.7%** overall, **205,474** unique hallucinated names, **2.23 million** samples, **43%** repeated in all ten re-runs, **58%** repeated more than once. The earlier sample-size conflict resolves to 2.23M. One wording issue remains: whether 19.7% is a share of packages or of code samples. The 2026 re-evaluation ("The Range Shrinks, the Threat Remains", arXiv 2605.17062) is SNIPPET-CONFIRMED: rates of **4.62% to 6.10%** across five frontier models from late 2025 to early 2026, and **53 shared hallucinated names still registrable** after disclosure.

### Cited Findings
- **Spracklen et al., USENIX Security 2025: SNIPPET-CONFIRMED.**
  - Paper: "We Have a Package for You! A Comprehensive Analysis of Package Hallucinations by Code Generating LLMs", 34th USENIX Security Symposium, 13 to 15 August 2025, Seattle; authors Joseph Spracklen, Raveen Wijewickrama, A H M Nazmus Sakib, Anindya Maiti, Bimal Viswanath, Murtuza Jadliwala ([USENIX presentation page](https://www.usenix.org/conference/usenixsecurity25/presentation/spracklen); [paper PDF](https://www.usenix.org/system/files/usenixsecurity25-spracklen.pdf); both blocked for fetch).
  - Exact-phrase search ("19.7%" + "205,474"), summary: "analyzed 2.23 million code samples using 16 popular code-generating models across Python and JavaScript. Of those samples, 440,445 — 19.7% — contained at least one hallucinated package name, and across those hallucinations, the study identified 205,474 unique fabricated package names." "The average percentage of hallucinated packages is at least 5.2% for commercial models and 21.7% for open-source models." Results: the USENIX paper and presentation page, [CSA research note, April 2026](https://labs.cloudsecurityalliance.org/research/csa-research-note-slopsquatting-ai-supply-chain-20260419-csa/), [the UTSA poster](https://sprite.utsa.edu/publications/posters/USENIX_Package_Hallucination_Poster.pdf) (blocked), [USENIX ;login:](https://www.usenix.org/publications/loginonline/we-have-package-you-comprehensive-analysis-package-hallucinations-code), [HOL Blog](https://hol.org/blog/slopsquatting-ai-hallucinations-supply-chain-attacks).
  - Repetition: "43% of hallucinated packages were repeated every time and 58% of hallucinated packages were repeated more than once across ten runs" ([Socket](https://socket.dev/blog/slopsquatting-how-ai-hallucinations-are-fueling-a-new-class-of-supply-chain-attacks); [Mend](https://www.mend.io/blog/the-hallucinated-package-attack-slopsquatting/); [Infosecurity Magazine](https://www.infosecurity-magazine.com/news/ai-hallucinations-slopsquatting/); [Help Net Security, 2025-04-14](https://www.helpnetsecurity.com/2025/04/14/package-hallucination-slopsquatting-malicious-code/); [DevOps.com](https://devops.com/ai-generated-code-packages-can-lead-to-slopsquatting-threat/); [TechRadar](https://www.techradar.com/pro/security/ai-hallucinated-names-resembling-popular-libraries-created-for-slopsquatting-attacks)). The first summary adds the test design: "500 prompts that produced fake packages were run ten more times each."
  - **Sample-size conflict (D9): resolved to 2.23M.** The "576,000" figure did not appear in any result this round. It may come from an earlier preprint; that is a guess.
  - **Wording caveat, unresolved:** `industry_scale_pain.md` says "19.7% of recommended packages did not exist". This round's summary says 440,445 of 2.23M *samples* "contained at least one hallucinated package". The summary may be paraphrasing loosely (440,445 / 2,230,000 ≈ 19.75%, so the ratio fits whichever unit is right). Until the paper is read, write "about 19.7% of generated package references (440,445 of about 2.23M) were hallucinated" and note the unit is unconfirmed, or cite the per-model averages (5.2% commercial, 21.7% open-source) instead.
- **"The Range Shrinks, the Threat Remains" (arXiv 2605.17062): SNIPPET-CONFIRMED.**
  - Full title: "The Range Shrinks, the Threat Remains: Re-evaluating LLM Package Hallucinations on the 2026 Frontier-Model Cohort"; author named in the summary as independent researcher Aleksandr Churilov ([arXiv abstract](https://arxiv.org/abs/2605.17062); [arXiv PDF](https://arxiv.org/pdf/2605.17062); [Socket, "New Study Identifies 53 Slopsquatting Targets Across 5 Frontier LLMs"](https://socket.dev/blog/slopsquatting-targets-across-frontier-llms); [awesomepapers.io listing](https://awesomepapers.io/cybersecurity/papers/2605.17062)).
  - Models: five released October 2025 to March 2026 (named in the summary as Claude Sonnet 4.6, Claude Haiku 4.5, GPT-5.4-mini, Gemini 2.5 Pro and DeepSeek V3.2).
  - Scale and rates: "199,845 paired Python and JavaScript prompts validated against PyPI and npm master lists"; "overall hallucination rates between 4.62% (Claude Haiku 4.5) and 6.10% (GPT-5.4-mini)".
  - Shared names: "127 package names (109 on PyPI, 18 on npm) that all five evaluated models invent identically; after coordinated disclosure with PyPI Security and Socket.dev, 53 of these (41 on PyPI, 12 on npm) remain registrable by an attacker".
  - Caveat in the summary: "no evidence that any of the 53 names have been registered maliciously".
  - Single-author preprint, not peer-reviewed as far as seen.

### Inferences
- The frontier-model rate (about 5 to 6%) sits at the low end of the 2025 study's range (5.2% commercial). The spread between models narrowed, but the problem did not go away: identical hallucinations across vendors create a shared, predictable target list.
- For the report: slopsquatting is a real but narrow supply-chain vector. Registry-side defences (PyPI and Socket blocking names after disclosure) handled 74 of 127 shared names. It is not strong evidence for a new language; it argues for dependency allowlists and lockfile review, which are process tools.

### Gaps
- The USENIX paper text was not opened, so the unit behind 19.7% is not settled.
- No report of an actual slopsquatting attack with a hallucinated name was checked this round.

## 6. Developer population (SlashData, Evans Data, JetBrains, GitHub)

### Takeaway
SlashData's figure of **47.2 million developers worldwide in early 2025**, of whom **36.5 million are professionals**, is SNIPPET-CONFIRMED. A March 2026 CNCF/SlashData release puts cloud-native developers at 19.9M in Q1 2026, "roughly 39%" of all developers. That implies a total of about 51M in early 2026, but the figure is derived here, not stated in any source seen. Evans Data's latest seen figure is **27 million** (professional developers), from Evans Data's own press-release titles only. GitHub's 225M users (July 2026) was already CONFIRMED last round. No JetBrains population figure was searched.

### Cited Findings
- **SlashData, 2025: SNIPPET-CONFIRMED.**
  - Search summary: "there are 47.2 million developers in the world, representing a 3-year 50% increase from early 2022." "From early 2022 to early 2025, the number of professional developers grew significantly—by 70%—from 21.8 million to 36.5 million. However, the amateur developer population declined by over 1 million in the last year." Growth "decelerated to 10% in the last 12 months", after 15% (2022 to 2023) and 21% (2023 to 2024).
  - Method, per the summary: a "bottom-up methodology" using "the number of GitHub and Stack Overflow accounts, along with employment statistics from the USA and the European Union".
  - Results: [SlashData, "There are 47.2 million developers in the world"](https://www.slashdata.co/post/global-developer-population-trends-2025-how-many-developers-are-there) (slashdata.co blocked last round, not retried); [SlashData on X](https://x.com/SlashDataHQ/status/1916850760647581938) ("There are 47.2 million developers in the world, a 3-year 50% increase"); [ShiftMag](https://shiftmag.dev/there-are-47-million-developers-in-the-world-5200/); [byteiota, "Developer Population Hits 47M, Growth Slows to 10%"](https://byteiota.com/developer-population-hits-47m-growth-slows-to-10/); [Portal.hr](https://www.portal.hr/en/novosti/trisedma/97179-u-svijetu-postoji-47-milijuna-programera).
- **SlashData with CNCF, Q1 2026: SNIPPET-CONFIRMED for the cloud-native count.**
  - Search summary: "The global cloud native developer community has grown to 19.9 million developers as of Q1 2026", up from "15.6 million developers in Q3 2025", "a 28% increase in just six months". "This represents roughly 39% of the entire global developer population."
  - Data: "the 31st edition of SlashData's Developer Nation survey, fielded between December 2025 and January 2026, which reached more than 12,500 respondents from 100 countries".
  - Results: [CNCF announcement, 2026-03-24](https://www.cncf.io/announcements/2026/03/24/cncf-and-slashdata-report-finds-cloud-native-community-reaches-nearly-20-million-developers/) (cncf.io blocked); [SlashData, "There are 19.9M Cloud Native Developers in Q1 2026"](https://www.slashdata.co/post/there-are-19-9m-cloud-native-developers-in-q1-2026); [PR Newswire](https://www.prnewswire.com/news-releases/cncf-and-slashdata-report-finds-cloud-native-community-reaches-nearly-20-million-developers-302722734.html).
  - A critical take is titled "CNCF's 19.9 Million Developers, +28% in Six Months: The Math That Says It's Not New Growth" ([bex.co, 2026-07-29](https://bex.co/blog/2026/07/29/cncf-2026-cloud-native-survey-ai-developers-golden-path)), title only.
  - Derived, not sourced: 19.9M / 0.39 ≈ 51M total developers in Q1 2026. SlashData's own 2026 population report was not found.
- **Evans Data: single-source snippet; year not confirmed.**
  - Press-release titles: "Worldwide Developer Population Grows to 27 Million" ([Evans Data, release 365](https://evansdata.com/press/viewRelease.php?pressID=365)) and "Worldwide Developer Population Grows to 26.4 Million" ([release 350](https://evansdata.com/press/viewRelease.php?pressID=350)). The summary dates the 27M figure to 2024 and says an older projection put 2026 at "29.573 million" ([Developer Population Numbers 2022-2027](https://evansdata.com/press/viewRelease.php?pressID=303)).
  - The summary notes Evans "focus on professional developers, while … SlashData use broader definitions that include hobbyist and student developers."
  - evansdata.com was blocked for fetch. A "Worldwide Developer Population Report 2025" listing exists ([Evans Data](http://evansdata.com/reports/viewRelease.php?reportID=9)); its figure was not seen.
- **GitHub:** 180M+ developers (October 2025) and 225M users (July 2026) were CONFIRMED last round from Microsoft earnings pages (`gap_fill_industry.md` H1). Not rechecked.
- **JetBrains: COULD NOT VERIFY.** Not searched; the search budget went to higher-priority items. jetbrains.com is blocked.

### Inferences
- **Professional developers number roughly 27M to 37M**, depending on the firm (Evans about 27M, SlashData 36.5M professionals in early 2025). **All developers, including hobbyists and students, number about 47M to 51M.** GitHub's 225M counts accounts, about 4 to 5 times the SlashData total, so it is an upper bound on reach, not a head count.
- **Growth is slowing** (SlashData: 21% to 10% a year). A new language's addressable population is growing more slowly than the 2022 to 2024 figures suggest.

### Gaps
- SlashData's own 2026 total, Evans Data's 2025 report figure, and any JetBrains estimate were not seen.
- No per-language counts (C/C++, npm, PyPI users) were found.

## 7. Uber data races, COBOL/SSA, CISQ

### Takeaway
The Uber PLDI 2022 figures are SNIPPET-CONFIRMED as written in the earlier notes. DR.FIX is now filled in: a PLDI 2025 paper reporting that Uber's LLM-plus-analysis tool fixed **193 races over 18 months with an 86% patch acceptance rate**. It also mentions **about 4,000 races fixed at Uber** overall, up from "over 1,000" in 2022. No 2026 outcome for the DOGE/SSA COBOL rewrite was found; coverage stops in mid-2025. No CISQ estimate newer than the 2022 report ($2.41T) was found.

### Cited Findings
- **Uber, "A Study of Real-World Data Races in Golang" (PLDI 2022): SNIPPET-CONFIRMED.**
  - Search summary: "deployed a dynamic data race detector in Uber's 46 million lines of Go codebase hosting 2100 distinct microservices, found over 2000 data races, and fixed over 1000 data races, spanning 790 distinct code patches submitted by 210 unique developers over a six-month period." It concludes that "the abundance of concurrency alongside language idioms and nuances make Go programs highly susceptible to data races."
  - Results: [PLDI 2022 paper page](https://pldi22.sigplan.org/details/pldi-2022-pldi/53/A-Study-of-Real-world-Data-Races-in-Golang); [arXiv 2204.00764](https://arxiv.org/abs/2204.00764); [ACM DL](https://dl.acm.org/doi/10.1145/3519939.3523720); [Uber blog, "Data Race Patterns in Go"](https://www.uber.com/us/en/blog/data-race-patterns-in-go/); [Uber blog, "Dynamic Data Race Detection in Go Code"](https://www.uber.com/ca/en/blog/dynamic-data-race-detection-in-go-code/).
  - Independence caveat: these are the paper and the authors' own blog posts, not outside coverage. They agree with each other, and none could be opened.
- **DR.FIX (PLDI 2025): SNIPPET-CONFIRMED (weak; copies of one paper).**
  - Search summary: Dr.Fix "combines large language models (LLMs) with program analysis to generate fixes for data races", using "retrieval-augmented generation (RAG) and code skeletonization". "Deployed at Uber, Dr.Fix demonstrated practical utility by addressing 193 data races over an 18-month period, achieving an 86% acceptance rate for developer-approved patches." "the system maintains a database of high-quality data race fixes with 272 examples selected from a total of 4,000 fixed data races."
  - Results: [arXiv 2504.15637](https://arxiv.org/abs/2504.15637); [ACM DL, PACMPL, doi 10.1145/3729265](https://dl.acm.org/doi/10.1145/3729265); [PLDI 2025 paper page](https://pldi25.sigplan.org/details/pldi-2025-papers/20/DR-FIX-Automatically-Fixing-Data-Races-at-Industry-Scale); [ResearchGate](https://www.researchgate.net/publication/392664702_DRFIX_Automatically_Fixing_Data_Races_at_Industry_Scale).
  - Update to the old notes: the venue is PLDI 2025 (published in PACMPL), and "4,000 fixed data races" is the newer Uber-wide cumulative count.
- **COBOL line counts: COULD NOT VERIFY (not re-searched).** The 220B (Reuters, 2017) vs 800B+ (Micro Focus-commissioned survey) range stays at the earlier notes' level ([The Stack](https://www.thestack.technology/cobol-in-daily-use/)). SSA's "over 60 million lines of COBOL" reappeared in this round's summaries, tied to the Warren–Wyden letters and 2025 coverage ([Warren–Wyden letter](https://www.warren.senate.gov/imo/media/doc/warren_wyden_letter_to_bisignano_on_ssa_it_modernization.pdf); [Gizmodo](https://gizmodo.com/doge-plans-to-rewrite-entire-social-security-codebase-in-just-a-few-months-report-2000582062)).
- **DOGE/SSA rewrite outcome through 2026: COULD NOT VERIFY.**
  - Two searches ("Social Security COBOL migration DOGE 2026 status update"; "Social Security Administration COBOL modernization 2026 Bisignano mainframe AI") returned only March to June 2025 items: the Wired-sourced plan ([Techmeme, 2025-03-28](https://www.techmeme.com/250328/p14)), [CIO, "IT leaders on DOGE's bold COBOL ambitions: Pure folly"](https://www.cio.com/article/3974083/it-leaders-on-doges-bold-cobol-ambitions-pure-folly.html), and Senators Warren and Wyden's letters of April and June 2025 ([Warren press release, 2025-06-11](https://www.warren.senate.gov/newsroom/press-releases/as-doge-launches-risky-tech-experiment-warren-wyden-press-social-security-commissioner-on-potential-catastrophic-benefit-cuts)).
  - One summary line: "DOGE has been silent about its COBOL plans since the original news reports in late March, and the agency did not respond to a request for clarification". That is 2025 coverage, not a 2026 status.
  - The summary also says SSA's "existing multi-step IT modernization plans from 2017 have not been completed due to a lack of funding" (from the senators' letter).
  - No 2026 report of completion, partial cutover, cancellation or failure was found.
- **CISQ: no newer estimate found.** Search summary: "the cost of poor software quality in the United States has reached an estimated $2.41 trillion. This figure comes from their 2022 report, which appears to be the most recent comprehensive report available", with "accumulated technical debt exceeding $1.52 trillion" ([CISQ technical reports page](https://www.it-cisq.org/technical-reports/); [Black Duck](https://www.blackduck.com/resources/analyst-reports/cost-poor-quality-software.html); [Embedded Computing Design](https://embeddedcomputing.com/technology/software-and-os/os-filesystems-libraries/poor-software-quality-costs-trillions-yes-trillions)).
  - The same summary gave a breakdown of "$1.56 trillion in operational software failures, $260 billion in unsuccessful development projects" without tying it to an edition. Do not attach it to the 2022 report without checking; it may come from an earlier edition.

### Inferences
- **Go data races remain a steady, measured cost at Uber.** The cumulative count went from 1,000+ fixed in six months (2022) to about 4,000 fixed (2025 paper), and the response is tooling (LLM-generated fixes), not a language change. This is a real but in-process concurrency problem that Rust-style ownership addresses; the AWS October 2025 race was distributed and would not be.
- **The SSA rewrite has no public outcome.** The report should not claim it succeeded, failed or stalled; say "no public status since mid-2025 was found".
- **CISQ's 2022 figure is the latest.** Label it "2022 estimate" whenever it is used in 2026 text.

### Gaps
- DOGE/SSA 2026 status; primary COBOL line counts; GAO-25-107795 figures (not re-searched).
- The Uber and DR.FIX papers were not opened (arXiv, ACM, uber.com blocked).
- A CISQ successor report, if one is in preparation, was not found.
