# Addendum B — Repo provenance and ARC42 review (VeriChain + MAS)

Date: 2026-10-22 · Status: DRAFT · Verified = opened primary source this session

## 1. Provenance ("foul play?")
- Verified: `origin/main` has 64 commits, all authored `DamainRamsajan <dpramsajan@gmail.com>`, 2026-05-06 → 2026-05-19, none GPG-signed. GitHub shows 64 commits, not a fork, 0 stars/forks.
- Verified: the history is the ASL project's (v15.2 toolchain, npm @asl-lang/cli, landing page, language book).
- Verified: all four VeriChain/MAS docs (`VERICHAIN_ARC42.md`, `verichain_v6_ARCHITECTURE.md`, `verichain_v6_TASKLIST.md`, `MAS_ARC42_ARCHITECTURE.md`) arrived together in one commit, 546743e, 2026-05-19 19:01 -0400 ("Replaced md files"). ADR-001 itself says VeriChain scaffolds inside the ASL repo, so this fits a local ASL folder pushed to the wrong remote.
- Not provable from here: who pushed. Author fields are forgeable. Check GitHub user Security log, org Audit log, deploy keys, tokens/authorized apps, and `git remote -v` in your local ASL folder.
- Correction: my earlier "single flattened commit" claim was wrong (shallow clone).

## 2. VeriChain ARC42: two copies
`VERICHAIN_ARC42.md` = cleaned copy (adds C4 context diagram, no duplicated Batch 2 block). `verichain_v6_ARCHITECTURE.md` = same content with Batch 2 duplicated. Treat ARC42 as canonical, archive v6. Both carry the same "Blueprint Integrity Hash" (a patterned UUID, not a computed hash).

## 3. MAS ARC42 findings
1. Sections 2.2 (domain model) and 4.1–4.3 (runtime scenarios) are empty (diagrams lost); several `$` amounts are garbled.
2. Closing-window maker strategy (buy at $0.90–0.95 with "~85% certainty"): EV per share = p − price. At p=0.85, price=0.925 → −0.075 (≈ −8%) before fees; positive only if p > price. Fills also suffer adverse selection.
3. Settlement contradiction: ADR-007 says Lightning-only because USDC is freezable; Polymarket V2 collateral is pUSD, an ERC-20 on Polygon backed by USDC (Polymarket docs). Charters are in sats; no BTC↔pUSD bridge is specified. ADR-S05 flags only Kalshi.
4. Fees: Polymarket added taker-only fees to 15-minute crypto markets (Jan 2026, up to ~3%, highest near 50%); makers pay none and receive daily rebates. The doc's maker/arb economics need this fee model.
5. Evidence from outside: one-week on-chain study (Feb 2–8 2026, 46,945 wallets): 36.8% profitable, median PnL −$3, makers net +$728k, top 5 wallets $811k; PnL pre-fee, single week.
6. "Guaranteed" arb (Gabagool, cross-platform): two orders on a CLOB are not atomic (legging risk); position caps of $20–$50 make profits tiny relative to costs.
7. Statistics: Sharpe ≥3 and rebalancing from a trailing 20-trade window is noise, not signal.
8. "Every trade NANOZK-proven" is infeasible for API-calling agents (see Addendum A, G2).
9. All agents need ASL v0.2.0 charters and speculation; the ASL code in this repo has no charter or speculation support.
10. Unresearched: jurisdictional access to Polymarket/Kalshi and trading-agent regulation.

## 4. Relationship and recommendation
MAS is a tenant of VeriChain (uses market-data-ingestor, nanozk-prover, dp-store, lightning-adapter, governance). Keep VeriChain neutral; move MAS to its own repo as a later "first customer". Do not delete this repo yet: archive it, then create a fresh VeriChain repo from ARC42 with the four standard files.

## Sources
- https://docs.polymarket.com/v2-migration
- https://www.theblock.co/post/384461/polymarket-adds-taker-fees-to-15-minute-crypto-markets-to-fund-liquidity-rebates
- https://telonex.io/research/top-crypto-traders-polymarket-15m
