# VeriChain — Roadmap

Rule: design gates first; no code before the G1 spec is approved by Damain.

## Design and build gates
| Gate | Deliverable | Done when |
|---|---|---|
| G0 | Repo reset, four standard files, `docs/SOURCES.md` triaging every cited claim | claims marked verified / unverified / refuted |
| G1 | ARC42 v7 and MAS v2 approved; Decision Primitive v2 schema frozen; test vectors (see ARC42 v7 Appendix A) written | Damain approves |
| G2 | Minimal Rust tile log: checkpoint, inclusion and consistency proofs | passes vectors; runs in <1 GB RAM on a 4 GB Chromebook |
| G3 | Witness cosigning + split-view detection test | ≥2 independent witnesses |
| G4 | COSE receipts + RFC 3161 / OpenTimestamps anchoring + verifier CLI | verifier passes vectors offline |
| G5 | MCP/A2A gateway adapter (C1) and ASL `prov!`/discharge hook (C2) | end-to-end demo |
| G6 | PQ hybrid checkpoint signing, rotation test, evidence renewal | rotation test passes |

## Commercial track (hypotheses; see addendum D)
| Step | Goal | Kill / continue |
|---|---|---|
| M0 | 10 design-partner conversations (banking/fintech, agent platforms, an insurer, an auditor) | ≥3 willing to pilot, or revisit thesis |
| M1 | Open-source spec + verifier published; reference vectors | external implementation verifies a receipt |
| M2 | First paid pilot of hosted log + evidence pack | signed pilot |
| M3 | Compliance mappings (EU AI Act Art. 12, NIST agent guidance, SEC/FINRA) | reviewed by a practitioner |
| M4 | Standards participation (SCITT agent-receipt profile) | draft or interop test accepted |

## Financial objectives (assumptions; see addendum H)
| Horizon | Objective | Kill / pivot |
|---|---|---|
| M3 | ≥10 conversations, ≥3 willing to pilot | <3 → revisit segment |
| M6 | verifier published; ≥2 paid pilots | 0 paid → thesis review |
| M12 | ~US$250k ARR base (≈10 × US$25k); stretch ≈ US$600k | <US$60k → change segment or stop |
| Trading | learning goals only; capital cap min(5%, US$5k) | gate failure → stop strategy |

## Deferred
ORCHID consensus, UBE/World ID, agent exchange, Lightning settlement, trading swarm.

## MAS (trading) track — see `docs/arc42/MAS_ARC42_v2.md`
| Gate | Deliverable | Done when |
|---|---|---|
| T0 | Data pipeline + simulator; experiment registry; DSR/PBO tooling | reproducible backtest; validated against known fills |
| T1 | `mas-risk` + `mas-exec` + `vc-signer` integration (paper trading, evidence emitted) | 0 unevidenced orders; kill switch tested |
| T2 | LP-1 paper ≥ 60 days | markouts/fills within tolerance |
| T3 | Legal gate + micro-live ≥ 90 days | clean reconciliation; Sharpe CI lower bound > 0 |
| T4 | Scale in steps / add FC-1 | capacity tests pass |
Rule: company revenue never depends on MAS P&L.
