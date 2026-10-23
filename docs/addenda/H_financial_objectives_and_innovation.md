> **Partly superseded by Addendum J §3 (financial objectives v2).** P1–P9 and legal notes stand.

# Addendum H — Financial objectives and profit innovations

Date: 2026-10-23 · Status: DRAFT for approval · Author's note: Damain delegated "you set the financial objectives". Every number below is **my assumption**, not a forecast, and is revisable. Sources and verification status: `docs/SOURCES.md`.

## 1. Headline conclusion
**Trading cannot fund this company; evidence and assurance revenue must.**
- 1%/month on US$5,000 ≈ US$50/month. A US$10k/month trading income at 1%/month needs ≈ US$1M of capital. Not available, and edge is unproven (Addendum F; MAS v2 gates).
- So: the trading swarm is a **research and proof-of-product track** (it produces real, verifiable decision evidence — the best demo VeriChain can have), never the revenue plan.
- Revenue plan = selling witnessed, verifiable evidence of agent actions to buyers who face audit, insurance, or regulation.

## 2. Financial objectives I set (assumptions, revisable)

| Horizon | Objective | Arithmetic / basis | Kill or pivot if |
|---|---|---|---|
| Month 3 | ≥10 design-partner conversations; ≥3 willing to pilot (LOI or written interest) | Addendum G outreach list | <3 willing after 10 → revisit buyer segment |
| Month 6 | Open verifier + spec published; ≥1 external party verifies a receipt; ≥2 **paid** pilots | pilot price assumption US$5k–15k each | zero paid pilots → thesis review |
| Month 12 base | ~10 paying customers × ~US$25k ≈ **US$250k ARR** | assumption; compare Chainguard-style open-core, which reached far larger scale on a much bigger market | <US$60k ARR → change segment or stop |
| Month 12 stretch | ≈ US$600k ARR (e.g., 12 × US$50k) | needs one regulated-buyer anchor | — |
| Cost ceiling | Hosting + witnessing + legal ≤ US$1,500/month until first paid pilot | solo founder, free-tier object storage | exceeded without revenue → pause hosted tier |
| Trading (MAS) | **Learning objectives, not return targets**: S0 research with DSR ≥ 0.95 at true trial count; paper ≥60 days; micro-live capital cap = lesser of 5% of liquid risk capital or US$5,000 | MAS v2 §10.1 | any stage-gate failure → stop that strategy |

Why these are defensible: they are sized to a solo operator, tied to leading indicators (conversations → pilots → paid), and each has a kill rule. They are not market-share claims. Market-size figures (e.g., an analyst estimate of agent observability/governance growing from about US$1.7B in 2026) are context only.

## 3. Profit innovations, ranked

| # | Innovation | Why it could make money | Effort | Verdict |
|---|---|---|---|---|
| P1 | **Completeness-by-construction** (C2 chokepoint signer; already headline) | auditors/insurers need to know nothing was omitted; competitors log only what agents choose to report | in plan | keep as lead |
| P2 | **Agent and tool provenance profile** — new statement kinds `artifact` (tool, MCP server, skill, model, prompt/config build) and `attestation` (who vetted it). Gateway can refuse/flag unsigned tools | research found no major MCP client verifies tool signatures natively, and a large share of scanned agent skills had serious flaws; supply-chain provenance is a proven paying category (Sigstore/Chainguard analog) | low–medium: reuses the same log | **adopt in v1 scope as a profile** (ARC42 v7 ADR V13) |
| P3 | **Evidence packs for insurers and certifiers** (AIUC-1-style certification, agent liability underwriting) | evidence that controls actually ran is what underwriting and audits consume | medium; needs a pilot with a real insurer/auditor | adopt as M3 target; AIUC-1 details **unverified** (page fetch was blocked) |
| P4 | **Compliance mapping** (EU AI Act Art. 12 logging; Digital Omnibus delays high-risk dates to Dec 2027 / Aug 2028 — dates were verified in an earlier session from secondary sources; recheck against the Official Journal; US bank model-risk guidance replaced April 2026) | gives buyers a reason to buy before the deadline and a document their auditor accepts | low (writing) | adopt M3 |
| P5 | **A2 assurance via GPU confidential-computing attestation** instead of zk-LLM proofs | attestation of the runtime is available on current hardware; zk proofs for large models remain too costly/immature | medium | A2 = TEE attestation (already so); do not sell A3 for LLMs |
| P6 | **Non-dilutive funding** | open-source infrastructure grants fit the open-core spec/verifier | low | NLnet's last call I could find closed 1 Jun 2026; the live page fetch was blocked, so **unverified** — check for the next call and other funds |
| P7 | **Verified-agent trading terminal earning builder fees** (Polymarket builder fees: up to 100 bps taker / 50 bps maker, paid to the builder wallet — per docs I read) | revenue from other traders' volume instead of trading alpha; VeriChain receipts are the differentiator | high: customer funds flows, possible licensing, geo and regulatory review in Trinidad & Tobago and user jurisdictions | **defer** — only after counsel review; do not start before the evidence product has paying customers |
| P8 | **AR-2: combinatorial / rebalancing arbitrage scanner, paper only** | a published study estimates tens of millions in realized arbitrage on Polymarket, so the opportunity exists, but competition, leg risk and latency are severe | medium | paper-only research item in MAS v2 |
| P9 | LP-1 market making with reward capture | maker edge and the reward formula are documented; rewards are pro-rata and competed away | in plan | keep; expect small returns |

Explicitly **rejected** for revenue: taker closing-window strategies (edge ≈ 0 before fees), trailing-window capital reallocation, Lightning/token mechanics, selling signals or managing others' money (fund/advisory regulation).

## 4. Pricing hypotheses (to test, not assert)
- Open-core: Apache-2.0 spec, log, verifier. Paid: hosted witnessing + retention, evidence packs, compliance mappings, SLA.
- Pilot US$5–15k; annual contract US$25–50k; add-ons per agent fleet size or retained evidence volume.
- Test in the first ten conversations; adjust from buyers' answers.

## 5. Spec amendments driven by this addendum
- ARC42 v7: ADR V13 (tool/artifact provenance profile; new statement kinds), §1.2 revenue goal pointer, financial objectives reference.
- MAS v2: M11 (capital cap), M12 (builder-fee terminal deferred), AR-2 paper-only scanner, reward-competition caveat.
- ROADMAP: financial objectives table.
- SOURCES: new claims with status.

## 6. What I could not verify (flagged)
- AIUC-1 requirements and certification mechanics (fetch blocked).
- NLnet current open call and eligibility (fetch blocked).
- Builder-fee rates and terms may change; confirm against current Polymarket docs before any plan relies on them.
- Combinatorial arbitrage realized-profit estimates are from one paper.
- Any customer price point above is an assumption.
