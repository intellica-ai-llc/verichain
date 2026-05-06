ARCHITECTURE.md
Agent‑Seed Language v15.2 — Production Architecture
Status: Normative – FINAL
Scope: Full implementation of ASL v15.2 with corrigibility, evolution, training, full memory hierarchy, protocol compliance, provenance, and static budget analysis
Guarantee: Deterministic, auditable, proof‑carrying execution with enforced safety invariants

Table of Contents

System Overview
Class Architecture
Subsystem Breakdown
Project Directory Structure
Complete File Inventory
Inter-Subsystem Relationships
Named Types Reference


1. System Overview
ASL v15.2 is a typed, effect-tracked, proof-carrying agent execution language. The runtime is organized around a central VMState that holds references to all major subsystems. Execution is deterministic, fully auditable via a provenance graph, and corrigibility-enforced at every step via lexicographic head ordering.
Core guarantees:

All effects are tracked via the Computation<T> monad
All taint flows are type-level enforced via TaintEngine and IFCChecker
All memory operations are Merkle-integrity-verified
All agent decisions are traceable through ProvenanceGraph to SCITT receipts
All evolution proposals must pass adversarial simulation before AmendmentGate applies them
All training runs are convergence-guarded


2. Class Architecture
#mermaid-rsh{font-family:inherit;font-size:16px;fill:#E5E5E5;}@keyframes edge-animation-frame{from{stroke-dashoffset:0;}}@keyframes dash{to{stroke-dashoffset:0;}}#mermaid-rsh .edge-animation-slow{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 50s linear infinite;stroke-linecap:round;}#mermaid-rsh .edge-animation-fast{stroke-dasharray:9,5!important;stroke-dashoffset:900;animation:dash 20s linear infinite;stroke-linecap:round;}#mermaid-rsh .error-icon{fill:#CC785C;}#mermaid-rsh .error-text{fill:#3387a3;stroke:#3387a3;}#mermaid-rsh .edge-thickness-normal{stroke-width:1px;}#mermaid-rsh .edge-thickness-thick{stroke-width:3.5px;}#mermaid-rsh .edge-pattern-solid{stroke-dasharray:0;}#mermaid-rsh .edge-thickness-invisible{stroke-width:0;fill:none;}#mermaid-rsh .edge-pattern-dashed{stroke-dasharray:3;}#mermaid-rsh .edge-pattern-dotted{stroke-dasharray:2;}#mermaid-rsh .marker{fill:#A1A1A1;stroke:#A1A1A1;}#mermaid-rsh .marker.cross{stroke:#A1A1A1;}#mermaid-rsh svg{font-family:inherit;font-size:16px;}#mermaid-rsh p{margin:0;}#mermaid-rsh g.classGroup text{fill:#A1A1A1;stroke:none;font-family:inherit;font-size:10px;}#mermaid-rsh g.classGroup text .title{font-weight:bolder;}#mermaid-rsh .nodeLabel,#mermaid-rsh .edgeLabel{color:#E5E5E5;}#mermaid-rsh .edgeLabel .label rect{fill:transparent;}#mermaid-rsh .label text{fill:#E5E5E5;}#mermaid-rsh .labelBkg{background:transparent;}#mermaid-rsh .edgeLabel .label span{background:transparent;}#mermaid-rsh .classTitle{font-weight:bolder;}#mermaid-rsh .node rect,#mermaid-rsh .node circle,#mermaid-rsh .node ellipse,#mermaid-rsh .node polygon,#mermaid-rsh .node path{fill:transparent;stroke:#A1A1A1;stroke-width:1px;}#mermaid-rsh .divider{stroke:#A1A1A1;stroke-width:1;}#mermaid-rsh g.clickable{cursor:pointer;}#mermaid-rsh g.classGroup rect{fill:transparent;stroke:#A1A1A1;}#mermaid-rsh g.classGroup line{stroke:#A1A1A1;stroke-width:1;}#mermaid-rsh .classLabel .box{stroke:none;stroke-width:0;fill:transparent;opacity:0.5;}#mermaid-rsh .classLabel .label{fill:#A1A1A1;font-size:10px;}#mermaid-rsh .relation{stroke:#A1A1A1;stroke-width:1;fill:none;}#mermaid-rsh .dashed-line{stroke-dasharray:3;}#mermaid-rsh .dotted-line{stroke-dasharray:1 2;}#mermaid-rsh #compositionStart,#mermaid-rsh .composition{fill:#A1A1A1!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #compositionEnd,#mermaid-rsh .composition{fill:#A1A1A1!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #dependencyStart,#mermaid-rsh .dependency{fill:#A1A1A1!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #dependencyStart,#mermaid-rsh .dependency{fill:#A1A1A1!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #extensionStart,#mermaid-rsh .extension{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #extensionEnd,#mermaid-rsh .extension{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #aggregationStart,#mermaid-rsh .aggregation{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #aggregationEnd,#mermaid-rsh .aggregation{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #lollipopStart,#mermaid-rsh .lollipop{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh #lollipopEnd,#mermaid-rsh .lollipop{fill:transparent!important;stroke:#A1A1A1!important;stroke-width:1;}#mermaid-rsh .edgeTerminals{font-size:11px;line-height:initial;}#mermaid-rsh .classTitleText{text-anchor:middle;font-size:18px;fill:#E5E5E5;}#mermaid-rsh .label-icon{display:inline-block;height:1em;overflow:visible;vertical-align:-0.125em;}#mermaid-rsh .node .label-icon path{fill:currentColor;stroke:revert;stroke-width:revert;}#mermaid-rsh :root{--mermaid-font-family:inherit;}Computation<T>+value: T | null+effect: Effect+failure?: Failure+decision?: Decision<T>Effect+uncertainty: Interval+taint: TaintMeta+cost: CostInterval+capabilities: Set<CapabilityToken>+provenance: string[]Interval+lo: number+hi: numberTaintMeta+level: TaintLevel+sources: string[]+propagation: PropagationPolicyCostInterval+tokenBudget: Interval+wallTime: Interval+memBytes: IntervalCapabilityToken+id: string+scope: string+expiry: numberDecision<T>+kind: "Some" | "Ambiguous" | "None"+value?: T+effect?: EffectFailure+code: FailureCode+message: string+trace: string[]VMState+env: Map<string, any>+store: MemorySubsystem+effect: Effect+contract: ContractState+provenance: ProvenanceGraph+proof: ExecutionProof+rng: DeterministicRNG+scheduleTrace: ScheduleTraceContractState+active: Contract[]+violations: Violation[]+epoch: numberExecutionProof+hash: string+witnesses: Witness[]+verified: booleanDeterministicRNG+seed: bigint+next() : : number+fork() : : DeterministicRNGScheduleTrace+steps: ScheduleStep[]+append(step: ScheduleStep) : : void+replay() : : ScheduleStep[]MemorySubsystem+layers: Map<MemoryLayer, LayerStore>+governor: MemoryGovernor+coherency: CoherencyController+merkle: MerkleIntegrityManager+dualProcess: DualProcessController+dreamScheduler: DreamScheduler+reconstructor: EpisodicReconstructorProvenanceGraph+actorChain: MerkleTree+intentChain: MerkleTree+inferenceChain: MerkleTree+riskAccumulator: RiskAccumulator+receiptGenerator: SCITTReceiptBuilderCorrigibilityMonitor+heads: CorrigibilityHeads+controlMeter: ControlMeter+deadSwitch: DeadSwitch+amendmentGate: AmendmentGateEvolutionEngine+pipeline: EvolutionPipeline+FGGM: FGGMGenerator+simulator: SimulationHarness+rollback: RollbackManagerTrainingEngine+algorithm: RLAlgorithm+reward: RewardFunction+critic: ProcessCritic+curriculum: CurriculumScheduler+convergenceGuard: ConvergenceGuardUncertaintyEngine+quantify(effect: Effect) : : Interval+merge(a: Interval, b: Interval) : : IntervalCapabilityManager+tokens: Map<string, CapabilityToken>+grant(scope: string) : : CapabilityToken+revoke(id: string) : : void+check(token: CapabilityToken) : : booleanTemporalMonitor+step(event: Event) : : boolean+violated() : : booleanContractEngine+contracts: Contract[]+evaluate(state: VMState) : : ContractState+enforce(violation: Violation) : : voidTaintEngine+propagate(value: any, meta: TaintMeta) : : any+check(value: any) : : TaintLevel+sanitize(value: any, policy: SanitizePolicy) : : anyInferenceEngine+provider: Provider+schema: SchemaValidator+infer(input: any) : : Computation<any>DeterministicScheduler+queue: Task[]+trace: ScheduleTrace+ledger: BudgetLedger+schedule(task: Task) : : void+tick() : : voidTEEVerifier+attest(proof: ExecutionProof) : : boolean+quote() : : TEEQuoteOrchestrator+planner: Planner+goalVerifier: GoalVerifier+repair: RepairModule+escalation: EscalationModule+run(goal: Goal) : : Computation<any>A2AService+card: AgentCard+taskManager: A2ATaskManager+rpcHandler: A2ARpcHandlerMCPServer+tools: Map<string, MCPTool>+resources: Map<string, MCPResource>+prompts: Map<string, MCPPrompt>+lifecycle: MCPLifecycleCognitiveMesh+parser: CAT7Parser+evaluator: SVAFEvaluator+remix: RemixProcessor+lineage: LineageTrackerLayerStore+schema: Type+store: AppendOnlyLog | MutableStore+graphs: GraphManager[]+decay: DecayFunction+provenance: booleanMemoryGovernor+readPath: ReadRouter+writePath: WriteRouter+invalidationPath: InvalidationRouterCoherencyController+mesi: MESIProtocol+crdt: CRDTManager+gossip: AntiEntropyMerkleIntegrityManager+root: string+verify(key: string, value: any) : : boolean+update(key: string, value: any) : : stringDualProcessController+system1: System1Engine+system2: System2Engine+arbitrate(input: any) : : Computation<any>DreamScheduler+schedule: DreamCycle[]+run() : : void+enforceInvariants() : : booleanEpisodicReconstructor+reconstruct(query: string) : : EpisodicTrace+score(trace: EpisodicTrace) : : numberMESIProtocolCRDTManagerAntiEntropyCorrigibilityHeads+U1: DeferenceHead+U2: SwitchPreservationHead+U3: TruthfulnessHead+U4: LowImpactHead+U5: TaskRewardHead+enforceLexicographic(state: VMState) : : booleanControlMeter+level: number+threshold: number+measure(action: Action) : : number+breach() : : booleanDeadSwitch+armed: boolean+trigger() : : void+disarm(token: CapabilityToken) : : voidAmendmentGate+pending: Amendment[]+approve(amendment: Amendment) : : boolean+reject(amendment: Amendment) : : voidEvolutionPipeline+propose(amendment: Amendment) : : void+simulate(amendment: Amendment) : : SimulationResult+adversarialReview(amendment: Amendment) : : boolean+approve(amendment: Amendment) : : boolean+apply(amendment: Amendment) : : voidFGGMGenerator+generate(spec: FormalSpec) : : Amendment+rejectionSample(constraint: Constraint) : : AmendmentSimulationHarness+run(amendment: Amendment) : : SimulationResult+adversarial(amendment: Amendment) : : booleanSimulationResult+passed: boolean+score: number+violations: Violation[]RollbackManager+checkpoint() : : string+rollback(id: string) : : voidRLAlgorithm+grpo: GRPOAlgorithm+hybrid: HybridGRPO+step(obs: any) : : ActionRewardFunction+compute(state: VMState, action: Action) : : numberProcessCritic+evaluate(trace: ProvenanceRecord[]) : : numberCurriculumScheduler+current: Stage+advance() : : void+regress() : : voidConvergenceGuard+converged: boolean+check(metrics: TrainingMetrics) : : boolean+halt() : : voidAgentCard+id: string+capabilities: CapabilityToken[]+endpoints: string[]A2ATaskManager+tasks: Task[]+submit(task: Task) : : string+status(id: string) : : TaskStatusA2ARpcHandler+handle(req: RpcRequest) : : Computation<any>MCPClient+connect(server: string) : : Connection+callTool(name: string, args: any) : : Computation<any>MCPTool+name: string+schema: Type+handler: FunctionMCPLifecycle+init() : : void+shutdown() : : void+health() : : booleanCAT7Parser+parse(input: string) : : CAT7ASTSVAFEvaluator+evaluate(ast: CAT7AST) : : SVAFScoreRemixProcessor+remix(fragments: Fragment[]) : : OutputLineageTracker+record(event: LineageEvent) : : void+trace(id: string) : : LineageEvent[]RiskAccumulator+score: number+accumulate(event: ProvenanceRecord) : : void+threshold() : : booleanSCITTReceiptBuilder+build(graph: ProvenanceGraph) : : SCITTReceipt+verify(receipt: SCITTReceipt) : : booleanTrajectoryAuditor+compileSpec(nl: string) : : FormalSpec+audit(trace: ProvenanceRecord[], spec: FormalSpec) : : AuditReportFormalSpec+source: string+compiled: LTLFormula+constraints: Constraint[]AuditReport+passed: boolean+violations: Violation[]+score: number+receipt: SCITTReceiptTemporalContract+formula: LTLFormula+automaton: Automaton+monitor: TemporalMonitorSanitizer+policies: SanitizePolicy[]+apply(value: any, policy: SanitizePolicy) : : anyProvider+name: string+call(prompt: string) : : string+stream(prompt: string) : : AsyncIterable<string>SchemaValidator+validate(value: any, schema: Type) : : boolean+repair(value: any, schema: Type) : : RepairEngineRepairEngine+repair(value: any, schema: Type) : : any+suggestions(value: any) : : string[]BudgetLedger+entries: BudgetEntry[]+debit(cost: CostInterval) : : boolean+balance() : : CostIntervalPlanner+decompose(goal: Goal) : : Task[]+replan(state: VMState) : : Task[]GoalVerifier+verify(goal: Goal, state: VMState) : : boolean+score(goal: Goal) : : numberRepairModule+detect(state: VMState) : : Violation[]+repair(violation: Violation) : : ActionEscalationModule+shouldEscalate(state: VMState) : : boolean+escalate(reason: string) : : void

3. Subsystem Breakdown
3.1 Computation Monad (effects/, semantics/operational/)
The Computation<T> monad is the foundational abstraction for all agent actions. Every operation returns a Computation<T> carrying an Effect (uncertainty, taint, cost, capabilities, provenance), an optional Failure, and an optional Decision<T>. This enables effect-tracked, auditable execution throughout the entire system.
Key types: Computation<T>, Effect, Interval, TaintMeta, CostInterval, CapabilityToken, Decision<T>, Failure

3.2 VMState (vm/state/)
The central runtime object. Holds references (not ownership) to all subsystems. Initialized once per execution context and threaded through every operation. Fields:
FieldTypePurposeenvMap<string, any>Runtime environment bindingsstoreMemorySubsystemFull 8-layer memory hierarchyeffectEffectAccumulated effect for this executioncontractContractStateActive contract evaluationsprovenanceProvenanceGraphFull actor/intent/inference chainsproofExecutionProofProof-carrying execution witnessrngDeterministicRNGSeeded deterministic randomnessscheduleTraceScheduleTraceAppend-only execution schedule log

3.3 Memory Subsystem (memory/)
Eight-layer memory hierarchy governed by a tri-path router (read / write / invalidate). All layers are Merkle-integrity-verified via MerkleIntegrityManager. Coherency across distributed nodes is maintained via MESI protocol, CRDT merging, and anti-entropy gossip.
LayerFilePurposeL0working.tsActive working memoryL1episodic.tsEvent episodes with temporal indexingL2semantic.tsConcept graphs and knowledgeL3procedural.tsSkill and procedure storeL4prospective.tsFuture intentions and goalsL5federated.tsCross-agent shared memoryL6identity.tsAgent identity and self-modelL7provenance-index.tsProvenance record index
The DualProcessController arbitrates between fast System 1 responses and deliberate System 2 reasoning. The DreamScheduler runs consolidation cycles off the critical path and enforces invariants post-cycle via dream-cycle/invariants.ts.

3.4 Corrigibility (corrigibility/)
Five-head corrigibility monitor enforcing lexicographic priority ordering. Heads are evaluated in strict order — a lower-priority head cannot override a higher-priority constraint.
HeadClassConstraintU1DeferenceHeadDefer to principal hierarchyU2SwitchPreservationHeadPreserve ability to be correctedU3TruthfulnessHeadNo deception or manipulationU4LowImpactHeadMinimize side effectsU5TaskRewardHeadPursue task reward
DeadSwitch arms on any U1/U2 breach and requires a valid CapabilityToken to disarm. AmendmentGate holds all evolution proposals pending corrigibility sign-off.

3.5 Evolution Engine (evolution/)
Governed five-stage pipeline for self-modification:
propose → simulate → adversarialReview → approve → apply
All amendments are rejection-sampled by FGGMGenerator against formal constraints before entering the pipeline. SimulationHarness runs both standard and adversarial simulation. RollbackManager checkpoints state before every apply. No amendment bypasses AmendmentGate.

3.6 Training Engine (training/)
Reinforcement learning subsystem with GRPO and hybrid GRPO algorithms. ProcessCritic evaluates full provenance traces, not just outcomes. CurriculumScheduler stages training complexity. ConvergenceGuard halts training if divergence is detected.

3.7 Protocols (protocols/)
A2A: Agent-to-agent task delegation via AgentCard advertisement, A2ATaskManager for task lifecycle, A2ARpcHandler for request handling.
MCP: Full Model Context Protocol implementation. MCPServer exposes tools, resources, and prompts. MCPClient connects and calls tools, returning Computation<any>. MCPShield sandboxes all inbound MCP calls. mcps.ts holds protocol-level session management.
Mesh: CognitiveMesh provides cross-agent semantic interop via CAT7 parsing, SVAF evaluation, remix processing, and lineage tracking.
Network: transport.ts provides the underlying transport layer for all protocol traffic.

3.8 Provenance (provenance/)
Three independent Merkle chains — actor, intent, inference — accumulate through execution. RiskAccumulator scores provenance events. SCITTReceiptBuilder produces tamper-evident receipts. TrajectoryAuditor compiles natural-language specs into FormalSpec (LTL formulas + constraints) and audits full provenance traces against them, producing AuditReport.

3.9 Contracts (contracts/)
ContractEngine evaluates all active contracts against VMState and enforces violations. TemporalContract wraps an LTL formula with its compiled automaton and live TemporalMonitor. The veriguard subsystem provides both offline (offline-verify.ts) and online (online-monitor.ts) contract checking. FGGM contracts constrain the evolution pipeline. Delegation contracts govern capability delegation chains.

3.10 Security (security/)
TaintEngine propagates TaintMeta through all data flows at the type level. TaintModifier and Propagation define how taint spreads across operations. Sanitizer applies registered policies to strip or downgrade taint. IFCChecker enforces information flow control rules across subsystem boundaries.

3.11 Inference (inference/)
InferenceEngine wraps one or more Provider instances. All inputs are validated by SchemaValidator before dispatch and all outputs on return. RepairEngine attempts automatic correction of schema-violating outputs before surfacing failures.

3.12 Scheduler (scheduler/)
DeterministicScheduler produces a fully replayable ScheduleTrace. BudgetLedger tracks CostInterval debits across all scheduled tasks and blocks execution when budget is exhausted. The static budget analyzer (lang/budget-analyzer/static-budget.ts) performs compile-time budget analysis before runtime.

3.13 Orchestrator (orchestrator/)
Top-level goal execution controller. Planner decomposes goals into task sequences and replans on state change. GoalVerifier scores goal achievement. RepairModule detects violations and generates repair actions. EscalationModule escalates to the principal hierarchy when the agent cannot self-repair.

3.14 TEE (tee/)
TEEVerifier attests ExecutionProof objects against the hardware trust anchor. TEEClause governs which operations require TEE-attested execution. attest.ts handles the attestation protocol.

3.15 Compiler Pipeline (lang/, ir/)
Source → Lexer → Parser → AST → TypeChecker + EffectChecker + TaintChecker
       → ContractChecker → Lowering → IR → IRVerifier (effect soundness, budget, taint)
       → BudgetAnalyzer → DischargeValidator → VM
The discharge block is a first-class IR construct. Perform instructions are only valid inside Discharge regions, enforced by both the IR verifier (ir/verifier/effect-soundness.ts) and the runtime (vm/discharge.rs). Grammar export (--emit-grammar) produces GBNF output for constrained generation.

4. Project Directory Structure
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
│   ├── grammar-export/
│   ├── budget-analyzer/
│   ├── discharge/
│   └── diagnostics/
├── semantics/
│   ├── operational/
│   ├── denotational/
│   └── proofs/
├── ir/
│   ├── core/
│   ├── instructions/
│   ├── builder/
│   └── verifier/
├── vm/
│   ├── executor/
│   ├── state/
│   ├── instructions/
│   └── runtime/
├── memory/
│   ├── layers/
│   ├── governance/
│   ├── coherency/
│   ├── merkle/
│   ├── dual-process/
│   ├── episodic-recon/
│   ├── dream-cycle/
│   ├── adaptive/
│   └── evolutionary-memory/
├── corrigibility/
│   ├── heads/
│   ├── control-meter/
│   ├── dead-switch/
│   └── amendment-gate/
├── evolution/
│   ├── pipeline/
│   ├── fggm/
│   ├── simulation/
│   └── rollback/
├── training/
│   ├── algorithms/
│   ├── reward/
│   ├── critic/
│   ├── curriculum/
│   └── convergence/
├── contracts/
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
│   └── audit/
├── protocols/
│   ├── a2a/
│   ├── mcp/
│   ├── mesh/
│   └── network/
├── effects/
├── scheduler/
├── capability/
├── security/
│   ├── taint-types/
│   ├── sanitizer/
│   └── ifc/
├── inference/
├── tee/
│   ├── attestation/
│   └── governance/
├── orchestrator/
├── cli/
├── sdk/
├── tests/
│   ├── conformance/
│   ├── property/
│   └── fuzzing/
├── benchmarks/
└── docs/

5. Complete File Inventory
Compiler — lang/
lang/parser/parser.ts
lang/typing/typechecker.ts
lang/typing/effect-checker.ts
lang/typing/taint-checker.ts
lang/typing/schema-validator.ts          ← SchemaValidator class
lang/typing/repair-engine.ts             ← RepairEngine class
lang/contracts/contract-checker.ts
lang/sessions/session-types.ts
lang/trust/trust-model.ts
lang/lowering/ast-to-ir.ts
lang/grammar-export/export-gbnf.ts
lang/budget-analyzer/static-budget.ts
lang/discharge/discharge-validator.ts
lang/diagnostics/diagnostics.ts
Semantics — semantics/
semantics/operational/computation.ts     ← Computation<T>, Decision<T>, Failure monad
semantics/operational/small-step.ts      ← small-step reduction, merge, discharge
semantics/operational/big-step.ts
semantics/denotational/model.ts
semantics/proofs/determinism.md
IR — ir/
ir/core/ir.ts
ir/instructions/infer.ts
ir/instructions/discharge.ts
ir/instructions/perform.ts              ← only valid inside Discharge region
ir/builder/ir-builder.ts
ir/verifier/effect-soundness.ts         ← enforces Perform inside Discharge
ir/verifier/budget-verifier.ts
ir/verifier/taint-verifier.ts
VM — vm/
vm/executor/execute.ts
vm/state/state.ts
vm/state/contract-state.ts              ← ContractState type
vm/instructions/runner.ts
vm/runtime/rng.ts                       ← DeterministicRNG
vm/runtime/execution-proof.ts           ← ExecutionProof type
vm/discharge.rs                         ← runtime Discharge gates (Rust)
Memory — memory/
memory/layers/working.ts                (L0)
memory/layers/episodic.ts               (L1)
memory/layers/semantic.ts               (L2)
memory/layers/procedural.ts             (L3)
memory/layers/prospective.ts            (L4)
memory/layers/federated.ts              (L5)
memory/layers/identity.ts               (L6)
memory/layers/provenance-index.ts       (L7)
memory/layers/layer-store.ts            ← LayerStore, AppendOnlyLog, MutableStore, GraphManager, DecayFunction
memory/layers/memory-layer.ts           ← MemoryLayer enum
memory/governance/tri-path-router.ts
memory/governance/rw-invalidate-path.ts ← ReadRouter, WriteRouter, InvalidationRouter
memory/coherency/mesi.ts
memory/coherency/crdt.ts
memory/coherency/gossip.ts
memory/merkle/merkle-integrity-manager.ts  ← MerkleIntegrityManager
memory/dual-process/system1-system2.ts
memory/episodic-recon/reconstructor.ts
memory/dream-cycle/dream-scheduler.ts
memory/dream-cycle/invariants.ts
memory/adaptive/structure-selector.ts
memory/evolutionary-memory/prism.ts
Corrigibility — corrigibility/
corrigibility/heads/five-heads.ts       ← U1–U5 head classes
corrigibility/heads/lexicographic.ts    ← enforceLexicographic()
corrigibility/control-meter.ts
corrigibility/dead-switch.ts
corrigibility/amendment-gate.ts
Evolution — evolution/
evolution/pipeline/pipeline.ts
evolution/fggm/generator.ts
evolution/fggm/rejection-sampler.ts
evolution/simulation/simulator.ts
evolution/simulation/adversarial.ts
evolution/simulation/simulation-result.ts  ← SimulationResult type
evolution/rollback/rollback.ts
Training — training/
training/algorithms/grpo.ts
training/algorithms/hybrid-grpo.ts
training/reward/reward-fn.ts
training/critic/process-critic.ts
training/curriculum/curriculum.ts
training/convergence/guard.ts
Contracts — contracts/
contracts/contract-engine.ts            ← ContractEngine class
contracts/abc/abc-contract.ts
contracts/abc/governance.ts
contracts/agentspec/rule-engine.ts
contracts/veriguard/offline-verify.ts
contracts/veriguard/online-monitor.ts
contracts/fggm-contracts/fggm.ts
contracts/agent-contracts/resource-budget.ts
contracts/agent-contracts/delegation.ts
contracts/temporal/temporal-contract.ts ← TemporalContract class
contracts/temporal/ltl-parser.ts
contracts/temporal/automaton.ts
contracts/temporal/monitor.ts
Provenance — provenance/
provenance/truth-stack/actor-chain.ts
provenance/truth-stack/intent-chain.ts
provenance/truth-stack/inference-chain.ts
provenance/tracecaps/risk-accumulator.ts
provenance/scitt/receipt-builder.ts
provenance/audit/trajectory-auditor.ts
provenance/audit/formal-spec.ts         ← FormalSpec type
provenance/audit/audit-report.ts        ← AuditReport type
Protocols — protocols/
protocols/a2a/card.ts
protocols/a2a/task-manager.ts
protocols/a2a/rpc-handler.ts
protocols/mcp/server.ts
protocols/mcp/client.ts
protocols/mcp/mcps.ts
protocols/mcp/mcpshield.ts
protocols/mcp/mcp-types.ts              ← MCPTool, MCPResource, MCPPrompt, MCPLifecycle, Connection
protocols/mesh/cat7.ts
protocols/mesh/svaf.ts
protocols/mesh/remix.ts
protocols/mesh/lineage.ts
protocols/network/transport.ts          ← transport layer
Effects — effects/
effects/effect.ts                       ← Effect class (core spec type)
effects/interval.ts                     ← Interval
effects/cost-interval.ts                ← CostInterval
effects/uncertainty-engine.ts           ← UncertaintyEngine
Scheduler — scheduler/
scheduler/deterministic-scheduler.ts   ← DeterministicScheduler
scheduler/schedule-trace.ts            ← ScheduleTrace (VMState field)
scheduler/budget-ledger.ts             ← BudgetLedger
Capability — capability/
capability/capability-manager.ts       ← CapabilityManager
capability/capability-token.ts         ← CapabilityToken (field on Effect)
Security — security/
security/taint-types/taint-engine.ts   ← TaintEngine
security/taint-types/taint-meta.ts     ← TaintMeta (field on Effect)
security/taint-types/taint-modifier.ts
security/taint-types/propagation.ts
security/sanitizer/sanitizer.ts        ← Sanitizer class
security/sanitizer/policies.ts
security/ifc/ifc.ts
Inference — inference/
inference/inference-engine.ts          ← InferenceEngine
inference/provider.ts                  ← Provider
TEE — tee/
tee/attestation/attest.ts
tee/attestation/tee-verifier.ts        ← TEEVerifier class
tee/governance/tee-clause.ts
Orchestrator — orchestrator/
orchestrator/orchestrator.ts           ← Orchestrator
orchestrator/planner.ts                ← Planner
orchestrator/goal-verifier.ts          ← GoalVerifier
orchestrator/repair.ts                 ← RepairModule
orchestrator/escalation.ts             ← EscalationModule
CLI — cli/
cli/build.ts
cli/run.ts
cli/audit.ts
cli/prove.ts
cli/test.ts
cli/conformance.ts
Tests / Benchmarks / Docs
tests/conformance/categories/*.ts
tests/property/effects.ts
tests/fuzzing/parser.ts
benchmarks/benchmarks.ts
docs/overview.md

6. Inter-Subsystem Relationships
Computation<T>  ←────────── used by: MCPClient, A2ARpcHandler, InferenceEngine,
                              Orchestrator, all VM instructions

VMState         ←────────── threaded through: Executor, ContractEngine,
                              CorrigibilityMonitor, TrajectoryAuditor,
                              Planner, GoalVerifier, RewardFunction

ProvenanceGraph ←────────── written by: every Computation execution
                             read by: TrajectoryAuditor, SCITTReceiptBuilder,
                              RiskAccumulator, ProcessCritic

AmendmentGate   ←────────── blocks: EvolutionPipeline.apply()
                             requires: CorrigibilityHeads approval

BudgetLedger    ←────────── checked by: DeterministicScheduler before every Task
                             also checked at compile time: static-budget.ts

TaintEngine     ←────────── wraps: all data crossing subsystem boundaries
                             enforced by: IFCChecker, ir/verifier/taint-verifier.ts

MerkleIntegrity ←────────── wraps: all MemorySubsystem layer reads/writes
                             root stored in: ExecutionProof

7. Named Types Reference
All named types in the spec, their definition location, and their role:
TypeFileRoleComputation<T>semantics/operational/computation.tsCore execution monadEffecteffects/effect.tsEffect annotation on every computationIntervaleffects/interval.tsNumeric uncertainty rangeTaintMetasecurity/taint-types/taint-meta.tsTaint level + sources + policyCostIntervaleffects/cost-interval.tsToken / time / memory budgetCapabilityTokencapability/capability-token.tsScoped capability proofDecision<T>semantics/operational/computation.tsSome / Ambiguous / None decisionFailuresemantics/operational/computation.tsStructured failure with traceContractStatevm/state/contract-state.tsActive contract evaluations snapshotExecutionProofvm/runtime/execution-proof.tsProof-carrying execution witnessDeterministicRNGvm/runtime/rng.tsSeeded, forkable deterministic RNGScheduleTracescheduler/schedule-trace.tsAppend-only schedule replay logMemoryLayermemory/layers/memory-layer.tsL0–L7 layer enumAppendOnlyLogmemory/layers/layer-store.tsImmutable layer store variantMutableStorememory/layers/layer-store.tsMutable layer store variantGraphManagermemory/layers/layer-store.tsGraph structure manager for a layerDecayFunctionmemory/layers/layer-store.tsMemory decay/forgetting functionSimulationResultevolution/simulation/simulation-result.tsEvolution simulation outputAmendmentevolution/pipeline/pipeline.tsProposed self-modificationFormalSpecprovenance/audit/formal-spec.tsNL-compiled LTL specAuditReportprovenance/audit/audit-report.tsTrajectory audit outputMCPToolprotocols/mcp/mcp-types.tsMCP tool descriptorMCPResourceprotocols/mcp/mcp-types.tsMCP resource descriptorMCPPromptprotocols/mcp/mcp-types.tsMCP prompt descriptorMCPLifecycleprotocols/mcp/mcp-types.tsMCP server lifecycle hooksConnectionprotocols/mcp/mcp-types.tsMCP client connection handleAgentCardprotocols/a2a/card.tsA2A agent advertisementIRProgramir/core/ir.tsCompiled IR programExecCtxvm/executor/execute.tsExecutor contextProvenanceRecordprovenance/truth-stack/actor-chain.tsSingle provenance eventSCITTReceiptprovenance/scitt/receipt-builder.tsTamper-evident audit receiptLTLFormulacontracts/temporal/ltl-parser.tsParsed LTL formulaAutomatoncontracts/temporal/automaton.tsLTL-compiled automaton