# Gap fill: industry (partial primary reads)

The local session on the owner's machine started the industry checks in Chrome but hit its
usage limit before writing its notes file. The items below are taken from its progress log,
not from a finished file, so they carry less detail (no quotes, and some URLs missing). They
still come from pages read directly and take precedence over the search-summary results in
`gap_fill_industry.md` where the two disagree.

## Memory safety

- CONFIRMED: Google's "Rust in Android: move fast and fix things" is a **November 2025**
  post. The old security.googleblog.com URL with `/2025/11/` redirects to it, which settles
  the November 2025 vs April 2026 date conflict.
- CONFIRMED: Google's September 2024 Android post, including the drop in memory-safety
  vulnerabilities from 76% of Android vulnerabilities (2019) to 24% (2024). This was a [TK]
  recollection in industry_scale_pain.md.
- CONFIRMED: the GTIG 2025 zero-day review.

## C++ safety

- CORRECTED: Profiles are targeting **C++29**, not C++26. C++26 itself brings a hardened
  standard library, defined behaviour for reads of uninitialized variables, and contracts.
  Source named in the log: Herb Sutter's "C++26 is done" trip report (URL not recorded).
- The Register's Safe C++ article sat behind a bot check and was skipped; Sutter's report
  was used as the primary source instead.
- CONFIRMED: TrapC has **no release**. Its January 2026 post aimed for Q1 2026, and nothing
  on the homepage says it has shipped.
- Carbon and Fil-C were being checked when the session stopped; no results recorded.

## US policy

- TENTATIVE CORRECTION: CISA's current "Product Security Bad Practices" guidance, version 2.0
  (January 2025), no longer contains the "publish a memory-safety roadmap by 1 January 2026"
  deadline. The session was checking the document's change record to confirm this when it
  stopped, so treat it as likely but not confirmed. If true, the report should not describe
  1 January 2026 as a live federal deadline.

## Not reached

DARPA TRACTOR, the EU Cyber Resilience Act dates, Linux kernel Rust status, the supply-chain
figures (Sonatype, Shai-Hulud, chalk/debug, slopsquatting), the outage post-mortems
(CrowdStrike, Google Cloud, Cloudflare, AWS), Uber's data-race study, COBOL/SSA, CISQ, and
developer-population counts. For these, use `gap_fill_industry.md`.
