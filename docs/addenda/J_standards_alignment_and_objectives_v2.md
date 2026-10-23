# Addendum J — Verified standards alignment, assurance path, and financial objectives v2

Date: 2026-10-23 · Complements Addendum I. `[V]` = primary source read this session; `[S]` secondary; `[A]` assumption.

## 1. Corrections and confirmations to Addendum I
- **RFC 9964 (ML-DSA for JOSE and COSE) is Standards Track, May 2026 `[V]`.** Algorithm ids ML-DSA-44/65/87 = **−48/−49/−50**; key type AKP (kty 7), public key `pub` (label −1), private key a 32-byte seed; context string must be empty; HashML-DSA excluded; **no hybrid/composite scheme is defined**. Decision: hybrid = **COSE_Sign** with two signatures (Ed25519 −19 and ML-DSA-65 −49) over the same payload (ARC42 v9 V28). The earlier "confirm ids" caveat is closed.
- Rekor v2 reached GA on 10 Oct 2025 on Tessera tiles; **witnessing was still "to be implemented soon"** `[V]`. The public witness ecosystem cannot be assumed; VeriChain recruits its own (V30).
- HIP-4 facts (live 2 May 2026; validator-curated; USDH; fees on close/settle; no maker rebates on outcome books; 100 req/min) `[S: Chainstack]` are consistent with Addendum I.

## 2. New findings and what they change
| Finding | Source | Change |
|---|---|---|
| `draft-sharif-agent-audit-trail` v08 (13 Oct 2026): per-record JSON with agent identity, principal, action, outcome, trust level; `prev_hash` chain over JCS; ES256 default, ML-DSA-65 optional; describes SCITT as complementary; individual draft, not endorsed `[V]` | datatracker | closest standards-adjacent competitor; mitigated by **`vc-aat` import/export adapter** and positioning VeriChain as the witnessed, completeness-enforcing superset (V27) |
| CSAI **AARM** (policy engine, approval service, receipt generator, telemetry exporter); NIST CAISI initiative still in RFI/concept-paper phase; CoSAI IAM paper; FIDO agentic-auth WG (Apr 2026); CSA note: **"audit receives less explicit coverage"** than identity/authorization `[V: CSA note; S for others]` | CSA research note, NIST page | audit is the under-served standards slot; join AARM WG, comment on NCCoE paper, publish mapping |
| Payments: x402 Foundation (40 launch members incl. AWS, Circle, Cloudflare, Google, Mastercard, Visa; >100M cumulative transactions on Base through Q1 2026), AP2 (Intent/Cart/Payment mandates, 60+ partners, rail-agnostic), Stripe/Tempo MPP (Mar 2026), OpenAI/Stripe ACP `[S: Fireblocks]` | blog | `vc-pay` implements adapters for all; AP2 mandates map naturally to statements; none is source of truth (V26) |
| **VeriAttn** (arXiv 2606.16352): TEE verifies attention computed on an untrusted GPU using randomized checks, about two orders of magnitude lower latency than zkLLM on comparable models; tolerance-based; Intel TDX only; no confidentiality `[S: single preprint]` | arXiv | A2 verified inference is the practical LLM assurance (V29); research gate R4 reproduces it |
| Short-horizon Polymarket contract manipulation, Kalshi maker/taker returns (Addendum I §7) | preprint / paper | LP-2 restricted to ≥15 min; selection, not "maker" status, is the edge (MAS v4 M17–M18) |

## 3. Financial objectives v2 (supersedes Addendum H §2; all assumptions, revisable)
Why revised: capacity is no longer a binding constraint (multi-agent factory, ARC42 v9 §4.6), and more products ship by month 12: evidence cloud, hosted runtime (beta), compliance/certification packs.

| Revenue line (month-12 base) | Assumption | ARR |
|---|---|---|
| Evidence cloud (hosted log, witnessing, retention) | 10 customers × US$25k | US$250k |
| Hosted runtime (agents on VeriChain, beta customers) | 6 × US$30k | US$180k |
| Compliance / certification evidence packs (EU AI Act Art. 12 mapping, insurer/AIUC-style certification support) | 8 × US$12k | US$96k |
| **Base total** | | **≈ US$500k** |
| Stretch | one regulated anchor customer (~US$250k) plus certification-channel partners | ≈ US$1.2M |
| Kill / pivot | < US$100k ARR at month 12 → change segment or product mix | |

Leading indicators and kill rules from Addendum H stand (M3: ≥10 conversations, ≥3 pilots; M6: verifier published, ≥2 paid pilots). New: M6 also requires a submitted AARM mapping and at least one external witness operator onboarded; M9 hosted runtime beta with ≥2 design partners.
Spend assumptions `[A]`: factory compute and human verifier time are the main costs; external security audit before assurance claims or customer funds (order of US$50–150k); funding option: the sector's 2026 rounds (e.g., AIUC US$40M Series A and several US$100M+ rounds in agent security `[S: tracker]`) make an equity raise credible to staff human verifiers and pay for audits; the raise is a decision for the owner, not assumed.
Trading remains a learning/proof track: capital cap per MAS M11 (operator-set); company revenue never depends on it.

## 4. Open items
AIUC-1 requirement text; NLnet current call (both fetches blocked earlier); VeriAttn reproduction; x402/AP2 spec details; AARM draft text; NCCoE paper content; OSC trading study.
