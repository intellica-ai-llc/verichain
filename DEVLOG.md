# VeriChain — Devlog

## 2026-10-23
- Repo reset to a clean design skeleton. Previous `main` preserved at branch `archive/pre-reset-2026-10-23`. The ASL language repo is separate (`agentseedlanguage-cpu/agentseed`).
- Decisions (Damain): v1 = evidence layer; accept a weaker gateway tier for non-ASL agents; completeness-by-construction as headline; trading swarm decoupled from revenue; ORCHID to research track.
- Added addenda A (gap analysis), B (provenance + ARC42/MAS review), C (frontier and differentiation), D (revenue and flywheel).

## 2026-10-22
- Reviewed repo, found it was an ASL copy plus one docs commit (all authored by DamainRamsajan; no tampering evidence). Verified ORCHID and NanoZK sources: single-author preprints, claims weaker than cited in v6. Checked Polymarket V2/fees and market data against the swarm ARC42.

## 2026-10-23 (later)
- Reset applied to `main` per Damain's approval: ASL copy and v6 files removed (old tree kept at `archive/pre-reset-2026-10-23`; v6 ARC42, MAS ARC42 and v6 task list copied to `docs/archive/`).
- Added addendum E (pricing signals, bank-regulation evidence, proposed targets) and `docs/SOURCES.md` (first claim triage).
- Open: replace targets in E with Damain's real financial objectives; run design-partner conversations (ROADMAP M0).

## 2026-10-23 (spec baseline)
- Research F (trading: academic domains + evidence) and G (go-to-market answers) added.
- Wrote `docs/arc42/VERICHAIN_ARC42_v7.md` and `docs/arc42/MAS_ARC42_v2.md`. Key changes vs v6: no own consensus; SCITT/tile-log/witness design; intent/outcome completeness; signer chokepoint (C2 without ASL); MAS rebuilt around maker/inventory control, forecast layer with limits outside the model, stage gates; trading removed from revenue assumptions.
- Correction to Addendum B: the empirical favorite–longshot study shows ≥90¢ purchases earned ≈ +0.83¢/$ before fees, so the closing-window taker idea is edge ≈ 0–1¢ before fees (fees dominate), not the −8% I computed from the v6 doc's assumed 85% certainty.
- Next: Damain reviews both specs; then G1 (freeze schema, write vectors), then code.

## 2026-10-23 — Financial objectives and profit innovation
- Added addendum H: objectives set by assistant per Damain's delegation; trading demoted from revenue to proof/learning track.
- Amended ARC42 v7 (V13 tool provenance kinds), MAS v2 (M11–M13), ROADMAP, SOURCES.
- AIUC-1 and NLnet page fetches were blocked on permission; recorded as unverified.

## 2026-10-23 — G1 started
- Damain approved objectives and specs.
- Cargo workspace with `vc-core` (RFC 8785, SHA-256, chain rules) and `vc-log` (RFC 9162 tree, inclusion/consistency proofs); tests pass, incl. RFC 8785 example and CT 8-leaf reference roots; `spec/statement.schema.json` added.
- G2 started: signed checkpoints (C2SP note format) with operator + witness-quorum verification and split-view check in `vc-log`; `vc-cose` COSE_Sign1 (Ed25519). All tests pass.

## 2026-10-23 — Scope reversal: no cutting
- Damain: build the entire VeriChain; first bring both ARC42s to production grade.
- Wrote `VERICHAIN_ARC42_v8.md` (layers L0–L6, functional requirements, threat model, deployment, ADRs V14–V22, research gates R1–R6, legal gates L-1..L-7, gate map G1–G12) and `MAS_ARC42_v3.md` (hosted agents, strategy catalog, capital phases, external-capital mandates, gates S0–S4).
- v7 and MAS v2 marked superseded; ROADMAP gates rewritten; SOURCES extended.

## 2026-10-23 — Research round 3; specs v9 / v4
- Withdrew solo-builder and calendar-speed assumptions (owner direction: multi-agent regimented factory).
- Added Addenda I (earlier pass) and J (verified standards alignment, assurance path, financial objectives v2).
- ARC42 v9: standards alignment §3.4, positioning §3.5, A2 verified-inference §4.5, factory §4.6 and Appendix G, ADRs V23–V30, research gate R7, G0.5.
- MAS v4: LP-2 restricted to ≥15 min, FC-2/SV-1, resolution/manipulation rules, decisions M17–M21.
