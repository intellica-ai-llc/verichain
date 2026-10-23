# VeriChain — Gap Analysis & Architecture Addendum A

Date: 2026-10-22 · Baseline: `verichain_v6_ARCHITECTURE.md` (arc42, 15 May 2026) · Status: DRAFT for review, no code yet

## 0. How to read this

- **Verified** = I opened the primary source this session. **Unverified** = the repo cites it, I did not check it. **Judgement** = my analysis.
- The repo's provenance table marks most claims "VERIFIED 93–100%", but its source is "【chat】", not a paper. Treat those labels as unverified until each is checked.

## 1. Ground truth about the repo

| Finding | Evidence |
|---|---|
| Only the ASL toolchain exists as code (seedc ≈5.7k LOC, seedvm ≈5.3k, small CLI tools). One commit. | repo tree |
| **None of the 16 VeriChain containers exist in this repo.** No `crates/`, `verify/`, `contracts/`. The Dafny proof (C-005), `.env.example`, Terraform and Ansible referenced by the doc are absent. (Note: the Verity repo is a separate project and is not VeriChain code; an earlier version of this file wrongly treated it as such.) | repo tree |
| `README.md` describes AGENT-SEED v15.2, not VeriChain. Version labels conflict (ASL 0.1.0 / 0.2.0 / 1.0.0 / v15.2). | README, doc names |
| `verichain_v6_ARCHITECTURE.md` contains the whole "Batch 2" section twice (lines ~140–600 and ~601–1130). | doc |
| Scope drift: v6 is a "Web 4.0 agent economy" (consensus, Lightning, UBE token, exchange, governance, Polymarket trading). Your earlier description was a *Merkle-proofed evidence anchoring layer*. | doc vs your brief |

## 2. Gaps (ranked by how much they threaten success)

### G1 — Scope is ~10× what one person can build, and the core is buried
Sixteen containers for a solo founder on a 4 GB Chromebook. The part with a real market (tamper-evident, independently verifiable evidence of agent decisions) is one container (`dp-store`) inside an economy design. *Judgement.*

### G2 — Load-bearing research claims are weaker than the doc states
- **ORCHID (verified):** real paper (arXiv 2605.12211), but a **single-author v1 preprint, simulation-only** (Watts-Strogatz networks, 10–150 nodes), built on Kuramoto sync plus the Orch-OR hypothesis and a "Quantum Secret Sharing" layer. No peer review found. "100% consensus at 40% Byzantine" is a simulation outcome under the author's fault model. It exceeds the classic 1/3 bound for signature-based BFT, so it cannot be assumed to hold against an adaptive adversary. Kuramoto dynamics use floating point, which is a determinism hazard for consensus. The doc treats it as "VERIFIED 98%" and puts it in the critical path.
- **NANOZK (verified):** real (arXiv 2603.18046, single author). The current version (v2) reports **~3.5–3.7 KB/layer and ~22 ms verify**, not the doc's 5.5 KB / 24 ms. Full-block end-to-end proofs were measured only up to hidden width **d=128**. GPU numbers at d=768 are projections. It uses 16-bit lookup approximations for softmax/GELU/norm. The prompt is not hidden from the prover. The doc's "ε < 1e⁻³⁷" I could not confirm from the abstract (unverified).
  **Consequence:** conformance item C-006 ("every Decision Primitive carries a NANOZK proof") is infeasible for agents that call hosted models (Claude, GPT and similar), because you do not hold the weights. It is also incompatible with the <100 ms trading-loop target.
- **Not checked this session:** Moon & Varshney (containment), Spera hypergraph closure, TRUST / Safety–Profitability, Σᴿ Legitimate Envelope, AgentCity, PASTE/B-PASTE, zkAgent, SAPL "2× faster". Verify each before it appears in a customer or investor document.

### G3 — The log itself is under-specified versus the standards that already exist
- No inclusion/consistency-proof format, no signed-checkpoint format, no tile layout, no leaf/node domain separation, no split-view (equivocation) defence, no monitor/auditor role.
- Hash inconsistency: keccak256 for content IDs vs SHA-256 in `hashtree-core`.
- "≥f+1 replicas agree" is not a substitute for witnessed checkpoints.
- Standards to align to (verified): **RFC 9943 SCITT Architecture** (Proposed Standard, June 2026: signed statements → transparency service → COSE receipts, over RFC 9942/9162-style append-only logs); **tile-based logs** (Trillian-Tessera; Sigstore Rekor v2 moved to this); the **witness network**, where independent witnesses cosign checkpoints only after checking a consistency proof, protecting against split views.

### G4 — No trusted time, no long-term-evidence story
A hash chain proves order, not *when*. Missing: RFC 3161 / eIDAS-qualified timestamps or public anchoring. For multi-year evidence you also need renewal when algorithms age (IETF Evidence Record Syntax lineage; a PQC-ERS draft exists, so this is an active area). The doc mentions hybrid ML-DSA but not key rotation, signature-size cost, or a hash-based fallback (SLH-DSA) for long-lived roots.

### G5 — What exactly is attested, and privacy
A log proves *a statement was registered*, not that it was true. `DecisionPrimitive` stores `output: Value`. Putting payloads in an immutable log collides with GDPR erasure and with confidentiality. Store **commitments** (salted hashes) on-log, payloads off-log.

### G6 — Infrastructure plan contradicts its own rules
- ADR-003 puts 4 of 9 validators on Oracle (44%), violating C-014 (no provider >33%).
- Consensus over Tor conflicts with the <4 s finality target.
- Supabase free tier (500 MB) cannot hold an append-only ledger.
- The "~2-week free-credit window" makes infrastructure survival depend on trading profit.

### G7 — Regulatory and legal are asserted, not researched
- EU AI Act logging (Art. 12) is a real demand driver. High-risk dates were provisionally delayed (Annex III → 2 Dec 2027, Annex I → 2 Aug 2028, per a 22 June 2026 report; **verify final enactment**).
- Unresearched: Lightning custody and money-transmission rules, UBE token (securities), World ID biometric data, DUNA wrapper. These need counsel, and none should block the evidence layer.

### G8 — Competitive position
Verified landscape: **Sigstore Rekor v2 / Trillian-Tessera** (software-supply-chain transparency), **SCITT RFC 9943** and an IETF draft for **AI agent receipts**, **ERC-8004** with a proposed "verifiable AI system transparency" extension, **Nillion Blacklight**, open-source agent "capability receipts" tooling (raucle). *Judgement:* competing on the log primitive is a losing fight. The defensible niche is the **agent-specific layer**: ASL-native provenance (`prov!`), decision schema, post-quantum posture, and compliance packaging for regulated buyers (Verity/banking).

### G9 — Delivery risk
The v1 launch plan is a 2-week sprint to out-trade bots on Polymarket. That is speculative, unrelated to evidence infrastructure, and puts real capital at risk. Decouple it from VeriChain's success criteria.

## 3. Addendum A — decisions

**A-1 Tier the scope.** L0 Evidence (build now) → L1 Identity/Reputation → L2 Proof tiers → L3 Consensus/Settlement → L4 Governance/Economy. Only L0 is in v1. L3/L4 stay in the doc as research, not commitments.

**A-2 L0 is a SCITT-profile transparency service.**
- Statements are COSE_Sign1; receipts are COSE receipts per RFC 9943.
- Log is a tile-based Merkle log, SHA-256 with leaf/node domain separation (RFC 9162 style), signed checkpoints, inclusion and consistency proofs.
- Single sequencer in v1; **external witnesses cosign checkpoints** (N-of-M policy). No BFT consensus needed for this to be trustworthy.
- Replaces `dp-store` Postgres ledger with tile storage on object storage (cheap, fits free tiers). Postgres is an index only.

**A-3 Decision Primitive v2.** Commitments only (salted hashes of input/output), signed statement, and an explicit **`assurance_level`**: A0 signed · A1 deterministic replay · A2 TEE attestation · A3 zk proof. Never claim "100% proven". Drop C-006; replace with "each primitive declares its level and the verifier enforces the buyer's minimum".

**A-4 Time and longevity.** RFC 3161 timestamp on every checkpoint plus periodic public anchor. Hybrid Ed25519 + ML-DSA for operational signing; evaluate SLH-DSA for the checkpoint root key. Define key-rotation and evidence-renewal procedure.

**A-5 Consensus.** ORCHID moves to a research track (benchmark it against CometBFT/HotStuff under adaptive faults before any dependency). If multi-operator ordering is ever needed, use an established BFT protocol.

**A-6 Infrastructure.** Remove Tor from the consensus path (keep optional for client access). Redo the diversity math for the real witness/sequencer topology. Budget real hosting; do not tie it to trading revenue.

**A-7 Conformance rewrite.** Replace C-006, C-008, C-014, C-015 with: published test vectors, interop with an existing tlog verifier, witness-cosigned checkpoint verification, and a split-view detection test.

**A-8 Repo hygiene (your standing convention).** Add `README.md` for VeriChain, `ARCHITECTURE.md` (v6 deduplicated + this addendum), `DEVLOG.md`, `ROADMAP.md`. Remove the duplicated Batch 2 block. Put every external claim in a `SOURCES.md` with a status (verified / unverified / refuted).

## 4. Proposed path to code (design gates first)

| Gate | Deliverable | Done when |
|---|---|---|
| G0 | Four-file hygiene, SOURCES.md | claims triaged |
| G1 | Decision Primitive v2 + SCITT profile spec + test vectors | you approve the spec |
| G2 | Minimal Rust log: tiles, checkpoint, inclusion/consistency proofs | runs in <1 GB RAM on your Chromebook |
| G3 | Witness cosigning + split-view test | 2 independent witnesses |
| G4 | COSE receipts + RFC 3161 anchor | verifier CLI passes vectors |
| G5 | ASL `prov!` → statement emission | end-to-end demo |
| G6 | PQ hybrid signing | rotation test passes |

## 5. Decisions needed from you

1. **Scope:** is v1 = L0 Evidence only (my recommendation), or the full economy?
2. **Trading launch:** decouple from VeriChain's success criteria?
3. **ORCHID:** research track only, or do you want it kept in the critical path?

## Sources (verified this session)
- ORCHID: https://arxiv.org/abs/2605.12211
- NanoZK: https://arxiv.org/abs/2603.18046
- RFC 9943 SCITT: https://datatracker.ietf.org/doc/html/rfc9943
- Rekor v2 / Tessera: https://blog.sigstore.dev/rekor-v2-alpha/
- Witness network: https://blog.transparency.dev/can-i-get-a-witness-network
- EU AI Act delay: https://datamatters.sidley.com/2026/06/22/eu-lawmakers-reach-provisional-agreement-to-delay-key-eu-ai-act-obligations/
- Also surfaced in search, not opened: PQC-ERS IETF draft, ERC-8004 transparency extension, draft-noa-scitt-ai-agent-receipt, Nillion Blacklight, raucle.
