# Phase 0 network check (2026-09-30)

Probed with `curl` through the container's egress proxy at the start of the Phase 0 session.
`000` means the proxy refused the connection.

| Result | Hosts |
|---|---|
| Blocked (`000`) | arxiv.org, export.arxiv.org, dl.acm.org, www.usenix.org, metr.org, midspiral.com, lemmascript.org, dafny.org, github.blog, news.ycombinator.com, api.npmjs.org (download counts), api.osv.dev, nvd.nist.gov, builds.dotnet.microsoft.com |
| Reachable | registry.npmjs.org (package metadata and tarballs), pypi.org, www.nuget.org, `git clone` of public GitHub repositories, GitHub release downloads (`github.com/<org>/<repo>/releases/download/...`) |
| Refused for repositories outside this session's scope | api.github.com (403), github.com/advisories (403) |

The primary sites are still blocked, as they were on 2026-09-24 (see
`research_notes/New programming language pain points/network_check.md`). This session
therefore works from package registries and source code, which is enough for the LemmaScript
assessment: its npm package, its public GitHub repository and the Dafny 4.11.0 release all
downloaded. Facts taken only from a web-search snippet are tagged **[snippet]** in the Phase 0
notes; weekly download counts could not be refreshed because api.npmjs.org is blocked.

Tools installed for Phase 0:

- `lemmascript` 0.6.4 from npm, and its source at commit `097f18f` (2026-09-19) from GitHub.
- Dafny 4.11.0 (`dafny-4.11.0-x64-ubuntu-22.04.zip`, self-contained, bundles Z3).
- `z3-solver` 5.1.0.0 from PyPI (Z3 5.1.0). Dafny bundles its own Z3 4.12.1 and 4.14.1.
- Node.js 22.22.2 and TypeScript from npm.
