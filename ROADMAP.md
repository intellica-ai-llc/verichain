# VeriChain — Roadmap

Rule: design gates first; no code before the G1 spec is approved by Damain.

## Design and build gates
| Gate | Deliverable | Done when |
|---|---|---|
| G0 | Repo reset, four standard files, `docs/SOURCES.md` triaging every cited claim | claims marked verified / unverified / refuted |
| G1 | Decision Primitive v2 + SCITT profile + completeness/assurance definitions + test vectors | Damain approves |
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

## Deferred
ORCHID consensus, UBE/World ID, agent exchange, Lightning settlement, trading swarm.
