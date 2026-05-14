# B2 – Type System
Goal: The type checker must implement Hindley‑Milner inference, affine resource tracking, effect row polymorphism, and information‑flow taint analysis.

Task	Description	Acceptance
B2.1	Name resolution – resolve all identifiers to their definition site, build scope graphs.	All nameres tests pass; undefined names produce clear errors.
B2.2	Type inference – implement Algorithm W with let‑polymorphism, gradual typing, and nominal types.	Property‑based tests verify well‑typed programs don't crash; ill‑typed programs produce errors.
B2.3	Affine type tracking – enforce linear usage of capabilities and owned values.	Tests verify that double‑use of a consumed value is rejected.
B2.4	Effect checking – compute effect rows; verify perform is always inside discharge.	Effect‑soundness tests pass; ill‑scoped perform is rejected.
B2.5	Taint analysis – implement the Clean/Agnostic/Tainted lattice; track program‑counter taint.	Taint‑violation tests pass; sanitization gates work.
B2.6	Contract checking – verify ABC contracts, AgentSpec rules, and temporal contracts at compile time.	Contract‑violation tests pass.
B3 – Lowering Pass (Complete)
Goal: Every AST node must lower to correct, verified IR.

Task	Description	Acceptance
B3.1	Lower all control‑flow constructs – if, match, loop, while, for – to basic blocks.	IR verifier accepts the output; VM executes correctly.
B3.2	Lower all memory operations – mem.load, mem.store, mem.traverse, mem.consolidate.	VM memory layer tests pass.
B3.3	Lower agent operations – spawn, send, recv, transfer.	Multi‑agent integration tests pass.
B3.4	Lower effectful operations – discharge, perform, infer, observe.	Effect tests pass in the VM.
B3.5	Lower pipeline/redirection/coprocess expressions.	Pipeline integration tests pass.
B4 – IR & VM Completion
Goal: Every opcode in the IR must have a correct VM implementation.

Task	Description	Acceptance
B4.1	Implement arithmetic/comparison/logical ops in the VM.	test_simple_add and similar tests pass.
B4.2	Implement memory layer ops – MemLoad, MemStore, MemQuery, MemPromote, MemDecay.	Memory subsystem tests pass.
B4.3	Implement agent ops – AgentSpawn, AgentSend, AgentRecv.	Multi‑agent test passes.
B4.4	Implement effect ops – Discharge, Perform, Infer, Observe.	Effect tests pass.
B4.5	Implement heartbeat/dream ops.	Heartbeat‑loop test passes.
B4.6	Implement confidence/capability ops.	Confidence gate test passes.
B4.7	Implement provenance/proof ops.	Proof‑generation test passes.
B4.8	Implement pipeline/federation/corrigibility ops.	Full‑system integration test passes.
B4.9	Proof‑carrying execution – generate ExecutionProof with Merkle roots and effect trace.	Proof‑verification test passes.
B4.10	Deterministic replay – verify that identical seeds produce identical traces.	Replay test passes.
B5 – Memory Subsystem
Goal: Full 8‑layer memory with Ebbinghaus decay, Merkle integrity, CRDT federation, and dual‑process retrieval.

Task	Description	Acceptance
B5.1	Complete all 8 layer stores – L0‑L7 with schemas and access patterns.	Memory‑layer unit tests pass.
B5.2	Implement anti‑echo filtering and schema validation on every write.	Anti‑echo tests pass.
B5.3	Implement MESI coherency protocol and CRDT gossip.	Coherency tests pass.
B5.4	Implement Merkle tree updates and proof generation.	Merkle‑integrity tests pass.
B5.5	Implement Dual‑Process Controller (System‑1 / System‑2 gating).	Gating tests pass.
B5.6	Implement Dream Cycle with all 6 phases and idempotency invariant.	Dream‑cycle tests pass.
B5.7	Implement Episodic Reconstruction (master‑assistant).	Recon tests pass.
B5.8	Implement Adaptive Memory structure selector.	Adaptive‑memory tests pass.
B6 – Standard Library (seed‑std)
Goal: A minimal but useful standard library that demonstrates the language's capabilities.

Task	Description	Acceptance
B6.1	seed::prelude – common types and traits.	Compiles into .aslb; VM can load.
B6.2	seed::agent – base Agent trait, lifecycle hooks.	Agent‑definition tests pass.
B6.3	seed::memory – typed memory operations.	Memory‑operation tests pass.
B6.4	seed::inference – typed inference with schema validation.	Inference‑operation tests pass.
B6.5	seed::protocols – A2A, MCP client stubs.	Protocol stubs compile.
B6.6	seed::capability – token management.	Capability‑op tests pass.
B6.7	seed::provenance – event logging and proof export.	Provenance tests pass.
B7 – Integration & Conformance Tests
Goal: Prove the compiler and VM satisfy the ASL‑CONF‑15 suite.

Task	Description	Acceptance
B7.1	Write a comprehensive integration‑test harness that compiles and runs .seed programs.	Harness can run all example programs.
B7.2	Implement the Level‑1 conformance tests (Core, Effects, Uncertain, Cognitive).	All Level‑1 tests pass.
B7.3	Implement the Level‑2 conformance tests (Memory, Safety, Capability, Trust, MCP, A2A, Federation, Mesh, Session).	All Level‑2 tests pass.
B7.4	Implement the Level‑3 conformance tests (Observability, Identity, Provenance, Guardrails, Corrigibility, ISA).	All Level‑3 tests pass.
B7.5	Implement the Level‑4 conformance tests (Evolution, Training, Grammar‑Strata).	All Level‑4 tests pass.
B7.6	Achieve Level‑5 certification with adversarial simulation and red‑team audit.	Level‑5 certification awarded.  ---> 
# Phase B Complete 

# Switched to MLP tsk track --> see MLP task list md  -> MLP completed --> Will complete phse one and phase zero before continuing from C

# Phase Zero Task List
These are today‑executable, non‑invasive changes that harden the project before Phase C.

#	Task	Effort	Rationale
PZ‑1	Gate ast_dump.txt behind a feature flag (or remove)	S	Debug artefact left in release path
PZ‑2	Verify/remove protocols.rs module declaration	S	Avoid broken compilation if file missing
PZ‑3	Fix test_read_source_stdin to use Cursor	M	Unblock CI
PZ‑4	Create MAINTENANCE.md playbook	S	Institutional memory
PZ‑5	Create CHANGELOG.md with v0.1.0 entry	S	User‑facing
PZ‑6	Add seed --version (if missing)	S	Basic CLI hygiene
PZ‑7	Add smoke test to CI (seed run hello.seed)	S	Guards regressions
PZ‑8	Add .editorconfig and pre‑commit hook template	S	Reduces formatting churn
PZ‑9	Generate REPO_STATE.yaml (optional but nice)	S	Handoff accuracy
PZ‑10	Populate CONTRIBUTING.md and PR template	M	Opens door to contributors


# Phase 1 – Ecosystem & Community Foundation (post‑Phase 0, pre‑Phase C)
ID	Task	Rationale (why it’s independent of C/D)
P1‑1	Adopt DCO and set up a GitHub check for sign‑off	Purely legal/process; no code changes
P1‑2	Create CODE_OF_CONDUCT.md and GOVERNANCE.md	Standard community docs, zero dependency on toolchain
P1‑3	Set up issue templates (bug, feature, doc, security) and a PR template	GitHub‑only; helps triage immediately
P1‑4	Implement a lightweight triage bot (GitHub Actions) that auto‑labels new issues and pings maintainers	Works with existing repo; no compiler/Vm features needed
P1‑5	Document the release policy (versioning, support window, deprecation timeline)	Policy document only; can reference current toolchain
P1‑6	Add seed update command to CLI (check GitHub Releases for newer version, download binary)	The CLI already exists; just needs a new subcommand and HTTP client – no new compiler/VM code
P1‑7	Enable reproducible builds (dockerized build, SOURCE_DATE_EPOCH) and generate an SBOM (SPDX) for releases	Build‑system improvement; no language changes
P1‑8	Create SECURITY.md and enable GitHub private vulnerability reporting	Standard security practice, no code required
P1‑9	Configure Dependabot/Renovate for automated dependency updates	CI‑only, no toolchain changes
P1‑10	Write CONTRIBUTING.md with development setup, test commands, and PR checklist	Documentation; helps onboard contributors now
P1‑11	Build a searchable static site for error codes (the codes themselves will be added in Phase 0, so this depends only on Phase 0 being done)	Static site generation; uses error codes output by the compiler (Phase 0)
P1‑12	Define the language edition system and deprecation policy (decoupled from actual implementation)	Design artifact; can be decided before implementing features
P1‑13	Create an rfcs/ directory with a template and lightweight consensus process	Process only; no code changes
Deferred (depend on Phase C/D)
Original ID	Task	Dependency
E4.1	Integrate conformance suite into CI	Conformance suite requires the full language subset (Phase D)
E6.3	WebAssembly‑based playground	Needs a complete, stable compiler and VM (post‑Phase D)
E8.1	Opt‑in crash reporting with privacy safeguards	VM must be feature‑complete to capture meaningful crash dumps (Phase D)
E4.2	Automated changelog generation	Nice to have now, but best adopted when releases carry real feature content (post‑C) — can be set up during C/D, not urgent
E3.3 / full supply chain with Sigstore	Already partially covered by P1‑7; Sigstore signing may be added later	No strict dependency, but low priority until binaries are more mature


# phase C – Compiler Completion (6‑8 weeks) 
Goal: The compiler must parse, type‑check, and lower every ASL v15.2 construct (S0‑S2) to correct, verified IR.

#	Task	Description
C1	Complete the lexer keyword table	Add every remaining keyword from the v15 spec to token.rs – contract, temporal, guardrail, think, route, session, capability, trust, corrigibility, provenance, federation, mesh, identity, heartbeat, dream, memory, ontology, prompt, evolution, training, and all S2/S3 keywords.
C2	Complete the parser – declarations	Implement parsing for all remaining top‑level and member declarations: contract, temporal_contract, guardrail, think_profile, routing_policy, session, capability_grant, trust_policy, corrigibility, provenance, federation, mesh, identity, heartbeat, dream, memory_hierarchy, ontology, prompt, evolution_policy, training_regimen.
C3	Complete the parser – expressions	Implement parsing for: infer<T>, observe, transfer, prov!, mesh_send/mesh_recv, mesh_call, session_call, discharge with, perform requires, grant, attenuate, delegate, revoke, sanitize, coprocess, named_pipe, job_control, printf, history, completion, restricted_mode.
C4	Type checker – name resolution	Build scope graphs, resolve every identifier to its definition site, and reject undefined names with clear error messages.
C5	Type checker – Hindley‑Milner inference	Implement Algorithm W with let‑polymorphism, gradual typing (Uncertain<T>, ?), and nominal types (structs, enums, agents).
C6	Type checker – affine resource tracking	Enforce linear usage of capabilities and owned values; reject double‑use after move.
C7	Type checker – effect rows	Compute effect rows for every expression; verify that perform is always lexically inside a discharge block.
C8	Type checker – taint analysis	Implement the Clean ≤ Agnostic ≤ Tainted lattice; track program‑counter taint through branches; reject unsanitized flows into capability‑exercising operations.
C9	Type checker – contract verification	Verify ABC contracts, AgentSpec rules, temporal contracts, and FGGM output guarantees at compile time.
C10	Lowering pass – all expressions	Lower every AST node to correct, verified IR: control flow, memory ops, agent ops, effectful ops, pipelines, confidence gates, think budgets, etc.
C11	Lowering pass – all declarations	Lower heartbeat phases, dream phases, memory configurations, ontology constraints, and evolution policies to their respective IR sections.
C12	Grammar export	Implement seedc --emit-grammar --stratum S0 --format gbnf to produce a GBNF grammar file for constrained LLM decoding.
C13	Compile‑time context budget analysis	Implement static analysis that computes worst‑case token usage (P0+P1+P2) and rejects programs exceeding declared budgets.



# Phase D – Virtual Machine Completion (4‑6 weeks)
Goal: Every opcode in the IR must execute correctly in the VM, with full memory, agent, effect, provenance, and corrigibility support.

#	Task	Description
D1	Complete arithmetic/logical ops	Implement all remaining arithmetic, comparison, bitwise, and conversion ops.
D2	Memory subsystem – all 8 layers	Implement L0‑L7: Working, Episodic, Semantic, Procedural, Prospective, Federated, Identity, Provenance. Each layer gets its schema, decay function, and access pattern.
D3	Memory governance	Implement tri‑path router (read/write/invalidate), anti‑echo filtering, Merkle integrity on every write, schema validation, and consent enforcement.
D4	Memory coherency	Implement MESI cache coherency for strongly‑consistent layers; CRDT‑backed eventual consistency for federated layers with vector clocks and anti‑entropy gossip.
D5	Dual‑process memory	Implement System‑1 (fast pattern‑match) and System‑2 (full graph traversal) retrieval with a gating function.
D6	Dream cycle	Implement the 6‑phase dream cycle (review, resolve, consolidate, compress, prune, write_journal) with formal pre/post‑conditions and idempotency.
D7	Agent ops	Implement AgentSpawn, AgentSend, AgentRecv with mailboxes and supervision trees.
D8	Effect system	Implement Discharge/Perform runtime gates with uncertainty, taint, cost, and capability threshold checks.
D9	Heartbeat loop	Implement the autonomous OODA loop: observe, decide, act_or_sleep, log, update_memory. Sleep tool with wake conditions.
D10	Confidence system	Implement ConfidenceGate and ConfidenceAsk with interval‑based thresholding.
D11	Capability tokens	Implement ed25519‑signed capability tokens with attenuation, delegation, and revocation.
D12	Provenance chain	Implement SPICE Truth Stack (actor, intent, inference Merkle chains), TraceCaps monotone risk accumulation, SCITT receipts, and JSON‑LD export.
D13	Corrigibility monitor	Implement five‑head utility monitor (U1‑U5) with lexicographic priority, control meter, dead‑man’s switch, and amendment gate.
D14	Evolution engine	Implement SEVerA/FGGM pipeline: propose → simulate → adversarial review → approve → apply with atomic rollback.
D15	Training engine	Implement GRPO/PPO training loops with process critic, curriculum scheduler, and convergence guard.
D16	Temporal contracts	Implement LTL parser, Büchi automaton compiler, and runtime monitor with SMT integration.
D17	TEE attestation	Implement Intel TDX / AMD SEV / Arm CCA attestation verification and trust scoring.
D18	Orchestrator	Implement goal decomposition planner, verifier, repair module, and escalation module.
D19	Proof‑carrying execution	Generate ExecutionProof with trace hash, contract satisfaction, taint safety, capability validity, and temporal satisfaction fields.
D20	Deterministic replay	Verify that identical seeds produce byte‑identical schedule traces.


# Phase E – Standard Library (3‑4 weeks)
Goal: Ship a production‑grade standard library that developers import with use seed::....

#	Task	Description
E1	seed::prelude	Core traits and types: Agent, Memory, Computation<T>, Uncertain<T>, Result, Option.
E2	seed::agent	Agent lifecycle hooks, heartbeat configuration, dream schedule.
E3	seed::memory	Typed memory operations for all 8 layers; graph traversal; search; consolidation.
E4	seed::inference	infer<T> with schema derivation, confidence interval computation, and model routing.
E5	seed::uncertain	Uncertain<T> monad: pure, bind, map, observe, gate – the U1‑U6 API.
E6	seed::protocols	A2A client/server, MCP client/server, Cognitive Mesh (CAT7, SVAF).
E7	seed::capability	Token management, attenuation, delegation, revocation, hypergraph closure.
E8	seed::provenance	Event logging, Merkle proof generation, SCITT receipt export.
E9	seed::crypto	Ed25519 signing, SHA3‑256 hashing, DID derivation, PASETO v4.
E10	seed::io / seed::net	File I/O, HTTP client, TCP/UDP sockets.

# Phase F – Tooling & Developer Experience (4‑6 weeks)
Goal: Opening a .seed file in any editor gives full IDE support.

#	Task	Description
F1	Language server – diagnostics	Publish real compiler errors/warnings as you type, with source‑span highlights.
F2	Language server – completion	Context‑aware completion for keywords, identifiers, section names, and stdlib symbols.
F3	Language server – hover	Type information, documentation, and examples on hover.
F4	Language server – navigation	Go‑to‑definition, find‑references, document symbols.
F5	Language server – rename	Semantic rename across the workspace.
F6	Tree‑sitter grammar	Write grammar.js for syntax highlighting; publish tree-sitter-agentseed to npm.
F7	VS Code extension	Package LSP binary + Tree‑sitter grammar; publish to VS Code Marketplace.
F8	Formatter (seedfmt)	CST‑based lossless formatting; seed fmt and seed fmt --check commands.
F9	Linter (seed lint)	Style and correctness rules; configurable via seed.toml.
F10	Debug adapter (seeddbg)	Step‑through debugging, breakpoints, variable inspection, memory layer browsing.
F11	Neovim / Emacs / Zed configs	Provide LSP + Tree‑sitter setup snippets for each editor.


# Phase G – Distribution & Installation (2‑3 weeks)
Goal: One‑command install on every platform.

#	Task	Description
G1	Multi‑platform binaries	GitHub Actions workflow building signed binaries for Linux (x86_64, arm64), macOS (x86_64, arm64), Windows (x86_64).
G2	Shell installer	install.sh – detects platform, downloads correct binary, verifies SHA256, installs to ~/.agentseed/bin/.
G3	Homebrew formula	Submit to homebrew/core (or custom tap).
G4	npm package	@agentseed/cli – thin wrapper that downloads the platform binary on postinstall.
G5	pip package	agentseed – similar wrapper for Python developers.
G6	Scoop bucket	Windows package manager support.
G7	aqua / mise registry	Declarative tool version management integration.
G8	Docker image	Official Docker image for CI/CD and server deployments.


# Phase H – Documentation & Playground (3‑4 weeks)
Goal: A curious developer can learn ASL in an afternoon.

#	Task	Description
H1	The ASL Book	Complete language guide (mdBook): Getting Started, Language Reference, Standard Library, Agent Programming, Corrigibility, Evolution, Deployment.
H2	API reference	Auto‑generated from doc comments (cargo doc); hosted at docs.agentseed.org.
H3	Interactive playground	Compile the VM to WASM; embed Monaco editor; run .seed files in the browser.
H4	Tutorials	“Your First Agent,” “Building a Research Assistant,” “Multi‑Agent Federation,” “Self‑Evolving Agents.”
H5	Example gallery	Curated collection of agent programs: research, coding, planning, creative writing, data analysis.
H6	Video walkthroughs	5‑minute setup, 15‑minute language tour, 30‑minute deep dive.


# Phase I – Package Registry (2‑3 weeks)
Goal: Developers can share and install agent packages.

#	Task	Description
I1	SPI (Seed Package Index) server	Axum‑based registry with package upload, download, search, and versioning.
I2	seed publish	Package and upload a .seed library to the registry.
I3	seed install	Download and install a package from the registry with dependency resolution.
I4	Seed.lock	Reproducible builds via lockfile with content hashes.
I5	Package signing	Ed25519 signatures on every package; verification on install.


# Phase J – Community & Launch (2‑3 weeks)
Goal: Thousands of developers know about AGENT‑SEED and can use it.

#	Task	Description
J1	CONTRIBUTING.md	Clear contribution guide, code of conduct, development setup.
J2	GitHub Discussions + Discord	Community channels for questions, RFCs, and collaboration.
J3	awesome-agentseed	Curated list of community packages, tutorials, and projects.
J4	Landing page (agentseed.org)	Clear value proposition, install command, demo video, documentation links.
J5	Launch blog post	“Introducing AGENT‑SEED: A Programming Language Where Agents Have Memory, Heartbeats, and Dreams.”
J6	Show HN	Hacker News launch with live demo.
J7	Reddit launch	r/programming, r/rust, r/MachineLearning, r/artificial.
J8	Conference talks	Submit to Strange Loop, RustConf, AI Engineer Summit, POPL, PLDI.
J9	Hackathon	Online hackathon with prizes for best agent programs.
J10	Swag	Stickers, t‑shirts, and a custom seed command‑line theme.


# Phase K – Post‑Launch Iteration (ongoing)
#	Task	Description
K1	Bug tracker triage	Respond to GitHub issues within 48 hours.
K2	Monthly releases	Regular cadence with release notes and migration guides.
K3	RFC process	Formal process for language evolution proposals.
K4	Performance benchmarks	Track compile times, VM throughput, and memory usage.
K5	Ecosystem growth	Nurture community packages, integrations, and tooling.








1. Graph Normalization — Differentiable Max‑Weight Independent Set
Mathematics

x
i
(
t
+
1
)
=
x
i
(
t
)
exp
⁡
 ⁣
(
w
i
−
∑
j
∈
N
(
i
)
w
j
x
j
(
t
)
)
∑
k
∈
V
x
k
(
t
)
exp
⁡
 ⁣
(
w
k
−
∑
j
∈
N
(
k
)
w
j
x
j
(
t
)
)
x 
i
(t+1)
​
 = 
∑ 
k∈V
​
 x 
k
(t)
​
 exp(w 
k
​
 −∑ 
j∈N(k)
​
 w 
j
​
 x 
j
(t)
​
 )
x 
i
(t)
​
 exp(w 
i
​
 −∑ 
j∈N(i)
​
 w 
j
​
 x 
j
(t)
​
 )
​
 
This is the replicator dynamics for the MWIS problem. The sequence converges to a binary vector that exactly solves MWIS, not just a heuristic. Each step costs 
O
(
∣
E
∣
)
O(∣E∣); on million‑edge graphs the solution is found in seconds on a CPU and within 1% of the optimum.

Where it goes in the task list

D18 (Orchestrator) – Goal decomposition into non‑conflicting sub‑tasks is exactly an MWIS problem. Replace the current greedy decomposition in Orchestrator::accept_goal with a call to a GraphNormalization solver.
New sub‑task: D18.1 – Implement MWIS‑based goal planner via Graph Normalization.

D6 (Dream Cycle) – During Consolidate, selecting which episodic entries to promote while blocking semantically identical duplicates is again an MWIS (anti‑echo graph).
New sub‑task: D6.1 – Use GraphNormalization for anti‑echo‑aware consolidation.

Future – Mixture‑of‑Experts routing – When ASL adds native MoE, routing with mutually exclusive expert selection maps to MWIS. Create a placeholder task:
New sub‑task: D26 – Mixture‑of‑Experts routing via Graph Normalization.


2. FalconGEMM — Lower‑Complexity Matrix Multiplication
Mathematics

FalconGEMM partitions a matrix product 
A
×
B
A×B into a tree of sub‑multiplications so that

M
(
m
,
k
,
n
)
=
∑
i
=
1
R
M
(
m
i
,
k
i
,
n
i
)
,
∑
i
m
i
k
i
n
i
<
m
k
n
.
M(m,k,n)= 
i=1
∑
R
​
 M(m 
i
​
 ,k 
i
​
 ,n 
i
​
 ), 
i
∑
​
 m 
i
​
 k 
i
​
 n 
i
​
 <mkn.
The runtime is predicted by

T
pred
=
α
h
⋅
flops
+
β
h
⋅
bytes
,
T 
pred
​
 =α 
h
​
 ⋅flops+β 
h
​
 ⋅bytes,
and the cheapest LCMA is selected. Outperforms cuBLAS/MKL by 7.6‑17.9%.

Where it goes

D1 (Arithmetic ops) is not enough; inference GEMM is a high‑level library call. Add a new optimisation task:
New sub‑task: D27 – Integrate FalconGEMM under feature‑flag seedvm‑falcon for all infer<T> and embedding search GEMM calls.

3. EVIL + CodeEvolve — Interpretable Skill Synthesis
Mathematics

EVIL maintains a population of Python functions 
p
p. Fitness is

F
(
p
)
=
LogLike
(
p
,
D
)
−
λ
⋅
len
(
p
)
.
F(p)=LogLike(p,D)−λ⋅len(p).
LLM‑guided mutation:

p
new
←
LLM
(
p
parent
,
error_signal
)
.
p 
new
​
 ←LLM(p 
parent
​
 ,error_signal).
Every 
r
r generations, a reflection step:

Reflection
=
LLM
(
p
best
(
1
)
,
…
,
p
best
(
k
)
)
Reflection=LLM(p 
best
(1)
​
 ,…,p 
best
(k)
​
 )
biases subsequent mutations. CodeEvolve adds island GAs with migration:

P
(
i
+
1
)
 
mod
 
K
(
t
+
1
)
=
P
(
i
+
1
)
 
mod
 
K
(
t
)
∪
Top
m
(
P
i
(
t
)
)
.
P 
(i+1)modK
(t+1)
​
 =P 
(i+1)modK
(t)
​
 ∪Top 
m
​
 (P 
i
(t)
​
 ).
Where it goes

D14 (Evolution engine) – The evolution pipeline currently uses LLM‑only synthesis. Add a new synthesis strategy:
New sub‑task: D14.1 – Add strategy: "evil" synthesis that evolves highly interpretable ASL functions internally.

C10/C11 (Lowering pass) – The compiler’s lowering could be improved by an EVIL‑evolved peephole optimiser.
New sub‑task: C11.1 – Use CodeEvolve to synthesise peephole optimisation rules for the IR lowering pass.

4. SCM — Splitting‑Counting‑Merging for Hierarchical Aggregation
Mathematics

For a tree 
T
T with 
N
N leaves, preprocess in 
O
(
N
)
O(N) so that the mode in any subtree is answered in 
O
(
1
)
O(1):

mode
(
v
)
=
arg
⁡
max
⁡
c
∈
C
∑
I
∈
Decomp
(
v
)
Freq
(
c
,
I
)
.
mode(v)=arg 
c∈C
max
​
  
I∈Decomp(v)
∑
​
 Freq(c,I).
30× faster than Range‑Mode baselines on billions of values.

Where it goes

D4 (Memory coherency – federated analytics) – The federated L5 layer queries aggregate statistics over entity subtrees.
New sub‑task: D4.1 – Build SCM index over federated fact store for O(1) subtree‑mode queries.

5. NGO‑IR — Neural Global Optimisation for Hyper‑Parameters
Mathematics

Given noisy evaluations 
y
j
=
f
(
x
j
)
+
ϵ
j
y 
j
​
 =f(x 
j
​
 )+ϵ 
j
​
 , a spline 
S
S is fitted. A transformer 
Φ
θ
Φ 
θ
​
  predicts updates:

x
^
(
t
+
1
)
=
x
^
(
t
)
+
Φ
θ
(
S
,
{
y
j
}
,
x
^
(
t
)
)
.
x
^
  
(t+1)
 = 
x
^
  
(t)
 +Φ 
θ
​
 (S,{y 
j
​
 }, 
x
^
  
(t)
 ).
Training loss:

L
(
θ
)
=
E
f
[
∥
x
^
(
T
)
−
x
∗
∥
2
∥
x
∗
∥
2
+
ϵ
]
+
α
  
E
f
[
(
f
(
x
^
(
T
)
)
−
f
(
x
∗
)
)
2
]
.
L(θ)=E 
f
​
 [ 
∥x 
∗
 ∥ 
2
​
 +ϵ
∥ 
x
^
  
(T)
 −x 
∗
 ∥ 
2
​
 
​
 ]+αE 
f
​
 [(f( 
x
^
  
(T)
 )−f(x 
∗
 )) 
2
 ].
Reduces positional error from 36% to 8%.

Where it goes

D15 (Training engine) – Use NGO‑IR for automatic configuration tuning (decay schedules, heartbeat intervals, confidence thresholds) with far fewer evaluations than Bayesian optimisation.
New sub‑task: D15.1 – Integrate NGO‑IR for agent hyperparameter optimisation.

6. Beagle — GPU‑Parallel Genetic Programming
Mathematics

Beagle evolves RPN strings 
p
∈
L
p∈L with fitness

f
(
p
)
=
1
n
∑
i
=
1
n
(
p
(
x
i
)
−
y
i
)
2
.
f(p)= 
n
1
​
  
i=1
∑
n
​
 (p(x 
i
​
 )−y 
i
​
 ) 
2
 .
Population sizes up to 
10
7
10 
7
  individuals; genetic operators run massively parallel on GPU. Outperforms neural nets by up to 61% on symbolic regression.

Where it goes

D14 (Evolution engine) – As an alternative synthesis method for safety‑critical, auditable formulas that must satisfy FGGM contracts.
New sub‑task: D14.2 – Add Beagle‑based synthesis for closed‑form, provably bounded policies.

7. TurboQuant — Zero‑Loss KV‑Cache Compression
Mathematics

TurboQuant uses PolarQuant (Haar rotation + polar decomposition) + Quantized JL:

TQ
(
x
)
=
Π
~
⋅
PolarQuant
(
x
)
.
TQ(x)= 
Π
~
 ⋅PolarQuant(x).
The overhead per data block is independent of dimension and asymptotically optimal. Achieves 6× compression with zero accuracy loss.

Where it goes

D10 (Confidence system / inference) – When ContextOverflow fires, compress the KV‑cache using TurboQuant before retrying infer<T>.
New sub‑task: D10.1 – Integrate TurboQuant KV‑cache compression into inference overflow handler.

D2 (Memory subsystem L2) – Compress embedding vectors in L2 semantic memory without retrieval accuracy loss.
New sub‑task: D2.1 – Apply TurboQuant to semantic‑layer vector storage.

8. bsort / TwinArray — Non‑Comparison Sorting for Dream & Gossip
Mathematics (bsort)

bsort
(
A
,
b
)
=
{
A
,
b
<
0
 or 
∣
A
∣
≤
1
,
bsort
(
A
0
,
b
−
1
)
 
∥
 
bsort
(
A
1
,
b
−
1
)
,
else
,
bsort(A,b)={ 
A,
bsort(A 
0
​
 ,b−1)∥bsort(A 
1
​
 ,b−1),
​
  
b<0 or ∣A∣≤1,
else,
​
 
with 
A
0
=
{
x
:
bit
b
(
x
~
)
=
0
}
A 
0
​
 ={x:bit 
b
​
 ( 
x
~
 )=0}, 
A
1
=
{
x
:
bit
b
(
x
~
)
=
1
}
A 
1
​
 ={x:bit 
b
​
 ( 
x
~
 )=1}. Time 
O
(
w
n
)
O(wn), space 
O
(
w
)
O(w). TwinArray adds conditional dense‑key handling for 2.7× speedup.

Where it goes

D6 (Dream cycle – compress phase) – Replace the standard sort in DreamPhase::Compress with bsort for 64‑bit keys.
Add to task: D6 – “Use bsort for sorting episodic entries during compress.”

D4 (Memory coherency – gossip) – Sorting dirty keys for Merkle diff generation uses TwinArray.
Add to task: D4 – “Use TwinArray for key sorting in gossip rounds.”

9. Layerwise LQR — Second‑Order Preconditioning for Differentiable Training
Mathematics

min
⁡
{
Δ
θ
ℓ
}
∑
ℓ
=
1
L
(
Δ
θ
ℓ
⊤
Q
ℓ
Δ
θ
ℓ
+
2
q
ℓ
⊤
Δ
θ
ℓ
)
 s.t. 
Δ
θ
ℓ
=
A
ℓ
Δ
θ
ℓ
−
1
+
B
ℓ
u
ℓ
.
{Δθ 
ℓ
​
 }
min
​
  
ℓ=1
∑
L
​
 (Δθ 
ℓ
⊤
​
 Q 
ℓ
​
 Δθ 
ℓ
​
 +2q 
ℓ
⊤
​
 Δθ 
ℓ
​
 ) s.t. Δθ 
ℓ
​
 =A 
ℓ
​
 Δθ 
ℓ−1
​
 +B 
ℓ
​
 u 
ℓ
​
 .
LLQR learns structured preconditioners 
H
ℓ
H 
ℓ
​
  (diagonal, K‑FAC) without forming the global curvature.

Where it goes

D15 (Training engine) – When ASL adds differentiable computations (e.g., prompt tuning), LLQR provides second‑order convergence.
New sub‑task: D15.2 – Implement LLQR‑based optimiser for differentiable training blocks.
(High effort; defer to Phase K if needed.)

10. ParEVO — Automatic Parallelisation of Compiler & VM Phases
Mathematics

Fitness:

F
(
p
)
=
1
correct
(
p
,
T
d
)
⋅
(
α
⋅
Speedup
(
p
)
−
β
⋅
Complexity
(
p
)
)
.
F(p)=1 
correct
​
 (p,T 
d
​
 )⋅(α⋅Speedup(p)−β⋅Complexity(p)).
LLM‑guided crossover and mutation using execution profiles (cache misses, load balance). 106× geometric‑mean speedup on parallel tasks.

Where it goes

Phase C (Compiler Completion) – ParEVO can synthesise the parallel orchestration for independent compiler phases (type‑checking, lowering).
New task: C14 – Parallelise compiler pipeline using ParEVO.

Phase D (VM Completion) – Parallelise memory‑layer operations (e.g., DREAM compress across layers).
New task: D28 – Parallelise memory/dream operations with ParEVO.

Summary: Improvement ➔ Task Mapping
Improvement	Task(s)
Graph Normalization	D18.1 (Orchestrator), D6.1 (DREAM consolidation), D26 (MoE routing)
FalconGEMM	D27 (GPU‑accelerated GEMM for inference)
EVIL + CodeEvolve	D14.1 (skill synthesis), C11.1 (compiler peephole opt)
SCM	D4.1 (federated analytics index)
NGO‑IR	D15.1 (hyperparameter tuning)
Beagle	D14.2 (auditable policy synthesis)
TurboQuant	D10.1 (KV‑cache compression), D2.1 (L2 vector compression)
bsort / TwinArray	D6 (compress sort), D4 (gossip key sorting)
Layerwise LQR	D15.2 (differentiable training optimiser)
ParEVO	C14 (parallel compiler), D28 (parallel memory ops)
All recommendations stay fully within ASL’s core invariants: uncertainty never silently collapses, every effect is provenance‑tracked, and every synthesised policy is auditable and rollback‑able. If you want me to write the actual Rust signatures or elaborate any integration in detail, just say the word.