# Network reachability check

Run on 2026-09-24 from the research container, before the gap-filling round. Each host was
probed with `curl` through the container's egress proxy and spot-checked with the web-fetch
tool. Both paths go through the same egress policy, so a host blocked for one is blocked for
the other. Web search still works but returns only titles, URLs and short snippets.

## Hosts HANDOFF.md listed as blocked: all still blocked

Every host below got `403` to `CONNECT` from the egress gateway (a policy denial, not a site
error). The web-fetch tool returned `EGRESS_BLOCKED` for the ones tested with it.

| Host | curl | web-fetch |
|---|---|---|
| survey.stackoverflow.co | blocked | blocked |
| arxiv.org, export.arxiv.org | blocked | blocked |
| dl.acm.org | blocked | blocked |
| www.usenix.org | blocked | blocked |
| conf.researchr.org, pldi25.sigplan.org, www.sigplan.org | blocked | blocked |
| metr.org | blocked | blocked |
| www.jetbrains.com, lp.jetbrains.com, blog.jetbrains.com | blocked | not tried |
| github.blog | blocked | not tried |
| dblp.org | blocked | not tried |
| mlsys.org, openreview.net | blocked | not tried |
| blog.google, security.googleblog.com, research.google | blocked | not tried |
| www.cisa.gov, www.darpa.mil | blocked | not tried |
| www.sonatype.com, blog.cloudflare.com, www.cloudflare.com | blocked | not tried |

## Other hosts the gap round needs: also blocked

stackoverflow.blog, dora.dev, 2025.stateofjs.com, blog.rust-lang.org, rust-lang.org, go.dev,
isocpp.org, open-std.org, lean-lang.org, www.modular.com, docs.modular.com,
www.moonbitlang.com, ziglang.org, gleam.run, www.roc-lang.org, bun.sh, vercel.com,
lwn.net, www.kernel.org, en.wikipedia.org, web.archive.org, semanticscholar.org,
scholar.google.com, ieeexplore.ieee.org, link.springer.com, drops.dagstuhl.de,
proceedings.neurips.cc, proceedings.mlr.press, openreview.net, huggingface.co,
www.veracode.com, www.coderabbit.ai, www.gitclear.com, www.faros.ai, www.crowdstrike.com,
aws.amazon.com, status.cloud.google.com, www.theregister.com, arstechnica.com,
news.ycombinator.com, www.reddit.com, medium.com, epoch.ai, www.swebench.com, and every
university host tried (cmu, mit, upenn, cam, cornell, princeton, stanford, utexas, mpi-sws,
ethz, berkeley, ubc). 133 of the first 150 hosts probed were blocked, then 86 of the next 92.

## Reachable

| Host | Useful for |
|---|---|
| cloud.google.com | DORA 2025 report landing page (`/resources/content/2025-dora-ai-assisted-software-development-report`) |
| www.microsoft.com | Microsoft Research pages (some blog paths answer 403 from the site itself) |
| developer.android.com | Android docs (the memory-safety posts live on the blocked security.googleblog.com) |
| www.anthropic.com | Anthropic announcements (Bun acquisition) |
| kotlinlang.org, www.swift.org, developer.apple.com, www.oracle.com | Language docs and release notes |
| pypi.org | Package metadata |
| Web search | Titles, URLs and snippets from blocked sites; not full text |
| Scholar Gateway (MCP) | Full-text search, but its corpus is Wiley journals; it holds no arXiv, ACM, USENIX or PMLR papers |

## After the network setting was changed

The environment's network access was then set to allow all domains. About 15 minutes of
re-probing (every 30 seconds) showed no change: arxiv.org, survey.stackoverflow.co,
dl.acm.org and metr.org still got `403` to `CONNECT`. The running container keeps the policy it
started with, so the new setting should apply from the next session. A browser inside the
container goes out through the same proxy, so it does not change this.

## Consequence for the gap round

Primary sources for most flagged items (arXiv preprints, ACM and USENIX proceedings,
conference award pages, survey sites, vendor reports, Google and Cloudflare blogs) cannot be
read from this container. Items checked only through search snippets are tagged
`[snippet-verified]`, not CONFIRMED. The fix is in the environment's network settings
(allow these domains, or switch to full network access); routing around the egress policy
is not an option.
