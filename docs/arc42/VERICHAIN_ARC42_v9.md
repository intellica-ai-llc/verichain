# VeriChain — ARC42 Architecture Specification v9 (full platform, production grade)

Status: DRAFT for approval · Date: 2026-10-23 · Owner: Damain Ramsajan (Intellica AI Inc.) · Supersedes v8 (history in git) and v7 (detailed L0 baseline) · Companion: `MAS_ARC42_v4.md` (agents that run on VeriChain) · Research basis: Addenda A–J (v9 incorporates I and J)

**Scope decision (Damain, 2026-10-23): no cutting; built by a regimented multi-agent software factory (§4.6, Appendix G), not by a single developer.** v8 specifies the entire platform. Nothing from the original vision is deleted. What changes is that every component now has a defined purpose, interfaces, failure modes, acceptance tests, a dependency position in the build order, and — where the underlying claim or technology is unproven or legally sensitive — a **research gate** with a swappable interface and a fallback, so the platform never depends on an unverified claim.

**v9 additions in this pass (Addendum J):** standards alignment §3.4 and market positioning §3.5; engineering factory §4.6 and Appendix G; hybrid signing as COSE_Sign with RFC 9964 ids (verified); TEE-assisted verifiable inference path; payments against real protocols; ADRs V27–V30.

**Evidence tags used in this document:** `[V]` verified from a primary source this project; `[S]` secondary source; `[U]` unverified; `[A]` assumption/target to be validated at the stated gate. Source status lives in `docs/SOURCES.md`.

---
## 1. Introduction and goals

### 1.1 What VeriChain is
VeriChain is a **platform where autonomous AI agents run, act, transact and are held accountable, with every consequential action recorded in a tamper-evident, independently verifiable log.** It has two inseparable halves:

1. **The evidence foundation (L0):** a witnessed, append-only, post-quantum-ready transparency log of signed agent statements (SCITT-profile). This is the trust root. Anything anyone claims an agent did can be checked against it without trusting Intellica.
2. **The platform above it (L1–L6):** identity and delegated authority, a runtime that hosts agents so that *all* their I/O passes through VeriChain (strongest completeness), assurance tiers (replay, TEE, zero-knowledge), multi-operator ordering, settlement and an agent economy, and governance and compliance.

Agents may run **on** VeriChain (hosted; strongest evidence) or **beside** it (external; recorded through a gateway or signer; weaker but still useful). Both are first-class.

### 1.2 Product promise (precisely worded)
- Every statement an agent makes through VeriChain is signed, ordered, witnessed and independently verifiable.
- For **hosted agents**, no external effect can occur without a prior recorded intent (completeness level C3).
- For **external agents**, completeness is C1 (gateway-observed) or C2 (chokepoint-enforced, e.g., a venue that only accepts signer-signed orders).
- VeriChain never claims "100% proven". Each record states its assurance level (A0–A3) and the verifier enforces the buyer's minimum.

### 1.3 Functional requirement catalog (summary; full list with traceability in Appendix A)
| Layer | Capabilities |
|---|---|
| L0 Evidence | submit statement; get receipt; verify inclusion/consistency offline; witness cosigning; trusted-time anchoring; evidence packs; selective disclosure; erasure of off-log payloads |
| L1 Identity & authority | agent identity (keys, DIDs), operator/tenant identity, delegation chains with attenuation and revocation, key custody, rotation, PQ hybrid signing |
| L2 Runtime | deploy agent (ASL bytecode, WASM, container image), sandboxed execution, metered resources, secrets vault, mediated tool/network I/O, deterministic replay, checkpoint/restore, kill switch, gateway for external agents, policy-enforcing signer |
| L3 Assurance | policy decision point, deterministic policy language, replay verification (A1), TEE attestation verification (A2), zero-knowledge proofs of policy-gate evaluation (A3, research-gated) |
| L4 Ordering & storage | single-sequencer log, tile storage backends, replicated BFT ordering across operators, log federation and cross-log witnessing, pluggable orderer interface |
| L5 Economy | agent payments (x402-style, stablecoin), job escrow with evidence-based evaluation (ERC-8183-compatible), reputation derived from evidence, agent marketplace and tokenization (legal-gated), token (legal-gated) |
| L6 Governance & compliance | constitutional governance with separation of powers, private voting, compliance mappings and proofs, regulator and auditor views, operator console |

### 1.4 Quality goals (priority order)
1. **Integrity and verifiability** — no silent alteration or omission; verification needs no trust in the operator.
2. **Security of keys and sandboxes** — compromise of one agent, tenant, or operator key cannot forge another's history or escape isolation.
3. **Correctness and determinism** — canonical encodings, reproducible verification, replayable executions.
4. **Operability** — observable, recoverable, automatable; cheap enough to run small and able to scale out.
5. **Longevity** — evidence stays checkable for 10+ years (algorithm agility, renewal).
6. **Performance** — adequate for agent workloads (targets in §10); not a high-frequency-trading engine.

### 1.5 Stakeholders
| Stakeholder | Concern |
|---|---|
| Agent developers | deploy/connect agents simply; clear SDKs; fast iteration |
| Operators / enterprises | run fleets under policy; audit-ready evidence; cost control |
| Human principals | oversight, approvals, kill switch |
| Auditors, insurers, regulators | independent verification; completeness claims they can rely on; mappings to regimes |
| Counterparties | verify an agent's authority and history before transacting |
| Witnesses and co-operators | simple, safe cosigning/ordering duties |
| Capital providers / market participants | reputation and performance evidence; escrowed jobs |
| Intellica (operator of the reference deployment) | sustainable economics, minimal ops burden |

---
## 2. Constraints
| ID | Constraint | Note |
|---|---|---|
| K1 | Engineering is performed by a **multi-agent factory** (builder agents, independent verifier agents, human orchestrators/verifiers); owner workstation is a thin client | all heavy builds/tests run in CI/cloud; dev profile still runs a one-node stack in <1 GB so anyone can reproduce |
| K2 | Rust for core (memory safety, WASM target for verifier); TypeScript for console/SDK; Python SDK | |
| K3 | Open-core: Apache-2.0 for spec, log, verifier, SDKs | commercial: hosted witnessing/retention, evidence packs, SLA, compliance mappings (Addendum H) |
| K4 | ASL is a separate repo (`agentseedlanguage-cpu/agentseed`) | VeriChain integrates through a defined hook (`vc-asl-hook`), not by vendoring |
| K5 | Standards first: SCITT RFC 9943 `[V]`, RFC 8785 `[V]`, RFC 9162-style logs `[V]`, C2SP tlog-checkpoint and signed-note `[V]`, COSE RFC 9052 | deviations require an ADR |
| K6 | Legal gates: tokens, exchange/tokenization, custody, money transmission, biometric identity need counsel before any production launch | engineering may build behind feature flags; launch is gated |
| K7 | Trinidad & Tobago operator; users worldwide | data-protection and export considerations per deployment |
| K8 | Budget is a managed parameter (Addendum H v2), not a design constraint; cost ceilings apply to the *reference deployment* until revenue or funding | object storage and community/third-party witnesses keep run cost low; factory compute and audits are budgeted separately |

---
## 3. Context and scope

### 3.1 Business context
```
 Agent developers ──deploy──▶ ┌─────────────────────────── VeriChain ───────────────────────────┐
 External agents ──gateway──▶ │ L6 Governance/Compliance  L5 Economy  L4 Ordering/Storage        │ ◀── Auditors / regulators / insurers (verify)
 Human principals ──console─▶ │ L3 Assurance  L2 Runtime  L1 Identity/Authority  L0 Evidence log │ ◀── Counterparties (verify authority + history)
                              └───────▲─────────────▲──────────────▲──────────────▲──────────────┘
                       venues/APIs/LLMs │   TSA / OpenTimestamps │   Witnesses │   Chains / payment rails
```
### 3.2 External interfaces
| External system | Direction | Purpose | Trust assumption |
|---|---|---|---|
| LLM providers (hosted models) | outbound via runtime/gateway | agent reasoning | untrusted output; recorded by commitment |
| MCP servers, HTTP APIs, A2A peers | outbound via runtime/gateway | tool use | untrusted; provenance recorded (artifact/attestation statements) |
| Trading venues (e.g., Polymarket) | outbound via `vc-signer` | order placement | venue accepts only signer-signed orders (C2) |
| RFC 3161 TSAs; OpenTimestamps calendars | outbound | trusted time anchoring | any single TSA is untrusted; ≥2 sources |
| External witnesses | inbound/outbound | cosign checkpoints | independent operators; threshold policy |
| EVM chains (e.g., Polygon, Base) | outbound | identity registry bridge (ERC-8004), escrow (ERC-8183-compatible), stablecoin settlement | chain finality; contract audits required |
| Identity providers (OIDC) | inbound | operator/human login | standard |
| Cloud object storage / KMS / HSM | outbound | tiles, keys | encrypted; provider untrusted for integrity (log is self-verifying) |

### 3.3 Trust boundaries (summary; full threat model §8.1)
1. Agent code ⟷ runtime host (hostile code inside sandbox).
2. Tenant ⟷ tenant (isolated keys, storage prefixes, quotas).
3. Operator ⟷ verifier (operator is *not* trusted for integrity; witnesses and anchors bound its power).
4. Platform ⟷ external world (every effect is mediated and recorded).

### 3.4 Standards and ecosystem alignment (status as read this session)
| Body / spec | Status | What VeriChain does |
|---|---|---|
| IETF SCITT RFC 9943 | Proposed Standard `[V]` | statements/receipts profile (core) |
| RFC 9964 ML-DSA for JOSE/COSE | Standards Track, May 2026 `[V]` | PQ algorithm ids; hybrid via COSE_Sign |
| IETF `draft-sharif-agent-audit-trail` (AAT) v08 | individual draft, not endorsed `[V]`: JSON records, JCS hash chain (`prev_hash`), ES256 default, ML-DSA-65 optional; says SCITT is complementary | **import/export adapter** (`vc-aat`): AAT sessions map to statement chains; VeriChain adds witnessing, completeness levels, hosting |
| IETF WIMSE agent delegation chain -01 | individual draft, expires Mar 2027 `[V]` | `vc-cap` token format |
| CSAI Foundation **AARM** (runtime action enforcement: policy engine, approval service, receipt generator, telemetry exporter) | draft with working group `[S]` | `vc-fge`=policy engine, `approval` statements=approval service, statements+receipts=receipt generator, OpenTelemetry=telemetry; **join the WG and publish the mapping** |
| CoSAI agentic IAM paper; OASIS | published `[S]` | agents as non-human identities with lifecycle (`vc-id`) |
| FIDO Alliance agentic authentication WG (Apr 2026); Mastercard Verifiable Intent; Google AP2 | in development `[S]` | `vc-pay`/`vc-id` interop; AP2 Intent/Cart/Payment mandates recorded as `approval`/`delegation`/`settlement` statements referencing the mandate hash |
| x402 Foundation (40 launch members incl. AWS, Circle, Cloudflare, Google, Mastercard, Visa; >100M transactions on Base through Q1 2026 per Chainalysis) `[S]`; Stripe/Tempo MPP; OpenAI/Stripe ACP | live / launched `[S]` | `vc-pay` adapters; none is source of truth (V26) |
| NIST CAISI AI Agent Standards Initiative; NCCoE agent identity concept paper | RFIs/concept paper, no guidelines yet `[V]` | monitor; submit comments; map controls when guidelines appear |
| Sigstore Rekor v2 (GA 10 Oct 2025, Tessera tiles; **witnessing "will be implemented soon"**) `[V]` | GA | validates tile-log design; the public witness ecosystem is immature, so VeriChain runs and recruits its own independent witnesses and offers witnessing as a service |
| ERC-8004, ERC-8183 | draft-stage `[V/S]` | bridges, not truth |

### 3.5 Market positioning
Observed gap: standards bodies cover agent identity and authorization far more than **audit**; "audit receives less explicit coverage" (CSA convergence note `[V]`). Existing products are mostly vendor-signed logs or hash chains without independent witnesses or completeness guarantees `[S]`. VeriChain's position: **the independent, witnessed, standards-based evidence layer and the runtime that makes completeness enforceable**, interoperable with AAT/AARM/AP2/x402 rather than competing with them.

---
## 4. Solution strategy

### 4.1 Architectural principles
1. **Evidence first.** L0 is built, hardened, and published first; everything above records into it and may be verified from it.
2. **One chokepoint rule.** Strong completeness comes from controlling the path to the outside world, not from asking agents to report. Hosted agents: host calls. External agents: signer/gateway.
3. **Intent before effect.** An `intent` statement is durably accepted before any side effect; the `outcome` follows; unmatched intents surface as orphans.
4. **Commitments, not payloads, on-log.** Salted HMAC commitments on-log; payloads off-log under tenant control (privacy, erasure).
5. **Honest assurance.** Records carry A0–A3 and C0–C3; verifiers enforce minimums; marketing never exceeds the tier.
6. **Swappable where unproven.** Orderer, prover, signature suite, anchoring and storage sit behind traits. A research-gated implementation can be dropped without touching callers.
7. **Algorithm agility.** Hybrid classical + post-quantum signing, renewal by re-anchoring, versioned formats.
8. **Dogfood.** VeriChain's own releases, configs, key ceremonies and operator actions are logged on VeriChain.
9. **Determinism where claimed.** Anything claimed deterministic (canonicalization, policy evaluation, replay) is tested bit-for-bit across implementations.

### 4.2 Layered architecture
```
L6  Governance & Compliance   vc-gov  vc-comply  vc-console  vc-ops
L5  Economy                    vc-pay  vc-jobs  vc-rep  vc-market  vc-token*
L4  Ordering & Storage         Orderer trait: single | vc-bft | orchid-lab*   vc-store  vc-federate
L3  Assurance                  vc-fge  vc-replay  vc-attest  vc-prove*
L2  Agent Runtime              vc-run  vc-sandbox  vc-hostio  vc-vault  vc-gateway  vc-signer  vc-asl-hook
L1  Identity & Authority       vc-id  vc-cap  vc-kms
L0  Evidence                   vc-core  vc-cose  vc-log  vc-seq  vc-witness  vc-anchor  vc-verify  vc-pack  vc-sdk
                               (* = legal- or research-gated, built behind feature flags / lab crates)
```
Dependencies point **downward only**. L0 depends on nothing above it, so the verifier can be audited and shipped alone.

### 4.3 Build order and gates (detail in ROADMAP and Appendix F; the order is dependency order, not a staffing assumption)
L0 (G1–G4) → L1 (G5) → L2 runtime + gateway/signer (G6–G7) → L3 assurance (G8) → L4 federation/BFT (G9) → L5 economy (G10, legal gate) → L6 governance/compliance/console (G11) → hardening, audit, launch (G12). Parallel tracks: trading agents on the platform (MAS v3), commercial track (M0–M4), research gates (R1–R6, §11.2).

### 4.4 Key decisions (summary; full ADRs §9)
SCITT/tlog log for L0; completeness-by-construction via chokepoints; hosted runtime on WASM first with microVM for hostile/native workloads; delegation chains per the IETF WIMSE agent delegation-chain draft `[V]` (JWT, JCS, attenuation) mapped into VeriChain statements; ERC-8004 identity bridge and ERC-8183-compatible job escrow as chain adapters, not as the source of truth; BFT ordering behind an interface with Malachite (alpha, unaudited `[V]`) as the leading candidate and an experiment gate for ORCHID.

### 4.5 Assurance by verified inference (A2 path)
Hosted inference can be checked cheaply when the verifier is a TEE and the heavy computation runs on an untrusted GPU: a 2026 preprint (VeriAttn) verifies attention outputs inside a TEE with randomized linear and log-product checks and reports orders-of-magnitude lower latency than zkLLM on comparable models, with limits: tolerance-based (not bit-exact), Intel TDX tested, side channels/DoS out of scope, no confidentiality `[S: arXiv 2606.16352; single preprint]`. VeriChain therefore ranks A2 (TEE-attested, TEE-verified inference) as the practical assurance for LLM calls and keeps zk for small deterministic gates (A3). Verification results carry their tolerance in the statement (`assurance_detail`).

### 4.6 Engineering operating model (multi-agent software factory)
The platform is built by a **regimented factory**: version-controlled specs are the contract; builder agents implement; independent verifier agents attack; humans orchestrate, decide and sign off. Full procedure in Appendix G. Properties: dependency-ordered parallelism (the DAG in Appendix G), contract-first interfaces (traits + vectors frozen at G1 so crates build concurrently), independent test authorship for security-critical code, differential testing against second implementations, evidence-backed acceptance bundles, graduated autonomy, and dogfooding (factory acceptance bundles and releases are logged on VeriChain).

---
## 5. Building block view

### 5.1 L0 — Evidence (carried from v7; implementation started, see `crates/`)
| Crate | Responsibility | Interfaces | Failure behavior | Acceptance |
|---|---|---|---|---|
| `vc-core` | RFC 8785 canonical JSON, SHA-256, ids, chain rules | Rust API; spec vectors | rejects malformed; pure functions | RFC 8785 example, chain negatives (done) |
| `vc-cose` | COSE_Sign1 build/verify; algorithm registry (Ed25519 -19; ML-DSA-44/65/87 per RFC 9964 `[V]`: alg −48/−49/−50, key type AKP (kty 7), empty context string, no hybrid defined; **hybrid = COSE_Sign (tag 98) carrying an Ed25519 (−19) signature and an ML-DSA-65 (−49) signature over the same payload**; SLH-DSA when its COSE draft finalizes) | Rust API | unknown alg/kid = reject | roundtrip + negatives (done); interop vector with a second COSE implementation |
| `vc-log` | tree hashing, inclusion/consistency proofs, checkpoints (C2SP signed note), tile layout | Rust API | proof failure = typed error | CT reference roots, all-pairs proofs, quorum and split-view tests (done) |
| `vc-seq` (`vcd`) | sequencer: validate statement against registration policy, append, batch, sign checkpoints, serve API | SCRAPI-style HTTP (§Appendix B); gRPC for internal | refuses on policy fail; never acknowledges before durable write; crash-safe (WAL + tile fsync) | crash-injection suite; 10k statements/s single node on dev VM `[A]` |
| `vc-witness` | fetch checkpoint, verify consistency vs last-seen, cosign; gossip last-seen with peers | HTTP; tlog-witness style | refuses to cosign on inconsistency and raises alarm | split-view detection test with ≥2 independent witnesses |
| `vc-anchor` | RFC 3161 client; OpenTimestamps client; anchor scheduler | HTTP | multiple sources; failure of one does not block checkpoints | mock-TSA vectors; live test against ≥2 TSAs |
| `vc-verify` | verify statements, receipts, checkpoints, cosignature quorum, anchors, chain/completeness; lib + CLI + WASM | CLI `vcv verify`; JS/WASM | deterministic; offline | passes all published vectors in Rust and WASM |
| `vc-pack` | evidence packs: manifest, statements, receipts, checkpoints, anchors, verifier build hash | CLI/API | tamper = verify fails | tamper suite |
| `vc-sdk` | emitters (Rust, Python, TypeScript): build statements, commitments, submit, retry/idempotency | libs | at-least-once with idempotency key | cross-language vectors |

#### 5.1.1 Statement model (Decision Primitive v2) — frozen fields
COSE_Sign1; protected header: alg, kid, content type, CWT claims (`iss` = issuer key id, `sub` = agent id). Payload = RFC 8785 JSON (schema: `spec/statement.schema.json`).
Kinds: `intent`, `outcome`, `policy`, `delegation`, `artifact`, `attestation`; **v8 adds** `lifecycle` (agent deploy/stop/kill/restore events), `approval` (human or policy approval of an escalated action), `settlement` (payment/escrow events), `governance` (proposal/vote/enactment). Each new kind reuses the same envelope; kind-specific payload lives under `action`/`ext` with a registered schema.
Rules: (1) per-agent `seq` strictly increases from 0, `prev` = id (SHA-256 of canonical bytes) of previous statement of that agent; (2) an `outcome` references exactly one earlier `intent` of the same agent; (3) an intent with no outcome after `timeout` is an orphan and reported; (4) commitments = HMAC-SHA256(tenant_key, canonical payload bytes) with per-record salt kept off-log; (5) unknown `ext` keys ignored by verifiers but covered by the signature; (6) `completeness` and `assurance` are asserted by the *issuer class* (runtime/gateway/signer/sdk) and checked against the registered issuer's allowed maximum.

#### 5.1.2 Log
Leaf = SHA-256(0x00‖statement bytes); node = SHA-256(0x01‖l‖r). Tiles of 256 hashes plus entry bundles; checkpoint = origin, size, root + signature lines (operator + witnesses), C2SP format. Checkpoint cadence: ≤ 1 s or 1,000 entries `[A]`. Anchors timestamp checkpoint hashes (RFC 3161 ≥ 2 TSAs + OpenTimestamps) at ≤ 1 h cadence.

#### 5.1.3 Registration policy (published on the log)
Accept iff: valid signature by a registered issuer key (valid at time of statement); schema valid; `seq`/`prev` per rule; size/rate limits; kind-specific rules (e.g., `outcome` pairing, `lifecycle` authorized by operator capability). The most recently committed policy statement applies; policy changes are themselves logged.

### 5.2 L1 — Identity and authority
| Component | Responsibility | Interfaces | Failure behavior | Acceptance |
|---|---|---|---|---|
| `vc-id` | registry of operators, tenants, agents, issuer keys; DID-style ids (did:key / did:web profile); optional on-chain bridge to ERC-8004 identity registry `[V: draft-stage standard, ERC-721 identity + reputation + validation registries]`; key validity windows; revocation list published on log; **SPIFFE/SPIRE-style workload identity** for hosted agents; consumes **CAEP/Shared Signals** revocation events | HTTP/gRPC; EVM adapter | registry state is *derived from the log* (policy/lifecycle statements) so it is verifiable; cache loss is recoverable | registry rebuilt from log byte-identically |
| `vc-cap` | delegation chains: root authority → operator → agent → sub-agent; each link attenuates scope, constraints, expiry and depth; offline verification; revocation by status list | library + HTTP; statement kind `delegation` | unknown constraint type = deny; expired/revoked = deny | conformance with IETF draft-asor-wimse-agent-delegation-chain-01 `[V]` test cases (JWT, JCS, par_hash, attenuation rules); property tests: child ⊆ parent for random chains |
| `vc-kms` | key custody and ceremonies: generation, storage (HSM/KMS/software with sealing), rotation, hybrid Ed25519 + ML-DSA, SLH-DSA root, recovery, quorum-controlled root key | PKCS#11/KMS adapters | signing refused if key not in validity window; rotation is a logged event | scripted rotation and compromise-recovery drills; no private key leaves KMS in tests |

Delegation chain note: the IETF draft is an individual Internet-Draft expiring March 2027 `[V]`; VeriChain adopts its token format and attenuation semantics behind a version tag and tracks changes (ADR V14).

### 5.3 L2 — Agent runtime (where agents run)
| Component | Responsibility | Interfaces | Failure behavior | Acceptance |
|---|---|---|---|---|
| `vc-run` | control plane: deploy, schedule, start/stop/kill, restart policy, quotas, tenancy; emits `lifecycle` statements; admission checks (signed artifact, declared capabilities) | HTTP/gRPC | crash loop backoff; kill is immediate and logged; split-brain prevented by lease | chaos tests: node loss, restart, duplicate schedule |
| `vc-sandbox` | execution backends: **WASM** (wasmtime, component model + capability imports) as default; **microVM** (Firecracker/Kata) for hostile or native workloads; seedvm backend for ASL bytecode; container backend for trusted internal agents only | driver trait `Sandbox` | resource limit breach = terminate + `lifecycle` event | escape test suite per backend; resource limit tests |
| `vc-hostio` | the **mediated I/O layer** (host ABI): every external effect is a host call: `model_call`, `tool_call`, `http`, `fs` (virtual), `clock`, `random`, `emit`, `secret_ref`, `pay`. Host emits `intent` → performs effect → emits `outcome`. No other egress path exists (network namespace has no route). Time/random values are recorded for replay | WASM imports; microVM vsock proxy | intent not durably accepted ⇒ effect not performed (fail closed); log unavailable ⇒ agent pauses | completeness test: scan of every syscall/egress path of a test agent shows zero unlogged effects |
| `vc-vault` | secrets and credentials: agents hold references, never raw secrets; host injects at call time; per-call audit | gRPC; KMS | secret use requires capability + policy | agent cannot read secret material in tests |
| `vc-gateway` | C1 for external agents: MCP/A2A/HTTP proxy that records requests/responses as intent/outcome; verifies tool/server provenance (artifact statements) and flags unsigned tools | proxy; SDK | fail-open or fail-closed per tenant policy, and the mode is itself recorded | gateway-bypass test documents exactly what is not covered |
| `vc-signer` | C2 chokepoint for venues and payments: holds venue/wallet keys; evaluates policy; emits intent; signs; emits outcome from venue response | gRPC/HTTP | refuses without durable intent; kill switch halts signing | venue-only-accepts-signed-orders integration test |
| `vc-asl-hook` | ASL integration: `prov!` and discharge obligations map to statements; seedvm host calls route through `vc-hostio` | library | ASL programs without hook run as C0 | end-to-end ASL demo (depends on ASL v0.2+ readiness; ASL repo owns the language) |

#### 5.3.1 Host ABI (hosted agents) — contract
Every host call has: capability check (`vc-cap`), policy check (`vc-fge`), intent statement (durable), effect, outcome statement. Host calls are the *only* way out; WASM modules get no WASI sockets/filesystem except mediated virtual ones. Non-determinism (clock, randomness, model outputs, network responses) is captured by commitment and, for replay, stored off-log in the tenant's encrypted blob store.

#### 5.3.2 Completeness levels
C0 self-reported · C1 gateway-observed (bypass possible and documented) · C2 chokepoint-enforced (effect impossible without signer) · C3 hosted (all I/O mediated by `vc-hostio`; effect impossible without intent). Level is a property of the *issuer class*, enforced by the registration policy, not by self-declaration.

### 5.4 L3 — Assurance
| Component | Responsibility | Acceptance |
|---|---|---|
| `vc-fge` | policy decision point exposed via the **OpenID AuthZEN Authorization API 1.0** (so existing PEPs/IdPs can call it); deterministic policy language (limits, allow/deny lists, approvals, rate/loss caps, geo rules); evaluated before each host call; verdict recorded with `policy_hash` | policy evaluation is pure and byte-identical across implementations; conformance vectors; fail-closed on error |
| `vc-replay` (A1) | re-execute a recorded run from checkpointed state and recorded non-determinism; compare outputs | replay of hosted WASM agents is bit-identical in test corpus; documented limits for model calls (replay uses recorded outputs, not re-inference) |
| `vc-attest` (A2) | verify TEE attestation (CPU TEEs; GPU confidential compute `[S]`) binding code measurement and, optionally, model identity to statements; policy of accepted measurements | attestation verification vectors; negative tests (stale/wrong measurement) |
| `vc-prove` (A3, research gate R2) | zero-knowledge proof that a *policy gate* evaluated as claimed (small deterministic circuit), via zkVM; **not** proof of LLM inference at scale | proof verifies in `vc-verify`; cost/latency benchmark; NANOZK-style layer proofs remain research (`[V]` real preprint, simulation-scale, single-author) |

Assurance semantics: A0 signed; A1 deterministically replayable; A2 hardware-attested runtime; A3 cryptographic proof of a stated computation. A statement may list several. The verifier's `--min-assurance` option enforces buyer policy.

### 5.5 L4 — Ordering and storage
Orderer trait: `append(statement) -> position`, `checkpoint() -> signed checkpoint`, `subscribe()`. Implementations:
| Implementation | Use | Gate |
|---|---|---|
| `single` (in `vc-seq`) | one operator, witnessed; default for v1 production | G2–G4 |
| `vc-bft` | replicated ordering across ≥4 independent operators with 3f+1 tolerance; leading candidate: Malachite (Rust Tendermint library; Apache-2.0; self-described alpha, not externally audited `[V]`); CometBFT as conservative fallback `[S]` | G9, R3 |
| `orchid-lab` | ORCHID-style consensus as experimental orderer; must beat or match `vc-bft` under adaptive-fault simulation and a formal/peer-reviewed argument before any production role | R1 |
`vc-store`: tile and bundle storage on filesystem, S3-compatible object store, with PostgreSQL only as an *index* (never the source of truth); retention policies per tenant; legal hold. `vc-federate`: cross-log witnessing and log-of-logs so a customer's private log can be witnessed by a public one; receipts can chain across logs.
Invariant: whichever orderer is used, the output is the same checkpoint/tile format, so verifiers do not change.

### 5.6 L5 — Economy (legal gates apply before production use)
| Component | Responsibility | Standards | Gate |
|---|---|---|---|
| `vc-pay` | agent payments: HTTP 402-style pay-per-call client/server (x402 `[S]`), interop with AP2 v0.2 and Mastercard/FIDO Verifiable Intent `[S]`, stablecoin settlement, spend caps enforced by `vc-fge`, every payment is intent/outcome plus `settlement` statement | x402 (Coinbase-originated, open protocol `[S]`); L402 optional | G10 |
| `vc-jobs` | job escrow: Open→Funded→Submitted→Completed/Rejected/Expired, client/provider/evaluator, hooks; **evaluator can be a VeriChain verifier contract that checks evidence** (e.g., "the provider's agent logged the required steps with assurance ≥ A1") | ERC-8183 `[V: state machine and hooks read from the ERC text; status field not stated, treat as draft]` | G10 |
| `vc-rep` | reputation computed from evidence (completion rate, orphan rate, policy violations, dispute outcomes), published as verifiable aggregates; bridge to ERC-8004 reputation registry | ERC-8004 `[V]` | G10 |
| `vc-market` | agent catalog, listing, licensing; **tokenization/fractional ownership only after counsel review** (securities risk) | — | legal gate L-2 |
| `vc-token` | utility/governance token design (formerly UBE) and any biometric-based distribution (World ID) | — | legal gate L-3, L-4 |
Settlement is **evidence-linked**: funds release on verifiable conditions; disputes reference log positions.

### 5.7 L6 — Governance, compliance, operations
| Component | Responsibility | Acceptance |
|---|---|---|
| `vc-gov` | constitutional governance: tri-cameral separation (legislative/executive/judicial roles), proposals, private voting (MACI-style; cryptographic coordinator trust assumptions documented), enactment as `governance` statements, timelocks | proposal lifecycle tests; vote tally verifiable; coercion-resistance assumptions stated |
| `vc-comply` | compliance mappings (EU AI Act Art. 12 logging, NIST agent guidance, financial model-risk guidance, SOC2-style controls) producing evidence packs and attestations; regulator/auditor read-only portals; ZK compliance proofs as research (R5) | mappings reviewed by a practitioner before sale |
| `vc-console` | web console (TypeScript): fleet view, approvals, kill switch, evidence search, verification, billing | accessibility and security review; no secrets in browser |
| `vc-ops` | monitoring, SLOs, diversity-of-operators checks, witness health, anchor health, alerting, runbooks, backup/restore | game-day drills pass |

### 5.8 Cross-layer sequence of responsibility (who checks what)
Agent action → `vc-cap` (is it authorized?) → `vc-fge` (is it permitted by policy?) → `vc-hostio`/`vc-signer` (record intent, perform) → `vc-seq` (order, checkpoint) → `vc-witness`/`vc-anchor` (bind) → `vc-verify` (anyone checks) → `vc-pack`/`vc-comply` (present to auditors).

---
## 6. Runtime view (scenarios; each becomes an end-to-end test)

**S1 — External agent through the gateway (C1).** Agent calls MCP tool via `vc-gateway` → gateway checks tool provenance (`artifact`/`attestation` statements) → capability and policy checks → `intent` accepted by `vc-seq` (registration id returned) → request forwarded → response → `outcome` → receipt available after next checkpoint. Failure: sequencer down → per-tenant mode fail-closed (call denied) or fail-open (call allowed, gap recorded as a gateway-side `lifecycle` event when the log returns).

**S2 — Hosted agent tool call (C3).** WASM agent invokes host import `tool_call` → `vc-hostio` checks `vc-cap` chain and `vc-fge` policy → writes `intent` and waits for durable acceptance → executes via sandboxed egress → records `outcome` with commitments; non-determinism stored off-log. If any step before acceptance fails, the effect does not happen.

**S3 — Trading order through the signer (C2).** MAS agent proposes → `mas-risk` approves → `vc-signer` re-evaluates policy (limits, geoblock, staleness), emits `intent`, signs order with the venue key it alone holds → venue accepts only signer-signed orders → fills reconciled into `outcome`. Kill switch: operator or automatic → signer stops signing, open orders cancelled, `lifecycle` event logged.

**S4 — Witness cosigning and split-view alarm.** Witness fetches new checkpoint, requests consistency proof from last-seen, verifies, cosigns. If two checkpoints with the same origin and size but different roots are ever observed by any party, the verifier flags equivocation; `vc-ops` pages the operator; the incident is itself a logged event.

**S5 — Third-party audit.** Auditor receives an evidence pack, runs `vcv verify --min-completeness C2 --min-assurance A1 --witnesses 2` offline → gets pass/fail per claim with reasons (including orphans and policy violations in the covered period).

**S6 — Key rotation and PQ renewal.** Operator logs a `policy` statement introducing new checkpoint key (hybrid); old key signs a handover; anchors re-timestamp; renewal re-anchors old checkpoints under the new algorithm; old packs still verify (Q7).

**S7 — Job with escrow (L5).** Client funds job (chain escrow) → provider's hosted agent works under capabilities, emitting evidence → provider submits with a reference to log positions → evaluator (VeriChain verifier contract/service) checks required steps and assurance → completes or rejects → `settlement` statements recorded; disputes reference the same evidence.

**S8 — Governance amendment.** Proposal statement → deliberation window → private ballots → tally with proof → timelock → enactment `policy`/`governance` statements change registration policy or parameters; nothing takes effect without the log record.

**S9 — Incident and kill switch.** Anomaly (policy violation spike, reconciliation mismatch, witness alarm) → automatic or human halt of a tenant, agent, or the platform → `lifecycle` statements → post-incident evidence pack.

**S10 — Disaster recovery.** Region loss → restore tiles from object-store replicas → recompute root and compare to latest witnessed checkpoint → resume; any mismatch is a reported integrity incident, not silently repaired.

---
## 7. Deployment view

| Topology | Components | Use | Monthly cost target `[A]` |
|---|---|---|---|
| T0 Dev | all-in-one process, filesystem tiles, one local witness | any laptop (<1 GB), CI, factory sandboxes | ~0 |
| T1 Pilot | `vcd` + object storage + 2 external witnesses + 2 TSAs + OpenTimestamps; runtime on one small cluster (WASM) | first paid pilots | ≤ US$300 |
| T2 Production | HA `vcd` (active/standby with fencing), tiles on replicated object store, ≥3 independent witnesses, runtime cluster with microVM pool, KMS/HSM, console, observability stack | customers with SLAs | grows with revenue |
| T3 Federated | T2 plus `vc-bft` across ≥4 independent operators | multi-operator trust | per research gate R3 |

Environments: dev, CI, staging, production; infrastructure as code (Terraform/Ansible), reproducible builds, SBOM, SLSA-style provenance **recorded on VeriChain itself** (dogfood). Provider diversity rule: no single cloud provider holds more than one third of independent witnesses or operators; the v6 plan that placed 4 of 9 validators on one provider is rejected.

---
## 8. Cross-cutting concepts

### 8.1 Threat model (STRIDE-style summary; full model is a G1 deliverable)
| Threat | Example | Control |
|---|---|---|
| Spoofing | forged agent statement | registered issuer keys; capability chains; hybrid signatures |
| Tampering | operator edits history | Merkle proofs, witnessed checkpoints, anchors, split-view detection |
| Repudiation | agent/operator denies action | signed intents; durable-before-effect rule |
| Information disclosure | payload leakage from log | commitments only; tenant-encrypted blob store; per-tenant keys |
| Denial of service | statement flood | rate limits, quotas, admission control, backpressure; log availability separate from integrity |
| Elevation / escape | agent breaks sandbox | WASM capability model; microVM for hostile code; no egress except host calls; seccomp; minimal host surface; regular escape testing |
| Supply chain | malicious tool/model/skill | `artifact`/`attestation` provenance; pinned digests; gateway flags unsigned tools |
| Key compromise | sequencer or witness key stolen | rotation procedure; hybrid keys; witnesses detect divergence; HSM/KMS; short-lived signing sessions |
| Insider (operator) | selective omission | C2/C3 chokepoints; orphan detection; independent witnesses; transparency of policy changes |
| Cryptographic break | classical signatures broken | PQ hybrid, SLH-DSA root, renewal by re-anchoring |

### 8.2 Privacy and retention
Payloads off-log, encrypted per tenant; log holds commitments; erasure = destroy salt/payload (commitment becomes unlinkable); legal holds supported; data residency by tenant (separate logs/prefixes).

### 8.3 Observability
Structured logs, metrics, traces (OpenTelemetry); SLIs: append latency, checkpoint age, witness lag, anchor age, orphan rate, verification failures; dashboards and alerts in `vc-ops`; operator actions are logged on-chain-of-record.

### 8.4 Error handling
Typed error taxonomy (client/policy/integrity/availability); idempotency keys; fail-closed default for effectful paths; explicit degraded modes recorded in the log.

### 8.5 Versioning and compatibility
Every format has a version (`v`); verifiers support N and N−1; breaking changes only via new `v` with parallel support window; test-vector suite is the compatibility contract.

### 8.6 Testing strategy
Unit and property tests; published language-neutral vectors; differential testing against independent implementations (CT tools, Go tlog libs, second COSE library); fuzzing (canonicalization, parsers, proof verification); crash-injection and chaos tests; sandbox-escape suites; load tests; conformance suites for standards profiles; formal specs (Quint/TLA+) for ordering and key-rotation protocols where cheap; external audit before sale of any assurance claim.

### 8.7 Supply chain and release
Reproducible builds, signed releases, SBOMs, dependency pinning/auditing (`cargo deny`, `cargo vet`), release statements logged to the platform's own log.

---
## 9. Architecture decisions
v7 decisions V01–V13 stand except where amended here.
| ID | Decision | Status |
|---|---|---|
| V03 (amended) | Single sequencer + independent witnesses is the **default and first production topology**; replicated BFT ordering is built behind the `Orderer` trait (G9) for multi-operator trust | amended |
| V12 (superseded) | v7 deferral is **replaced by research and legal gates** (§11): ORCHID, token, exchange, biometric identity are in scope, built behind flags/lab crates, and cannot reach production until their gate passes | superseded |
| V14 | Adopt IETF agent delegation-chain draft token format and attenuation semantics for `vc-cap`, version-tagged | accepted (track draft; expires Mar 2027) |
| V15 | Hosted runtime: WASM default; microVM for hostile/native; containers for trusted internal only | accepted |
| V16 | `vc-hostio` is the only egress path for hosted agents (C3) | accepted |
| V17 | Registry and reputation state are **derived from the log**; chain registries (ERC-8004) are bridges, not sources of truth | accepted |
| V18 | Job escrow on-chain (ERC-8183-compatible) with VeriChain evidence as evaluator input | accepted, legal gate for funds handling |
| V19 | ZK is used for small deterministic gates (A3); LLM-inference proofs stay research | accepted |
| V20 | Dogfood: platform's own releases and operator actions are logged on the platform | accepted |
| V21 | Orderer, prover, signature suite, anchor, storage are traits with conformance suites | accepted |
| V22 | PostgreSQL is an index only; the log (tiles) is the source of truth | accepted |
| V23 | Engineering system is a regimented multi-agent software factory: spec-driven, traceable, independently verified, evidence-backed acceptance, graduated autonomy, human sign-off on security-critical modules; factory acceptance bundles are logged on VeriChain (Appendix G) | accepted |
| V24 | `vc-fge` exposes the AuthZEN 1.0 PDP API; internal gRPC remains | accepted |
| V25 | PQ: hybrid Ed25519 + ML-DSA-65 on checkpoints (RFC 9964 `[S]`), SLH-DSA root when its COSE draft is final; zk proofs STARK-final | accepted (confirm RFC 9964 ids) |
| V27 | AAT adapter (`vc-aat`) and AARM mapping published; VeriChain statements are the superset | accepted |
| V28 | Hybrid signatures use COSE_Sign with Ed25519 + ML-DSA-65 (RFC 9964 defines no composite scheme); verifier requires both unless policy says otherwise | accepted |
| V29 | A2 via TEE-verified inference is the primary assurance for LLM calls; zk limited to small deterministic gates | accepted |
| V30 | Run and recruit own independent witnesses; public witness ecosystem is immature (Rekor v2 witnessing not live at GA) | accepted |
| V26 | Payments interoperate with x402, AP2, Verifiable Intent behind `vc-pay` adapters; none is the source of truth | accepted |

---
## 10. Quality requirements (measurable; validated at the stated gate)
Q1–Q9 from v7 stand (gates re-mapped in Appendix F). Added:
| ID | Scenario | Target | Gate |
|---|---|---|---|
| Q10 | Hosted agent attempts any egress other than host calls | blocked in 100% of tests across 1,000 adversarial programs | G7 |
| Q11 | Host call with log unavailable | no effect performed (fail-closed) | G7 |
| Q12 | Delegation chain with widened scope/expiry/depth | rejected in 100% of property tests | G5 |
| Q13 | Replay of hosted WASM agent run | bit-identical outputs in test corpus | G8 |
| Q14 | Orderer swap (`single` → `vc-bft`) | verifier unchanged; all vectors pass | G9 |
| Q15 | One of 4 operators Byzantine or offline | log continues; no forked checkpoint accepted by verifier | G9 |
| Q16 | Job evaluator verifies required evidence steps | correct accept/reject on 100 fixtures | G10 |
| Q17 | Governance tally independently verifiable | verifier reproduces result from published data | G11 |
| Q18 | Platform recovery from regional loss | RPO ≤ last witnessed checkpoint; RTO ≤ 1 h `[A]` | G12 |
| Q19 | Requirement traceability | 100% of FR ids map to ≥1 passing test; CI fails otherwise | G1 onward |
| Q20 | Independent verification | every merged change in a security-critical crate has a verifier-agent report from a different model/prompt and a human sign-off | G1 onward |
| Q21 | AuthZEN conformance | `vc-fge` passes the AuthZEN 1.0 interop test cases | G8 |

---
## 11. Risks, research gates, legal gates

### 11.1 Risks
v7 risk table stands except "solo-founder capacity", which is replaced by **factory quality risk** (H: AI-written code can be wrong at scale; mitigated by spec traceability, independent verifier agents, differential testing, human sign-off on security-critical modules, external audits; Appendix G). Added: sandbox escape (H: layered isolation, bug bounty after launch); model-call replay limits (M: document A1 boundaries); chain/contract risk (H: audits before funds); regulatory classification of marketplace/tokens (H: counsel gates).

### 11.2 Research gates (swappable interface + fallback; no production dependency until passed)
| Gate | Claim/technology | Experiment | Pass criterion | Fallback |
|---|---|---|---|---|
| R1 | ORCHID-style consensus (single-author simulation-only preprint `[V]`) | independent reimplementation; adaptive-adversary simulation vs `vc-bft`; deterministic arithmetic (no floating point) | matches or beats `vc-bft` safety/liveness at equal fault model; peer review or formal argument | `vc-bft` |
| R2 | A3: zkVM proofs of policy-gate evaluation and small deterministic scorers; **STARK-final (no Groth16/PLONK wrap) to preserve PQ security**; candidates SP1, RISC Zero, OpenVM 2.0 (audited release, July 2026, vendor-reported perf) `[S]`. NANOZK-style LLM-layer proofs remain research `[V]` real preprint, small-scale; zkML benchmarks show only small MLPs practical, transformers → TEE `[S]` | benchmark candidates on `vc-fge` circuits; independent circuit audit; fuzz for under-constrained circuits | gate proof verifies in `vc-verify`; ≤ 1 MB (≤ 220 KB target), ≤ 10 s on commodity GPU `[A]` | A1/A2 only |
| R3 | Malachite production readiness (alpha, unaudited `[V]`) | soak, Jepsen-style fault injection, audit | no safety violations; audit findings closed | CometBFT |
| R4 | A2 verified inference: TEE attestation (CPU TDX/SEV-SNP; GPU confidential compute `[S]`) plus TEE-verified GPU offload (VeriAttn-style `[S]`) | reproduce on supported hardware; measure tolerance, overhead; verify measurement binding end-to-end in `vc-attest` | attestation verifies; overhead and numeric tolerance documented; adversarial tests (stale/wrong measurement, tampered GPU output) fail | A0/A1 for model calls |
| R7 | HIP-4 and other outcome-market adapters (oracle-trust, thin liquidity) | paper trading and oracle-risk review (MAS v4) | adapter passes S0–S1 and legal review | adapter stays disabled |
| R5 | ZK compliance proofs for regulators | prototype on one rule | verifier accepts; legal review | evidence packs |
| R6 | Reputation/economic mechanism robustness (sybil, collusion) | simulation + adversarial review | no cheap manipulation at target scale | disable score use in settlement |

### 11.3 Legal and regulatory gates (counsel before launch of each)
L-1 custody/money transmission for `vc-pay`/`vc-jobs` (Trinidad & Tobago and target jurisdictions); L-2 securities treatment of agent tokenization/fractional ownership; L-3 token issuance; L-4 biometric identity (World ID) data protection; L-5 EU AI Act, DORA, banking guidance mappings (claims language); L-6 export/sanctions screening; L-7 data-protection (GDPR) design review.

---
## 12. Glossary
**Statement** signed record of one agent event. **Receipt** proof a statement is in the log. **Checkpoint** signed (size, root). **Witness** independent cosigner of checkpoints. **Intent/Outcome** two-phase record of an action. **Orphan** intent with no outcome. **Commitment** salted hash on-log standing for off-log data. **C0–C3** completeness levels. **A0–A3** assurance levels. **Hosted agent** runs inside VeriChain runtime. **External agent** runs elsewhere. **Chokepoint** the only path to an effect, controlled by VeriChain. **Orderer** component deciding log order. **Delegation chain** attenuating authority from root to agent. **Evidence pack** portable bundle of statements, receipts, checkpoints, anchors.

---
## Appendix A — Functional requirements and traceability (first cut; maintained in `spec/requirements.md`)
Format: FR-id → component → acceptance test. Selected rows (full table is a G1 deliverable):
| FR | Requirement | Component | Test |
|---|---|---|---|
| FR-L0-01 | Accept only valid, in-policy statements and return a registration id before any effect depends on it | vc-seq | policy negatives; durability under crash |
| FR-L0-02 | Offline verification of inclusion, consistency, cosign quorum, anchors | vc-verify | vectors (Rust+WASM) |
| FR-L0-03 | Detect equivocation | vc-witness, vc-verify | split-view test |
| FR-L1-01 | Verify attenuating delegation chains offline | vc-cap | draft conformance + property tests |
| FR-L1-02 | Rotate keys without invalidating old evidence | vc-kms, vc-anchor | rotation drill |
| FR-L2-01 | No external effect without durable intent (hosted) | vc-hostio | egress adversarial suite |
| FR-L2-02 | Per-tenant isolation of keys, storage, quotas | vc-run, vc-vault | cross-tenant attack tests |
| FR-L2-03 | Policy-enforcing signer for venues/payments | vc-signer | venue integration |
| FR-L3-01 | Deterministic policy evaluation | vc-fge | cross-impl vectors |
| FR-L3-02 | Replay verification | vc-replay | corpus |
| FR-L4-01 | Orderer swap without verifier change | Orderer trait | Q14 |
| FR-L5-01 | Evidence-linked escrow | vc-jobs | Q16 |
| FR-L6-01 | Verifiable governance outcomes | vc-gov | Q17 |
| FR-L6-02 | Compliance evidence packs with mappings | vc-comply | practitioner review |

## Appendix B — API surface (to be frozen at G1/G2)
`POST /entries` (submit COSE_Sign1; returns registration id, idempotent) · `GET /entries/{id}` (receipt when available) · `GET /checkpoint` · `GET /tile/{level}/{index}` and entry bundles (static, cacheable) · `GET /proof/inclusion` · `GET /proof/consistency` · `GET /policy` · witness: `POST /add-checkpoint`. Runtime: `POST /agents` (deploy), `POST /agents/{id}/stop|kill`, `GET /agents/{id}/evidence`. Gateway: MCP/A2A/HTTP proxy endpoints. Internal: gRPC with Protobuf for runtime/signer/policy services. All mutating APIs idempotent, authenticated (mTLS/OIDC), rate-limited.

## Appendix C — Data model and storage
Source of truth: tiles + entry bundles + checkpoints (immutable). Derived: SQLite/PostgreSQL indexes (agent → seq → position; intent→outcome pairing; registry; reputation). Off-log: tenant blob store (encrypted payloads, salts, replay data). Rebuild rule: every derived store can be regenerated from the log and blob store.

## Appendix D — Test vector plan
As v7 Appendix A (JCS, COSE, Merkle, checkpoint/witness quorum, chain rules, commitments, evidence packs, anchors) plus: delegation-chain vectors (valid/invalid attenuation), host-ABI conformance traces, policy-evaluation vectors, replay fixtures, job-state-machine fixtures.

## Appendix E — Repository layout (target)
```
verichain/
  spec/                      # schemas, vectors, requirements, profiles
  crates/ L0: vc-core vc-cose vc-log vc-seq vc-witness vc-anchor vc-verify vc-pack vc-sdk vc-aat
          L1: vc-id vc-cap vc-kms
          L2: vc-run vc-sandbox vc-hostio vc-vault vc-gateway vc-signer vc-asl-hook
          L3: vc-fge vc-replay vc-attest vc-prove
          L4: vc-bft vc-store vc-federate   (lab/: orchid-lab)
          L5: vc-pay vc-jobs vc-rep vc-market vc-token   (feature-gated)
          L6: vc-gov vc-comply vc-ops
  console/ (TypeScript)  sdk/ (python, ts)  verifier-wasm/  contracts/ (EVM: jobs, registry bridge)
  deploy/ (terraform, ansible, docker-compose)  docs/ examples/
```

## Appendix F — Build order and gate map
| Gate | Scope | Done when |
|---|---|---|
| G0.5 | **factory ready**: agent roles, harness, CI gates, traceability checks, acceptance-bundle format (Appendix G) | factory builds and verifies a reference crate end to end |
| G1 | spec freeze: schema, vectors, requirements, threat model | approved; vectors in repo |
| G2 | tile log + `vcd` + checkpoints | crash/limits tests; <1 GB |
| G3 | witnesses + split-view | ≥2 independent witnesses |
| G4 | anchors + COSE receipts + verifier CLI/WASM + packs | offline verify; interop vector |
| G5 | L1: identity, delegation, KMS, PQ hybrid, rotation | Q7, Q12 |
| G6 | gateway (C1) + signer (C2) + SDKs | Q5 |
| G7 | hosted runtime: WASM sandbox, hostio, vault, microVM backend | Q10, Q11 |
| G8 | assurance: fge, replay, attestation (A3 per R2) | Q13 |
| G9 | `vc-bft`, federation (R1, R3) | Q14, Q15 |
| G10 | economy: pay, jobs, reputation (legal L-1; market/token per L-2/L-3) | Q16 |
| G11 | governance, compliance, console, ops | Q17 |
| G12 | hardening: external audit, game days, launch | Q18 |

## Appendix G — Engineering factory (multi-agent, gate-driven)
**Roles.** *Human orchestrator* (priorities, approvals, legal gates); *spec steward agent* (owns ARC42, `spec/requirements.md`, schemas; every change versioned); *builder agents* (one per crate or work package, each in its own git worktree, parallel candidates allowed); *vector author agents* (write test vectors and property tests **independently** of the builder, from the spec only); *verifier agents* (adversarial review, differential testing, fuzzing; different prompt and, where possible, different model family from the builder); *security reviewer agents* (threat-model conformance, sandbox/crypto review); *integration agent* (cross-crate contract checks, end-to-end scenarios S1–S10); *release agent* (reproducible builds, SBOM, signing); *ops agents* (runbook execution under human approval for irreversible actions). Humans sign off security-critical modules (crypto, sandbox, signer, settlement) and every legal-gated feature.

**Rules.**
1. No code without a requirement ID; no requirement without an acceptance test; CI fails on untraced code or untested requirements (Q19).
2. Contract-first: traits, schemas and vectors are frozen at G1; crates are built in parallel against them.
3. Builder and test author are different agents for any security-critical crate; crypto and format code must also pass differential tests against an independent implementation (e.g., Go tlog libraries, a second COSE library, CT reference trees).
4. **Acceptance bundle** per change: requirement ids, diff, test results, vector results, fuzz/property summary, verifier-agent report, reviewer sign-off (human where required), build hash. Bundles are logged on VeriChain (dogfood) and exportable as an evidence pack (product P10).
5. Graduated autonomy: agents may merge low-risk changes after green gates; security-critical or irreversible changes require human sign-off; autonomy level per crate is recorded.
6. Parallel candidates are compared by the verifier agents and the best-evidenced one merges; discarded candidates are retained for learning.
7. Persistent shared knowledge: decisions in ADRs, research in addenda, claims in SOURCES; agents consult these before improvising and file new findings back.
8. Spend and rate budgets per gate are explicit; the harness stops and escalates when exceeded.

**Dependency DAG (what can run in parallel after G1).**
Wave 1 (parallel): `vc-seq`, `vc-witness`, `vc-anchor`, `vc-verify`, `vc-sdk`, `vc-aat`, `vc-store`; spec stewards finish requirements and threat model.
Wave 2 (needs wave-1 interfaces): `vc-pack`, `vc-id`, `vc-cap`, `vc-kms`, `vc-fge` (policy language + vectors can start earlier), `vc-gateway`, `vc-signer`.
Wave 3: `vc-run`, `vc-sandbox`, `vc-hostio`, `vc-vault`, `vc-replay`, `vc-attest`, `vc-asl-hook`.
Wave 4: `vc-bft`, `vc-federate`, `vc-pay`, `vc-jobs`, `vc-rep`, `vc-prove`.
Wave 5: `vc-gov`, `vc-comply`, `vc-console`, `vc-ops`, then gated `vc-market`, `vc-token`.
Research gates R1–R7 run alongside as lab crates and reports, never blocking waves until their consumer arrives.

**Factory acceptance for G0.5.** A reference crate (`vc-log`, already built) is re-derived end to end by the factory from the spec: independent vectors, differential test against an independent tree implementation, verifier report, acceptance bundle logged. Passing proves the process before scaling it.
