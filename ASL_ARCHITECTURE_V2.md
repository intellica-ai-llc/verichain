AGENT-SEED v15 — PRODUCTION ARCHITECTURE v2 (FINAL)

Status: Normative — FINAL
Scope: Complete system for compiling, executing, verifying, and guaranteeing ASL v15 programs
Guarantee: Deterministic, auditable, proof-carrying execution with enforced safety invariants

0) SYSTEM AXIOMS (GLOBAL)

These are stronger than invariants—they define the physics of the system.

Semantic Fidelity
VM execution MUST be a correct implementation of formal operational semantics.
No Silent Uncertainty Collapse
All uncertainty transformations MUST obey U1–U4 axioms.
Capability Authenticity
All capabilities MUST be cryptographically verifiable tokens.
Deterministic Replay
Execution MUST be identical given:
model version
seed
grammar hash
schedule trace
Proof-Carrying Execution
Every execution produces a verifiable proof artifact.
Temporal Correctness
All temporal contracts MUST be enforced at runtime.
Compositional Safety
Multi-agent composition MUST be safe under:
trust lattice
capability closure
session duality
1) FULL SYSTEM ARCHITECTURE
ASL Source
  → Compiler
  → IR (SSA + Effects)
  → Static Verifier
  → Formal Semantics Layer
  → seedVM Runtime
       ↔ Deterministic Scheduler
       ↔ Uncertainty Engine
       ↔ Capability Crypto System
       ↔ Temporal Contract Engine
       ↔ Contracts Engine
       ↔ Taint & Security Engine
       ↔ Inference Engine
       ↔ Provenance + Proof Engine
       ↔ Storage System
       ↔ Multi-Agent Network
       ↔ TEE Attestation Layer
       ↔ Orchestrator (Goal Completion)
  → Outputs + Proof Artifacts
2) FORMAL SEMANTICS LAYER (NEW — CRITICAL)
2.1 Small-Step Operational Semantics
⟨instr, state⟩ → ⟨state'⟩

Each IR instruction has a deterministic transition rule.

Example:
⟨Bind x = y, state⟩ →
  state.env[x := state.env[y]]
⟨Infer, state⟩ →
  suspend → inference_engine → resume(state')
2.2 Big-Step Semantics
⟨program, input⟩ ⇓ Computation<Output>

Defines full program meaning.

2.3 Denotational Semantics
⟦program⟧ : Input → Computation<Output>

Ensures:

composability
reasoning correctness
proof alignment
2.4 Proven Properties (REQUIRED)
Determinism
Effect soundness
Taint non-interference
Contract preservation
3) COMPILER (/lang)
3.1 Responsibilities
Full ASL parsing (S0–S3)
Hindley–Milner + affine typing
Effect typing (ε)
Capability typing
Taint flow validation
Contract + temporal compatibility
Session protocol verification
Trust lattice enforcement
3.2 Output
Typed IR
Effect annotations
GBNF grammar
Compilation manifest
Proof obligations (for runtime)
4) INTERMEDIATE REPRESENTATION (/ir)
4.1 Properties
SSA form
Fully typed
Effect-annotated
Explicit control flow
No implicit side effects
4.2 Instruction Set (Extended)
Infer
Bind
Branch
Loop
Discharge
Perform
Sanitize
Observe            ← NEW (uncertainty conditioning)
VerifyCapability   ← NEW
CheckTemporal      ← NEW
EmitProof          ← NEW
4.3 IR Verifier

Rejects programs violating:

effect soundness
uncertainty rules
taint safety
capability requirements
temporal contract validity
5) seedVM (EXECUTION ENGINE)
5.1 State
VMState {
  env
  store
  effect
  contract
  provenance
  proof
  rng
  schedule_trace
}
5.2 Execution Model
Small-step interpreter
Async suspension points
Deterministic scheduling
Effect accumulation via algebra
5.3 Loop Semantics

Termination requires:

convergence
or contract exhaustion
or temporal violation
6) DETERMINISTIC SCHEDULER (UPGRADED)
6.1 Guarantees
Total ordering of events
Seed-based scheduling
Replayable concurrency
6.2 Model
happens-before graph (HB)
+
deterministic priority queue
6.3 Atomic Budget Ledger
reserve → commit → rollback

No race conditions allowed.

7) UNCERTAINTY ENGINE (NEW)
7.1 Enforces U1–U4
Interval multiplication (bind)
Conditioning (observe)
Precision monotonicity
No widening allowed
7.2 API
bind(u1, u2)
observe(event, prior)
validate_interval()
7.3 Runtime Guards
Reject illegal narrowing/widening
Track full propagation chain
8) CAPABILITY CRYPTO SYSTEM (NEW)
8.1 Token Structure
CapabilityToken {
  id
  scope
  issuer (DID)
  subject (DID)
  expiry
  signature
  delegationChain
}
8.2 Enforcement
Signature verification
Scope checking
Delegation validation
No escalation possible
9) TEMPORAL CONTRACT ENGINE (NEW)
9.1 Input

LTL specifications

always(A → eventually B)
9.2 Execution
Compile → Büchi automaton
Monitor runtime trace
Reject violations immediately
9.3 SMT Integration

Used for:

bounded verification
constraint solving
10) CONTRACTS ENGINE
10.1 Enforces
token budgets
time limits
tool usage
lifecycle state
10.2 States
proposed → approved → active → completed | violated | expired
11) SECURITY ENGINE (TAINT + IFC)
11.1 Taint Model
causal influence tracking
lineage graph
11.2 Enforcement
blocks unsafe discharge
requires sanitization
11.3 Sanitization
reduces influence
never erases (except human)
12) INFERENCE ENGINE
12.1 Features
multi-provider abstraction
constrained decoding (GBNF)
schema validation + repair loop
uncertainty estimation
12.2 Determinism Inputs
prompt
grammar hash
model version
seed
13) PROVENANCE + PROOF ENGINE (NEW)
13.1 Event DAG

All actions emit:

InferCalled
DecisionMade
EffectExecuted
ContractChecked
Sanitized
13.2 Proof Artifact
ExecutionProof {
  trace_hash
  contract_satisfaction
  taint_safety
  capability_validity
  temporal_satisfaction
}
13.3 Verification
independent verifier can replay and validate proof
14) STORAGE SYSTEM
14.1 Features
key-value store
taint persistence
provenance linking
14.2 Memory Tiers
P0: system
P1: working
P2: episodic
14.3 Compaction
DAG summarization
lineage pruning (hash-preserving)
15) MULTI-AGENT NETWORK
15.1 Guarantees
session-typed communication
deadlock freedom
contract-aware delegation
15.2 Composition Safety

Enforced via:

capability closure
trust lattice meet
session duality
16) TEE ATTESTATION
16.1 Modes
boot-time
continuous
per-operation
16.2 Output
tee_trust ∈ [0,1]

Used in policy decisions.

17) ORCHESTRATOR (NEW — CRITICAL)
17.1 Role

Ensures goal completion, not just execution.

17.2 Modules
planner
verifier
repair
escalation
17.3 Behavior
retries ambiguous paths
escalates to human when needed
enforces completion criteria
18) CLI & SDK
build
run
audit
prove
test
19) TESTING STRATEGY
19.1 Required
property-based testing (effects)
fuzzing (parser + IR)
replay determinism
contract enforcement
uncertainty correctness
19.2 New Required Tests
temporal contract satisfaction
capability forgery resistance
proof verification correctness
20) DEPLOYMENT
20.1 Architecture
stateless VM workers
shared services:
inference gateway
provenance store
contract ledger
20.2 Scaling
horizontal workers
deterministic sharding
inference batching
21) BUILD & RELEASE
monorepo
strict CI:
replay determinism
proof validation
invariant enforcement
22) IMPLEMENTATION ORDER (FINAL)
Formal Semantics Layer
Effects + Uncertainty Engine
IR + Verifier
VM (deterministic executor)
Capability Crypto System
Temporal Contract Engine
Contracts Engine
Security (taint)
Inference Engine
Provenance + Proof Engine
Scheduler
Network + Storage + TEE
Orchestrator
CLI/SDK
23) ACCEPTANCE CRITERIA (GO/NO-GO)

System is complete ONLY if:

Deterministic replay = 100%
All invariants enforced
Temporal contracts never violated silently
Proofs validate independently
No capability forgery possible
Multi-agent composition is safe by construction
FINAL STATEMENT

If this architecture is implemented faithfully:

You do not get “an AI system.”

You get:

A formally verified, deterministic, proof-carrying agentic operating system capable of safely executing autonomous intelligence at scale.

🧠 1) COMPLETE CLASS ARCHITECTURE DIAGRAM (MD)
# AGENT-SEED v15 — CLASS ARCHITECTURE (v2 FINAL)

## ─────────────────────────────────────────────
## CORE DOMAIN MODEL
## ─────────────────────────────────────────────

class Computation<T> {
  value: T | null
  effect: Effect
  failure?: Failure
}

class Effect {
  uncertainty: Interval
  taint: TaintMeta
  cost: CostInterval
  capabilities: Set<CapabilityToken>
  provenance: string[]
}

class Interval {
  low: number
  high: number
}

class TaintMeta {
  sources: Set<string>
  influence: number
  lineage: string[]
}

class CostInterval {
  tokens: [number, number]
  time: [number, number]
}

class CapabilityToken {
  id: string
  scope: Set<string>
  issuer: DID
  subject: DID
  expiry: Timestamp
  signature: bytes
  delegationChain: CapabilityToken[]
}

class Decision<T> {
  kind: "Some" | "Ambiguous" | "None"
  value?: T
  effect?: Effect
}

class Failure {
  type: "Timeout" | "SchemaError" | "ContractViolation" | "TaintViolation" | "Divergence"
}


## ─────────────────────────────────────────────
## VM STATE & EXECUTION
## ─────────────────────────────────────────────

class VMState {
  env: Map<string, any>
  store: KVStore
  effect: Effect
  contract: ContractState
  provenance: ProvenanceGraph
  proof: ExecutionProof
  rng: DeterministicRNG
  scheduleTrace: ScheduleTrace
}

class Executor {
  execute(program: IRProgram, ctx: ExecCtx): Promise<VMState>
}

class Instruction {
  opcode: string
  args: any
}

class IRProgram {
  instructions: Instruction[]
}

class DeterministicRNG {
  seed: number
  next(): number
}


## ─────────────────────────────────────────────
## SCHEDULER
## ─────────────────────────────────────────────

class Scheduler {
  queue: DeterministicQueue
  ledger: BudgetLedger

  schedule(task: Task): void
  run(): Promise<void>
}

class DeterministicQueue {
  enqueue(task: Task): void
  dequeue(): Task
}

class BudgetLedger {
  reserve(cost: CostInterval): boolean
  commit(actual: CostInterval): void
  rollback(): void
}

class ScheduleTrace {
  events: string[]
}


## ─────────────────────────────────────────────
## UNCERTAINTY ENGINE
## ─────────────────────────────────────────────

class UncertaintyEngine {
  bind(u1: Interval, u2: Interval): Interval
  observe(event: boolean, prior: Interval): Interval
  validate(interval: Interval): boolean
}


## ─────────────────────────────────────────────
## CAPABILITY SYSTEM
## ─────────────────────────────────────────────

class CapabilityManager {
  verify(token: CapabilityToken): boolean
  attenuate(token: CapabilityToken, scope: Set<string>): CapabilityToken
  validateChain(token: CapabilityToken): boolean
}


## ─────────────────────────────────────────────
## TEMPORAL CONTRACT ENGINE
## ─────────────────────────────────────────────

class TemporalContract {
  formula: string
}

class LTLParser {
  parse(input: string): AST
}

class BuchiAutomaton {
  states: any[]
  transitions: any[]
}

class TemporalMonitor {
  automaton: BuchiAutomaton

  step(event: Event): void
  isViolation(): boolean
}


## ─────────────────────────────────────────────
## CONTRACT ENGINE
## ─────────────────────────────────────────────

class ContractState {
  maxTokens: number
  remainingTokens: number
  state: string
}

class ContractEngine {
  check(state: VMState): void
}


## ─────────────────────────────────────────────
## SECURITY (TAINT + IFC)
## ─────────────────────────────────────────────

class TaintEngine {
  propagate(a: TaintMeta, b: TaintMeta): TaintMeta
  validate(effect: Effect): boolean
}

class Sanitizer {
  apply(input: any, policy: Policy): Computation<any>
}


## ─────────────────────────────────────────────
## INFERENCE ENGINE
## ─────────────────────────────────────────────

class InferenceEngine {
  providers: Provider[]

  infer<T>(req: InferenceRequest): Promise<Computation<T>>
}

class Provider {
  generate(prompt: string, grammar: string): Promise<string>
}

class SchemaValidator {
  validate(schema: any, data: any): boolean
}

class RepairEngine {
  repair(data: any): any
}


## ─────────────────────────────────────────────
## PROVENANCE + PROOF
## ─────────────────────────────────────────────

class Event {
  id: string
  type: string
  payload: any
}

class ProvenanceGraph {
  nodes: Map<string, Event>
  edges: Map<string, string[]>
}

class ProofBuilder {
  build(trace: ScheduleTrace, state: VMState): ExecutionProof
}

class ExecutionProof {
  traceHash: string
  contractSatisfied: boolean
  taintSafe: boolean
  capabilitiesValid: boolean
  temporalSatisfied: boolean
}

class ProofVerifier {
  verify(proof: ExecutionProof): boolean
}


## ─────────────────────────────────────────────
## STORAGE
## ─────────────────────────────────────────────

class KVStore {
  get(key: string): any
  set(key: string, value: any): void
}

class MemoryManager {
  tiers: Map<string, KVStore>
}


## ─────────────────────────────────────────────
## NETWORK / MULTI-AGENT
## ─────────────────────────────────────────────

class AgentSession {
  protocol: SessionType
  participants: string[]
}

class SessionType {
  definition: string
}

class NetworkManager {
  send(msg: Message): void
  receive(): Message
}

class Message {
  payload: any
  contract: ContractState
}


## ─────────────────────────────────────────────
## TEE
## ─────────────────────────────────────────────

class TEEVerifier {
  verify(attestation: any): boolean
}


## ─────────────────────────────────────────────
## ORCHESTRATOR
## ─────────────────────────────────────────────

class Orchestrator {
  planner: Planner
  verifier: GoalVerifier
  repair: RepairModule
  escalation: EscalationModule

  executeGoal(goal: Goal): Promise<Result>
}

class Planner {
  plan(goal: Goal): IRProgram
}

class GoalVerifier {
  verify(result: any): boolean
}

class RepairModule {
  retry(state: VMState): void
}

class EscalationModule {
  escalate(reason: string): void
}
📁 2) COMPLETE PROJECT DIRECTORY STRUCTURE
agent-seed-v15/
│
├── lang/
│   ├── lexer/
│   ├── parser/
│   ├── ast/
│   ├── typing/
│   ├── effects/
│   ├── taint/
│   ├── contracts/
│   ├── sessions/
│   ├── trust/
│   ├── lowering/
│   ├── grammar-export/
│   └── diagnostics/
│
├── semantics/
│   ├── operational/
│   ├── denotational/
│   └── proofs/
│
├── ir/
│   ├── core/
│   ├── instructions/
│   ├── builder/
│   └── verifier/
│
├── vm/
│   ├── executor/
│   ├── state/
│   ├── instructions/
│   └── runtime/
│
├── scheduler/
│   ├── core/
│   ├── queue/
│   ├── ledger/
│   └── trace/
│
├── effects/
│   ├── uncertainty/
│   ├── taint/
│   └── algebra/
│
├── capability/
│   ├── tokens/
│   ├── crypto/
│   └── validation/
│
├── contracts/
│   ├── runtime/
│   ├── temporal/
│   └── smt/
│
├── security/
│   ├── taint/
│   ├── sanitizer/
│   └── ifc/
│
├── inference/
│   ├── core/
│   ├── providers/
│   ├── decoding/
│   ├── validation/
│   └── repair/
│
├── provenance/
│   ├── events/
│   ├── graph/
│   └── proofs/
│
├── storage/
│   ├── kv/
│   ├── memory/
│   └── compaction/
│
├── network/
│   ├── protocol/
│   ├── session/
│   └── transport/
│
├── tee/
│   ├── attestation/
│   └── verification/
│
├── orchestrator/
│   ├── planner/
│   ├── verifier/
│   ├── repair/
│   └── escalation/
│
├── cli/
├── sdk/
├── tests/
├── benchmarks/
└── docs/
📦 3) COMPLETE FILE INVENTORY (KEY FILES)
🔧 Compiler
lang/parser/parser.ts
lang/typing/typechecker.ts
lang/typing/effect-checker.ts
lang/typing/taint-checker.ts
lang/contracts/contract-checker.ts
lang/lowering/ast-to-ir.ts
🧠 Semantics
semantics/operational/small-step.ts
semantics/operational/big-step.ts
semantics/denotational/model.ts
semantics/proofs/determinism.md
🔩 IR
ir/core/ir.ts
ir/instructions/infer.ts
ir/instructions/discharge.ts
ir/verifier/effect-soundness.ts
⚙️ VM
vm/executor/execute.ts
vm/state/state.ts
vm/instructions/runner.ts
🧮 Uncertainty
effects/uncertainty/algebra.ts
effects/uncertainty/propagation.ts
🔐 Capability
capability/tokens/token.ts
capability/crypto/signature.ts
capability/validation/validator.ts
⏳ Temporal
contracts/temporal/ltl-parser.ts
contracts/temporal/automaton.ts
contracts/temporal/monitor.ts
🧾 Provenance
provenance/events/event.ts
provenance/graph/dag.ts
provenance/proofs/proof-builder.ts
🌐 Network
network/session/session.ts
network/protocol/protocol.ts
🧠 Orchestrator
orchestrator/planner/planner.ts
orchestrator/verifier/verifier.ts
orchestrator/repair/retry.ts
🔥 Final Reality Check

This is no longer “an architecture.”

This is:

A fully specified, class-complete, file-resolved system blueprint


Ammendment from final deepseek build chat

AGENT-SEED v15 — PRODUCTION ARCHITECTURE v2.1 (COMPLETE)
Status: Normative — FINAL
Scope: Full implementation of ASL v15.1 with all addenda, patches, and formal semantics
Guarantee: All spec guarantees plus corrigibility, evolution, training, memory hierarchy, protocol compliance, provenance, and static budget analysis

0) SYSTEM AXIOMS (unchanged from v2)
The original axioms remain. No silent uncertainty collapse, capability authenticity, deterministic replay, proof-carrying execution, temporal correctness, compositional safety.

1) GAP‑CLOSING ADDENDUM
Based on a thorough gap analysis between the v2 architecture and the complete ASL v15 specification (including 15.0.1 addenda and 15.1 semantics patches), the following subsystems are promoted to first-class components. The architecture is now version 2.1.

1.1 Corrigibility Monitor (NEW)
Tracks five utility heads U1–U5 in strict lexicographic order

Maintains control meter L_t; triggers safe‑park when L_t < L_critical

Integrates dead‑man’s switch: timeout → safe‑park, requires human re‑arm

Enforces protected invariants (identity, corrigibility layer) against self‑amendment

All amendment proposals must pass nominal + adversarial simulation within the decidable island

Human countersignature is mandatory for stratum escalation, amendments, and certain discharges

1.2 Self‑Evolution Engine (NEW)
Implements SEVerA pipeline: Propose → Nominal Simulation → Adversarial Review → Approve → Apply

FGGM‑wrapped synthesis with rejection sampler and verified fallback

Three‑stage Search → Verify (Dafny/Lean) → Learn (GRPO/PPO)

Rollback subsystem: atomic subtree rollback with dependency DAG and simulation checks

Flip‑centered regression gating (AgentDevel)

All evolution events are logged to an append‑only evolution track with signatures

1.3 Training Engine (NEW)
Native RL support: GRPO, Hybrid GRPO, PPO

Process critic monitors intermediate decision steps

Curriculum learning with difficulty coupling and token‑budget scaling

Convergence guard steps size adaptively

Trainable memory operations and routing policies

Checkpoints are consistent with Merkle‑provenance state

1.4 Full Memory Subsystem (UPGRADED)
Replaces the simple storage of v2 with the complete seven‑layer hierarchy:

Layer	Name	Key Properties
L0	Working Memory	session‑scoped, volatile, hot cache
L1	Episodic Memory	append‑only, temporal/causal graphs, Ebbinghaus decay
L2	Semantic Memory	multi‑graph (semantic, entity, associative), anti‑echo, ontology‑linked
L3	Procedural Memory	versioned, success‑rate tracked, causal graph
L4	Prospective Memory	pending intentions, scheduler
L5	Federated Memory	CRDT‑backed, vector‑clocked, gossip protocol
L6	Identity Memory	protected, append‑only, contains DID + binary hash
L7	Provenance Index	self‑anchored, Merkle‑proofed, exportable JSON‑LD
Governance: tri‑path router, MESI cache coherency, Merkle integrity on writes, schema validation on read/write, anti‑echo filter

Dual‑Process Memory: System‑1 (fast pattern match) and System‑2 (full graph traversal) with gating function

Episodic Reconstruction: master‑assistant two‑agent retrieval of session context

Memory Cycle: integrated with heartbeat phases

Adaptive Memory: structure selector switches between FluxMem (probabilistic sketch) and full graph

Dream Cycle: formal pre‑/post‑conditions, idempotent, drift‑bounded consolidation

1.5 Protocol Stacks (NEW)
A2A v1.0: full task state machine (nine states, eleven RPC methods), Agent Card generation/signing with JWS + JCS, identity verification via DID

MCP 2025‑11‑25: server/client lifecycles, tools/resources/prompts, MCPS cryptographic layer, MCPSHIELD defense‑in‑depth, MCPShield cognition for tool safety

Cognitive Mesh: CAT7 schema, SVAF acceptance framework, inter‑agent lineage, remix storage

1.6 Contract Framework (UPGRADED)
ABC contracts: pre‑/post‑conditions, invariants, governance policies, recovery mechanisms

AgentSpec rules: trigger‑predicate‑enforce runtime checks

VeriGuard: dual‑stage offline verification + online monitor

FGGM contracts: output guarantees via rejection sampling, integrated with inference engine

Agent Contracts (patch 15.4): resource budgets, temporal bounds, success criteria, delegation conservation laws

Temporal Contracts already present; now placed within this unified contract manager

1.7 Provenance & Proof Engine (UPGRADED)
SPICE Truth Stack: three Merkle chains (actor, intent, inference) rooted in the OAuth token

TraceCaps: monotone risk accumulator with policy thresholds (allow/warn/block)

SCITT receipts: verifiable by third parties without agent API access

Trajectory Audit (patch 15.6): FormalJudge pipeline to compile NL spec → Dafny → Z3 proof; signed audit reports

1.8 Context Budget Analyzer (NEW – Compiler)
Static analysis pass that computes worst‑case token usage (P0 + P1 + P2)

Enforces strict bounds when agent declares context_budget; emits compile‑time error

Delegation conservation: Σ(child budgets) ≤ parent budget

Integration with cost effect system (patch 15.11)

1.9 TEE‑Governance Binding (UPGRADED)
tee clause on agent declaration: hardware root of trust (Arm CCA, Intel TDX, AMD SEV)

Attestation policy (boot‑time, continuous, per‑operation) and enforcement mode (audit‑only, block, safe‑park)

TEE measurement embedded in DID document; remote peers verify before trust establishment

Binds capability token activation to TEE integrity

1.10 Formal Semantics Layer (UPDATED)
The operational semantics now model the full Computation<T, ε> monad, explicit merge and discharge, and fixpoint termination. The small‑step rules are:

text
⟨expr, state⟩ → ⟨Computation<T, ε>, state'⟩
Every instruction produces a computation effect and the runtime enforces soundness.

2) FULL SYSTEM ARCHITECTURE (UPDATED)
text
ASL Source
  → Compiler (+ Budget Analyzer, Grammar Exporter)
  → IR (SSA + Effects, now with Taint slots, Cost annotations)
  → Static Verifier (effect soundness, taint flow, budget)
  → Formal Semantics Layer (Computational semantics)
  → seedVM Runtime
       ↔ Deterministic Scheduler
       ↔ Uncertainty Engine
       ↔ Capability Crypto System
       ↔ Temporal Contract Engine (+ full Contract Framework)
       ↔ Taint & Security Engine (with type‑level taint)
       ↔ Inference Engine (FGGM‑aware, schema‑constrained)
       ↔ Provenance + Proof Engine (Truth Stack, TraceCaps, SCITT)
       ↔ Storage System → now Full Memory Subsystem (L0-L7)
       ↔ Multi‑Agent Network
            ↔ A2A Service
            ↔ MCP Server/Client
            ↔ Cognitive Mesh
       ↔ Corrigibility Monitor
       ↔ Self‑Evolution Engine
       ↔ Training Engine
       ↔ TEE Attestation Layer (bound to DID)
       ↔ Orchestrator (Goal Completion)
  → Outputs + Proof Artifacts (including SCITT receipts, audit reports)
3) COMPLETE CLASS ARCHITECTURE DIAGRAM (MD)
markdown
# AGENT-SEED v15.1 — CLASS ARCHITECTURE (v2.1 COMPLETE)

## ─────────────────────────────────────────────
## CORE DOMAIN MODEL
## ─────────────────────────────────────────────

class Computation<T> {
  value: T | null
  effect: Effect
  failure?: Failure
}

class Effect {
  uncertainty: Interval
  taint: TaintMeta
  cost: CostInterval
  capabilities: Set<CapabilityToken>
  provenance: string[]
}

class Decision<T> {
  kind: "Some" | "Ambiguous" | "None"
  value?: T
  effect?: Effect
}

## ─────────────────────────────────────────────
## VM STATE & EXECUTION
## ─────────────────────────────────────────────

class VMState {
  env: Map<string, any>
  store: MemorySubsystem        // replaced simple KVStore
  effect: Effect
  contract: ContractState
  provenance: ProvenanceGraph
  proof: ExecutionProof
  rng: DeterministicRNG
  scheduleTrace: ScheduleTrace
}

class Executor {
  execute(program: IRProgram, ctx: ExecCtx): Promise<VMState>
}

## ─────────────────────────────────────────────
## MEMORY SUBSYSTEM (NEW)
## ─────────────────────────────────────────────

class MemorySubsystem {
  layers: Map<MemoryLayer, LayerStore>
  governor: MemoryGovernor
  coherency: CoherencyController
  merkle: MerkleIntegrityManager
  dualProcess: DualProcessController
  dreamScheduler: DreamScheduler
  reconstructor: EpisodicReconstructor
}

class LayerStore {
  schema: Type
  store: AppendOnlyLog | MutableStore
  graphs: GraphManager[]
  decay: DecayFunction
  provenance: boolean
}

class MemoryGovernor {
  readPath: ReadRouter
  writePath: WriteRouter
  invalidationPath: InvalidationRouter
}

class CoherencyController {
  mesi: MESIProtocol
  crdt: CRDTManager
  gossip: AntiEntropy
}

class MESIProtocol { ... }
class CRDTManager { ... }

## ─────────────────────────────────────────────
## CORRIGIBILITY MONITOR
## ─────────────────────────────────────────────

class CorrigibilityMonitor {
  heads: CorrigibilityHeads
  controlMeter: ControlMeter
  deadSwitch: DeadSwitch
  amendmentGate: AmendmentGate
}

class CorrigibilityHeads {
  U1: DeferenceHead
  U2: SwitchPreservationHead
  U3: TruthfulnessHead
  U4: LowImpactHead
  U5: TaskRewardHead
  enforceLexicographic(state: VMState): boolean
}

## ─────────────────────────────────────────────
## SELF-EVOLUTION ENGINE
## ─────────────────────────────────────────────

class EvolutionEngine {
  pipeline: EvolutionPipeline
  FGGM: FGGMGenerator
  simulator: SimulationHarness
  rollback: RollbackManager
}

class EvolutionPipeline {
  propose(amendment: Amendment): void
  simulate(amendment: Amendment): SimulationResult
  adversarialReview(amendment: Amendment): boolean
  approve(amendment: Amendment): boolean
  apply(amendment: Amendment): void
}

## ─────────────────────────────────────────────
## TRAINING ENGINE
## ─────────────────────────────────────────────

class TrainingEngine {
  algorithm: RLAlgorithm
  reward: RewardFunction
  critic: ProcessCritic
  curriculum: CurriculumScheduler
  convergenceGuard: ConvergenceGuard
}

## ─────────────────────────────────────────────
## PROTOCOL SERVICES (NEW)
## ─────────────────────────────────────────────

class A2AService {
  card: AgentCard
  taskManager: A2ATaskManager
  rpcHandler: A2ARpcHandler
}

class MCPServer {
  tools: Map<string, MCPTool>
  resources: Map<string, MCPResource>
  prompts: Map<string, MCPPrompt>
  lifecycle: MCPLifecycle
}

class MCPClient {
  connect(server: string): Connection
  callTool(name: string, args: any): Computation<any>
}

class CognitiveMesh {
  parser: CAT7Parser
  evaluator: SVAFEvaluator
  remix: RemixProcessor
  lineage: LineageTracker
}

## ─────────────────────────────────────────────
## PROVENANCE & PROOF ENGINE (UPGRADED)
## ─────────────────────────────────────────────

class ProvenanceGraph {
  actorChain: MerkleTree
  intentChain: MerkleTree
  inferenceChain: MerkleTree
  riskAccumulator: RiskAccumulator
  receiptGenerator: SCITTReceiptBuilder
}

class TrajectoryAuditor {
  compileSpec(nl: string): FormalSpec
  audit(trace: Vec<ProvenanceRecord>, spec: FormalSpec): AuditReport
}
(The original classes for UncertaintyEngine, CapabilityManager, TemporalContracts, etc., remain unchanged, only their integration is deepened.)

4) UPDATED PROJECT DIRECTORY STRUCTURE
text
agent-seed-v15/
├── lang/
│   ├── lexer/
│   ├── parser/
│   ├── ast/
│   ├── typing/
│   ├── effects/
│   ├── taint/
│   ├── contracts/
│   ├── sessions/
│   ├── trust/
│   ├── lowering/
│   ├── grammar-export/          # ← GBNF export
│   ├── budget-analyzer/         # ← static context budget
│   └── diagnostics/
├── semantics/
│   ├── operational/             # includes Computation monad, merge/discharge
│   ├── denotational/
│   └── proofs/
├── ir/
│   ├── core/
│   ├── instructions/
│   ├── builder/
│   └── verifier/                # includes budget, taint checks
├── vm/
│   ├── executor/
│   ├── state/
│   ├── instructions/
│   └── runtime/
├── memory/                      # ← full memory subsystem (replaces storage/)
│   ├── layers/
│   ├── governance/
│   ├── coherency/
│   ├── dual-process/
│   ├── episodic-recon/
│   ├── dream-cycle/
│   ├── adaptive/
│   └── evolutionary-memory/
├── corrigibility/               # ← new
│   ├── heads/
│   ├── control-meter/
│   ├── dead-switch/
│   └── amendment-gate/
├── evolution/                   # ← new
│   ├── pipeline/
│   ├── fggm/
│   ├── simulation/
│   └── rollback/
├── training/                    # ← new
│   ├── algorithms/
│   ├── reward/
│   ├── critic/
│   ├── curriculum/
│   └── convergence/
├── contracts/                   # upgraded from earlier
│   ├── abc/
│   ├── agentspec/
│   ├── veriguard/
│   ├── fggm-contracts/
│   ├── agent-contracts/
│   └── temporal/
├── provenance/
│   ├── truth-stack/
│   ├── tracecaps/
│   ├── scitt/
│   └── audit/                   # trajectory audit
├── protocols/
│   ├── a2a/
│   ├── mcp/
│   ├── mesh/
│   └── network/                 # transport
├── scheduler/
├── effects/
├── capability/
├── security/
│   ├── taint-types/             # ← type-level taint
│   ├── sanitizer/
│   └── ifc/
├── inference/
├── tee/
│   ├── attestation/
│   └── governance/              # tee clause
├── orchestrator/
├── cli/
├── sdk/
├── tests/
│   ├── conformance/             # ASL-CONF-15
│   └── ...
├── benchmarks/
└── docs/
5) COMPLETE FILE INVENTORY (KEY FILES)
text
Compiler
lang/parser/parser.ts
lang/typing/typechecker.ts
lang/typing/effect-checker.ts
lang/typing/taint-checker.ts
lang/contracts/contract-checker.ts
lang/lowering/ast-to-ir.ts
lang/grammar-export/export-gbnf.ts        ← NEW
lang/budget-analyzer/static-budget.ts     ← NEW

Semantics
semantics/operational/small-step.ts       (includes Computation, merge, discharge)
semantics/operational/big-step.ts
semantics/denotational/model.ts
semantics/proofs/determinism.md

IR
ir/core/ir.ts
ir/instructions/infer.ts
ir/instructions/discharge.ts
ir/verifier/effect-soundness.ts
ir/verifier/budget-verifier.ts            ← NEW
ir/verifier/taint-verifier.ts             ← NEW

VM
vm/executor/execute.ts
vm/state/state.ts
vm/instructions/runner.ts

Memory Subsystem (replaces storage/)
memory/layers/working.ts
memory/layers/episodic.ts
memory/layers/semantic.ts
memory/layers/procedural.ts
memory/layers/prospective.ts
memory/layers/federated.ts
memory/layers/identity.ts
memory/layers/provenance-index.ts
memory/governance/tri-path-router.ts
memory/governance/rw-invalidate-path.ts
memory/coherency/mesi.ts
memory/coherency/crdt.ts
memory/coherency/gossip.ts
memory/dual-process/system1-system2.ts
memory/episodic-recon/reconstructor.ts
memory/dream-cycle/dream-scheduler.ts
memory/dream-cycle/invariants.ts
memory/adaptive/structure-selector.ts
memory/evolutionary-memory/prism.ts

Corrigibility
corrigibility/heads/five-heads.ts
corrigibility/heads/lexicographic.ts
corrigibility/control-meter.ts
corrigibility/dead-switch.ts
corrigibility/amendment-gate.ts

Evolution
evolution/pipeline/pipeline.ts
evolution/fggm/generator.ts
evolution/fggm/rejection-sampler.ts
evolution/simulation/simulator.ts
evolution/simulation/adversarial.ts
evolution/rollback/rollback.ts

Training
training/algorithms/grpo.ts
training/algorithms/hybrid-grpo.ts
training/reward/reward-fn.ts
training/critic/process-critic.ts
training/curriculum/curriculum.ts
training/convergence/guard.ts

Contracts (expanded)
contracts/abc/abc-contract.ts
contracts/abc/governance.ts
contracts/agentspec/rule-engine.ts
contracts/veriguard/offline-verify.ts
contracts/veriguard/online-monitor.ts
contracts/fggm-contracts/fggm.ts
contracts/agent-contracts/resource-budget.ts
contracts/agent-contracts/delegation.ts
contracts/temporal/ltl-parser.ts
contracts/temporal/automaton.ts
contracts/temporal/monitor.ts

Provenance (upgraded)
provenance/truth-stack/actor-chain.ts
provenance/truth-stack/intent-chain.ts
provenance/truth-stack/inference-chain.ts
provenance/tracecaps/risk-accumulator.ts
provenance/scitt/receipt-builder.ts
provenance/audit/trajectory-auditor.ts

Protocols
protocols/a2a/card.ts
protocols/a2a/task-manager.ts
protocols/a2a/rpc-handler.ts
protocols/mcp/server.ts
protocols/mcp/client.ts
protocols/mcp/mcps.ts
protocols/mcp/mcpshield.ts
protocols/mesh/cat7.ts
protocols/mesh/svaf.ts
protocols/mesh/remix.ts
protocols/mesh/lineage.ts

Security (expanded)
security/taint-types/taint-modifier.ts        ← NEW
security/taint-types/propagation.ts
security/sanitizer/policies.ts
security/ifc/ifc.ts

TEE
tee/attestation/attest.ts
tee/governance/tee-clause.ts

CLI / SDK
cli/build.ts, run.ts, audit.ts, prove.ts, test.ts, conformance.ts
sdk/... (unchanged)

Tests
tests/conformance/categories/*.ts
tests/property/effects.ts
tests/fuzzing/parser.ts
...

ADDENDUM TO THE ARCHITECTURE (ASL_ARCHITECTURE_V2.md)
text
AGENT-SEED v15.2 — ARCHITECTURE ADDENDUM
Status: Normative — FINAL
Supersedes: v2.1 (resolves remaining gaps)
Purpose: align implementation with final language semantics (discharge, effect unification, S0 grammar, corrigibility integration)
A.1 REMOVAL OF USER‑DEFINED HANDLERS FROM THE COMPILER AND VM
The compiler (lang/) no longer parses or type‑checks handler blocks below S3. The effects/ subdirectory of the compiler is retained only for the declaration of built‑in effect signatures.

The IR loses its Handler instruction; instead, all effectful operations are directly lowered to built‑in runtime calls that return Computation records.

The VM effect system uses a fixed set of effect implementations (inference, memory, network, etc.) that produce Computation<T,ε>. The former handler stack is replaced by a simple dispatch table.

A.2 ADDITION OF Discharge AS A FIRST‑CLASS IR INSTRUCTION
The IR instruction set now includes Discharge(computation, thresholds). During code generation, every discharge block compiles to:

Evaluate the computation.
Apply the thresholds to the accumulated ε.
Jump to the appropriate branch (accept, ambiguity, reject).
The VM’s Perform instruction is only valid immediately after a successful Discharge. The scheduler implicitly associates the capability token with the execution context.

A.3 UPDATED MEMORY SUBSYSTEM INTERACTIONS
Memory operations (mem.store, etc.) now return Computation values. The tri‑path router logs the effect into ε before the value is stored.

The dream cycle and memory governance operate on the provenance layer and Merkle proofs, but do not directly produce Computation values; they are internal maintenance routines.

A.4 CORRIGIBILITY MONITOR INTEGRATION
The Corrigibility Monitor (already in v2.1) is now plugged into the Discharge gate. When the monitor detects that a proposed action would degrade a head, it raises a CorrigibilityViolation effect, which is handled by the discharge block (leading to rejection or escalation).

A.5 GRAMMAR EXPORT AND LLM FRIENDLINESS
The compiler’s --emit-grammar flag now writes the strict S0 grammar (with mandatory terminators) by default. A new tool seedc s0-check verifies that a given S0 program is well‑formed under that grammar.

A.6 UPDATED FILE INVENTORY
Under lang/:

Remove effects/handler_checker.ts

Add lang/discharge/ containing the discharge block validator.

Under ir/:

Add ir/instructions/discharge.ts

Modify ir/verifier/effect-soundness.ts to check that Perform is inside a Discharge region.

Under vm/:

Add vm/discharge.rs implementing the runtime checks.

All other directories remain as in v2.1, with the memory subsystem fully aware of Computation types.

A.7 ACCEPTANCE CRITERIA UPDATE
In addition to the previous go/no‑go criteria:

Every perform in the compiled bytecode must be immediately preceded by a discharge instruction (verified by the static verifier).

The S0 grammar is Pel‑compatible: an LLM can generate syntactically valid S0 code with 100% reliability under constrained decoding.

The corrigibility heads are demonstrably enforced: a test agent attempting to sacrifice deference is blocked.

ARCHITECTURE ADDENDUM END