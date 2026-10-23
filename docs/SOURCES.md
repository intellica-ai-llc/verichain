# SOURCES — claim triage (first pass, 2026-10-23)

Status: **verified** = primary source opened; **partial** = source opened, claim differs; **unverified** = not checked; **secondary** = relies on analyst/vendor writing.

| Claim (where used) | Status | Note |
|---|---|---|
| ORCHID (Weinberg) — 100% consensus at ≤40% Byzantine, <4 s at n=30 (v6 ARC42) | partial | arXiv 2605.12211: single-author v1 preprint, simulation only (n=10–150), uses Kuramoto + Orch-OR + "Quantum Secret Sharing"; not peer reviewed. Deferred to research track |
| NANOZK — 5.5 KB/layer, 24 ms, ε<1e⁻³⁷ (v6 ARC42) | partial | arXiv 2603.18046 (v2): ~3.5–3.7 KB/layer, ~22 ms; end-to-end measured only to width d=128; GPU figures projected; ε value not confirmed |
| Moon & Varshney containment; Spera hypergraph closure; TRUST / Safety–Profitability; Σᴿ envelope; AgentCity; PASTE/B-PASTE; zkAgent; SAPL "2×" | unverified | cited only as "【chat】" in v6 |
| RFC 9943 SCITT Architecture, Proposed Standard, Jun 2026 | verified | basis of v1 receipts |
| Rekor v2 / Trillian-Tessera tile logs; witness network cosigning checkpoints | verified | transparency.dev and sigstore blog posts |
| Merkle Tree Certificates (IETF PLANTS) — inclusion-proof approach to PQ | verified (secondary: LWN) | draft; Chrome experiment targeted end of 2027 |
| EU AI Act Digital Omnibus: Reg (EU) 2026/1744, in force 27 Jul 2026; Annex III → 2 Dec 2027; Annex I → 2 Aug 2028 | verified (secondary: CSA note, law-firm notes) | confirm Art. 12 text before marketing |
| NIST AI Agent Standards Initiative (17 Feb 2026); NCCoE concept paper | verified (secondary: CSA note) | not final guidance |
| Polymarket V2: pUSD ERC-20 on Polygon backed by USDC; taker-only fees on 15-min crypto markets; maker rebates | verified | docs.polymarket.com; The Block |
| Telonex study: 36.8% of 46,945 wallets profitable (Feb 2–8 2026) | verified | pre-fee, one week |
| x402 ≈ $24M in 30 days to Jul 2026 | secondary | BlindPay comparison |
| Market size $1.68B (2026) → $8.62B (2031) | secondary | Mordor; vendor/analyst figures |
| Pricing ranges ($0.01–0.08/action; $5–100k+/month tiers) | secondary, weak | Monetizely; cites 2023 Forrester; do not use for pricing decisions |
| SR 11-7 replaced Apr 2026 (SR 26-2 / OCC 2026-13); DORA 4-hour notice; 17a-4 mapping | secondary | Diagrid article; mapping is the author's inference |
| Polymarket "67K→1.13M" bot; Foresight News 25-strategy post-mortem; PolyBench | unverified | cited from chat |
| Circle froze 122 addresses in 2026; KelpDAO $290M hack via 1-of-1 DVN; ERC-8004 live 29 Jan 2026; Alabama DUNA SB 277 | unverified | ARC42 v6 claims |

## Added 2026-10-23 (trading and go-to-market research)
| Claim | Status | Note |
|---|---|---|
| Polymarket favorite–longshot bias: <10¢ buys lose 19.3¢/$; ≥90¢ buys +0.83¢/$ (588M trades); sign flips when grouped by parent event; absent in Sports | verified | arXiv 2609.12878; fees not addressed in the abstract |
| Kalshi: makers earn higher returns than takers; low-price contracts underperform break-even after fees | verified (abstract only) | CEPR DP20631; magnitudes not read |
| Polymarket liquidity-rewards formula S=((v−s)/v)²·b; two-sided rule outside 0.10–0.90; per-minute sampling; daily payout | verified | docs.polymarket.com/programs/liquidity-rewards |
| Optimal market making for binary contracts (HJB); illustrative variance reduction 28.11→10.34 at same mean PnL | verified (abstract), parameters uncalibrated | arXiv 2607.17991 |
| Explicit belief→trade layer with limits outside the LLM profitable in small replay (+15.9% ROI on 42 rows; +5.9% with CI incl. zero on 132 rows) | verified, tiny sample | arXiv 2607.03015 |
| Prophet Arena: LLMs ≈ markets in forecasting; simulated returns | secondary | Decrypt summary only |
| Polymarket geoblock list; T&T not listed | verified | not legal clearance |
| Cross-venue gaps 2–5¢ after fees, ~80% close within an hour | anecdotal | simplefunctions.dev, unpublished log |
| sUSDe-style yield compressed to high single digits Q2 2026; 4–30% realized 2024–25 | secondary, single source | eco.com article |
| Deflated Sharpe Ratio corrects for multiple testing and non-normality | verified (abstract) | SSRN 2460551 |
| MinTRL ≈ (1.645/SR)² years for iid normal returns | derived | standard result; assumptions stated |
| Chainguard ≈ $100M ARR (Jun 2026), 150+ customers, $3.5B valuation | secondary (Sacra estimate) | analog only |
| Glosten–Milgrom, Kyle, Avellaneda–Stoikov, Almgren–Chriss, Kelly, PBO | established literature | not re-opened this session |
| RFC 9942 is referenced by RFC 9943 for verifiable-data-structure details (title and content not opened) | partial | read RFC 9942 at G1 |
| C2SP tlog-tiles / tlog-checkpoint / tlog-cosignature formats | unverified | read at G1 |
| Public witness-network onboarding for a new log | unverified | confirm at G3 |
| ML-DSA / SLH-DSA Rust crate maturity and audit status | unverified | confirm at G1 |
