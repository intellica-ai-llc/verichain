# Addendum F — Trading: academic domains and what the evidence says about making money

Date: 2026-10-23 · Status: DRAFT · Tags: **[V]** = source opened this session · **[K]** = established literature from my knowledge, not re-opened · **[J]** = my judgement.
Not financial advice. Nothing here is a promise of returns; the evidence below says most participants lose.

## 1. Academic domains to cover (and why each matters)
| Domain | What it gives us | Status |
|---|---|---|
| Market microstructure & optimal market making (Glosten–Milgrom adverse selection, Kyle, **Avellaneda–Stoikov** inventory-aware quoting) | how to quote, skew and size; why spreads exist | [K]; prediction-market HJB model **[V]** (arXiv 2607.17991) |
| Prediction-market economics (favorite–longshot bias, maker/taker returns, mechanism design: CLOB vs LMSR, Hanson) | where persistent edge may exist | **[V]** arXiv 2609.12878, Kalshi "Makers and Takers" (CEPR DP20631) |
| Forecasting science (proper scoring rules, calibration, aggregation/extremizing, base rates) | turning beliefs into calibrated probabilities | [K]; LLM forecasting benchmarks **[V]** Prophet Arena |
| Bet sizing & risk (Kelly, fractional Kelly, CVaR, drawdown control) | survival; avoids ruin | [K]; quarter-Kelly layer **[V]** arXiv 2607.03015 |
| Statistical inference in finance (multiple testing, **Deflated Sharpe Ratio**, Probability of Backtest Overfitting, minimum track-record length) | knowing whether a result is luck | DSR **[V]** (SSRN 2460551); PBO [K] |
| Optimal execution & market impact (Almgren–Chriss) | slippage, order splitting | [K] |
| Carry / basis trading and derivatives (funding rates, perp–spot basis, liquidation, counterparty risk) | treasury yield | **[V]** Ethena/sUSDe article (single source) |
| Information economics & oracle design (UMA disputes, resolution-rule risk) | settlement risk | [K]; partly **[V]** (simplefunctions) |
| Online learning / bandits / conformal prediction | adapting allocations without overfitting | [K] |
| Cryptoeconomics & MEV | on-chain execution risk | [K] |
| Law & regulation (access/jurisdiction, derivatives law, fund regulation) | whether you may trade, and with whose money | geoblock **[V]**; rest unresearched |

## 2. Evidence by strategy family
### 2.1 Favorite–longshot bias (the best-documented structural edge)
- **[V]** Polymarket, 588M trades by 2.48M accounts: purchases below 10¢ lose **19.3¢ per dollar**; purchases at or above 90¢ earn **+0.83¢ per dollar** (before fees; paper does not address fees). Pattern holds in Crypto and Politics, absent in Sports. Sensitive to aggregation: longshots lose 6.3¢/$ when contracts are weighted equally but *gain* 4.1¢ when grouped by parent event.
- **[V]** Kalshi (300k+ contracts): low-price contracts win far less often than break-even after fees; **makers earn higher returns than takers**; returns show the same pattern. The paper's exact magnitudes were not visible on the abstract page.
- **Implication [J]:** the supported direction is *be the counterparty to longshot buyers* (sell/offer longshots, post liquidity) and *avoid being the longshot buyer*. The edge in ≥90¢ favorites is under 1¢ per dollar before fees, so the "buy 0.90–0.95 at closing" idea in MAS v1 is at best a razor-thin edge; with Polymarket's taker fees on 15-minute crypto markets (up to ~3%, Addendum B) it is likely negative for takers. My earlier −8% figure (Addendum B) assumed the doc's 85% certainty; the empirical calibration is much closer to the price, so the correct statement is: *edge ≈ 0 to 1¢ before fees, fees dominate, so do not trade it as a taker.*

### 2.2 Market making and incentives
- **[V]** Optimal market-making model for binary contracts (arXiv 2607.17991): optimal spread and skew depend on inventory, price and time to resolution; spreads widen near 0.5 close to settlement. In the authors' Monte Carlo (illustrative, uncalibrated parameters) the optimal policy left mean PnL almost unchanged (12.39 vs 12.47) but cut PnL standard deviation 28.11 → 10.34 and 5% VaR 32.41 → 4.20. **Lesson: inventory/risk control is the product; it does not by itself create alpha.**
- **[V]** Polymarket liquidity rewards: only incentivized markets pay; score per order S = ((v−s)/v)² · b (v = max qualifying spread, s = distance from adjusted midpoint), so quotes near the midpoint score quadratically more; two-sided quoting is required when the midpoint is below 0.10 or above 0.90 (single-sided counts at 1/3 between); book sampled once per minute (up to 1,440 samples/day); payouts are pro rata among makers and paid daily at midnight UTC, minimum $1. Makers pay no trading fees; taker fees on 15-minute crypto markets fund rebates (The Block, Addendum B).
- **[V]** Telonex (one week, Feb 2026, pre-fee): makers net +$728,501 while takers lost overall; only 36.8% of wallets profitable; top 5 wallets $811k.
- **Implication [J]:** the most evidence-supported *systematic* source of return is **liquidity provision with inventory control, quoting near the midpoint in rewarded markets, skewed to avoid buying longshots.** Returns are competitive (pro-rata rewards), capacity-limited, and exposed to adverse selection around news and resolution.

### 2.3 Cross-venue and intra-venue arbitrage
- **[V, anecdotal]** One practitioner log (≈30 pairs daily, ≈200 widening events over six months, unpublished): ~80% of pairs trade within 2–5¢ after fees; ~80% of gaps wider than 5¢ close within an hour, ~95% within four hours; Kalshi fee ≈7% of profit on winners (stated as changing); resolution rules differ (Kalshi CFTC-vetted, Polymarket looser; UMA dispute risk).
- **Implication [J]:** opportunities are real but short-lived and thin; "guaranteed profit" is false when legs execute separately, rules differ, and fees move. Treat as an opportunistic scanner with strict rule-equivalence checks, small size, and a net-of-fees threshold, not as a core strategy.

### 2.4 Forecast-driven directional trading
- **[V]** Prophet Arena (UChicago SIGMA Lab; launched Aug 2025): LLMs reported roughly on par with markets on live events; simulated (not real-money) returns; "early" results, few details (Decrypt summary; not the paper).
- **[V]** arXiv 2607.03015: good probability scores often fail to produce profits; adding an explicit deterministic layer (rank by time-normalized expected return, **quarter-Kelly sizing, stake/exposure/stop-loss/drawdown limits enforced outside the LLM prompt**) was the only policy with positive ROI in a controlled replay (+15.9% ROI, stake-weighted Sharpe 0.42, **42 traded rows**; on a 132-row fused replay +5.9% with an interval including zero). Small sample; do not extrapolate.
- **Implication [J]:** the research direction that fits VeriChain/ASL: *risk limits outside the model*, with every decision evidenced. Edge from LLM forecasts is unproven.

### 2.5 Funding-rate carry
- **[V, single article]** sUSDe-style delta-neutral yield: compressed to high single digits in Q2 2026; 2024–2025 realized roughly 4–30%, mostly 8–18%; can turn negative; risks: exchange failure, depeg, slashing, contract bugs, 7-day unstake.
- **Implication [J]:** a *treasury yield with counterparty risk*, not alpha; size by exchange-failure loss tolerance.

## 3. Statistics: how much track record is enough? (derived, [K] math)
For roughly i.i.d. normal returns, the track record needed for 95% confidence that the true Sharpe is above zero is about (1.645 / SR)² years (SR annualized). SR = 1 → ~2.7 years; SR = 2 → ~0.7 years (~8 months); SR = 3 → ~0.3 years (~4 months). Fat tails, autocorrelation and many trials lengthen this (Deflated Sharpe). Consequences: a **20-trade trailing Sharpe (MAS v1 rebalancer) is statistical noise**, and "Sharpe ≥ 3" claims need months of live data before they mean anything.

## 4. Legal and access facts
- **[V]** Polymarket geoblock: fully blocked (no opening or closing): Iran, Syria, Cuba, North Korea and parts of Ukraine; close-only (frontend and API) for the US, UK, France, Germany, Italy, Australia, Singapore, Thailand, Taiwan and others; orders from blocked regions are rejected. **Trinidad and Tobago is not listed.** Terms of service beyond the geoblock page, tax, and the legality of *your* activity were not researched.
- **[J]** Trading only **own capital**. Managing others' money or selling "verified returns" to allocators raises fund/advisory regulation questions: needs counsel before any external capital or marketing of performance.

## 5. What this means for "making more money trading"
1. No source found supports exponential trading returns. Supported edges are **small, capacity-limited and competitive**: (a) rewarded, inventory-controlled liquidity provision skewed against longshot buying; (b) niche calibrated forecasting with strict sizing; (c) carry as treasury.
2. Scale comes from **breadth and automation** (many markets, tight risk control, capital efficiency), not from one clever strategy.
3. **Survival dominates**: ruin avoidance (fractional Kelly, exposure grouped by parent event, hard limits outside the model) matters more than extra alpha.
4. Treat trading as **dogfood and proof** for VeriChain's verified track records; company revenue must not depend on it.
5. Gate every strategy: backtest with DSR/PBO controls → paper → micro-live → scale, with kill criteria (MAS v2).

## Sources
- https://arxiv.org/abs/2609.12878 · https://cepr.org/publications/dp20631 · https://docs.polymarket.com/programs/liquidity-rewards
- https://arxiv.org/abs/2607.17991 · https://arxiv.org/abs/2607.03015 · https://decrypt.co/336298/ai-now-matches-prediction-markets-in-forecasting-real-events-study-finds
- https://docs.polymarket.com/api-reference/geoblock · https://simplefunctions.dev/concepts/cross-venue-convergence · https://eco.com/support/en/articles/15254002-ethena-usde-and-susde-2026-delta-neutral-yield
- https://papers.ssrn.com/abstract=2460551 · https://telonex.io/research/top-crypto-traders-polymarket-15m
