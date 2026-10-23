> **SUPERSEDED 2026-10-23 by `MAS_ARC42_v3.md` (full-scope, production-grade).** Retained as the detailed L0/baseline reference.

# MAS — Multi-Agent Trading Systems on VeriChain — ARC42 v2 (design baseline for coding)

Status: DRAFT for approval · Date: 2026-10-23 · Supersedes `docs/archive/MAS_ARC42_v6.md`
Evidence base: Addenda B, F, G; claim status in `docs/SOURCES.md`.
**Not financial advice. No strategy below is promised to be profitable; the research says most participants lose.** All numeric parameters are placeholders to be calibrated and approved by the operator.

---
## 1. Introduction and goals

### 1.1 Purpose
MAS is the **reference tenant and dogfood** for VeriChain: automated agents that trade prediction markets with **the operator's own capital**, emitting VeriChain evidence for every decision so that the resulting track record is independently verifiable. It exists to (1) learn whether any systematic edge exists, (2) prove the evidence stack under real load, and (3) seed the "verified performance" evidence-pack product. **Company revenue must not depend on its P&L.**

### 1.2 Goals (in priority order)
1. **Survive** (no ruin; hard limits enforced outside any model).
2. **Prove** (every decision evidenced; PnL reconciled from venue/chain data, never self-reported).
3. **Learn** (statistically honest evaluation; stage gates; kill criteria).
4. **Earn** (only after 1–3, from the best-supported edges).

### 1.3 Non-goals
Managing other people's money; marketing returns; "guaranteed" arbitrage claims; closing-window taker sniping; Sharpe targets set before data; trading jurisdictions or venues not cleared by the operator.

### 1.4 Stakeholders
Operator (Damain, sole risk-capital provider); VeriChain (evidence consumer); future auditors/allocators/marketplaces (evidence readers, not customers of trading); counsel (legal gate).

---
## 2. Constraints
| Constraint | Consequence |
|---|---|
| Own capital only (placeholder cap set by operator) | no pooled funds, no performance marketing without counsel |
| Venue: Polymarket CLOB V2 (collateral pUSD on Polygon, backed by USDC) | capital and limits denominated in USD terms (pUSD), not sats; no Lightning dependency |
| Jurisdiction | startup check against Polymarket geoblock endpoint; T&T not listed on the geoblock page (Addendum F) — **not** legal clearance; counsel review before scaling |
| Kalshi | disabled in v2 (fiat settlement, US-person/jurisdiction questions unassessed) |
| ASL v0.2.0 incomplete | agents are ordinary Rust/Python services using VeriChain gateway/signer (C1/C2), ASL port later |
| Dev machine 4 GB Chromebook | research/backtests run in CI or small cloud box; local runs use small samples |
| Free-credit "2-week window" from v6 | **removed**; run costs are budgeted, not funded by trading |

---
## 3. Context and scope
```
 Market data (Polymarket WS/REST) ─► Data service ─► Strategy agents ─► Selector (belief→trade) ─► Risk engine ─► Signer ─► Venue
                                          │                 │                  │                    │            │
                                          └─────────────────┴──────── intent/outcome statements ─► VeriChain log ◄── Reconciler (fills, chain)
 Operator console (limits, kill switch) ──► Risk engine, Signer
 Research harness (backtest/paper) ◄── historical data, resolved markets
```
In scope: data ingestion, strategy agents, risk engine, execution/signing, reconciliation, evidence emission, research harness, operator console. Out of scope: customer funds, tokenization, cross-chain bridging, Kalshi, Hyperliquid, KuCoin (optional phase 3).

---
## 4. Solution strategy
| Principle | Mechanism | Evidence |
|---|---|---|
| Models propose, deterministic code disposes | LLM/statistical forecasters can only emit probabilities; sizing, limits and signing are deterministic and outside the prompt | arXiv 2607.03015: explicit trading layer (selection + quarter-Kelly + limits outside the LLM) was the only profitable policy in a small replay |
| Chokepoint evidence | all venue orders must be signed by `vc-signer`; signer emits an `intent` receipt **before** signing and an `outcome` after fill/ack ⇒ no unevidenced order can reach the venue (C2 for venue actions) | VeriChain ARC42 v7 §6 S2 |
| Prefer the supported edge | rewarded, inventory-controlled liquidity provision skewed against buying longshots; forecasting only where calibrated and net-of-fee edge is shown | Addendum F §2 |
| Statistical honesty | DSR/PBO controls, minimum sample sizes, no allocation on short windows | Addendum F §3 |
| Group correlated risk | limits apply per parent event / cluster, not per contract | favorite–longshot result flips sign when grouped by event (arXiv 2609.12878) |
| Capacity realism | small sizes, many markets; no assumption of scale | pro-rata rewards; thin gaps |

---
## 5. Building block view

### 5.1 Level 1
| Component | Responsibility |
|---|---|
| `mas-data` | venue market data (order books, trades, rewards config), market metadata (parent event, resolution rules, dispute flags), geoblock check; normalized store |
| `mas-research` | backtester/paper-trader; calibration; DSR/PBO; markouts; experiment registry (counts every trial) |
| `mas-agents` | strategy agents (LP-1, FC-1, AR-1; CR-1 later) emitting **proposals** (quotes/orders with edge estimate and rationale) |
| `mas-selector` | belief→trade layer: edge net of fees/spread/slippage, ranking by time-normalized expected return, uncertainty haircut |
| `mas-risk` | deterministic risk engine: limits, Kelly cap, exposure grouping, drawdown/kill rules; **only component allowed to approve orders** |
| `mas-exec` | order management (place/cancel/replace), venue client (CLOB V2), rate limiting, nonce/idempotency |
| `vc-signer` (VeriChain) | policy-checked signing; intent/outcome receipts; holds the venue key |
| `mas-recon` | reconciles venue fills, balances, on-chain settlement → PnL ledger; flags mismatches |
| `mas-ops` | operator console: limits, pause, kill switch, alerts, daily report |
| `mas-forecast` (sandboxed) | LLM/statistical forecasters: no keys, no network except model APIs, no execution rights |

### 5.2 Strategy specifications (parameters are placeholders)
**LP-1 Rewarded midpoint maker (primary hypothesis)**
- Market selection: markets with active liquidity rewards (check reward config is non-empty), acceptable parent-event risk, no unresolved-dispute flag, time to resolution above threshold, rewards-per-risk estimate positive after estimated adverse selection (from markouts).
- Quoting: two-sided quotes within the market's max qualifying spread, at least minimum size; near the adjusted midpoint (reward score ∝ ((v−s)/v)²); both sides required when midpoint < 0.10 or > 0.90.
- Inventory control: Avellaneda–Stoikov-style skew by inventory and time to resolution; widen near 0.5 close to settlement (arXiv 2607.17991); hard inventory caps per market and per event.
- Longshot policy: do not accumulate long positions in contracts priced < 0.10 (the empirically worst side to buy); allow acquiring them only as hedged leg or at explicit net-edge.
- Event risk: pull/widen quotes around scheduled news and in the final window before resolution (threshold calibrated from markouts).
- Expected P&L sources: reward share + spread capture − adverse selection − inventory risk. **Reward competition is pro-rata; assume it erodes.**

**FC-1 Selective forecaster**
- Probabilities from an ensemble (base rates, statistical models, LLM forecasters), calibrated (isotonic/Platt) on resolved history; track Brier and log score vs the market continuously.
- Trade only if calibrated edge net of fee/spread/slippage exceeds threshold and model disagreement is below limit; size ≤ quarter-Kelly of the *haircut* edge, then capped by `mas-risk`.
- Never buy longshots on LLM conviction alone; require calibration evidence for the market class (crypto/politics; sports excluded initially because the bias evidence does not hold there).

**AR-1 Opportunistic complement/cross-venue scanner (paper only until proven)**
- Detect YES+NO complement mispricing and cross-venue gaps **net of all fees and bridge costs**; require rule-equivalence tag for any cross-venue pair; model leg risk explicitly (partial fills, one-sided exposure). Execute live only if paper fill data shows positive net expectancy after leg losses. Practitioner data suggest typical gaps of 2–5¢ after fees closing within hours (anecdotal).

**CR-1 Carry treasury (optional, phase 3)**
- Delta-neutral funding carry as treasury yield with a hard counterparty cap; off by default; separate legal/risk review. Historical yields in the 4–30% range, compressed to high single digits in Q2 2026 (single source), can go negative.

Removed from v1: closing-window taker strategy; "guaranteed profit" language; cross-chain arbitrage; Kalshi; capital split by trailing-20-trade Sharpe.

### 5.3 Risk engine rules (deterministic, evaluated before every approval)
| Rule | Default placeholder |
|---|---|
| Max gross exposure | X% of risk capital (operator sets) |
| Max exposure per parent event / correlation cluster | Y% |
| Max per-market inventory | Z contracts / % |
| Kelly fraction | ≤ 0.25 of edge after uncertainty haircut |
| Minimum net edge to open (FC-1) | > fee + spread + slippage estimate + margin |
| Daily loss limit | stop all strategies for the day |
| Drawdown limit (rolling) | pause strategy; human review to resume |
| Data staleness | no orders if book/feed older than T seconds |
| Geoblock | no orders if geoblock check fails or is stale |
| Kill switch | operator or automatic (reconciliation mismatch beyond tolerance) cancels all orders and halts signer |

Reward-competition caveat: LP-1 rewards are pro-rata by quoted size and spread score, so returns shrink as quoters join; treat reward income as an upside to be measured, not assumed.

### 5.4 Capital allocation
Fixed per-strategy risk budgets at start (placeholder split, e.g., LP-1 majority, FC-1 minority, AR-1 zero until paper-proven). Reallocation uses shrinkage estimates with minimum sample sizes and **never** a trailing-20-trade metric. Minimum track-record math (i.i.d. normal, 95% confidence true Sharpe > 0): ≈ (1.645/SR)² years: SR 1 → ~2.7 y; SR 2 → ~8 months; SR 3 → ~4 months (Addendum F §3). Fat tails and many trials lengthen this.

---
## 6. Runtime view
**R1 — LP-1 quote cycle:** data update → LP agent proposes bid/ask with size → selector checks reward-adjusted EV → risk approves (limits, staleness, geoblock) → signer emits `intent`, signs → exec places/replaces → venue ack → signer emits `outcome` (order id) → fills arrive → recon updates inventory/PnL → `outcome` for fills.
**R2 — FC-1 trade:** forecaster outputs probability + uncertainty → calibration → selector computes net edge and rank → quarter-Kelly size → risk approval → same signer path.
**R3 — Kill:** operator/automatic trigger → `mas-risk` freezes approvals → `mas-exec` cancels open orders → signer halts → `outcome` statements logged → incident note.
**R4 — Daily reconciliation:** compare internal positions/PnL with venue and chain; mismatches above tolerance trigger R3; produce signed daily report (a VeriChain statement) for the evidence pack.
**R5 — Research run:** register experiment (increments trial count) → backtest with realistic fills/fees/rewards → compute DSR/PBO using total trials → promotion decision recorded as a statement.

---
## 7. Deployment view
- One small VM (or container host) for `mas-data/agents/selector/risk/exec/recon`, a second isolated host (or process sandbox) for `vc-signer` with the venue key, VeriChain endpoint (hosted or self-hosted), object storage for market-data archives, a small Postgres/SQLite for ledger.
- Secrets: venue key only in `vc-signer`; agents never see it; model API keys only in `mas-forecast`.
- Observability: metrics (quote uptime, fill markouts, reward share, inventory, slippage), alerts to operator.
- Environments: research (offline), paper (live data, simulated fills), micro-live (capped), live.

---
## 8. Cross-cutting concepts
- **Evidence:** every proposal, approval, rejection (with reason), order, fill and kill is a statement; completeness C2 for venue actions; assurance A1 where deterministic replay of risk logic with recorded model outputs is implemented.
- **Honest metrics:** PnL from venue/chain; fees and rewards included; slippage measured; no post-hoc edits (log is append-only).
- **Reproducibility:** pinned data snapshots, seeded simulators, experiment registry.
- **Security:** least privilege; signer policy mirrors risk limits; two-person-style confirmation for limit increases (operator + delayed apply).
- **Legal gate:** own capital; counsel review before external capital or publishing performance; document jurisdictions.

---
## 9. Decisions
| ID | Decision | Why |
|---|---|---|
| M01 | Trade own capital only | avoids fund/advisory regulation; evidence product proves itself first |
| M02 | LP-1 is the primary hypothesis | best-supported structural source: maker edge (Kalshi, Telonex), longshot bias, rewards formula, optimal-quoting results |
| M03 | Venue chokepoint via `vc-signer` | exchange accepts only signer-signed orders ⇒ completeness by construction without ASL |
| M04 | Models propose, risk disposes | evidence for explicit trading layer; safety |
| M05 | No taker closing-window strategy | edge ≈ 0–1¢ before fees; taker fees dominate |
| M06 | AR-1 paper-only until leg-risk is measured | "guaranteed" arb is not guaranteed |
| M07 | USD-denominated (pUSD) accounting; no Lightning in MAS | venue collateral is USDC-backed; removes unspecified bridge |
| M08 | Capital allocation by shrinkage with minimum samples | short-window Sharpe is noise |
| M09 | Stage gates with kill criteria | avoid self-deception |
| M10 | ASL port later | ASL v0.2.0 incomplete |
| M11 | Micro-live capital cap: lesser of 5% of liquid risk capital or US$5,000; objectives are learning, not return | trading cannot fund the company (Addendum H §1) |
| M12 | Builder-fee trading terminal deferred until counsel review and paying evidence customers | custody/licensing/jurisdiction risk (Addendum H P7) |
| M13 | AR-2 combinatorial/rebalancing arbitrage scanner: paper-only | opportunity documented in one study; leg risk and competition unmeasured |

---
## 10. Quality and acceptance requirements

### 10.1 Stage gates (placeholder thresholds — operator to approve)
| Stage | Entry | Exit to next | Kill |
|---|---|---|---|
| S0 Research | data pipeline + simulator validated against known fills | strategy passes DSR ≥ 0.95 *at the true trial count* and PBO below chosen threshold; positive after fees, rewards, slippage | no edge after costs |
| S1 Paper (live data) | S0 passed | ≥ 60 days; realized markouts and fill rates within tolerance of backtest assumptions | slippage/markout materially worse than assumed |
| S2 Micro-live | S1 passed; legal gate cleared | ≥ 90 days at capped size; reconciliation clean; lower bound of live Sharpe CI > 0 | drawdown limit; reconciliation break |
| S3 Scale | S2 passed | scale in steps; re-test per step (capacity) | edge decays with size |

### 10.2 System quality scenarios
| ID | Scenario | Target |
|---|---|---|
| Q1 | No order reaches venue without a registered intent | 0 escapes in tests |
| Q2 | Kill switch halts all orders | < 5 s to cancel-all ack |
| Q3 | Stale data guard | no orders if feed older than T |
| Q4 | Reconciliation | daily; mismatch above tolerance triggers kill |
| Q5 | Evidence completeness | 100% of fills have intent+outcome pairs; orphans reported |
| Q6 | Reproducible backtest | same data + seed ⇒ identical PnL |

---
## 11. Risks and technical debt
| Risk | Severity | Mitigation |
|---|---|---|
| No edge exists after costs | H | stage gates; kill criteria; treat as learning cost |
| Reward programs change or competition erodes pro-rata share | H | measure realized reward share; make rewards optional input |
| Adverse selection around news/resolution | H | markout monitoring; quote pulling; inventory caps |
| Resolution/dispute risk (UMA) | M | exclude flagged markets; event-level caps |
| Venue/key compromise | H | signer isolation; withdrawal limits; alerts |
| Regulatory/jurisdiction (venue ToS, local law, tax) | H | counsel review; geoblock check; own capital only |
| Overfitting/backtest bias | H | experiment registry, DSR/PBO, paper stage |
| LLM forecaster miscalibration | M | calibration monitoring; capped sizing; ensemble |
| Operator overreach (raising limits impulsively) | M | delayed-apply limit increases; written rationale statement |

---
## 12. Glossary
**LP** liquidity provision. **Markout** price move after a fill, measuring adverse selection. **Parent event** grouping of related contracts. **Quarter-Kelly** 0.25 × Kelly fraction. **DSR/PBO** Deflated Sharpe Ratio / Probability of Backtest Overfitting. **pUSD** Polymarket USD collateral token backed by USDC. **Intent/outcome** VeriChain statement kinds.
