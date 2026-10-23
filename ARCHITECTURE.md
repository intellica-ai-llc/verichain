# VeriChain — Architecture summary

Authoritative specs (drafts, pending approval; v7 and MAS v2 are superseded baselines):
- `docs/arc42/VERICHAIN_ARC42_v8.md` — the full platform: L0 evidence log, L1 identity/authority, L2 agent runtime, L3 assurance, L4 ordering/storage, L5 economy, L6 governance/compliance.
- `docs/arc42/MAS_ARC42_v3.md` — the flagship tenant: agents running on VeriChain, strategy catalog, capital phases, external-capital mandates.
Evidence and rationale: `docs/addenda/A–G`. Claim status: `docs/SOURCES.md`. Superseded v6 documents: `docs/archive/`.

## One-paragraph architecture
Agents (via a gateway, a policy-enforcing signer, or later the ASL runtime) submit signed **statements** (intent before an effect, outcome after) to a sequencer that appends them to a **tile-based Merkle log** and returns SCITT (RFC 9943) receipts. Signed **checkpoints** are cosigned by independent **witnesses** and time-anchored. An open **offline verifier** checks signatures, inclusion, witness quorum, anchors and **completeness** (orphan intents, sequence gaps) and reports declared **completeness (C0–C3)** and **assurance (A0–A3)** levels. Payloads stay off-log behind salted commitments.

## Principles
1. Standards first (SCITT, COSE, RFC 8785, tile logs, witnesses).
2. Honest claims: no "100% proven"; levels on every record.
3. Completeness by construction at chokepoints (signer, VM discharge gate); gateway tier is labeled C1.
4. Commitments, not payloads; crypto-shredding for erasure.
5. Post-quantum at checkpoint level; renewal by re-anchoring.
6. Open verifier; open-core business (Apache-2.0 core; hosted witnessing, retention, packs commercial).
7. Models propose, deterministic code disposes (MAS).
