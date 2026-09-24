# Industry follow-up: settling the remaining unverified items (as of 24 September 2026)

_Status: in progress. This file is saved early and updated as searches complete._

_Method note. Inputs read first: `gap_fill_industry.md` and `primary_reads_industry_partial.md` (both in full) and a skim of `industry_scale_pain.md`. Items already CONFIRMED there were not rechecked. Status labels: CONFIRMED (page opened and read in this session), SNIPPET-CONFIRMED (page not opened, but two or more independent search results agree), CORRECTED (old value, new value, source), COULD NOT VERIFY (what was tried)._

_Fetch attempts this round, all EGRESS_BLOCKED (one try per domain, then stopped): blog.cloudflare.com, status.cloud.google.com, aws.amazon.com, www.cybcube.com, www.parametrixinsurance.com, nebius.com, www.cisa.gov, www.ic3.gov._

_The search tool returns a list of result titles and URLs plus one merged summary. It does not give a separate snippet per URL. "Search summary" below means that merged text. SNIPPET-CONFIRMED is used only when two or more independent outlets appear in the result list and their titles or the summary carry the claim._

## 1. Outage root causes

### Takeaway
All four post-mortem root causes now stand at SNIPPET-CONFIRMED level, and the crash-dump part of CrowdStrike was already CONFIRMED. One loss figure needs rewording: CyberCube's AWS estimate is a range of **$38M to $581M, with the likely outcome near the low end (about $40M)**, so "up to $581M" should not be quoted alone. 2026 brought at least two outages with published causes: Cloudflare's 20 February 2026 BYOIP outage (an automation bug in which an empty query parameter matched every prefix) and Microsoft 365 on 22 January 2026 (capacity shortfall during maintenance). Neither is a memory-safety or type error.

### Cited Findings
(pending)

### Inferences
(pending)

### Gaps
(pending)

## 2. CISA "Product Security Bad Practices" and 2026 US federal changes

### Takeaway
(pending)
