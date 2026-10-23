# Addendum C — Frontier research, competitors, and a differentiation thesis for VeriChain

Date: 2026-10-22 · Status: DRAFT for discussion · "Verified" = source opened this session; competitor map is SECONDARY (one GitHub issue by a competing author) and each product is unverified.

## 1. Provenance, resolved
`agentseedlanguage-cpu/agentseed` (ASL, 65 commits) and `intellica-ai-llc/verichain` (64) share 63 commits (merge-base dcf02ab, 19 May). The only commit unique to verichain is 546743e, which added the four VeriChain/MAS documents; ASL's two unique commits (20 May, 5 Jun) are later work. All authors are DamainRamsajan. Conclusion: verichain = ASL clone + one docs commit (consistent with pushing to the wrong remote). No sign of tampering. The ASL repo's `main` has no charter or speculation code; v0.2.0 exists only as `ASL_V_0_2_0_ARC42.md` (speculation limit still "Proposed").

## 2. Landscape (2026)
| Area | What exists | Status |
|---|---|---|
| Agent receipts (OSS) | Agent Receipts/Obsigna (W3C VC, Ed25519), protect-mcp (Cedar + Ed25519), agent-custody (Merkle), veritrail, AgentMint, Pipelock, others | mostly pre-v1, self-signed hash chains |
| Agent receipts (commercial) | Asqav (ML-DSA-65 per receipt), CertNode (RFC 3161 + OpenTimestamps), TrustNotch (RFC 6962 Merkle batches + OTS), ProofAgent (HMAC), ProofTrail, others | pricing often unpublished |
| IETF | RFC 9943 SCITT (Proposed Standard, Jun 2026); individual drafts: draft-noa-scitt-ai-agent-receipt-00 (COSE_Sign1 + RFC 8785 JSON; expires 25 Dec 2026), draft-sahu-agent-action-receipts-00, draft-farley-acta-signed-receipts, draft-marques-asqav-compliance-receipts | drafts, not endorsed |
| Research | arXiv 2606.04193 ("Notarized Agents"): witness-cosigned Merkle log, compared with SCITT | preprint |
| Log infrastructure | Trillian-Tessera / Rekor v2 tile logs; independent witness network (cosigned checkpoints) | production-grade, reusable |
| PQ | Merkle Tree Certificates (IETF PLANTS): sign checkpoints, ship inclusion proofs; Chrome experiment targeted end of 2027, wider use ~2029–30 | draft |
| Identity/standards | NIST CAISI AI Agent Standards Initiative (17 Feb 2026); NCCoE concept paper covers identification, authorization, delegation, logging/transparency; SP 800-53 overlays pending | in progress |
| Payments | x402 (Linux Foundation; ~165M cumulative txns / ~$50M by Apr 2026; ~$24M in 30 days to Jul 2026, mostly sub-dollar bots), AP2 (FIDO Alliance, Apr 2026), ACP (checkout shelved by Mar 2026). L402 not covered. Neither x402 nor AP2 reconciles what settles after authorization | evolving |
| Proofs | zkVMs production-grade (SP1, RISC Zero, OpenVM 2.0); block-proving cost fell ~45× in 2025; zkML for LLMs vendor-reported, not commodity; under-constrained circuits are the dominant bug class | mixed |

## 3. What the field does not yet do
Sources say: receipts are tamper-evident, not tamper-proof; they do not establish authorization; vendor Article 12 claims are unverified marketing; deterministic replay is deferred (draft-noa); settlement reconciliation is unsolved. My judgement: few use witness cosigning; per-receipt PQ signatures are expensive; no one can prove **completeness** (that every action produced a receipt) because they sit outside the agent as wrappers or gateways that can be bypassed.

## 4. Differentiation thesis (the log primitive alone is commoditizing)
1. **Completeness by construction.** ASL's discharge gate is a chokepoint: no effect leaves a computation without it. Make the gate emit a write-ahead receipt, so "no receipt, no effect". Completeness levels: C0 self-reported · C1 gateway-observed (bypassable) · C2 runtime-enforced (ASL VM) · C3 runtime-enforced + TEE-attested runtime.
2. **Authorization lineage.** Receipt carries capability-token chain + policy content hash, so it shows who delegated what, not just what happened.
3. **Witnessed non-equivocation.** Tile log + signed checkpoints + independent witness cosigning; receipts exported as SCITT (RFC 9943) COSE receipts.
4. **Batch post-quantum.** Sign checkpoints (hybrid Ed25519 + ML-DSA; evaluate SLH-DSA for root), give each receipt a hash-path proof (MTC idea), with an evidence-renewal procedure. Cheaper than per-receipt ML-DSA.
5. **Honest assurance ladder.** A0 signed · A1 deterministic replay · A2 TEE attestation · A3 zkVM proof that the *gate applied policy P to committed inputs and returned verdict V* (small, deterministic; not the LLM). Never claim "100% proven".
6. **Replay of everything except the model.** Record model outputs as oracle inputs; replay the deterministic remainder (ASL seeded determinism). Closes the gap draft-noa defers.
7. **Privacy and regulator verifier.** Salted commitments, selective disclosure, crypto-shredding for erasure; a single offline verifier CLI/WASM; mapping to EU AI Act Art. 12 and NIST NCCoE logging themes (mapping still to be done).
8. **Settlement-reconciliation receipts.** Link mandate (AP2/x402) → payment → outcome. Support x402/AP2 as evidence payloads; payment rails are optional, not core.

## 5. Risks to the "light years ahead" claim
- Crowded category; OSS clones are cheap. The moat is 1, 2 and 5 together, not the log.
- ASL coupling narrows the market. Needs a C1 gateway/SDK tier so non-ASL agents can onboard.
- ASL v0.2.0 features (charters, speculation) are not implemented; the evidence layer only needs `prov!` and discharge.
- Every cryptographic claim needs an independent audit and test vectors; ZK circuits need fuzzing plus audits.
- Standards churn (draft-noa expires Dec 2026). Track SCITT profiles.
- Drop "first / mathematically undeniable" claims; competitors exist.

## 6. Keep / defer from v6
Keep: dp-store→tile log, identity (DID + ERC-8004 adapter), compliance proofs, minimal governance. Defer: ORCHID consensus, UBE/World ID, exchange, Lightning-only settlement, trading swarm (MAS).

## 7. v1 component sketch
Receipt emitter (ASL VM hook + gateway adapter) → sequencer/tile log → witness client → anchor (RFC 3161 / OpenTimestamps) → verifier (CLI + WASM) → evidence-pack exporter → issuer/policy registry.

## Sources
RFC 9943 https://datatracker.ietf.org/doc/html/rfc9943 · draft-noa https://datatracker.ietf.org/doc/html/draft-noa-scitt-ai-agent-receipt-00 · competitor map https://github.com/JaredKlopstein/provenant/issues/6 · MTC https://lwn.net/Articles/1060941/ · ZK state https://wavect.io/blog/zero-knowledge-proofs-production-2026/ · payments https://blindpay.com/resources/more/agent-payment-protocols-compared.md · NIST https://labs.cloudsecurityalliance.org/research/csa-research-note-nist-ai-agent-standards-federal-framework/ · witness network https://blog.transparency.dev/can-i-get-a-witness-network · Rekor v2 https://blog.sigstore.dev/rekor-v2-alpha/
