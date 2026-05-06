AGENT-SEED v15 — PRODUCTION ARCHITECTURE (ENGINEERING SPEC)

Status: Normative
Scope: End-to-end system to compile, execute, verify, and operate ASL v15 programs in production
Audience: Compiler, runtime, platform, infra, and security engineers

0) System Goals & Non-Goals
Goals
Deterministic, replayable execution of agent programs
Unified effect system (uncertainty, taint, cost, capabilities, provenance)
Strong safety invariants (no silent uncertainty collapse; no unsanitized influence into effects)
First-class resource governance (contracts, budgets, deadlines)
Verifiable execution (semantic events + audit proofs)
Multi-provider inference with constrained decoding
Horizontal scalability (multi-agent, async, distributed)
Non-Goals
“Best effort” correctness; all correctness is enforced or fails
Ad hoc scripting runtime; everything passes through the VM
1) High-Level Architecture
ASL Source
  → Compiler (lex/parse/type/effects/taint/contracts)
  → IR (typed, effect-annotated, SSA-like)
  → Verifier (soundness checks)
  → seedVM (deterministic executor)
      ↔ Inference Engine (LLMs, constrained decoding)
      ↔ Scheduler (async + concurrency)
      ↔ Contracts Engine (budgets/lifecycle)
      ↔ Security (taint + sanitization)
      ↔ Provenance (semantic events + DAG)
      ↔ Storage (memory/state)
      ↔ Network (multi-agent sessions)
      ↔ TEE (attestation gating)
  → Outputs + Audit Artifacts (logs, proofs)
2) Core Invariants (MUST HOLD)
No raw values: all runtime values are Computation<T, ε>
Single effect system: uncertainty, taint, cost, capabilities, provenance unified
No side effects without discharge
Deterministic replay given: model version, seed, prompts, grammar hash
No unsanitized external influence into capability execution
Budget/contract enforcement is mandatory and atomic
All actions produce semantic events; audit operates on events

Violations → compile error (S1+) or runtime trap.

3) Data Model (Canonical Types)
3.1 Computation
export interface Computation<T> {
  value: T | null
  effect: Effect
  failure?: Failure
}
3.2 Effect (Unified)
export interface Effect {
  uncertainty: Interval        // [low, high]
  taint: TaintMeta            // causal influence
  cost: CostInterval          // tokens + time
  capabilities: Set<string>   // accumulated
  provenance: string[]        // event IDs (DAG refs)
}
3.3 Taint (Causal)
export interface TaintMeta {
  sources: Set<string>        // external, inferred, user, federated, custom
  influence: number           // [0,1]
  lineage: string[]           // node IDs
}
3.4 Cost
export interface CostInterval {
  tokens: [number, number]
  time: [number, number] // ms
}
3.5 Decision (Discharge)
export type Decision<T> =
  | { kind: "Some"; value: T }
  | { kind: "Ambiguous"; effect: Effect }
  | { kind: "None" }
3.6 Failure
export type Failure =
  | "Timeout"
  | "SchemaError"
  | "ContractViolation"
  | "TaintViolation"
  | "Divergence"
4) Effect Semantics
4.1 Merge (Normative)
merge(e1, e2):
  uncertainty = combine_uncertainty(e1.u, e2.u, κ)
  taint       = merge_taint(e1.t, e2.t)
  cost        = interval_add(e1.c, e2.c)
  caps        = union(e1.caps, e2.caps)
  prov        = append(e1.prov, e2.prov)
κ (dependency coefficient) MUST be tracked or conservatively approximated.
Uncertainty combine MUST avoid independence assumptions by default.
4.2 Discharge (Mandatory Gate)
discharge(comp, thresholds, contract): Decision<T>

Succeed iff:

comp.effect.uncertainty.high ≥ θ_confidence
comp.effect.taint.influence ≤ θ_taint
comp.effect.cost.tokens[1] ≤ contract.remainingTokens
required capabilities authorized

Else → Ambiguous or None.

4.3 Sanitization (Downgrade, not erase)
sanitize(x, policy) → Computation<T>
influence' = influence × reduction(policy)
Only human::review(principal) MAY reduce to 0
Must emit provenance event with policy + outcome
5) Compiler (/lang)
5.1 Responsibilities
Lex/parse full ASL (S0–S3)
Type check:
Hindley–Milner core + affine constraints
effect typing (attach ε)
taint flow constraints
contract compatibility
Lower AST → IR
Emit S0 grammar (GBNF) + manifest
5.2 Modules
/lang
  lexer/
  parser/
  ast/
  typing/
    typechecker.ts
    effect-checker.ts
    taint-checker.ts
    contract-checker.ts
  lowering/
    ast-to-ir.ts
  grammar-export/
    gbnf.ts
  diagnostics/
5.3 Static Analyses
Effect soundness: all ops produce ε
Taint safety: no flow into capability without sanitize/discharge
Cost bounds: conservative over-approx
Contract compatibility: composition validity
6) IR (/ir)
6.1 Properties
Fully typed
Effect-annotated per instruction
SSA-like with explicit control flow
No implicit side effects
6.2 Instruction Set (Core)
Infer { out, schema, effectAnn, cfg }
Bind  { out, in }
Branch{ cond, thenBlock, elseBlock }
Loop  { body, fixpointCond }
Discharge { in, thresholds }
Perform { effectName, args, cap }
Sanitize { in, policy }
6.3 Verifier
/ir/verifier
  effect-soundness.ts
  taint-flow.ts
  contract-safety.ts
  cost-bounds.ts
Reject IR that violates invariants before execution
7) seedVM (/vm)
7.1 State
export interface VMState {
  env: Record<string, any>
  store: Map<string, any>
  effect: Effect
  contract: ContractState
  provenance: ProvenanceGraph
  rng: DeterministicRNG
}
7.2 Execution Model

Small-step interpreter:

⟨instr, state⟩ → ⟨state'⟩
Async-aware: instructions may suspend (inference/tool calls)
Deterministic given fixed inputs/seed
7.3 Executor
execute(ir: IRProgram, ctx: ExecCtx): Promise<VMState>
Applies instructions sequentially or via scheduler for parallel blocks
Accumulates ε via merge
Emits semantic events each step
7.4 Fixpoint / Loops
fix operator with termination on:
convergence metric
contract exhaustion
context budget
Must declare convergence criterion
8) Inference (/inference)
8.1 Requirements
Provider abstraction (OpenAI/Anthropic/local)
Constrained decoding (GBNF)
Schema validation + repair loop
Uncertainty estimation
Full trace logging (prompt, params, grammar hash)
8.2 Flow
build prompt + grammar
 → provider.generate()
 → incremental grammar check (if supported)
 → parse/validate schema
 → if fail: repair/retry (bounded)
 → return Computation<T>
8.3 Modules
/inference
  core/
    engine.ts
  providers/
  decoding/
    constrained.ts
  validation/
    schema.ts
    repair.ts
9) Contracts (/contracts)
9.1 Model
ContractState {
  maxTokens
  maxToolCalls
  maxDuration
  remainingTokens
  state: proposed|approved|active|completed|violated|expired
}
9.2 Enforcement
Checked per instruction
Atomic budget accounting (see Scheduler)
Violations → immediate transition + event
9.3 Algebra
Sequential A ∘ B
Parallel A || B
Nested A[B]
Conservation laws enforced at compile-time (same unit) and runtime (cross-unit)
10) Security (/security)
10.1 Taint
Causal influence tracking (float + lineage)
Propagation: max + transforms (attenuate/amplify)
10.2 Sanitization
Policies define (reduction, confidence)
Human review = only zeroing path
10.3 IFC (Runtime Guards)
Block discharge if influence > θ_taint
Record all sanitization decisions
11) Provenance (/provenance)
11.1 Semantic Events
Event =
  | InferCalled { input, cfg }
  | DecisionMade { decision, thresholds }
  | EffectExecuted { name, args }
  | ContractChecked { state }
  | Sanitized { policy, before, after }
11.2 Graph
DAG of events with hashes
Append-only; supports compaction (summaries) with hash preservation
11.3 Audit
Trajectory audit consumes events, not raw logs
Outputs proof or counterexample
12) Scheduler (/scheduler)
12.1 Responsibilities
Async orchestration of:
inference calls
tool calls
sub-agent tasks
Concurrency control
Fairness and prioritization
12.2 Atomic Budgeting
Central token/time ledger
Operations:
reserve(costInterval)
commit(actualCost)
rollback() on failure
Prevent race conditions across parallel branches
13) Network / Multi-Agent (/network)
13.1 Protocol
Typed messages (session types)
Contract-aware delegation (budget partitioning)
13.2 Identity
DID-based identities
Attach TEE attestation when required
13.3 Failure
Timeouts, partial results, retries encoded in protocol
14) Storage (/storage)
Key-value store with:
taint persistence
provenance references
Memory tiers:
P0 (system), P1 (working), P2 (episodic) with bounds
15) TEE (/tee)
Verify attestation before enabling capabilities
Modes:
boot_time / continuous / per_operation
Expose tee_trust ∈ [0,1] into runtime for policy decisions
16) CLI & SDK
/cli
  build | run | audit | test

/sdk
  js client for embedding runtime + APIs
17) Testing Strategy
17.1 Suites
Unit (per module)
Conformance (spec rules)
Property-based (effect invariants)
Fuzz (parser, IR)
Integration (end-to-end)
Replay tests (determinism)
17.2 Must-have Tests
Taint blocks effect without sanitize
Discharge thresholds gate effects
Cost overrun triggers violation
Deterministic replay identical outputs/events
Grammar-constrained decoding yields zero syntax errors
18) Observability
Structured logs (JSON)
Metrics:
latency per instruction
token usage
retry counts
Tracing:
correlation IDs across inference/tool calls
Debug:
step-by-step replay viewer (using provenance)
19) Performance & Scaling
Async everywhere (non-blocking inference/tooling)
Batching inference where possible
Caching:
prompt → response (keyed by grammar + params)
State compaction:
provenance summarization
taint lineage pruning with hash preservation
20) Deployment Topology
Stateless VM workers (horizontal scale)
Shared services:
Inference gateway
Provenance store
Contract ledger (for atomic budgeting)
Optional:
TEE-enabled nodes for high-trust workloads
21) Build & Release
Monorepo with package boundaries
CI:
lint + typecheck
all test suites
deterministic replay checks
Versioning:
lockstep versions for core packages
Artifacts:
compiler binary
VM runtime
CLI
22) Implementation Order (Strict)
/effects (canonical library)
/ir + verifier
/vm (executor + state + discharge)
/inference (with constrained decoding + validation)
/contracts (runtime enforcement + ledger)
/security (taint + sanitize integrated)
/provenance (events + DAG)
/lang (full compiler + grammar export)
/scheduler (async + atomic budgets)
/network + /storage + /tee
CLI/SDK, observability, benchmarks
23) Acceptance Criteria (Go/No-Go)
All core invariants enforced in tests
100% deterministic replay across runs
Zero syntax errors under constrained decoding in 1k-sample test
No contract overruns without violation events
Audit produces valid proof or counterexample on sample trajectories
Horizontal scale test passes (N workers, no budget races)
24) Known Hard Problems (Handled)
LLM schema mismatch → repair loop + constrained decoding
Non-determinism → seed + full trace logging
Latency → async + batching + caching
State explosion → compaction + pruning
Human-in-loop stalls → timeouts + escalation policies
Budget races → atomic ledger in scheduler
25) Final Statement

This architecture is complete and internally consistent with the v15 spec and addenda:

unified effect system
causal taint tracking
resource-bounded execution
deterministic VM
verifiable audit trail

If the implementation adheres to this document, the system will be correct-by-construction within defined bounds and deployable.

