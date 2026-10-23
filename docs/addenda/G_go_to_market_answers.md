# Addendum G — Answers to the open commercial questions (recommendations)

Date: 2026-10-23 · Status: DRAFT · Tags as in Addendum F. Your real financial objectives are still not in the repo; the targets below are placeholders to replace.

## Q1. Which segment first: banks, or agent platforms / marketplaces?
**Recommendation: lead developer-first (open spec + verifier + gateway), run regulated pilots through your own Verity channel in parallel, and do not lead with cold bank sales.**
- Analog **[V, secondary]**: Chainguard (open-source trust infrastructure adjacent to Sigstore) is estimated by Sacra at ~$100M ARR (Jun 2026), 150+ paying customers (≈$0.67M average), valued $3.5B; product-led developer adoption feeds enterprise sales; revenue is recurring subscription with SLAs, compliance reporting and FIPS variants. Read: open trust tooling can become a large enterprise business, but the money comes from **enterprise subscriptions with compliance and SLA value**, not from the free layer. (Estimates, not audited.)
- Banks: sales cycles are long and the rules for agents are in flux (Addendum E; no enforcement action yet). Use Verity as a **reference deployment** and design partner instead.

## Q2. Hosted witness/anchoring product vs. verified-performance ledger?
**Recommendation: one platform, two packages.** Build the log once. The "verified performance ledger" is an *evidence-pack template* on that log (strategy provenance, no retroactive edits, limits and authorization lineage), sold to allocators and marketplaces. Hosted witnessing/retention is the infrastructure product that makes both credible. Do not build separate systems.
- Caution: selling performance verification to allocators implies fund-regulation questions (Addendum F §4). Start with **own-capital dogfood** (MAS v2) and bot-marketplace / counterparty use cases; get counsel before external capital.

## Q3. What financial objective should we design to?
**Recommendation:** adopt a placeholder ladder and replace it with your numbers:
| Horizon | Placeholder objective | Evidence basis |
|---|---|---|
| Month 3 | 10 design-partner conversations; ≥3 pilot LOIs | process target [J] |
| Month 6 | 2 paid pilots; open verifier published; 1 external implementation verifies a receipt | [J] |
| Month 12 | 10–15 paying customers; one standards contribution | [J]; Chainguard shows enterprise ACVs can be six figures, but that is a far larger company |
Kill criteria: no pilot willingness after 10 conversations in a quarter → revisit thesis.
Pricing: sell **assurance + compliance + retention**, not per-event receipts, because free OSS receipts are abundant (Addendum C). Run pricing discovery with design partners; published price ranges found so far are weak (Addendum E).

## Q4. First 10 conversations (suggested mix)
2 from your own stack (Verity banking users, VeriCrypt compliance users), 2 agent platforms using MCP, 2 bot/strategy marketplaces or allocators, 1 insurer/MGA, 1 auditor/assurance firm, 1 standards contact (SCITT agent-receipt authors), 1 regulator/government contact via the national programme.

## Q5. What unlocks "exponential"?
1. Be the **neutral verifier** that other vendors' receipts validate in (standards + test vectors).
2. **Cross-organization evidence links** (receipt references another party's receipt) so value grows with participants.
3. **Witness network** growth.
4. **Compliance mapping** as a reusable asset: each new regulation is a new pack, not new engineering.
5. Zero marginal infrastructure cost per receipt (≈1 KB each [J]) keeps margins high; cost is people and trust.

## Sources
- https://sacra.com/c/chainguard/ · Addenda C, D, E, F in this folder
