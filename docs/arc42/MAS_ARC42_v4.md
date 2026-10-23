# MAS — Multi-Agent Systems on VeriChain — ARC42 v4 (full scope, production grade)

Status: DRAFT for approval · Date: 2026-10-23 · Supersedes v3 and v2 (history in git) · Companion: `VERICHAIN_ARC42_v9.md` · Research basis: Addenda F, H, I, J
**v4 changes:** solo/dev-machine constraints removed (strategy code is built and verified by the platform's engineering factory, ARC42 v9 Appendix G); new evidence on manipulation of 5-minute contracts (LP-2 restricted), Kalshi maker/taker returns, Polymarket resolution mechanics, verified Hyperliquid HIP-4 facts; FC-2 and SV-1 added; AP2/x402 and AAT interoperability; decisions M17–M21.
**Scope decision (Damain, 2026-10-23): no cutting.** All original ambitions (swarms, cross-venue, yield, meta-allocation, external capital, marketplace) remain in scope. They are sequenced behind stage gates and legal gates so that capital and reputation are never put at risk by an unproven claim. Tags: `[V]` verified, `[S]` secondary, `[U]` unverified, `[A]` assumption. **Not financial advice; no strategy is promised profitable; research says most participants lose.** All numbers are placeholders for operator approval.

---
## 1. Introduction and goals
### 1.1 Purpose
MAS is the **flagship tenant of VeriChain**: a fleet of autonomous trading and treasury agents that run **on** the VeriChain runtime (hosted, C3 completeness) and trade through the policy-enforcing signer (C2 at the venue). Its purposes, in order: (1) **prove** the platform under real, adversarial conditions; (2) **learn** which systematic edges exist; (3) produce **independently verifiable track records** that can later back an agent marketplace and external capital; (4) **earn** where the evidence supports it. Trading is not the company's revenue plan (Addendum H); platform and evidence revenue are.

### 1.2 Goals (priority)
1. **Survive** — hard limits outside any model; no ruin. 2. **Prove** — every decision evidenced; P&L reconciled from venue/chain data, never self-reported. 3. **Learn** — statistically honest evaluation, kill criteria. 4. **Earn** — only from supported edges. 5. **Scale to others' capital** — only after gates S0–S3, a sufficient verified track record, and legal clearance.

### 1.3 Capital phases
| Phase | Capital | Preconditions |
|---|---|---|
| A | Operator's own, capped: lesser of 5% of liquid risk capital or US$5,000 (M11) | S0–S2 per strategy |
| B | Own capital, scaled stepwise | S3, capacity tests |
| C | External capital via VeriChain jobs/marketplace (`vc-jobs`, `vc-market`) | verified track record meeting the minimum-track-record math; legal gates L-1, L-2 and fund/advisory regulation reviewed by counsel; no marketing of returns without counsel |

### 1.4 Stakeholders
Operator (risk capital provider, kill authority); VeriChain platform (host, evidence); counsel; future allocators and auditors (evidence readers); venues (counterparties).

---
## 2. Constraints
| Constraint | Consequence |
|---|---|
| Venue: Polymarket CLOB V2, collateral pUSD (USDC-backed ERC-20 on Polygon) `[V]` | USD-denominated accounting; Lightning not used |
| Geoblock | startup and periodic check; Trinidad & Tobago was not on the geoblock list when checked `[V]` — not legal clearance |
| Fees | CLOB V2: taker-only fees on 15-minute crypto markets; makers pay none `[V]`; builder fees up to 100 bps taker / 50 bps maker exist for builders `[S]` |
| Additional venues | Kalshi (fiat/jurisdiction review); **Hyperliquid HIP-4**: live on mainnet since 2 May 2026, phase 1 validator-curated crypto-price binaries, YES/NO priced 0.001–0.999, collateral USDH, fees on close/settle, **no maker rebates on outcome books**, authorized-oracle settlement with optional challenge window, public API ~100 req/min `[S]`; KuCoin (data only). All are adapters behind legal/research gates |
| Resolution mechanics (Polymarket) | UMA optimistic oracle, US$750 bond, 2-hour challenge window, escalated disputes 48–72 h `[S]`; resolution-window and dispute state are risk-engine inputs |
| ASL | agents start as Rust/Python/WASM services on the hosted runtime; ASL port when ASL toolchain is ready (`vc-asl-hook`) |
| Engineering | strategy and risk code is built and independently verified by the factory under the same gates as the platform; research/backtests run in CI/cloud |
| Cost | run costs are a managed budget (Addendum H v2); never funded by trading P&L |

---
## 3. Context and scope
```
Market data ─► mas-data ─► strategy agents (hosted, WASM/microVM) ─► mas-selector ─► mas-risk (only approver) ─► vc-signer ─► Venue adapters
                               │ every host call ⇒ intent/outcome (C3)                                  │ intent before sign (C2)
                               └────────────────────────── VeriChain log ◄──────────────── mas-recon (fills, chain) ─► track-record packs
Operator console / kill ► mas-risk, vc-signer     Research harness ◄ history     Allocator (mas-allocator) ► budgets
```
In scope: data, strategy swarms, selector, risk, execution adapters, reconciliation, allocator, evidence and track-record packs, research harness, operator console, external-capital interfaces (gated). Out of scope until gates: customer funds custody (counsel), tokenized agent shares (L-2), cross-chain arbitrage.

---
## 4. Solution strategy
| Principle | Mechanism |
|---|---|
| Models propose, deterministic code disposes | forecasters emit probabilities only; sizing, limits, signing are deterministic and outside prompts (explicit belief→trade layer; evidence from a small recent study `[S]`) |
| Hosted + chokepoint evidence | agents run in the VeriChain runtime (C3); venue/wallet keys only in `vc-signer` (C2); neither agent code nor the operator can place an unevidenced order |
| Capability-bounded authority | operator → MAS root capability → swarm → agent; each link attenuates venue, market class, size, loss limits, expiry (`vc-cap`) |
| Prefer the supported edge | rewarded inventory-controlled liquidity provision; calibrated selective forecasting; the rest research-gated |
| Statistical honesty | experiment registry counts every trial; DSR/PBO; minimum track record ≈ (1.645/SR)² years (SR 1 → 2.7 y; SR 2 → 8 months; SR 3 → 4 months, i.i.d. normal, 95%) |
| Correlated risk grouped | limits per parent event/cluster (favorite–longshot sign flips when grouped by event `[S]`) |
| Capacity realism | small sizes, many markets; pro-rata rewards erode with competition |

---
## 5. Building block view
### 5.1 Components (level 1)
| Component | Responsibility | Runs as |
|---|---|---|
| `mas-data` | order books, trades, reward configs, market metadata (parent event, resolution rules, dispute flags), geoblock; archives | hosted service |
| `mas-research` | backtester/paper trader; calibration; DSR/PBO; markouts; experiment registry | CI / cloud batch |
| `mas-agents` | strategy agents emitting proposals | hosted agents (WASM or microVM) with delegated capabilities |
| `mas-forecast` | LLM/statistical forecasters; no keys; egress only to model APIs through `vc-hostio` | microVM-hosted |
| `mas-selector` | belief→trade: edge net of fees/spread/slippage; ranking; uncertainty haircut | hosted |
| `mas-risk` | deterministic risk engine; **only approver**; mirrors limits into signer policy | hosted, minimal, formally specified |
| `mas-exec` | order management, venue clients (adapters), rate limits, idempotency | hosted |
| `vc-signer` | policy-checked signing, holds keys | isolated host |
| `mas-recon` | reconciles venue fills/balances/chain → ledger; flags mismatches | hosted |
| `mas-allocator` (formerly Meta-Rebalancer) | budgets across strategies by shrinkage estimates with minimum samples; never trailing-20-trade Sharpe | hosted, human-approved changes |
| `mas-ops` | operator console, kill switch, alerts, daily signed report | console |
| `mas-pack` | track-record packs: reconciled P&L + evidence for an allocator/auditor | batch |
| `mas-venue-*` | venue adapters: `polymarket` (primary), `kalshi`, `hyperliquid-hip4`, data-only `kucoin` | hosted adapters, flag-gated |

### 5.2 Strategy catalog (parameters are placeholders)
| ID | Strategy | Status | Notes |
|---|---|---|---|
| LP-1 | Rewarded midpoint maker (primary hypothesis) | S0 | two-sided quotes near adjusted midpoint within max qualifying spread; reward score ∝ ((v−s)/v)²; both sides required outside 0.10–0.90 `[V]`; Avellaneda–Stoikov-style inventory skew (arXiv 2607.17991 `[S]`); never accumulate longshots <0.10 except as hedge or explicit net edge; pull/widen around news and resolution |
| LP-2 | Short-horizon crypto maker (successor to v6 "CMS") | research, restricted | **horizon ≥ 15 minutes only**: a 2026 preprint finds spot-price pushes near the close of 5-minute BTC contracts flip outcomes (winner 65% vs 41% normal in near-even cycles; ~1 in 300 traders flagged; manipulation largely absent in 15-minute contracts) `[S: arXiv 2606.31675]`, so 5-minute markets are an adversarial venue for makers; manipulation guard (spot-vs-venue divergence, abnormal late flow) pulls quotes; maker-only; taker legs forbidden; must pass markout tests |
| FC-2 | Domain-specialist forecaster verticals (crypto, macro, weather, politics, etc.) | research | claims of 15–30% annualized for specialization come from a blog with unverifiable cited data `[U]`; treated as hypothesis to test per vertical with calibration, not as expectation |
| SV-1 | Agent services (non-advisory data/compute/analytics sold via x402/AP2-style pay-per-call, using the platform) | research, legal review | monetizes capability rather than trading edge; no investment advice; per-service legal check |
| FC-1 | Selective calibrated forecaster | S0 | ensemble, calibrated (isotonic/Platt), trade only with net edge and low disagreement; ≤ quarter-Kelly of haircut edge; sports excluded initially |
| AR-1 | Complement/cross-venue scanner (successor to v6 "CPAS") | paper-only | net of fees and bridge cost; rule-equivalence tags; explicit leg risk |
| AR-2 | Combinatorial/rebalancing arbitrage scanner | paper-only | one study estimates ~US$40M realized on Polymarket `[S]`; competition and latency unmeasured |
| CR-1 | Carry/treasury (successor to v6 "SYS") | research, own legal review | delta-neutral funding carry with counterparty cap; off by default |
| MR-1 | Meta-allocation | with ≥2 live strategies | shrinkage; minimum samples; human approval for changes |
Evidence calibration `[V/S]`: Kalshi 2021–Apr 2025 (46,282 contracts): average return −9.64% for makers, −31.46% for takers; contracts ≤10¢ lose >60%; contracts >50¢ earn small positive returns (makers ≥50¢ ≈ +2.6%, high variance); bias weakened in 2025 — so **being a maker is not an edge by itself; edge comes from selection (side, price band, event type) and rewards act only as margin of safety**. Unverified popular claims kept out of expectations: 92% of Polymarket wallets zero/negative profit, 0.51% earn >US$1,000, arbitrage windows falling from 12.3 s (2024) to 2.7 s (2026), sub-100 ms bots capturing 73% `[U: blog citing Dune/Chainalysis]`.
Removed permanently as unsupported: closing-window taker sniping (edge ≈ 0 before fees `[V]` evidence: ≥90¢ buys +0.83¢/$ before fees), trailing-20-trade Sharpe allocation, guaranteed-profit language.

### 5.3 Risk engine rules (deterministic; evaluated before every approval and again in the signer)
Max gross exposure; per-event/cluster exposure; resolution-window and dispute-state rules (no new exposure inside the final window or on disputed/escalated markets without explicit net edge); manipulation guard for short-horizon markets; per-market inventory; Kelly ≤ 0.25 of haircut edge; minimum net edge > fee+spread+slippage+margin; daily loss limit; rolling drawdown pause; data-staleness cutoff; geoblock freshness; kill switch (operator or automatic on reconciliation mismatch); limit increases are delayed-apply with a logged rationale (M-operator-overreach control). Parameters are operator-approved placeholders until calibrated.

### 5.4 Delegated authority (links to `vc-cap`)
Root: operator signs a `delegation` granting MAS `venue.order` on named venues with max notional, loss cap, expiry. Swarm and agent delegations attenuate further. `vc-signer` accepts an order only if the chain verifies and `mas-risk` approval is attached; both are evidenced.

### 5.5 External-capital interfaces (Phase C, gated)
Allocator posts a job (ERC-8183-compatible escrow) or a mandate; MAS agent operates under a capability scoped to that mandate; evidence of every action is in the log; `mas-pack` produces reconciled track records; evaluator checks mandate compliance (limits respected, no orphan intents) before fee release. Not enabled until legal gates L-1/L-2 and fund-regulation review pass.

---
## 6. Runtime view
R1 LP-1 quote cycle (data → proposal → selector → risk → signer intent → place → ack → outcome → fills → recon). R2 FC-1 trade. R3 Kill (operator/automatic → risk freezes → exec cancels → signer halts → outcomes → incident pack). R4 Daily reconciliation (signed report; mismatch → R3). R5 Research run (register experiment; backtest with fees/rewards/slippage; DSR/PBO with total trials; promotion recorded as statement). R6 Capital change (limit increase proposed → rationale logged → delayed apply → signer policy updated). R7 Mandate lifecycle (Phase C: fund → operate → evaluate → settle).

---
## 7. Deployment view
Phases: research (offline) → paper (live data, simulated fills) → micro-live → live. Hosts: hosted runtime cluster (agents), isolated signer host (venue keys, HSM/KMS), data archive in object storage, small Postgres/SQLite ledger, console. Secrets: venue keys only in signer; model API keys only in `mas-forecast` via vault references. Observability: quote uptime, fill markouts, reward share, inventory, slippage, reconciliation breaks.

---
## 8. Cross-cutting concepts
Evidence for every proposal, approval, rejection (with reason), order, fill, kill, capital change (C3 for hosted effects; C2 for venue actions; A1 replay of risk logic with recorded model outputs). Honest metrics (venue/chain truth; fees and rewards included). Reproducibility (pinned snapshots, seeds, experiment registry). Security (least privilege; model sandbox; signer isolation; withdrawal limits). Legal gate for external capital, performance publication, and non-Polymarket venues.

---
## 9. Decisions
M01 own capital first; Phase C only after gates. M02 LP-1 primary hypothesis. M03 venue chokepoint via `vc-signer`. M04 models propose, risk disposes. M05 no taker closing-window strategy. M06 AR-1/AR-2 paper-only until leg risk and competition are measured. M07 USD (pUSD) accounting; no Lightning in MAS. M08 allocation by shrinkage with minimum samples. M09 stage gates with kill criteria. M10 ASL port later. M11 micro-live capital cap. M12 builder-fee terminal deferred pending counsel (now an L5 product, not a trading strategy). M13 AR-2 paper-only. **M14** agents run hosted on VeriChain (C3) rather than as external services once G7 lands; until then they run as external agents via signer (C2). **M15** external capital only via `vc-jobs` mandates with evidence-based evaluation. **M17** LP-2 limited to ≥15-minute horizons with a manipulation guard. **M18** strategy edge is judged net of selection-bias evidence (band/side/event type), not by "maker" status. **M19** AP2 mandates and x402 payments are recorded as statements referencing the mandate/payment hash; agent services (SV-1) never constitute investment advice. **M20** track-record packs export as SCITT receipts and as AAT sessions (`vc-aat`) for auditors. **M21** all MAS code is built and verified by the factory under ARC42 v9 Appendix G (independent verifier agents; human sign-off on `mas-risk` and `vc-signer` policy). **M16** all additional venues (Kalshi, HIP-4) are adapters behind legal/research gates and never share keys with the primary venue signer.

---
## 10. Quality requirements
### 10.1 Stage gates (per strategy; placeholders for operator approval)
| Stage | Entry | Exit | Kill |
|---|---|---|---|
| S0 Research | data pipeline and simulator validated vs known fills | DSR ≥ 0.95 at true trial count; PBO below threshold; positive after fees, rewards, slippage | no edge after costs |
| S1 Paper | S0 | ≥ 60 days; markouts and fill rates within tolerance of assumptions | slippage/markout materially worse |
| S2 Micro-live | S1; legal gate cleared | ≥ 90 days capped; clean reconciliation; lower CI bound of live Sharpe > 0 | drawdown limit; reconciliation break |
| S3 Scale | S2 | stepwise; re-test per step | edge decays with size |
| S4 External-mandate ready | S3 and verified track record meeting the minimum-length math; legal gates | counsel sign-off; evaluator tests pass | any compliance gap |
### 10.2 Scenarios
Q1 no order reaches venue without a registered intent: 0 escapes. Q2 kill: cancel-all ack < 5 s. Q3 stale-data guard. Q4 daily reconciliation, mismatch triggers kill. Q5 100% fills have intent+outcome. Q6 reproducible backtests. **Q7** hosted agent cannot reach the venue except through `vc-signer`. **Q8** a delegation widening limits is rejected by signer. **Q9** track-record pack verifies offline and reconciles to venue data within tolerance. **Q10** mandate evaluator rejects a run with a limit breach. **Q11** manipulation guard pulls LP-2 quotes within a bounded time in replayed manipulation episodes. **Q12** risk engine blocks new exposure on disputed/escalated markets in 100% of fixtures.

---
## 11. Risks and technical debt
No edge after costs (H: gates, learning cost). Reward programs change/competition (H: measure realized share). Adverse selection (H: markout monitoring, pulls). Resolution/dispute risk (M: exclude flagged markets, event caps). Key compromise (H: signer isolation, withdrawal limits). Regulatory (H: counsel, own capital first). Overfitting (H: registry, DSR/PBO, paper stage). LLM miscalibration (M). Operator overreach (M: delayed apply). Platform dependency (M: until G7, run as external agents via signer). External-capital regulation (H: gate S4). Venue concentration (M: adapters, per-venue caps). Settlement/price manipulation of short-horizon contracts (H: ≥15-minute only, guard, pulls). Oracle trust on HIP-4 and UMA dispute risk (H: adapter flagged research, event caps, exclusion rules).

---
## 12. Glossary
LP liquidity provision; markout price move after fill; parent event grouping of related contracts; quarter-Kelly 0.25×Kelly; DSR/PBO deflated Sharpe / probability of backtest overfitting; mandate scoped capital authority from an allocator; track-record pack reconciled P&L plus evidence; C2/C3 completeness levels (VeriChain §5.3.2).

## Appendix — Mapping to platform gates
MAS runs as external agents via signer after VeriChain G6; moves to hosted C3 after G7; external-capital mandates need G10 and S4. Details in ROADMAP MAS track T0–T6.
