# VeriChain — Roadmap

Rule: design gates first; no code before the G1 spec is approved by Damain.

## Design and build gates (platform; full scope per ARC42 v9 Appendix F)
| Gate | Scope | Done when |
|---|---|---|
| G0 | Repo reset, standard files, SOURCES | done |
| G0.5 | Engineering factory ready: roles, harness, CI gates, traceability, acceptance bundles (ARC42 v9 App. G) | **DONE 2026-10-24**: independent vector author + adversarial verifier found 15 defects (2 high); all fixed; differential vectors agree; bundle `factory/bundles/g05-l0-crates.json` |
| G1 | Spec freeze: schema, vectors, requirements, threat model | **FROZEN 2026-10-24 (owner approved; tag `g1-freeze`)**: `spec/statement.schema.json`, `spec/vectors/`, `spec/requirements.md`, `docs/THREAT_MODEL.md` |
| G2 | Tile log + `vcd` sequencer + checkpoints | crash/limits tests; <1 GB RAM |
| G3 | Witnesses + split-view detection | ≥2 independent witnesses |
| G4 | Anchors + COSE receipts + verifier CLI/WASM + evidence packs | offline verify; interop vector |
| G5 | L1: identity, delegation chains, KMS, PQ hybrid, rotation | Q7, Q12 |
| G6 | Gateway (C1) + signer (C2) + SDKs | Q5 |
| G7 | Hosted runtime: WASM sandbox, host ABI, vault, microVM backend | Q10, Q11 |
| G8 | Assurance: policy engine, replay, attestation (A3 per R2) | Q13 |
| G9 | BFT ordering + federation (R1, R3) | Q14, Q15 |
| G10 | Economy: payments, job escrow, reputation (legal L-1; market/token L-2/L-3) | Q16 |
| G11 | Governance, compliance, console, ops | Q17 |
| G12 | Hardening: external audit, game days, launch | Q18 |

## Commercial track (hypotheses; see addendum D)
| Step | Goal | Kill / continue |
|---|---|---|
| M0 | 10 design-partner conversations (banking/fintech, agent platforms, an insurer, an auditor) | ≥3 willing to pilot, or revisit thesis |
| M1 | Open-source spec + verifier published; reference vectors | external implementation verifies a receipt |
| M2 | First paid pilot of hosted log + evidence pack | signed pilot |
| M3 | Compliance mappings (EU AI Act Art. 12, NIST agent guidance, SEC/FINRA) | reviewed by a practitioner |
| M4 | Standards participation (SCITT agent-receipt profile) | draft or interop test accepted |

## Financial objectives v2 (assumptions; see addendum J §3)
| Horizon | Objective | Kill / pivot |
|---|---|---|
| M3 | ≥10 conversations, ≥3 willing to pilot | <3 → revisit segment |
| M6 | verifier published; ≥2 paid pilots; AARM mapping submitted; ≥1 external witness operator | 0 paid → thesis review |
| M9 | hosted runtime beta with ≥2 design partners | none → revisit runtime go-to-market |
| M12 | base ≈ US$500k ARR (evidence 250k + runtime 180k + packs 96k); stretch ≈ US$1.2M | <US$100k → change segment/mix |
| Trading | learning goals only; capital cap per MAS M11 | gate failure → stop strategy |

## MAS track — see `docs/arc42/MAS_ARC42_v4.md`
| Gate | Deliverable | Done when |
|---|---|---|
| T0 | Data pipeline + simulator; experiment registry; DSR/PBO tooling | reproducible backtest; validated against known fills |
| T1 | `mas-risk` + `mas-exec` + `vc-signer` integration (paper, evidence emitted) | 0 unevidenced orders; kill switch tested (needs platform G6) |
| T2 | LP-1 paper ≥ 60 days | markouts/fills within tolerance |
| T3 | Legal gate + micro-live ≥ 90 days | clean reconciliation; Sharpe CI lower bound > 0 |
| T4 | Scale in steps; add FC-1; move agents to hosted runtime (C3) | capacity tests; platform G7 |
| T5 | Research strategies (LP-2, AR-1, AR-2, CR-1) through S0–S2; additional venue adapters behind legal gates | per-strategy gates |
| T6 | External-capital mandates via jobs/marketplace | S4 + legal L-1/L-2 + platform G10 |
Rule: company revenue never depends on MAS P&L (Addendum H).
