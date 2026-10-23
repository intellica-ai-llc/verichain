# VeriChain — Architecture (v7 draft)

Supersedes `docs/archive/VERICHAIN_ARC42_v6.md`. Rationale: `docs/addenda/A–D`. Every external claim must be tracked in `docs/SOURCES.md` (to be created) as verified / unverified / refuted.

## 1. Goal and scope
v1 = **L0 Evidence layer**: signed, witnessed, append-only records of agent actions with offline verification. L1 identity/reputation, L2 proof tiers, L3 consensus/settlement and L4 governance/economy are deferred research, not commitments. The trading swarm (MAS) is a later tenant/dogfood, never a revenue assumption.

## 2. Principles
1. Standards first: SCITT (RFC 9943) statements/receipts, tile-based Merkle log with signed checkpoints, independent witness cosigning.
2. Completeness over completeness-claims: every receipt declares a completeness level (C0 self-reported · C1 gateway-observed · C2 runtime-enforced · C3 runtime-enforced + attested runtime).
3. Honest assurance: every record declares A0 signed · A1 deterministic replay · A2 TEE attestation · A3 zkVM proof of gate evaluation. Never "100% proven".
4. Commitments, not payloads: salted hashes on-log; payloads off-log; crypto-shredding for erasure.
5. Post-quantum by batch: sign checkpoints (hybrid Ed25519 + ML-DSA; evaluate SLH-DSA for root), per-receipt hash-path proofs, documented key rotation and evidence renewal.
6. Verifier is open, small, offline (CLI + WASM) and the product's trust anchor.

## 3. Components (v1)
Receipt emitter (ASL VM hook; MCP/A2A gateway adapter) → sequencer + tile log → witness client → time/anchor (RFC 3161, OpenTimestamps) → verifier → evidence-pack exporter → issuer/policy registry.

## 4. Decision Primitive v2 (to specify, gate G1)
COSE_Sign1 over canonical JSON (RFC 8785): action, principal, capability chain, policy content hash, verdict, input/output commitments, `completeness`, `assurance`, sequence/prev-hash. Test vectors required before code.

## 5. Open decisions
ORCHID stays research-only. Hosting: object-storage tiles, no consensus requirement in v1. Naming of the ASL hook interface depends on the ASL repo (v0.2.0 charters/speculation are not implemented there).
