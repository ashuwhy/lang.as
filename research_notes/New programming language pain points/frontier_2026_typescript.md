# TypeScript's unsound escape hatches and AI-generated TypeScript: demand-side evidence for a sound, contract-checked language on npm (as of 24 September 2026)

_Status: in progress. Saved early and updated as searches complete._

_Labels. CONFIRMED: page or API response read in this session. SNIPPET-CONFIRMED: two or more search results agree (quoted). UNVERIFIED: one snippet or secondary source only. LOCAL: the author's own SepInfer repo, read on disk (not independent evidence). NOTES: carried over from an earlier note in this folder, not rechecked._

_Access this session. Reachable: registry.npmjs.org (package metadata, the `/-/v1/search` endpoint, which reports weekly and monthly downloads, and the npm audit endpoint `/-/npm/v1/security/advisories/bulk`, which returns GitHub-advisory metadata), pypi.org, www.microsoft.com. Blocked (EGRESS_BLOCKED or HTTP 000): api.npmjs.org, www.npmjs.com, arxiv.org, nvd.nist.gov, services.nvd.nist.gov, cveawg.mitre.org, www.cve.org, api.osv.dev, osv.dev, conf.researchr.org, www.themoonlight.io, www.emergentmind.com, www.endorlabs.com, thehackernews.com, devblogs.microsoft.com, rescript-lang.org, github.com/advisories (403). Download counts below are the registry's own figures on 2026-09-24 (the search endpoint's `downloads.weekly` field, packages "updated" 2026-09-22)._

## 1. How often do real TypeScript bugs and CVEs come from unchecked types at runtime boundaries?

### Takeaway
(being written)

### Cited Findings

**Tang, Alimadadi, Sumner (MSR '26), "From Logic to Toolchains"**
- Authors are "TianYi Tang, Saba Alimadadi, and Nick Sumner from Simon Fraser University"; the study analyses "633 bug reports from 16 popular open-source repositories"; "the fault landscape is dominated not by logic or syntax errors but by tooling and configuration faults, API misuses, and asynchronous error-handling issues"; "while static typing in TypeScript has reduced traditional runtime and type errors, it has shifted fragility toward build systems and toolchains". Presented at "the 23rd International Conference on Mining Software Repositories (MSR '26), April 13–14, 2026, in Rio de Janeiro". SNIPPET-CONFIRMED (two searches, result lists include arXiv abs, arXiv HTML, ResearchGate, emergentmind and a Moonlight review) — [arXiv 2601.21186](https://arxiv.org/abs/2601.21186); [ResearchGate](https://www.researchgate.net/publication/400236972_From_Logic_to_Toolchains_An_Empirical_Study_of_Bugs_in_the_TypeScript_Ecosystem)
- The 12.4% figure (type-related errors "fell from approximately 33% in JavaScript to 12.4% in the TypeScript sample") comes from an earlier note and was NOT reproduced by this session's search (the search summary said it did not see the 12.4% figure). UNVERIFIED — NOTES (`frontier_2026_candidates.md` section E)

**TypePatrol (Galipelli, Staicu, Patra, ESEM 2026)**
- A search summary drawn from Staicu's researchr profile describes TypePatrol as "Adversarial Testing for Uncovering Security-Relevant Type Inconsistencies in JavaScript Libraries" presented in "ESEM's Technical Track". UNVERIFIED (one search; researchr blocked) — [Staicu profile](https://conf.researchr.org/profile/cristianalexandrustaicu); [ESEM 2026 Technical Track](https://conf.researchr.org/track/eseiw-2026/eseiw-2026-esem---technical-track)
- A second search for `"TypePatrol" type inconsistencies CVE` returned nothing relevant. The tool has almost no footprint in the search index as of 24 September 2026.
- The n8n link is through a person: The Hacker News lists "Endor Labs' Cris Staicu" among "as many as 10 security researchers" credited for CVE-2026-25049. UNVERIFIED (one search summary; page blocked) — [The Hacker News](https://thehackernews.com/2026/02/critical-n8n-flaw-cve-2026-25049.html)
- SepInfer's own write-up of TypePatrol: it "adversarially mutates unit-test values to surface the same primitive-type laundering in JS/TS libraries, finding hundreds of inconsistencies and a critical n8n CVE". The author's cross-run used TypePatrol's artifact: 15 TS libraries, 70 found sites (32 TS, 38 JS); of 27 typed TS sites SepInfer derived 25 in-fragment and flagged 0, so "the two detection surfaces are thus disjoint". One return site (form-data `getBoundary`, "annotated `string` but returning a `number`") was certified as a type-safety disagreement. LOCAL — `/home/user/sepinfer/paper/main-pldi.tex` (Related Work; appendix "Cross-Run Against a Dynamic Detector"), `/home/user/sepinfer/benchmarks/e1/OUTCOME.md`
- Caution from the author's own bibliography: `refs.bib` says a September 2026 check found that the DOI previously given for `typepatrol2026` "does not exist"; the entry now has no DOI. LOCAL — `/home/user/sepinfer/paper/refs.bib` lines 1–5

(more below as searches complete)

### Inferences
(pending)

### Gaps
(pending)

## 2. How often does AI-generated TypeScript use `any`, `as` or `!`, compared with human code?

(pending)

## 3. How large is the runtime-validation ecosystem, and how is it trending?

### Takeaway
Runtime validation is one of the largest categories on npm. On 24 September 2026 the registry reports Zod at 211.9M weekly downloads, just above the `typescript` compiler itself (209.2M), with 128,822 dependent packages; Ajv (JSON Schema) is larger still at 287.9M. Effect has grown fastest, to 26.3M a week, from 5M in November 2025 and 10M in April 2026 by its own blog. A cross-library interface (`@standard-schema/spec`, 80.7M a week) now exists so that frameworks can accept any of these validators. Much of this volume is transitive (pulled in by other packages), so downloads overstate direct use.

### Cited Findings

**Weekly downloads, registry figures on 2026-09-24** (CONFIRMED — `https://registry.npmjs.org/-/v1/search?text=<name>`, field `downloads.weekly`, exact-name match)

| Package | Weekly | Monthly | Dependents | Latest version (date) |
|---|---|---|---|---|
| ajv | 287,892,010 | 1,331,257,345 | 19,693 | 8.20.0 (2026-04-24) |
| zod | 211,933,203 | 1,022,521,791 | 128,822 | 4.6.5 (2026-09-13) |
| typescript (for scale) | 209,215,221 | 1,004,355,023 | 66,420 | 7.0.2 (2026-07-08) |
| @sinclair/typebox (TypeBox 0.x) | 81,212,632 | 410,407,371 | 6,708 | 0.34.52 (2026-07-11) |
| @standard-schema/spec | 80,710,923 | 401,689,688 | 1,326 | 1.1.0 (2025-12-15) |
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
| @total-typescript/ts-reset | 847,650 | 4,462,451 | 108 | 0.6.1 (2024-09-02) |
| typia | 241,879 | 1,163,880 | 331 | 15.0.0 (2026-09-22) |
| runtypes | 186,342 | 905,309 | 344 | 7.0.5 (2026-08-14) |

- Zod companions are large too: `zod-to-json-schema` 44,434,863/week and `zod-validation-error` 34,454,554/week. CONFIRMED — same endpoint
- `typescript-eslint` (the usual home of `no-explicit-any`, `no-unsafe-*` and `no-non-null-assertion` rules) has 64,393,188 weekly downloads. CONFIRMED — same endpoint
- Ajv is a JSON Schema validator for JavaScript in general, not TypeScript-specific, and much of its volume comes through build tooling; it should not be counted as "exists because TypeScript types are erased" without that caveat. (Inference from its role; no source read on its dependents' mix.)

**Trend**
- Zod: an earlier note gives about 212M weekly in September 2026 from npm trends, Socket and Snyk, which matches the registry's 211.9M; an older DEV post said "139M weekly downloads and one maintainer". NOTES (`frontier_2026_candidates.md` section E). Zod 4.0.0 was published 2025-07-09 and 4.6.0 on 2026-09-09. CONFIRMED — [registry zod](https://registry.npmjs.org/zod)
- Effect: "5M weekly downloads (November 2025), passing 10M weekly by mid-April 2026" per the project blog (earlier note, UNVERIFIED); registry now shows 26.3M, which would be about 2.6x since April if the blog figures are right. Effect 4.0 is at release-candidate stage (`rc` tag 4.0.0-rc.117; `latest` still 3.22.2). CONFIRMED (registry) — [registry effect](https://registry.npmjs.org/effect); blog figure NOTES — [This Week in Effect](https://effect.website/blog/this-week-in-effect/2026/04/17/)
- Valibot: an earlier comparison site gave 15.6M/week (flagged as suspicious); the registry shows 13.4M, so the order of magnitude holds. ArkType: earlier note "about 400K/week", registry now 1.35M. TypeBox: earlier note 5.5M/week; the registry shows 81.2M for `@sinclair/typebox` (mostly transitive) plus 8.2M for the new `typebox` package. The earlier TypeBox figure was badly low. CORRECTED — registry (above) vs [PkgPulse](https://www.pkgpulse.com/guides/zod-v4-vs-arktype-vs-typebox-vs-valibot-2026)
- io-ts (last release 2024-12-10), superstruct (2024-07-06) and fp-ts (2025-08-18) are no longer actively released; the growth is in Zod, Valibot, ArkType, TypeBox 1.x and Effect. CONFIRMED (release dates from registry)
- Caveat on inflation: PkgPulse notes Zod's count is "inflated by its inclusion as a peer/transitive dependency of tRPC, React Hook Form resolvers, and Drizzle ORM". NOTES (`frontier_2026_candidates.md`)

### Inferences
- The validator market is the market's answer to erased types, and it is already at compiler scale: Zod alone is downloaded as often as `typescript`. A new language that compiles contracts into boundary checks competes with (or should emit) these, not with nothing. Emitting Standard Schema-compatible validators would plug into the ecosystem that already exists.
- The market has not converged on one validator (Zod, Valibot, ArkType, TypeBox, Effect Schema, Ajv, Joi, Yup, class-validator all have millions of weekly downloads), and a shared interface (`@standard-schema/spec`) was needed, which suggests the "one declared type, one runtime check" problem is felt but solved in a fragmented way, with the type and the schema kept in sync by hand or by inference from the schema.

### Gaps
- Historical weekly downloads (for a proper trend line) need api.npmjs.org, which is blocked; the trend above rests on earlier-note snapshots.
- The share of validator downloads that is direct use versus transitive could not be measured.

## 4. TypeScript itself in 2026: TypeScript 7, TC39 type annotations, official stance on soundness

### Takeaway
(being written)

### Cited Findings
- **TypeScript 7.0 GA date.** The registry's `latest` tag is 7.0.2, published 2026-07-08T15:55:18Z; 7.0.1-rc was published 2026-06-18; 6.0.2 on 2026-03-23 and 6.0.3 on 2026-04-16; the `next` tag is 7.1.0-dev.20260924.1. The 7.0.2 package ships per-platform native binaries as optional dependencies (`@typescript/typescript-linux-x64`, `-darwin-arm64`, `-win32-x64` and about 17 others), consistent with the native (Go) compiler. CONFIRMED — [registry typescript](https://registry.npmjs.org/typescript)
- A side-by-side compatibility package `@typescript/typescript6` (6.0.2, 2026-07-06) has 3,907,396 weekly downloads, and the preview package `@typescript/native-preview` still has 6,079,557. CONFIRMED — registry search endpoint
- Earlier note: 7.0 GA "with 8–12x faster full builds", "no stable programmatic API until 7.1". SNIPPET-CONFIRMED in the earlier note — [InfoQ](https://www.infoq.com/news/2026/08/typescript-7-released/)
- **TypeScript 7 is a Go binary with the same checking options.** The `@typescript/typescript-linux-x64` 7.0.2 tarball contains a 24 MB `lib/tsc` executable whose strings name the Go toolchain `go1.26.4`. Every type-checking option of 6.0.2 is still present by name (`strict`, `noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `useUnknownInCatchVariables`, `strictBuiltinIteratorReturn`, `noPropertyAccessFromIndexSignature` and the rest), new 7.0 options `checkers`, `builders` and `singleThreaded` appear, and seven old options are gone: `importsNotUsedAsValues`, `keyofStringsOnly`, `noImplicitUseStrict`, `noStrictGenericChecks`, `preserveValueImports`, `suppressExcessPropertyErrors`, `suppressImplicitAnyIndexErrors`. Three of the removed ones (`noStrictGenericChecks`, `suppressExcessPropertyErrors`, `suppressImplicitAnyIndexErrors`) were switches that weakened checking. CONFIRMED (string scan of the registry tarball; this is a name-level check, not a behavioural test) — [registry tarball](https://registry.npmjs.org/@typescript/typescript-linux-x64/-/typescript-linux-x64-7.0.2.tgz)
- **TypeScript 6.0 turned `strict` on by default.** In 5.9.3's `lib/typescript.js`, `getStrictOptionValue` returns `!!compilerOptions.strict` when a strict sub-flag is unset (off unless asked for); in 6.0.2 it returns `compilerOptions.strict !== false` (on unless turned off). CONFIRMED (compiler source in the registry tarballs `typescript-5.9.3.tgz` and `typescript-6.0.2.tgz`). Secondary sources agree: 6.0 "shipped in March 2026 as the last JavaScript-based compiler release"; "Strict mode is now default. Projects that relied on strict: false implicitly will now see errors from strictNullChecks, noImplicitAny, strictFunctionTypes, strictBindCallApply, strictPropertyInitialization, noImplicitThis, useUnknownInCatchVariables, and alwaysStrict"; other new defaults are `module: "esnext"`, a floating `target` (es2025), `types: []` and `noUncheckedSideEffectImports: true`; deprecations such as `baseUrl` and `moduleResolution: node` "will be removed in TypeScript 7.0". SNIPPET-CONFIRMED — [TypeScript 6.0 release notes](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-6-0.html); [Socket](https://socket.dev/blog/typescript-6-0-released-final-javascript-based-version); [jsmanifest](https://jsmanifest.com/typescript-6-breaking-changes); [OpenReplay](https://blog.openreplay.com/whats-new-typescript-6-0/)
- 6.0.2 added only two compiler options over 5.9.3 (`ignoreConfig`, `stableTypeOrdering`); neither concerns soundness. CONFIRMED (diff of option declarations in the two tarballs)
- **Official stance on soundness (unchanged background):** TypeScript's design non-goals include "Apply a sound or 'provably correct' type system. Instead, strike a balance between correctness and productivity." SNIPPET-CONFIRMED (result list includes the Microsoft TypeScript-wiki design-goals page and two secondary write-ups) — [TypeScript Design Goals](https://github.com/microsoft/TypeScript-wiki/blob/main/TypeScript-Design-Goals.md); [dsebastien.net](https://www.dsebastien.net/2020-04-25-typescript-non-goals/)

(TC39 status: pending)

## 5. Sound or verified languages that compile to JavaScript/TypeScript and publish to npm

### Takeaway
(being written)

### Cited Findings

**Registry figures on 2026-09-24** (CONFIRMED — registry search endpoint and packuments)

| Package | What it is | Weekly | Dependents | Latest (date) |
|---|---|---|---|---|
| hermes-parser | Meta's parser for Flow-typed JS (used by React Native tooling) | 56,654,019 | 330 | 0.37.0 (2026-07-14) |
| flow-parser | Flow's parser | 5,318,962 | 372 | 0.333.0 (2026-09-24) |
| flow-bin | Flow type checker binary | 433,924 | 465 | 0.333.0 (2026-09-24) |
| assemblyscript | TypeScript-like language to Wasm (for comparison) | 124,967 | 159 | 0.28.20 (2026-07-22) |
| rescript | ReScript compiler | 33,972 | 207 | 12.3.1 (2026-08-23) |
| @rescript/runtime | ReScript 12 runtime | 29,356 | 18 | 12.3.1 (2026-08-23) |
| elm | Elm compiler installer | 26,908 | 37 | 0.19.2-0 (2026-07-06) |
| elm-test | Elm test runner | 23,251 | 9 | 0.19.2-1 (2026-08-21) |
| @rescript/core | ReScript 11 standard library | 15,330 | 75 | 1.6.1 (2024-10-16) |
| purescript | PureScript compiler installer | 7,563 | 11 | 0.15.16 (2026-03-15) |
| spago | PureScript build tool | 4,860 | 4 | 1.0.4 (2026-03-30) |

- ReScript release cadence: 11.0.0 on 2024-01-10, 12.0.0 on 2025-11-25, 12.1.0 2026-01-13, 12.2.0 2026-02-27, 12.3.0 2026-05-19, 12.3.1 2026-08-23; a `next-13` tag points to 13.0.0-alpha.6. CONFIRMED — [registry rescript](https://registry.npmjs.org/rescript)
- Elm's npm `latest` tag moved to 0.19.2-0 on 2026-07-06 (0.19.1 was the previous line). CONFIRMED — [registry elm](https://registry.npmjs.org/elm). What 0.19.2 changes was not checked.
- Gleam and Dafny do not publish their compilers to npm under those names (`gleam` on npm is an unrelated 2014 package with 5 weekly downloads; `dafny` is not in the search results). CONFIRMED — registry search endpoint
- For scale: all sound compile-to-JS compilers together (ReScript, Elm, PureScript, about 68K weekly) are about 0.03% of `typescript`'s 209M. Flow's parsers are large only because of React Native and Meta tooling, not because of independent Flow adoption.

**LemmaScript (Midspiral), the closest 2026 prior art to "contract-checked TypeScript"**
- npm package `lemmascript`: "A verification toolchain for TypeScript — generates Lean 4 or Dafny from annotated TS". First published 2026-04-01; 36 versions; latest 0.6.4 on 2026-09-19; 1,733 weekly downloads; 0 dependents. CONFIRMED — [registry lemmascript](https://registry.npmjs.org/lemmascript)
- Its README (read from the registry packument): "Write ordinary TypeScript with `//@ ` specification annotations. The toolchain generates verifiable code from your TypeScript — either in Dafny or Lean 4 (with Velvet/Loom)." It calls itself a "**Tech Preview**: the core idea is there, but support, semantics, and ergonomics are still evolving." The Dafny backend writes `foo.dfy.gen` (regenerated) and `foo.dfy` ("source of truth, with LLM/user proof additions"), and "The diff between them must be additions-only". A reusable GitHub Actions workflow "fails the build if any committed generated file is out of date". CONFIRMED — [registry lemmascript](https://registry.npmjs.org/lemmascript)
- Case studies listed in the README include brownfield, in-place verification of pieces of real projects: Hono's security middleware ("Four CVEs covered", including an IP-restriction bypass, CVE-2026-39409), opencode's permission system and patch parser, Infisical's permission-boundary glob check ("the privilege-escalation guard"), xyflow utilities, rallly's `validateRedirectUrl`, balanced-match, the pi and Flue agent harnesses; plus greenfield apps (for example "123 Dafny lemmas", "16-conjunct invariant"). These are the vendor's own showcase repos, not independent adoption. CONFIRMED (README text) — [registry lemmascript](https://registry.npmjs.org/lemmascript)
- Midspiral's site title: "The correctness layer for AI-generated software". The launch post is dated around 2026-04-22 by a news aggregator and was discussed on Lobsters. SNIPPET-CONFIRMED (search result list: Midspiral blog, lemmascript.com, E-Ink News, Lobsters, jsDelivr) — [Midspiral blog](https://midspiral.com/blog/lemmascript-a-verification-toolchain-for-typescript/); [lemmascript.com](https://lemmascript.com/); [Lobsters](https://lobste.rs/s/4tuujf/lemmascript_verification_toolchain_for)
- Related: `dafny2js` (metareflection) "generates JavaScript/TypeScript adapters from Dafny sources"; Dafny's JS backend emits "Javascript source consistent with Node.js v 16.0.0", run with `dafny run --target:js`. UNVERIFIED (search summary of dafny.org docs and the dafny2js repo) — [Dafny reference](https://dafny.org/dafny/DafnyRef/DafnyRef); [dafny2js](https://github.com/metareflection/dafny2js)

(more pending: Gleam JS target, ReScript adoption)

## 6. Notable 2024–2026 incidents traced to type confusion or unvalidated input, and post-mortem advice

### Cited Findings

**n8n CVE-2026-25049 (GHSA-6cqr-8cfr-67f8)**
- Advisory metadata from the npm audit endpoint: title "n8n Has Expression Escape Vulnerability Leading to RCE", severity critical, CWE-913 (not CWE-843 "type confusion"), affected `>=2.0.0 <2.5.2` and `<1.123.17`. CONFIRMED — npm audit endpoint (`POST https://registry.npmjs.org/-/npm/v1/security/advisories/bulk`), advisory URL [GHSA-6cqr-8cfr-67f8](https://github.com/advisories/GHSA-6cqr-8cfr-67f8)
- Root cause, quoted identically by several outlets: "the sanitization function assumes keys in property accesses are strings in attacker-controlled code. This assumption is correctly encoded in a TypeScript annotation, but it is not enforced through runtime type checking." One summary adds: "Even when developers correctly annotate the sanitizer's input parameter as a string, attackers can still pass non-string values at runtime, thereby bypassing the security controls." Fix: "implementing proper runtime type checking in the sanitization functions and expanding AST coverage to handle destructuring patterns." SNIPPET-CONFIRMED (two searches; result lists include Endor Labs, OPSWAT, SOCRadar, Fidelis, Tenable, The Hacker News, PurpleOps, hetmehta.com) — [Endor Labs](https://www.endorlabs.com/learn/cve-2026-25049-n8n-rce); [OPSWAT](https://www.opswat.com/blog/cve-2026-25049-expression-sandbox-escape-leading-to-remote-code-execution-in-n8n); [hetmehta.com](https://hetmehta.com/posts/n8n-type-confusion-rce); [SOCRadar](https://socradar.io/blog/cve-2026-25049-n8n-expression-escape/)
- CVE-2026-25049 followed an earlier expression-injection RCE, CVE-2025-68613, reported by Fatih Çelik; about 10 researchers were credited on 25049. UNVERIFIED (one search summary) — [The Hacker News](https://thehackernews.com/2026/02/critical-n8n-flaw-cve-2026-25049.html)

**n8n's advisory history as a whole** (CONFIRMED — npm audit endpoint, queried 2026-09-24 for n8n versions 1.0.0, 1.123.16 and 2.5.1; deduplicated by GHSA URL)
- 149 distinct GitHub advisories (225 entries before de-duplication across the 1.x and 2.x branches), 23 of them rated critical.
- 14 are expression or code sandbox escapes or sanitizer bypasses (JavaScript expression engine, JavaScript and Python task runners), several after CVE-2026-25049: "Expression Sandbox Escape Leads to RCE" (GHSA-vpcf-gvg4-6qwr, CVSS 9.9), "Expression sandbox escape via arrow-function bodies" (GHSA-gv7g-jm28-cr3m), "Expression Sandbox Escape via Class-Field Sanitizer Rebinding" (GHSA-hw8v-xxg5-vvvx), "Expression Sandbox Escape via Shared Builtin Tampering and Code-Printer Injection" (GHSA-6xcw-7xm6-48c6), "Legacy Expression Evaluator Sanitizer Bypass" (GHSA-pm35-fqvh-cq5g).
- 11 are prototype pollution (CWE-1321), including critical RCEs: "Prototype Pollution in XML Webhook Body Parser that Leads to RCE" (GHSA-q5f4-99jv-pgg5, CVSS 10), "HTTP Request Node Pagination Prototype Pollution to RCE" (GHSA-c8xv-5998-g76h), and "XML Node Prototype Pollution Patch Bypass" (GHSA-wrwr-h859-xh2r).
- One carries CWE-843 (type confusion) explicitly: "Send Email Node Arbitrary File Read and SSRF via Nodemailer Content-Object Type Confusion" (GHSA-2x35-3fw4-9jr4). CVE-2026-25049, the textbook "TypeScript annotation not enforced" case, is filed as CWE-913, so CWE-based counts of type confusion undercount this class.
- The largest single CWE classes are still conventional web bugs: XSS (CWE-79) 16, SQL injection (CWE-89) 13, incorrect authorization (CWE-863) 13.

**A census of 51 popular npm packages** (CONFIRMED — npm audit endpoint, 2026-09-24; every published version of each package submitted, results deduplicated by GHSA URL). Packages: tmp, binary-parser, qs, express, body-parser, axios, lodash, undici, ws, jsonwebtoken, multer, formidable, sanitize-html, dompurify, xml2js, fast-xml-parser, handlebars, ejs, pug, next, nuxt, vite, @angular/core, react-dom, vm2, node-fetch, got, cookie, path-to-regexp, semver, minimist, yargs-parser, json5, flatted, mongoose, sequelize, typeorm, knex, pg, mysql2, socket.io, tar, jose, validator, class-transformer, superagent, koa, fastify, hono, @trpc/server, graphql. This is a convenience sample chosen for this note, not a random one.
- 551 distinct advisories across the 51 packages. Top CWEs: XSS (CWE-79) 66, **prototype pollution (CWE-1321) 51**, uncontrolled resource consumption (CWE-400) 49, code injection (CWE-94) 33, CWE-770 33, information exposure (CWE-200) 28, path traversal (CWE-22) 27, **improper input validation (CWE-20) 26**.
- Explicit type confusion is filed rarely: 5 advisories carry CWE-843 or CWE-704, plus one titled "type-confusion" under CWE-20/22:
  - tmp: "Type-confusion bypass of `_assertPath` allows path traversal via non-string prefix/postfix/template" (GHSA-7c78-jf6q-g5cm, high, CVSS 8.2, affects 0.2.6). A write-up gives it as CVE-2026-49982: the library "relies on the .includes() method without verifying the input type", so arrays or duck-typed objects pass the check and "downstream string coercion subsequently restor[es] the traversal sequence". UNVERIFIED for the CVE number and wording (one source) — [CVEReports](https://cvereports.com/reports/CVE-2026-49982)
  - handlebars: three "JavaScript Injection via AST Type Confusion" advisories (GHSA-2w6w-674q-4c4q critical 9.8; GHSA-3mfm-83xf-c92r and GHSA-xhpv-hc6g-r9c6 high 8.1), CWE-843 + CWE-94, affecting `>=4.0.0 <=4.7.8`.
  - sequelize: "Unsafe fall-through in getWhereConditions" (GHSA-vqfx-gj96-3w95, critical 9.9, CWE-843).
  - tar (node-tar): "Process crash via PAX numeric path type confusion" (GHSA-w8wr-v893-vjvp, moderate, CWE-704).
- Prototype pollution, the other face of unvalidated JSON and object input in JavaScript, is common: 51 advisories in 19 of the 51 packages, 14 of them in axios alone (for example "Authentication Bypass via Prototype Pollution Gadget in `validateStatus` Merge Strategy", "Full Man-in-the-Middle via Prototype Pollution Gadget in `config.proxy`", "Patch Bypass: Proxy-Authorization Header Injection via Prototype Pollution — Incomplete Null-Prototype Fix").
- Recency (inference): the audit endpoint gives no dates. Its numeric advisory IDs rise over time, and n8n's February 2026 advisory has ID 1112923, so the 25 of 51 prototype-pollution advisories with IDs above that are probably from 2026. The tmp, handlebars, sequelize and tar type-confusion advisories all have IDs between 1115538 and 1123939, so they are probably 2026 too. Treat these dates as approximate.
- Background on the general pattern: "If one of these methods is used to validate user input, the validation could be prone to a potential bypass if an input of different type is provided (for example an array) and no checks are performed." UNVERIFIED (search summary of Snyk material) — [Snyk blog](https://snyk.io/blog/remediate-javascript-type-confusion-bypassed-input-validation/); [Snyk Learn](https://learn.snyk.io/lesson/type-confusion/)
