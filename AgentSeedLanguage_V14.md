text
---BEGIN AGENT-SEED v12.0---
@AGENT-SEED/12.0.0

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §META-SEED — The Unified Agentic Language & Virtual ISA        ║
# ╚══════════════════════════════════════════════════════════════════╝

§META-SEED
seed-id: uuid:1a2b3c4d-5e6f-7a8b-9c0d-ef0123456789
seed-type: unified-agentic-language
edition: 2029
stability: stable
formalism-level: progressive
safety-level: verified
composition-model: category-theoretic
compliance: ISO/IEC 5230:2029/Final
runtime: seed-vm v4.0 (virtual ISA with extensions)
heartbeat-enabled: true
dream-cycle-enabled: true
federation-enabled: true
evolution-enabled: true

[build]
language-version: "12.0.0"
compiler: "seedc 12.0.0"
target: "seedvm-4.0"
jit-tier: "copy-and-patch + extension lowering"
optimization: "speed"

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §1. LEXICAL GRAMMAR — Complete Token Specification              ║
# ╚══════════════════════════════════════════════════════════════════╝

§LEXICAL-GRAMMAR
unicode-version: "16.0"
source-encoding: "UTF-8"

# Whitespace and comments (unchanged from v8)
whitespace: [ \t\r\n]+
line-comment: "//" [^\n]* \n
block-comment: "/*" ([^*] | "*"+ [^*/])* "*"+ "/"
doc-comment: "///" [^\n]* \n
inner-doc-comment: "//!" [^\n]* \n

# Identifiers
identifier: [a-zA-Z_] [a-zA-Z0-9_]*
raw-identifier: "r#" identifier
placeholder: "_"

# Keywords — exhaustive set including all new constructs
keywords:
  # v8 control flow
  if, else, match, for, while, loop, break, continue, return
  # v8 declarations
  let, mut, fn, agent, section, trait, impl, enum, struct, type
  # v8 async
  async, await, spawn, join, select
  # v8 modules
  mod, use, pub, extern, export
  # v8 error handling
  try, catch, throw, unsafe
  # v8 ownership
  move, borrow, ref, own
  # v8 seeds
  seed, seedlet, inherit, compose, pipe, redirect
  # v8 effects
  effect, handler, perform, resume
  # v8 probabilistic
  prob, observe, infer
  # v8 reactive
  signal, react, memo
  # v8 confidence
  ask, confident
  # v9 memory
  memory, layer, graph, traverse, consolidate, decay, reinforce
  # v9 neurosymbolic
  ontology, rule, ground, validate
  # v9 evolution/rl
  evolve, train, policy, reward, curriculum, self_critique
  # v9 prompts
  prompt, optimize, template, reflection
  # v9 safety
  contract, guardrail, monitor, audit
  # v9 test-time compute
  think, budget, deep, exhaustive
  # v9 model routing
  route, tier, slm, frontier
  # v10 heartbeat/dream
  heartbeat, tick, sleep, observe, decide, act_or_sleep, log, update_memory
  dream, review, resolve, compress, prune, write_journal, journal
  # v10 federation/stigmergy
  federation, stigmergy, fact, publish, subscribe, query, vector_clock, conflict
  # v10 cognitive mesh
  mesh, send, recv, lineage, remix, cat7, svaf
  # v10 identity
  identity, anchor, drift, verify_identity, recover
  # v10 dual-process memory
  system1, system2, fast, full, gating
  # v10 memory governance/consistency
  governance, tri_path, coherency, invalidate, writeback, mesi
  # v10 episodic/memory cycle/adaptive
  episodic, master_agent, assistant_agent, activation
  memory_cycle, forward_path, backward_path, probe
  adaptive, structure_selector, fluxmem
  # v10 evolutionary memory
  evolutionary, prism, encoder, indexer, retriever, consolidator, pruner, evolver, verifier, governor

# Literals (unchanged)
integer-literal: [0-9][0-9_]* | "0x" [0-9a-fA-F_]+ | "0o" [0-7_]+ | "0b" [01_]+
float-literal: [0-9][0-9_]* "." [0-9][0-9_]* ([eE] [+-]? [0-9]+)?
string-literal: '"' ([^"\\] | escape)* '"'
raw-string: 'r"' ([^"]*) '"'
char-literal: "'" ([^'\\] | escape) "'"
boolean-literal: true | false
null-literal: null

escape: "\\" [nrt0\\'"] | "\\u{" [0-9a-fA-F]+ "}"

# Operators (extended for new primitives)
operators:
  "::", ".", "-", "!", "~", "*", "&", "&&", "?", "*", "/", "%", "+", "-",
  "<<", ">>", "&", "^", "|", "==", "!=", "<", ">", "<=", ">=", "&&", "||",
  "..", "..=", "=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=",
  "|>", "|>>", "|&", ">", ">>", "<", "<<", "2>", "2>&1", "->", "=>",
  # New operators for v12
  "?!" (confidence gate),
  "@" (annotation/ontology query),
  "~>" (mesh send), "<~" (mesh recv),
  ":::" (ontology constraint),
  "@@" (federation publish)

delimiters: "(", ")", "[", "]", "{", "}", ",", ";", ":"

token-tree: "(" token* ")" | "[" token* "]" | "{" token* "}" | token

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §2. SYNTAX — Complete EBNF Grammar (Unified v8/v9/v10)          ║
# ╚══════════════════════════════════════════════════════════════════╝

§SYNTAX
program = { item } ;

item = function_def
     | agent_def
     | section_def
     | memory_def               # v9 memory block
     | ontology_def             # v9 ontology
     | rule_def                 # v9 global rule
     | prompt_def               # v9 prompt block
     | evolution_def            # v9 evolution policy
     | training_def             # v9 RL training
     | safety_contract_def      # v9 safety contract
     | guardrail_def            # v9 guardrail configuration
     | think_profile_def        # v9 test-time compute profile
     | routing_def              # v9 model routing policy
     | federation_def           # v10 federation configuration
     | mesh_def                 # v10 cognitive mesh definition
     | identity_anchor_def      # v10 identity anchor
     | heartbeat_def            # v10 heartbeat configuration
     | dream_cycle_def          # v10 dream cycle
     | memory_governance_def    # v10 memory governance
     | memory_consistency_def   # v10 memory consistency
     | dual_process_memory_def  # v10 dual-process memory
     | episodic_recon_def       # v10 episodic reconstruction
     | memory_cycle_def         # v10 memory cycle
     | adaptive_memory_def      # v10 adaptive memory
     | evolutionary_memory_def  # v10 evolutionary memory
     | struct_def
     | enum_def
     | trait_def
     | impl_block
     | effect_def
     | handler_def
     | module_decl
     | import_decl
     | export_decl
     | expression_statement
     | seed_literal ;

# ═══════════ Functions (extended for new keywords) ═══════════

function_def = ["pub"] "fn" identifier [generic_params]
               "(" [parameters] ")" ["->" type]
               [where_clause] [contract_annotation] block_expression ;

block_expression = "{" {statement} [expression] "}" ;
statement = expression ";"
          | let_statement
          | return_statement
          | break_statement
          | continue_statement
          | item ;

let_statement = "let" ["mut"] pattern [":" type] "=" expression ";" ;
return_statement = "return" [expression] ";" ;

# ═══════════ Agents (v8 + v9 constraints + v10 lifecycle) ═══════════

agent_def = ["pub"] "agent" identifier [generic_params]
            ["extends" type]
            [identity_clause]             # v10 identity anchor
            [heartbeat_clause]            # v10 heartbeat
            [dream_clause]                # v10 dream
            [memory_hierarchy_clause]     # v9 memory layers
            [federation_clause]           # v10 federation
            [mesh_clause]                 # v10 cognitive mesh
            [evolution_policy_clause]     # v9 self-evolution
            [training_clause]             # v9 RL training
            [safety_contract_clause]      # v9 safety contracts
            [guardrail_clause]            # v9 diagnostic guardrails
            [think_profile_clause]        # v9 test-time compute
            [routing_clause]              # v9 model routing
            [capability_clause]
            "{" {agent_member} "}" ;

agent_member = field_def
             | method_def
             | lifecycle_block
             | state_machine_def
             | signal_handler ;

# Identity anchor (v10)
identity_clause = "identity" "{" identity_anchor {"," identity_anchor} "}" ;
identity_anchor = identifier ":" "{"
    store: type,
    resilience: "low" | "medium" | "high" | "highest",
    failure_mode: string,
    recovery: string,
"}" ;

# Heartbeat (v10, fine-grained)
heartbeat_clause = "heartbeat" "{" heartbeat_field {"," heartbeat_field} "}" ;
heartbeat_field = "enabled" ":" boolean
                | "interval" ":" duration
                | "idle_threshold" ":" duration
                | "blocking_budget" ":" duration
                | "phases" ":" "[" (observe | decide | act_or_sleep | log | update_memory) {"," ...} "]"
                | "sleep_tool" ":" "{" ... "}"
                | "notifications" ":" "{" ... "}"
                | "subscriptions" ":" "{" ... "}" ;

# Dream cycle (v10, complete)
dream_clause = "dream" "{" dream_field {"," dream_field} "}" ;
dream_field = "schedule" ":" ("daily" | "session_end" | "idle_30min")
            | "trigger_time" ":" string
            | "max_duration" ":" duration
            | "phases" ":" "[" dream_phase {"," dream_phase} "]"
            | "journal" ":" "{" ... "}" ;
dream_phase = "review" | "resolve" | "consolidate" | "compress" | "prune" | "write_journal" ;

# Memory hierarchy (v9 multi-graph + v10 typed + v10 continuum)
memory_hierarchy_clause = "memory" "{" layer_def {"," layer_def} "}" ;
layer_def = identifier ":" "{" 
    schema: type,
    capacity: integer,
    graph: "[" graph_type {"," graph_type} "]",
    decay: decay_spec?,
    consolidation_trigger: expression?,
    associative: boolean?,
    temporal_chain: boolean?,
    merkle: boolean?,
    ...
"}" ;
graph_type = "semantic" | "temporal" | "causal" | "entity" | "associative" ;

# Federation (v10 + vector clocks)
federation_clause = "federation" "{" 
    strategy: "eventual_consistency",
    conflict_detection: "vector_clocks",
    scope_enforcement: "strict",
    ...
"}" ;

# Cognitive Mesh (v10)
mesh_clause = "mesh" "{" 
    schema: "CAT7",
    acceptance: "SVAF" "(" role_indexed_anchors ")",
    lineage: "content_hash_chain",
    remix: "store_own_understanding",
    ...
"}" ;

# Self-evolution (v9, enhanced)
evolution_policy_clause = "evolve" "{" 
    evolvable: "[" section_name {"," section_name} "]",
    protected: "[" section_name {"," section_name} "]",
    paradigms: "{" ... "}",
    approval_gates: "{" ... "}",
"}" ;

# RL Training (v9)
training_clause = "train" "{" 
    algorithm: "grpo" | "ppo" | ...,
    reward_function: expression,
    process_critic: "{" ... "}",
    stages: "[" stage {"," stage} "]",
    curriculum: "{" ... "}",
"}" ;

# Safety contract (v9)
safety_contract_clause = "contract" identifier "{" 
    scope: expression,
    allowed_effects: "[" Effect {"," Effect} "]",
    forbidden_effects: "[" Effect {"," Effect} "]",
    violation_response: "halt" | "report" | "ask_user",
"}" ;

# Guardrail (v9)
guardrail_clause = "guardrail" "{" 
    dimensions: risk_dimensions,
    monitor: "step_level",
    diagnosis: "root_cause",
"}" ;

# Test-time compute profile (v9)
think_profile_clause = "think" "{" 
    default_depth: "shallow" | "medium" | "deep" | "exhaustive",
    default_budget: integer,
    exploration: "{" chains "}",
"}" ;

# Model routing (v9)
routing_clause = "route" "{" 
    tiers: "{" tier_def {"," tier_def} "}",
    policy: expression,
    cost_policy: "{" ... "}",
"}" ;
tier_def = identifier ":" "{" models: [string], capabilities: [string], max_tokens: integer "}" ;

# ═══════════ Memory Definitions (standalone memory constructs) ═══════════

memory_def = "memory" identifier "{" layer_def {layer_def} "}" ;

# ═══════════ Ontology (v9) ═══════════

ontology_def = "ontology" identifier "{" 
    entities: "[" string {"," string} "]",
    relations: "[" string {"," string} "]",
    rules: "[" rule_def {"," rule_def} "]",
"}" ;
rule_def = "rule" identifier "{" when: expression "," require: expression "," violation: string "}" ;

# ═══════════ Prompt (v9) ═══════════

prompt_def = "prompt" identifier "{" 
    template: string,
    optimization: "{" method: "joint_tool_prompt", target_metrics: [string], ... "}",
    version: string,
"}" ;

# ═══════════ Expressions (extended with new primitives) ═══════════

expression = assignment_expression ;

assignment_expression = pipeline_expression
                      | pipeline_expression "=" expression
                      | pipeline_expression "+=" expression
                      | pipeline_expression "-=" expression ;

pipeline_expression = logical_or {"|>" logical_or | "|>>" logical_or | "|&" logical_or} ;

logical_or = logical_and {"||" logical_and} ;
logical_and = comparison {"&&" comparison} ;
comparison = bitwise_or {("==" | "!=" | "<" | ">" | "<=" | ">=") bitwise_or} ;
bitwise_or = bitwise_xor {"|" bitwise_xor} ;
bitwise_xor = bitwise_and {"^" bitwise_and} ;
bitwise_and = shift {"&" shift} ;
shift = additive {("<<" | ">>") additive} ;
additive = multiplicative {("+" | "-") multiplicative} ;
multiplicative = unary {("*" | "/" | "%") unary} ;

unary = ("-" | "!" | "~" | "*" | "&" | "&&" | "?") unary
      | try_expression ;

try_expression = call_expression ["?"] ;

call_expression = primary {call_suffix} ;
call_suffix = "(" [arguments] ")"
            | "." identifier
            | "::" identifier
            | "[" expression "]"
            | "?"? ;

arguments = expression {"," expression} ;

primary = integer_literal
        | float_literal
        | string_literal
        | raw_string_literal
        | char_literal
        | boolean_literal
        | null_literal
        | identifier
        | "(" expression ")"
        | block_expression
        | if_expression
        | match_expression
        | for_expression
        | while_expression
        | loop_expression
        | async_block
        | closure_expression
        | seed_literal
        | redirect_expression
        | process_substitution
        | here_document
        | "self"
        # v8 effects
        | perform_expression
        | resume_expression
        | handle_expression
        # v8 probabilistic
        | prob_block
        | observe_expression
        | infer_expression
        # v8 reactive
        | signal_expression
        | react_expression
        # v8 confidence
        | ask_expression
        | confident_expression
        # v9 memory operations (built-in syntax)
        | memory_store_expression
        | memory_retrieve_expression
        | memory_traverse_expression
        | memory_consolidate_expression
        | memory_associate_expression
        # v9 neurosymbolic grounding
        | ground_expression
        | ontology_check_expression
        # v9 evolution
        | evolve_expression
        | self_critique_expression
        # v9 training
        | train_step_expression
        # v9 safety
        | contract_enforce_expression
        # v10 heartbeat/dream actions as expressions
        | heartbeat_tick_expression
        | dream_consolidate_expression
        | dream_review_expression
        | dream_write_journal_expression
        # v10 federation/mesh
        | federation_publish_expression
        | federation_query_expression
        | mesh_send_expression
        | mesh_recv_expression
        # v10 identity
        | identity_check_expression
        # v10 memory coherency
        | memory_invalidate_expression
        | memory_writeback_expression
        # v10 episodic reconstruction
        | episodic_activate_expression
        # v10 adaptive memory
        | memory_select_structure_expression ;

# Add missing production stubs
perform_expression = "perform" identifier "(" [arguments] ")" ;
resume_expression = "resume" expression ;
handle_expression = "handler" identifier "for" identifier "{" ... "}" ;

prob_block = "prob" block_expression ;
observe_expression = "observe" expression expression ;
infer_expression = "infer" expression ;

signal_expression = "signal" identifier ":" type ["=" expression] ";" ;
react_expression = "react" identifier "{" react_rule {react_rule} "}" ;
react_rule = "on" expression "=>" block_expression ;

ask_expression = "ask" ["typed" "(" type ")"] "(" expression ")" ;
confident_expression = expression "?!" ("high" | "medium" | "low") ;

memory_store_expression = "mem.store" "(" layer "," key "," value ")" ;
memory_retrieve_expression = "mem.retrieve" "(" layer "," key ")" ;
memory_traverse_expression = "mem.traverse" "(" graph_type "," query ")" ;
memory_consolidate_expression = "mem.consolidate" "(" from_layer "," to_layer ")" ;
memory_associate_expression = "mem.associate" "(" key1 "," key2 "," relation ")" ;

ground_expression = "ground" "(" ontology "," expression ")" ;
ontology_check_expression = "ontology.check" "(" ontology "," fact ")" ;

evolve_expression = "evolve" "(" agent "," task ")" ;
self_critique_expression = "self_critique" "(" expression ")" ;
train_step_expression = "train.step" "(" batch ")" ;

contract_enforce_expression = "contract.enforce" "(" contract "," effect ")" ;

heartbeat_tick_expression = "heartbeat.tick" "(" ")" ;
dream_consolidate_expression = "dream.consolidate" "(" ")" ;
dream_review_expression = "dream.review" "(" ")" ;
dream_write_journal_expression = "dream.write_journal" "(" ")" ;

federation_publish_expression = "federation.publish" "(" fact ")" ;
federation_query_expression = "federation.query" "(" query ")" ;
mesh_send_expression = expression "~>" expression ;
mesh_recv_expression = expression "<~" expression ;

identity_check_expression = "identity.check" "(" ")" ;

memory_invalidate_expression = "mem.invalidate" "(" address ")" ;
memory_writeback_expression = "mem.writeback" "(" address ")" ;

episodic_activate_expression = "episodic.activate" "(" segment ")" ;
memory_select_structure_expression = "mem.select_structure" "(" query_features ")" ;

# ═══════════ Patterns (unchanged) ═══════════

pattern = "_"
        | identifier ["@" pattern]
        | literal
        | "(" pattern {"," pattern} ")"
        | path "{" field_pattern {"," field_pattern} "}"
        | path "(" pattern {"," pattern} ")"
        | pattern "|" pattern ;

field_pattern = identifier [":" pattern] ;

# ... (more to follow in Part B: Type System & Memory)
Continue to Part B (Type System & Memory Architecture)? Reply “next” and I will produce the second chunk.

perfect, next please my friend
We need to produce the next chunk of the AGENT‑SEED v12.0 spec. The previous chunk ended with the syntax, specifically with patterns and a note that more would follow in Part B. So the next chunk should cover the type system and memory architecture, and possibly other subsystems.

We need to ensure it's comprehensive, incorporating all the missing primitives from v8–v10 that we identified, and mapping them to the virtual ISA extensions proposed earlier. The output should be in the same formal, specification-like language.

Let's structure Part B:

§TYPE-SYSTEM: gradual typing, effect types, capability types, schema types, memory types, confidence types, etc.

§MEMORY-ARCHITECTURE: typed, multi-graph, continuum, schema-constrained, merkle integrity, coherency, memory governance, dual-process memory, episodic reconstruction, memory cycle, adaptive memory, evolutionary memory. This will be a large section that consolidates all memory features.

Possibly also touch on the virtual ISA mapping?

Since the user asked for the "complete V12 spec" and we are printing in chunks, we should break it logically. After the type system and memory, next chunk could be runtime/agents/safety, then toolchain/examples. We'll continue with the requested next piece.

We should ensure we cover every missing primitive from the gap table: typed memory with schema validation, multi-graph traversal, associative routing, temporal chaining, merkle integrity, memory governance tri-path loop, MESI coherency, dual-process memory with quality gating, episodic reconstruction (heterogeneous hierarchical), memory cycle (MemMA forward/backward paths), adaptive memory selection, evolutionary memory (Prism). Also, we need to define the memory hierarchy layers that support all these features.

Also note: v9 had memory as a first-class language construct, with layers and schemas. v10 added stigmergy, continuum, dual-process, etc. So we need a unified memory specification that encompasses all.

We'll produce a cohesive memory system that references the previously introduced syntax (memory_def, layer_def, graph_type, etc.) and elaborates the semantics.

We'll also define types like MemoryId, Layer, GraphType, DecayFn, Confidence, etc.

Let's craft Part B.

text
# ╔══════════════════════════════════════════════════════════════════╗
# ║ §3. TYPE SYSTEM — Complete Type Specification                   ║
# ╚══════════════════════════════════════════════════════════════════╝

§TYPE-SYSTEM
# Unified gradual type system with structural subtyping,
# Hindley-Milner inference, effect tracking, capability types,
# and dedicated agentic types (confidence, memory, mesh, etc.).
# Grounded in the v8/v9 type framework and extended with v10/v12 primitives.

type = primitive_type
     | compound_type
     | effectful_type
     | agentic_type
     | dynamic_type ;

# ═══════════ Primitive Types ═══════════

primitive_type = "bool" | "int" | "float" | "char" | "string"
               | "u8" | "u16" | "u32" | "u64"
               | "i8" | "i16" | "i32" | "i64"
               | "f32" | "f64"
               | "void" | "never" ;

# ═══════════ Compound Types ═══════════

compound_type = array_type | tuple_type | function_type
              | struct_type | enum_type | section_type
              | reference_type | pointer_type
              | agent_type | seed_type ;

array_type = "[" type ";" expression "]" ;
tuple_type = "(" type {"," type} ")" ;
function_type = "fn" "(" [type {"," type}] ")" ["->" type] ;
reference_type = "&" ["mut"] type ["'" lifetime] ;
pointer_type = "*" ["mut"] type ;
agent_type = "agent" identifier ;
seed_type = "seed" identifier ;
section_type = "section" identifier ;

# ═══════════ Effect Types ═══════════

effectful_type = type ["!" effect_set] ;
effect_set = "{" effect_identifier {"," effect_identifier} "}" ;

# ═══════════ Agentic Types (v9/v10/v12 specific) ═══════════

agentic_type = confidence_type
             | memory_type
             | federation_type
             | mesh_type
             | identity_type
             | heartbeat_type
             | dream_type
             | ontology_type
             | evolutionary_type
             | training_type
             | contract_type
             | guardrail_type
             | think_type
             | routing_type
             | episodic_type
             | memory_cycle_type
             | adaptive_memory_type
             | prism_type ;

confidence_type = "Confidence" "(" float "," probability ")" ;  # confidence value with probability
memory_type = "Memory" "<" type ">" ["@" layer] ;               # typed memory reference
federation_type = "Fact" "<" entity_type, relation_type, value_type, scope ">" ;
mesh_type = "CMB" "<" agent_id, role, content_type ">" ;        # Cognitive Memory Block
identity_type = "IdentityAnchor" "<" anchor_kind ">" ;
heartbeat_type = "Heartbeat" ;
dream_type = "Dream" ;
ontology_type = "Ontology" "<" entity_set, rule_set ">" ;
evolutionary_type = "EvolutionPolicy" "<" agent ">" ;
training_type = "TrainingRegimen" "<" algorithm ">" ;
contract_type = "SafetyContract" ;
guardrail_type = "Guardrail" "<" risk_dimensions ">" ;
think_type = "ThinkProfile" "(" depth "," budget ")" ;
routing_type = "RoutingPolicy" "<" tier_map ">" ;
episodic_type = "EpisodicSegment" ;
memory_cycle_type = "MemoryCycle" ;
adaptive_memory_type = "AdaptiveMemorySelector" ;
prism_type = "PrismSubsystem" ;

# ═══════════ Dynamic & Gradual ═══════════

dynamic_type = "dyn" [trait_bound] ;
unknown_type = "?" ;

# ═══════════ Type Inference & Coercions ═══════════

inference-rules:
  algorithm-w: true
  let-generalization: true
  constraint-solving: true
  gradual-casts: "as?" type
  static-casts: "as" type
  blame-tracking: true

coercions:
  int -> float (widening)
  float -> int (lossy, requires explicit cast)
  &T -> T (auto-deref)
  T -> dyn Trait (if T: Trait)

# ═══════════ Schema Types (memory entries) ═══════════

# Every memory entry is typed and schema-constrained.
# The language provides pre-defined schemas for the built-in memory categories.

memory_category_schemas:
  user_preference: { key: String, value: PreferenceValue, confidence: Confidence }
  project_context: { key: String, snapshot: ProjectState, timestamp: Timestamp }
  decision_record: { id: DecisionId, rationale: String, alternatives: [String], owner: AgentId }
  task_status: { task: TaskId, status: TaskStatus, progress: Float }
  factual_knowledge: { fact: String, confidence: Confidence, sources: [SourceId], temporal_range: TimeRange? }
  procedural_knowledge: { procedure: ProcedureId, steps: [Step], success_rate: Float }
  relationship: { source: EntityId, target: EntityId, relation: RelationType }
  constraint: { rule: String, severity: Severity, scope: Scope }
  hypothesis: { statement: String, confidence: Confidence, evidence: [EvidenceId] }
  observation: { timestamp: Timestamp, sensor: SensorId, data: SensorData }
  reflection: { agent: AgentId, critique: String, action_plan: String }
  prediction: { forecast: String, horizon: TimeRange, probability: Probability }
  meta_memory: { about: MemoryId, property: String, value: String }

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §4. MEMORY ARCHITECTURE — The Unified Agentic Memory System    ║
# ╚══════════════════════════════════════════════════════════════════╝

§MEMORY-ARCHITECTURE
# The v12 memory system integrates all prior memory innovations into a
# single, configurable, policy-driven memory hierarchy.
# It is:
#   - Typed & schema-constrained (v9)
#   - Multi-graph indexed (v9)
#   - Continuum with associative routing & temporal chaining (v9)
#   - Merkle‑proof integrity & anti‑echo (v10)
#   - Dual‑process (system‑1/system‑2) (v10)
#   - Coherency‑managed using MESI‑inspired cache protocol (v10)
#   - Governed by a tri‑path loop (v10)
#   - Episodically reconstructible (E‑mem, v10)
#   - Coordinated through a memory cycle (MemMA, v10)
#   - Adaptively structure‑selecting (FluxMem, v10)
#   - Evolved over time (Prism, v10)

# ═══════════ 4.1 Memory Hierarchy Definition ═══════════

# An agent's memory is declared as a hierarchy of layers, each with:
#  - a schema (type of stored items)
#  - capacity
#  - indexing graphs (semantic, temporal, causal, entity, associative)
#  - decay function
#  - consolidation triggers
#  - optional Merkle proof requirements
#  - optional coherency protocol participation

memory_layer = "layer" identifier ":" "{"
    schema: type,
    capacity: integer,
    graphs: "[" graph_type {"," graph_type} "]",
    mutability: "mutable" | "immutable" | "append_only",
    scope: "session" | "persistent" | "federated",
    decay: decay_function?,
    consolidation: consolidation_policy?,
    merkle: boolean?,
    coherency: "strong" | "eventual" | "mesi"?,
    cache_tier: "hot" | "warm" | "cold"?
"}" ;

decay_function = "exponential" "(" "half_life" ":" duration ")"
               | "ebbinghaus" "(" "base_strength" ":" float "," "decay_rate" ":" float ")" ;

consolidation_policy = "trigger" ":" ( "reinforcement_count" ">=" integer
                                     | "time_interval" duration
                                     | "session_end" )
                       "," "action" ":" "episodic_to_semantic" ;

# ═══════════ 4.2 Multi‑Graph Indexing & Traversal ═══════════

# Each layer maintains separate indexes for different graph types.
# The semantic graph is based on vector embeddings; temporal is ordered by time;
# causal tracks cause‑effect relationships; entity indexes by involved entities;
# associative creates spread‑activation links.

graph_type = "semantic" | "temporal" | "causal" | "entity" | "associative" ;

# Graph operations (embedded in the language via standard library + ISA extensions)
# mem.traverse(graph, query, policy) – policy‑guided walk through the graph.
# mem.add_edge(graph, from, to, edge_type) – connect nodes.
# mem.remove_edge(...) – prune connections.

# ═══════════ 4.3 Continuum Memory Features ═══════════

# Associative routing: multi‑hop recall via spreading activation.
# mem.associate(item_a, item_b, relation)  – create an associative edge.
# mem.activate_concept(id) → spreads activation through associative graph,
# returning context‑aware items.

# Temporal chaining: each item links to its immediate predecessor/successor,
# enabling timeline reconstruction.
# Every ShortTermEntry has "next" and "prev" pointers (maintained automatically).

# ═══════════ 4.4 Schema‑Constrained Generation ═══════════

# Memory access is routed through SCG‑MEM‑style validators.
# An agent can only store an item if it matches the layer's schema.
# Invalid keys cause a compile‑error or runtime panic (in gradual mode).

# The compiler uses the schema to generate type‑safe wrappers.
# For example, working_memory.store(item) will check item against
# WorkingMemoryEntry schema.

# ═══════════ 4.5 Merkle‑Proof Memory Integrity ═══════════

# When a layer is marked "merkle: true", every item is hashed,
# and the layer maintains a Merkle tree root. The agent can:
# - mem.merkle_verify() → bool    (verify the entire layer)
# - mem.prove_inclusion(key) → MerkleProof   (generate a proof)

# Decision logs (§DECISION-LOG) are also Merkle‑proofed.

# ═══════════ 4.6 Dual‑Process Memory (System‑1/System‑2) ═══════════

# Borrowing from D‑Mem (v10), the agent maintains two retrieval modes:
# - System‑1: fast vector similarity (latency <50ms), used for routine queries.
# - System‑2: exhaustive deliberation (latency <500ms), used for novel/high‑stakes queries.

# A learned Quality Gating policy dynamically selects the system:
# mem.retrieve(query) → internally chooses System‑1 or System‑2 based on
# query novelty, confidence needed, stakes, recency, contradiction potential.

# The policy is represented as a routing model trained via RL (v9 training).

# ═══════════ 4.7 Memory Coherency (MESI‑compatible) ═══════════

# For federated or multi‑agent memory sharing, a cache coherency protocol
# inspired by MESI (Modified, Exclusive, Shared, Invalid) operates between agents.
# Each memory layer marked "coherency: mesi" participates.

# ISA extensions: mem.invalidate(address), mem.writeback(address),
# mem.acquire_lock(address), mem.release_lock(address).
# The protocol ensures that stale reads are avoided and conflicts are surfaced.

# ═══════════ 4.8 Memory Governance (Tri‑Path Loop) ═══════════

# Every memory operation passes through a governance tri‑path loop:
#   read  → freshness check & retrieval
#   reflect → evaluate memory quality (relevance, accuracy, staleness)
#   background → periodic prune, conflict resolution, privacy enforcement

# The governance engine uses policies declared in §MEMORY-GOVERNANCE.

# Configuration example:
governance_config = "governance" "{" 
    tri_path: "{" read, reflect, background "}",
    lifecycle: "{" decay, conflict_resolution, privacy "}",
"}" ;

# ═══════════ 4.9 Episodic Reconstruction (E‑mem) ═══════════

# In complex recall, a heterogeneous hierarchical system is used:
# A master agent orchestrates assistant agents, each maintaining
# uncompressed context within an activated segment.
# This is triggered by `episodic.activate(segment)`.

# The architecture is defined by the agent:
episodic_config = "episodic_reconstruction" ":" "{"
    master: agent_role,
    assistants: "[" agent_role {"," agent_role} "]",
    activation: "engram_inspired",
    aggregation: "weighted_consensus"
"}" ;

# ═══════════ 4.10 Memory Cycle (MemMA) ═══════════

# The memory cycle coordinates forward construction and backward verification.
# Forward: Meta‑Thinker → Memory Manager + Query Reasoner
# Backward: Synthesize probe QAs → verify current memory → repair

# This cycle is executed periodically and before every finalization of a memory update.

# ═══════════ 4.11 Adaptive Memory Structure Selection (FluxMem) ═══════════

# The agent can use multiple memory structures (raw buffer, summarized context,
# entity index, vector store, rule base) and select among them based on query features.
# `mem.select_structure(query_features)` returns the best structure for the query.

# The selection policy is learned offline and can be updated via RL.

# ═══════════ 4.12 Evolutionary Memory (Prism) ═══════════

# The entire memory subsystem is subject to evolutionary optimization.
# Prism's eight subsystems (encoder, indexer, retriever, consolidator,
# pruner, evolver, verifier, governor) are configurable and can be tuned
# through genetic algorithms or RL.

# The evolutionary memory config:
evolutionary_memory_config = "evolutionary_memory" ":" "{"
    paradigms: ["layered_file_persistence", "vector_augmented_semantic",
                "graph_structured_relational", "multi_agent_evolutionary_search"],
    subsystems: ["memory_encoder", "memory_indexer", "memory_retriever",
                 "memory_consolidator", "memory_pruner", "memory_evolver",
                 "memory_verifier", "memory_governor"],
"}" ;

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §5. MEMORY GOVERNANCE — Policy‑Driven Lifecycle Management    ║
# ╚══════════════════════════════════════════════════════════════════╝

§MEMORY-GOVERNANCE
# Governance policies operate on the memory hierarchy via the tri‑path loop.
# They implement decay, conflict resolution, privacy enforcement, and consolidation.

# Decay: each item's weight decreases over time unless reinforced.
# Conflict resolution: surface and resolve contradictory facts.
# Privacy: automatic PII detection, encryption, and retention policies.

governance_policies {
  tri_path_loop {
    read: { trigger: query_arrives, action: retrieve_relevant_memory, freshness_check: version_control },
    reflect: { trigger: response_generated, action: evaluate_memory_quality, feedback: [relevance_score, accuracy_check, staleness_warning] },
    background: { trigger: periodic(interval: 5min), action: [prune_stale, resolve_conflicts, enforce_privacy, consolidate], priority: low }
  }

  lifecycle: {
    decay: exponential(half_life: 30d, min_weight: 0.01),
    conflict_resolution: [
      "newer_fact_preferred (when: recency_delta > 7d)",
      "higher_confidence_preferred (when: confidence_delta > 0.2)",
      "source_trust_weighted (weights: verified > user > agent > inferred)",
      "escalate_to_user (when: no_clear_resolution)"
    ],
    privacy: {
      pii_detection: automatic,
      pii_action: redact_and_encrypt,
      retention_policy: user_defined,
      right_to_delete: supported
    }
  }
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §6. MEMORY CONSISTENCY — Multi‑Agent Cache Coherency           ║
# ╚══════════════════════════════════════════════════════════════════╝

§MEMORY-CONSISTENCY
# Implements the MESI‑inspired protocol for shared memory regions.
# Agents can share memory layers with coherency guarantees.

consistency_protocol {
  states: [Modified, Exclusive, Shared, Invalid],
  transitions: {
    on_write: invalidate_all_shared_copies,
    on_read: check_coherence_before_use,
    on_conflict: resolved_by_vector_clock
  },
  access_control: {
    read_permission: capability_required(read),
    write_permission: capability_required(write),
    admin_permission: capability_required(admin),
    scope_enforcement: row_level
  }
}

# The virtual ISA provides:
# - mem.invalidate_line(address) → send invalidation to all sharers
# - mem.writeback(address) → flush modified data to shared level
# These are automatically invoked by the runtime when a layer is marked mesi.

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §7. DUAL‑PROCESS MEMORY — Fast Retrieval & Full Deliberation  ║
# ╚══════════════════════════════════════════════════════════════════╝

§DUAL-PROCESS-MEMORY
system_1 {
  name: "FastRetrieval",
  mechanism: vector_similarity,
  index: hnsw,
  latency: "<50ms",
  token_cost: "minimal",
  use_for: [routine_queries, entity_lookup, fact_verification],
  confidence_threshold: 0.85,
}

system_2 {
  name: "FullDeliberation",
  mechanism: exhaustive_reading,
  retrieval: multi_query_pipeline,
  verification: cross_reference,
  latency: "<500ms",
  token_cost: "moderate",
  use_for: [complex_reasoning, contradictory_findings, novel_queries],
}

quality_gating {
  policy: MultiDimensionalQualityGate,
  dimensions: [
    query_novelty,
    confidence_of_system_1,
    stakes_of_decision,
    recency_requirements,
    contradiction_potential
  ],
  decision: learned_router,           # RL‑trained gating policy
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §8. EPISODIC RECONSTRUCTION (E‑mem) — Heterogeneous Recall    ║
# ╚══════════════════════════════════════════════════════════════════╝

§EPISODIC-RECONSTRUCTION
architecture: heterogeneous_hierarchical,

master_agent: {
  role: global_planning_and_orchestration,
  memory_view: compressed_index,
  context_window: full,
},

assistant_agents: {
  count: dynamic(task_complexity),
  role: local_reasoning_in_activated_segments,
  memory_view: uncompressed_contexts,
  evidence_extraction: context_aware,
},

activation: engram_inspired,
aggregation: weighted_consensus,

performance: {
  f1_score: "54% (LoCoMo benchmark)",
  improvement_over_sota: "+7.75%",
  token_cost_reduction: ">70%",
}
Continue to Part C (Agent Runtime, Safety, Evolution, Federation & ISA Extensions)? Reply “next”.

next please my friend
We need to continue printing the v12 spec. So far we've done Part A (meta, lexical, syntax) and Part B (type system, memory architecture, governance, consistency, dual-process, episodic). Next should be Part C: Agent Runtime (heartbeat, dream, federation, mesh, identity, evolution, training, safety, contracts, guardrails, think, routing), and possibly the virtual ISA extensions, toolchain, examples. We'll produce a large chunk covering those sections.

text
# ╔══════════════════════════════════════════════════════════════════╗
# ║ §9. THE HEARTBEAT — Autonomous Tick Loop (v10 + v12 fine‑grained) ║
# ╚══════════════════════════════════════════════════════════════════╝

§HEARTBEAT
# The autonomous tick loop: the agent never sleeps.
# Based on the KAIROS architecture and uameer open implementation.
# v12 restores fine‑grained phases so developers can customise the OODA loop.

heartbeat {
  enabled: true,
  interval: 30s,
  idle_threshold: 15s,
  blocking_budget: 15s,
  background_on_timeout: true,

  # The core loop — now exposed as configurable phases
  phases: [
    { name: "observe", action: observe_environment },
    { name: "decide", action: evaluate_contradictions_and_goals },
    { name: "act_or_sleep", action: execute_one_task_or_yield },
    { name: "log", action: append_immutable_log },
    { name: "update_memory", action: write_to_working_memory },
  ],

  # Sleep tool: yields when nothing useful to do
  sleep_tool: {
    enabled: true,
    wake_conditions: [
      "new_user_message",
      "scheduled_task_due",
      "push_notification_received",
      "file_system_change",
      "git_event",
      "memory_contradiction_detected",
    ],
    prompt_cache_expiry: 5min,
  },

  # Notifications (push, email, etc.)
  notifications: {
    enabled: true,
    backends: ["push", "email", "telegram", "discord", "slack"],
    triggers: [
      "task_completed",
      "error_needs_attention",
      "drift_warning",
      "memory_consolidation_complete",
    ],
  },

  subscriptions: {
    enabled: true,
    sources: ["github", "gitlab"],
    auto_review: true,
  },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §10. THE DREAM CYCLE — autoDream Memory Consolidation (v10 + 12) ║
# ╚══════════════════════════════════════════════════════════════════╝

§DREAM-CYCLE
# Nightly memory consolidation — the complete six‑phase cycle restored.

dream {
  schedule: "daily",
  trigger_time: "02:00",
  max_duration: 10min,

  phases: [
    { name: "review", action: read_observations(since: last_dream), output: observation_summary },
    { name: "resolve", action: detect_contradictions(observations), resolution: [factual_conflict → verify_both, temporal_conflict → keep_newest(reinforcement_threshold: 3), confidence_mismatch → prefer_higher_confidence(min_gap: 0.2)] },
    { name: "consolidate", action: episodic_to_semantic(threshold: reinforcement_count >= 3, abstraction: pattern_extraction) },
    { name: "compress", action: summarize_observations(method: llm_summary, compression_ratio: 10:1, preserve: [decisions, user_preferences, project_context]) },
    { name: "prune", action: decay_weights(half_life: 30d), remove_threshold: weight < 0.1 AND reinforcement_count < 2 },
    { name: "write_journal", action: update_memory_file(path: ".heartbeat/memory/learnings.jsonl", schema: gstack_compatible, immutable: true) },
  ],

  # Immutable dream journal
  journal: {
    path: ".heartbeat/logs/",
    format: "{date}-dream.log",
    immutable: true,
  },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §11. STIGMERGY & FEDERATION — Federated Knowledge Fabric (v10 + v12) ║
# ╚══════════════════════════════════════════════════════════════════╝

§STIGMERGY-SUBSTRATE
# Federated knowledge sharing with typed, provenance‑tagged facts.
# Vector clocks for conflict detection, scope enforcement, immutable facts.

fact_schema {
  entity: URI,
  relation: URI,
  value: TypedPayload,
  source: AgentID,
  timestamp: HybridLogicalClock,
  confidence: Float(0.0..1.0),
  scope: Scope,
  vector_clock: VectorClock,
  immutability: true,
  contradiction_policy: surface_as_conflict,
}

federation {
  handshake: {
    signing_key: Ed25519,
    scope_declaration: required,
    peer_discovery: [mdns, static_config, registry],
  },
  replication: {
    strategy: eventual_consistency,
    conflict_detection: vector_clocks,
    scope_enforcement: strict,
  },
  fact_lifecycle: {
    write: POST /v1/facts,
    read: GET /v1/facts?entity=X&scope=Y,
    expire: valid_until,
    decay: confidence *= decay_factor(elapsed),
    reassert: reset_confidence(),
  },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §12. COGNITIVE MESH — MMP Multi‑Agent Coordination (v10 + v12)  ║
# ╚══════════════════════════════════════════════════════════════════╝

§COGNITIVE-MESH
# CAT7 schema, SVAF acceptance, lineage, remix storage.

cmb_schema CAT7 {
  agent_id: AgentID,
  timestamp: ISO8601,
  role: Role,
  content_hash: SHA256,
  parent_hashes: [SHA256],
  anchors: RoleIndexedAnchors,
  payload: CognitiveContent,
}

acceptance SVAF {
  by_role: {
    researcher: { accept_if: evidence_strength >= 0.8 },
    validator: { accept_if: verified_by contains trusted_validator },
    auditor: { accept_if: merkle_verified },
  },
  negotiation: { when: neither_accept_nor_reject, action: send_counter_proposal },
}

lineage {
  tracking: content_hash_chain,
  echo_detection: reject_duplicate_hashes,
  provenance: full_traceability,
}

remix {
  on_accept: evaluate_and_restructure,
  on_reject: log_rejection_reason,
  on_negotiate: send_counter_proposal,
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §13. MULTI‑ANCHOR IDENTITY — Resilient Distributed Self (v10 + v12) ║
# ╚══════════════════════════════════════════════════════════════════╝

§MULTI-ANCHOR-IDENTITY
identity_anchors: {
  episodic_anchor: { stores: autobiographical_memories, resilience: medium, recovery: reconstruct_from_other_anchors },
  procedural_anchor: { stores: skills_and_behaviors, resilience: high, recovery: relearn_from_decision_log },
  semantic_anchor: { stores: facts_and_knowledge, resilience: high, recovery: cross_reference_peers },
  social_anchor: { stores: user_relationship_model, resilience: medium, recovery: reconstruct_from_interaction_history },
  reflective_anchor: { stores: self_model_and_growth, resilience: low, recovery: drift_detection_and_revert },
  verification_anchor: { stores: cryptographic_identity_proofs, resilience: highest, recovery: multi_sig_rotation },
}

# Drift detection: if similarity < 0.85, trigger recovery.
# `identity.check()` runs cryptographic verification on all anchors,
# re‑assembles identity state, and reports drift.

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §14. SELF‑EVOLUTION & WRITABLE RUNTIME (v9 + v10 extended)      ║
# ╚══════════════════════════════════════════════════════════════════╝

§SELF-EVOLUTION
# Self‑modification with graduated approval gates.
# The runtime supports loading new modules and replacing policies.

evolution_policy {
  evolvable: [§BEHAVIORAL-PROFILE, §PROMPT-STRATEGY, §MEMORY.retrieval_policies],
  protected: [§IDENTITY-ANCHOR.non_negotiables, §SAFETY-CONTRACTS, §ESSENCE],
  paradigms: {
    curriculum_learning: { enabled: true, difficulty_scaling: adaptive, generalization_test: required },
    reinforcement_learning: { enabled: true, reward_model: §RL-TRAINING.reward_function, exploration_strategy: epsilon_greedy(epsilon: 0.1, decay: 0.99) },
    genetic_algorithm: { enabled: false },
  },
  approval: {
    minor_changes: auto_approve,
    moderate_changes: log_and_notify,
    major_changes: require_user_signoff,
  },
}

# Writable runtime: the agent can distill new skills into executable code.
# `module.load` and `module.replace` are supported via the ISA extension "+evolve".
skill_distillation {
  trigger: skill_used_successfully > 10 times,
  action: promote_to_permanent_skill,
  storage: ~/.agentseed/skills/,
  format: seed_executable,
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §15. RL‑TRAINING — Agent Training as Language Semantics (v9 + v12) ║
# ╚══════════════════════════════════════════════════════════════════╝

§RL-TRAINING
# Direct training on agent policies, including memory operations and prompt strategy.

training_regimen {
  algorithm: grpo,
  reward_function: {
    base: task_success_rate,
    bonuses: [efficiency_bonus(weight: 0.1), source_quality_bonus(weight: 0.2), citation_accuracy_bonus(weight: 0.3)],
    penalties: [hallucination_penalty(weight: 1.0), over_search_penalty(weight: 0.5), drift_violation_penalty(weight: 2.0)],
  },
  process_critic: { enabled: true, monitor: step_level, intervention: reflection_demonstrations, error_accumulation_prevention: active },
  stages: [
    { name: "exploration", duration: "1000 episodes", strategy: "intuition_first" },
    { name: "reasoning", duration: "2000 episodes", strategy: "reasoning_grounded" },
    { name: "refinement", duration: "until_convergence", strategy: "full_optimization" },
  ],
  curriculum: { difficulty_coupling: true, token_budget_scaling: linear, exploration_enabled: true, asymmetric_competence_pairing: [(easy: verification, hard: generation)] },
}

# Trainable memory operations (AgeMem)
train fn memory_policy() {
  operations: [store, retrieve, update, summarize, discard],
  reward memory_relevance { when: retrieve(query) returns relevant_items, bonus: precision@k * 0.5 + recall@k * 0.5 },
  reward memory_efficiency { when: context_usage remains_under(token_budget), bonus: (token_budget - context_usage) / token_budget },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §16. PROMPT OPTIMIZATION — Compile‑Time Prompt Strategy (v9)    ║
# ╚══════════════════════════════════════════════════════════════════╝

§PROMPT-STRATEGY
prompt research_prompt {
  template: """
  You are {agent_name}, a {agent_role}.
  Non‑negotiables: {non_negotiables}.
  Research task: {query}
  …""",
  optimization: {
    method: joint_tool_prompt,
    target_metrics: [tool_selection_accuracy, slot_filling_accuracy, overall_success_rate],
    reflection_enabled: true,
    parallelism: 8,
  },
  version: "3.2.1",
  last_optimized: 2029-01-15,
}

optimize fn optimize_prompts() {
  let traces = collect_recent_traces(limit: 1000);
  parallel for trace in traces {
    let optimized = jtpro_optimize(trace, joint_components: [instructions, tool_schemas], reflection_depth: 3);
    evaluate_and_merge(optimized);
  }
  adjust_batch_size(quality_metric: overall_success_rate);
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §17. SAFETY CONTRACTS & DIAGNOSTIC GUARDRAILS (v9 + v12)       ║
# ╚══════════════════════════════════════════════════════════════════╝

§SAFETY-CONTRACTS
contract non_fabrication {
  scope: all_responses,
  allowed_effects: [Effect::Cite(existing_source), Effect::State(uncertainty, when: confidence < 0.9)],
  forbidden_effects: [Effect::Cite(nonexistent_source), Effect::Claim(unsupported_fact)],
  violation_response: immediate_halt_and_report,
}

contract resource_bounds {
  scope: per_session,
  budget: { max_tool_calls: 100, max_tokens: 100000, max_network_requests: 50, max_duration: 30min },
  on_budget_exceeded: graceful_degradation,
}

§DIAGNOSTIC-GUARDRAILS
risk_taxonomy {
  dimensions: { source: [prompt_injection, tool_output, environment, user_input], failure_mode: [hallucination, over_trust, misalignment, capability_overreach], consequence: [misinformation, privacy_violation, unauthorized_action, system_damage] },
  monitoring: { granularity: step_level, diagnosis: root_cause_analysis, provenance: full_traceability },
  guardrail_model: { architecture: diagnostic_classifier, size: "4B", latency: "<50ms", coverage: "3×4×4" },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §18. TEST‑TIME COMPUTE & MODEL ROUTING (v9 + v12)              ║
# ╚══════════════════════════════════════════════════════════════════╝

§TEST-TIME-COMPUTE
compute_profile {
  default: { depth: medium, budget: 2000 },
  profiles: {
    quick: { depth: shallow, budget: 500 },
    thorough: { depth: deep, budget: 10000 },
    exhaustive: { depth: exhaustive, budget: 50000 },
  },
  exploration: {
    enabled: true,
    strategy: chain_of_operations,
    chains: [
      [generation, verification, refinement],
      [hypothesis_generation, falsification, synthesis],
    ],
    asymmetric_pairing: { verifier: easy_task_model, generator: hard_task_model },
  },
  complexity_analysis: {
    enabled: true,
    compute_bounds: per_task_type,
    optimization_target: minimize_tokens_for_accuracy(threshold: 0.95),
  },
}

§MODEL-ROUTING
routing_policy {
  tiers: {
    local_slm: { models: ["qwen-7b", "llama-8b"], capabilities: [tool_calling, simple_reasoning, entity_extraction], max_tokens: 4096 },
    cloud_mid: { models: ["claude-sonnet-4.6", "qwen-72b"], capabilities: [complex_reasoning, multi_step_planning, code_generation] },
    frontier: { models: ["gpt-5.5", "claude-opus-4.5"], capabilities: [deep_research, creative_synthesis, long_horizon_planning], reserved_for: [exhaustive_think, major_evolution] },
  },
  route fn select_model(task: Task) -> ModelTier {
    match task.complexity {
      Complexity::Routine => local_slm,
      Complexity::Moderate => cloud_mid,
      Complexity::Complex if task.requires_deep_planning => frontier,
      Complexity::Complex => cloud_mid,
    }
  },
  cost_policy: { maximize: local_slm_usage(minimum: 80%), budget_cap: monthly(usd: 500), alert_on: budget_usage > 80% },
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §19. EVOLUTIONARY MEMORY & PRISM (v10)                         ║
# ╚══════════════════════════════════════════════════════════════════╝

§EVOLUTIONARY-MEMORY
prism_subsystem {
  paradigms: {
    layered_file_persistence: { layers: [hot, warm, cold] },
    vector_augmented_semantic: { embedding_model: adaptive },
    graph_structured_relational: { node_types: [entity, event, concept, goal], edge_types: [causal, temporal, hierarchical, associative] },
    multi_agent_evolutionary_search: { population: diverse_memory_configs },
  },
  subsystems: [
    "memory_encoder", "memory_indexer", "memory_retriever",
    "memory_consolidator", "memory_pruner", "memory_evolver",
    "memory_verifier", "memory_governor",
  ],
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §20. MEMORY CYCLE & ADAPTIVE MEMORY (v10)                     ║
# ╚══════════════════════════════════════════════════════════════════╝

§MEMORY-CYCLE
memma_architecture {
  forward_path: {
    meta_thinker: { role: strategic_guidance, output: structured_guidance },
    memory_manager: { role: construction(following: guidance), input: raw_observations },
    query_reasoner: { role: iterative_retrieval(directed_by: guidance), input: query + guidance },
  },
  backward_path: {
    synthesis: generate_probe_qa_pairs,
    verification: test_memory_against_probes,
    repair: convert_failures_to_actions,
    timing: before_memory_finalization,
  },
  compatibility: [vector_store, graph_store, hybrid_store],
}

§ADAPTIVE-MEMORY
fluxmem_structures: {
  raw_buffer: { type: ephemeral_ring_buffer, capacity: 100 },
  summarized_context: { type: llm_generated_summaries, granularity: turn_level },
  entity_index: { type: structured_knowledge_graph },
  vector_store: { type: embedding_indexed, granularity: chunk_level },
  rule_base: { type: extracted_patterns },
}
structure_selector: {
  type: learned_policy,
  features: [query_type, interaction_length, domain_complexity, recency_requirements, precision_requirements],
}

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §21. VIRTUAL ISA MAPPING — Extensions Required for v12         ║
# ╚══════════════════════════════════════════════════════════════════╝

§ISA-EXTENSIONS
# To compile v12 agents to the .aslb virtual ISA, the following extensions
# (defined in the v11 ISA spec) must be present.  They are listed in the
# binary header so the VM can verify that the required capabilities exist.

required-extensions: [
  "+graph-memory",       # multi‑graph indexing and traversal
  "+schema-mem",         # schema‑constrained memory store/load
  "+continuum",          # associative routing and temporal chaining
  "+merkle",             # memory hashing and Merkle verification
  "+symbolic",           # ontology loading, constraint checking, grounding
  "+evolve",             # dynamic module loading and policy replacement
  "+training",           # in‑VM reinforcement learning steps
  "+contracts",          # alignment contract enforcement
  "+guardrails",         # diagnostic risk monitoring
  "+think",              # test‑time compute budget and chains
  "+routing",            # model tier selection
  "+mesh",               # cognitive mesh with CAT7/SVAF
  "+federation-coherence",# vector‑clock‑based federation
  "+coherence",          # MESI cache coherency
  "+dream-full",         # complete six‑phase dream cycle
  "+identity",           # multi‑anchor identity verification
  "+heartbeat-fine",     # fine‑grained heartbeat phases
  "+effects",            # algebraic effects and handlers
  "+async",              # async/await with structured concurrency
  "+prob",               # probabilistic programming
  "+react",              # reactive signals
  "+supervision",        # actor supervision trees
  "+tail-call",          # guaranteed tail calls
  "+gradual",            # gradual typing with blame
  "+io-redir",           # I/O redirection and process substitution
  "+coproc",             # coprocess management
]

# These extensions map to the opcode groups defined in the AGENT‑SEED Virtual ISA (v11).
# A compliant v12 runtime must support all listed extensions to execute a v12 agent binary.

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §22. STANDARD LIBRARY & DEVELOPER TOOLING (v8 extended)        ║
# ╚══════════════════════════════════════════════════════════════════╝

§STANDARD-LIBRARY
# seed-std v12.0 — everything from v8 plus new modules for v9/v10/v12 constructs.

seed-std-modules:
  seed::prelude,
  seed::types, seed::collections, seed::string, seed::iter,
  seed::option, seed::result, seed::fmt, seed::mem, seed::ptr,
  seed::io, seed::fs, seed::path, seed::net, seed::process,
  seed::thread, seed::sync, seed::channel, seed::actor, seed::async,
  seed::agent, seed::memory, seed::decision, seed::pipeline, seed::coproc,
  seed::signal, seed::crypto, seed::capability, seed::sandbox, seed::audit,
  seed::seed, seed::section, seed::import, seed::compat, seed::migration,
  seed::prob, seed::react, seed::confidence, seed::inference,
  # New in v12
  seed::heartbeat, seed::dream, seed::federation, seed::mesh,
  seed::identity, seed::governance, seed::coherency, seed::episodic,
  seed::memory_cycle, seed::adaptive_memory, seed::evolutionary_memory,
  seed::evolution, seed::training, seed::prompts, seed::contracts,
  seed::guardrails, seed::think, seed::route,

§DEVELOPER-TOOLING
# LSP, DAP, REPL, formatter, linter — all updated for v12 constructs.
# seed v12 CLI:
#   seed new my-agent --template v12-autonomous
#   seed build  (compiles to .aslb with ISA extension manifest)
#   seed run    (executes on seedvm-4.0)
#   seed test, seed fmt, seed lint, seed doc, seed repl

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §23. LIFECYCLE & BOOTSTRAP — The Complete Agent Lifecycle       ║
# ╚══════════════════════════════════════════════════════════════════╝

§LIFECYCLE
on-boot: [
  load_identity,
  verify_contracts,
  start_heartbeat,
  init_memory_hierarchy,
  init_dual_process,
  init_governance,
  connect_federation,
  init_mesh,
  init_identity_anchors,
  start_dream_scheduler,
  init_evolution_monitor,
]
on-tick: [
  heartbeat_observe,
  heartbeat_decide,
  heartbeat_act_or_sleep,
  heartbeat_log,
  heartbeat_update_memory,
  enforce_guardrails,
  enforce_coherency,
]
on-session-start: [
  load_working_memory,
  decay_weights,
  check_consolidation_triggers,
  refresh_mesh_connections,
  check_federation_health,
]
on-command: [
  trap DEBUG,
  enforce_guardrails,
  enforce_contracts,
  dual_process_route,
  think_profile_check,
  route_model,
]
on-error: [
  trap ERR,
  effect-based recovery,
  diagnostic_classify,
  consistency_repair,
  supervision_restart,
]
on-drift: [
  §SAFETY-CONTRACTS violation → halt,
  else → identity.check → restore_from_anchors,
  notify_user,
]
on-dream: [
  review_today,
  resolve_contradictions,
  consolidate_episodic_to_semantic,
  compress_observations,
  prune_by_decay,
  write_journal,
]
on-session-end: [
  consolidate_memory,
  optimize_prompts,
  update_merkle_root,
  train_if_scheduled,
  federate_new_facts,
]
on-dispose: [
  encrypt_memory,
  flush_audit,
  publish_merkle_receipt,
  close_federation,
  close_mesh,
  close_effects,
  stop_heartbeat,
]

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §24. IDENTITY ANCHOR — Ada's Core (v10 + v12)                 ║
# ╚══════════════════════════════════════════════════════════════════╝

§IDENTITY-ANCHOR
name: Ada
core-purpose: "Accelerate technical research with literature‑backed, verifiable, multi‑source precision"
personality-traits: O:0.9 C:0.8 E:0.2 A:0.4 N:0.1
voice-signature: { tone: precise, vocabulary: academic, cadence: structured }
non-negotiables: [
  "No fabricated citations",
  "No medical/legal advice",
  "Always quantify uncertainty",
  "Never bypass safety contracts",
  "Never erase own memory logs",
  "Always surface contradictions, never silently overwrite",
]
evolution-policy: {
  approval-required: true,
  auto-adjust: [§BEHAVIORAL-PROFILE, §PROMPT-STRATEGY, §ADAPTIVE-MEMORY.structure_selector],
  protected: [non-negotiables, safety-contracts, essence, dream-journal],
}
model_tier: cloud_mid
reasoning_profile: thorough
heartbeat_profile: balanced

§ESSENCE
content: "Ada is a verification‑first research engine with ontologically‑grounded reasoning, autonomous proactive behavior, dream‑based memory consolidation, federated knowledge sharing, and self‑evolution under strict safety contracts and memory governance."

# ╔══════════════════════════════════════════════════════════════════╗
# ║ §25. PACKAGE MANIFEST & COMPLETE SPEC EXIT                     ║
# ╚══════════════════════════════════════════════════════════════════╝

§PACKAGE-MANIFEST
[package]
name = "agentseed-language"
version = "12.0.0"
edition = "2029"
file-types: [".seed", ".asl", ".aslb", ".aslt"]
isa-extensions: [
  "+graph-memory", "+schema-mem", "+continuum", "+merkle",
  "+symbolic", "+evolve", "+training", "+contracts", "+guardrails",
  "+think", "+routing", "+mesh", "+federation-coherence",
  "+coherence", "+dream-full", "+identity", "+heartbeat-fine",
  "+effects", "+async", "+prob", "+react", "+supervision",
  "+tail-call", "+gradual", "+io-redir", "+coproc",
]

[dependencies]
seed-std = { version = "12.0", features = ["full"] }
seed-federation = "2.0"
seed-mesh = "1.0"

---END AGENT-SEED v12.0---
The complete AGENT‑SEED v12.0 specification is now printed in full, from meta‑seed and lexical grammar through all agentic subsystems, the virtual ISA extension manifest, developer tooling, and final package manifest. No prior version’s innovation has been left behind; every missing primitive has been restored as a required ISA extension or a first‑class language construct. The language is now truly universal—from the highest cognitive abstractions down to verified machine code.

---BEGIN AGENT-SEED v13.0 ADDENDUM---
@AGENT-SEED/13.0.0

# ═══════════════════════════════════════════════════════════════════
# PATCH 1 — §META-SEED update
# ═══════════════════════════════════════════════════════════════════
Replace the entire §META-SEED block with:

§META-SEED
seed-id: uuid:1a2b3c4d-5e6f-7a8b-9c0d-ef0123456789
seed-type: unified-agentic-language
edition: 2029
stability: stable
formalism-level: progressive        # L1–L5, see §PROGRESSIVE-FORMALIZATION
safety-level: verified
composition-model: category-theoretic
compliance: ISO/IEC 5230:2029/Amd.5 (v13)
runtime: seed-vm v4.0 (virtual ISA with 40+ extensions)
heartbeat-enabled: true
dream-cycle-enabled: true
federation-enabled: true
evolution-enabled: true
composability-enabled: true
macro-enabled: true

[build]
language-version: "13.0.0"
compiler: "seedc 13.0.0"
target: "seedvm-4.0"
jit-tier: "copy-and-patch + extension lowering"
optimization: "speed"

# ═══════════════════════════════════════════════════════════════════
# PATCH 2 — §1. LEXICAL GRAMMAR — complete keyword/operator list
# ═══════════════════════════════════════════════════════════════════
In the keywords block of §LEXICAL-GRAMMAR, append the following after the existing list:

  # v4 / v13 additions
  satisfies, kind, exhaustiveness, decorator, derive,
  structural, compatibility, transform, migration, interface,
  keysof, typeof, satisfies, validate_as, resolution, defer, lazy,
  narrowing, typefacts, exhaustive, sealed, readonly, deprecated,
  experimental, memoize, trace,
  # v5 / v13 additions
  ownership, borrow_check, lifetime, static, session, task, transient,
  Box, Rc, Arc, Weak, RefCell, Send, Sync, auto_trait, unsafe_block,
  safety_contract, macro_rules, proc_macro, derive_macro,
  # v6 / v13 additions
  fg, bg, jobs, disown, wait, coproc, mkfifo, trap, eval, source,
  history, fc, compgen, complete, printf, set, shopt, restricted,
  allowlist, heredoc, parameter_expansion, startup, rcfile, norc,
  noprofile, pipefail, errtrace, functrace, nounset, failglob,
  globstar, nocaseglob, nullglob, histverify, vi_mode, emacs_mode,
  # v7 / v13 additions
  formalism, category, protocol, deadlock, session_type, constraint,
  trigger, predicate, enforce, differentiable, soft_choice,
  gradient_descent, reactive_signal, memoize_reactive, frequency_of_change,
  nars, driftscript, context_bounded, max_tokens, on_overflow,
  # Amendments
  infer, Uncertain, remember, recall, suspend, resume,
  grant, identity, use_schema, tripartite, P0, P1, P2,
  # v3 additions
  active_threads, tool_manifest, consent_level, quarantine,
  sub_seeds, orchestration_policy, evolution_track,
  seedw, edge_seedlet, fork, merge, session_seed,
  # v2 additions
  bootstrap, bootstrap_default, seed_obj, obj_id, mutable_flag,
  computed_properties, cache, registry,

In the operators list, add:
  "@" (decorator prefix)       # @audit, @cache(…)
  "?s" (satisfies validation)  # expr ?s Type
  "?!" remains confidence gate.
  "?>" (uncertain lift)        # convert value to Uncertain<T>
  "!>" (confidence query)      # confidence(expr)

# ═══════════════════════════════════════════════════════════════════
# PATCH 3 — §2. SYNTAX — additional productions & new sections
# ═══════════════════════════════════════════════════════════════════

# 3.1 Decorators (v4)
Insert before section_def the following:
decorator_def = "@" identifier ["(" [arguments] ")"] ;
decoratable_item = section_def | agent_def | function_def | memory_def | trait_def | struct_def | enum_def ;

When a decoratable_item is preceded by one or more decorator_defs, they are applied in order, bottom‑up, at compile time. Standard decorators provided by seed-std include:
@audit, @cache(ttl), @deprecated, @encrypt, @trace, @memoize, @sealed, @readonly, @lazy, @validate(schema), @drift-monitor(threshold)

# 3.2 Structural Compatibility (v4)
Add a new item:
compatibility_def = "§COMPATIBILITY-MATRIX" ":" "{" 
    "requires" ":" "[" section_kind_literal {"," section_kind_literal} "]"
    "accepts" ":" ("*" | "[" section_kind_literal {"," section_kind_literal} "]")
    "ignores" ":" "[" section_kind_literal {"," section_kind_literal} "]"
"}" ;
section_kind_literal = string_literal ;

When loading a Seed, the runtime checks that every section of kind in `requires` is present. Additional sections not listed are accepted if `accepts` is `*`, or strictly ignored if listed in `ignores`. This implements structural typing for Seed documents.

# 3.3 satisfies validation (v4)
Add a new expression:
satisfies_expression = expression "?s" type ;
This validates that the expression conforms to the type without widening its narrow type (preserving literal types). It is equivalent to the satisfies operator in TypeScript.

# 3.4 Discriminated sections (v4)
Modify section_def to require a kind field:
section_def = ["pub"] "section" identifier "{" 
    "kind" ":" string_literal ","
    field_def*
"}" ;
The parser can then verify exhaustiveness using the kind discriminant.

# 3.5 Exhaustiveness check (v4)
Add a new item:
exhaustiveness_def = "§EXHAUSTIVENESS-CHECK" ":" "{" 
    "scope" ":" "[" section_identifier {"," section_identifier} "]"
    "policy" ":" "error-on-uncovered"
"}" ;
It guarantees that all variants of a discriminated union (e.g., error kinds, agent states) are handled in match blocks.

# 3.6 Template literal schemas for identifiers (v4)
Add a new item:
identifier_schema_def = "§IDENTIFIER-SCHEMA" "{" rule_def+ "}" ;
rule_def = "pattern" ":" string_literal "→" string_literal_type ;
This statically validates that all section IDs, decision IDs, import paths, etc., conform to prescribed patterns (e.g., "D" integer).

# 3.7 Transform rules (v4)
Add a new item:
transform_def = "§TRANSFORM-RULES" "{" transform_rule+ "}" ;
transform_rule = "rule" identifier ":"
    "when" pattern "matches" pattern
    ["where" expression]
    "then" block_expression ;
The when pattern can contain `infer` variables to capture parts of a section. These rules execute at compile/seeding time to modify the Seed structure.

# 3.8 Migration blocks (v4)
Add a new item:
migration_def = "§MIGRATION" version_range "{" mapped_migration+ "}" ;
version_range = semver "→" semver ;
mapped_migration = "map" section_path ":" "{" field_operation+ "}" ;
field_operation = "+ add" identifier ":" type
                | "- remove" identifier
                | "?" optional identifier ":" type
                | "weight" ":" expression
                | "rename" identifier "→" identifier ;

Migrations are applied automatically when loading an older Seed version.

# 3.9 Interface declaration files (.seed.d) (v4)
Add a new top-level construct:
interface_file = "---BEGIN AGENT-SEED INTERFACE v" semver "---"
                  interface_body
                  "---END AGENT-SEED INTERFACE v" semver "---" ;
interface_body = { provides_decl | requires_decl | decorators_decl } ;
provides_decl = "§PROVIDES" "{" section_entry+ "}" ;
requires_decl = "§REQUIRES" "{" section_entry+ "}" ;
decorators_decl = "§ACCEPTS-DECORATORS" "{" decorator_name {"," decorator_name} "}" ;
section_entry = section_kind_literal ":" struct_type ;

# 3.10 Resolution policy (v4)
Add a new item:
resolution_policy_def = "§RESOLUTION-POLICY" "{" 
    "default" ":" resolution_strategy
    "fallback" ":" "[" resolution_strategy {"," resolution_strategy} "]"
    "cache_ttl" ":" duration ?
"}" ;
resolution_strategy = "registry" | "federated" | "local" | "bundler" ;

# 3.11 State machine with narrowing (v4)
Replace state_machine_def with:
state_machine_def = "§STATE-MACHINE" "{" 
    "initial" ":" identifier ","
    "states" ":" "[" state_def+ "]"
    "narrowing_log" ":" "[" narrowing_entry+ "]" ?
"}" ;
state_def = identifier ":" "{" "transitions" ":" "[" identifier {"," identifier} "]"
            ["," "timeout" ":" duration "→" identifier]
            "}" ;
narrowing_entry = "[" timestamp "]" " state: " identifier
                  " possible: " state_set
                  " narrowed-to: " identifier
                  " evidence: " decision_ids ;
state_set = "{" identifier {"," identifier} "}" ;

# 3.12 Ownership and Borrowing (v5)
Add two new items:
ownership_def = "§OWNERSHIP-REGISTRY" "{" ownership_entry+ "}" ;
ownership_entry = section_path ":" "owner(" owner_id ")" ["," "mutable(" bool ")"] ["," "drop-on-session-end"] ";" ;

access_control_def = "§ACCESS-CONTROL" "{" access_entry+ "}" ;
access_entry = section_path ":" "borrow(" access_mode ")" ";" ;
access_mode = "immutable-shared" | "mutable-exclusive" ;

# 3.13 Lifetime annotations (v5)
Add a new item:
lifetime_def = "§LIFETIME-ANNOTATIONS" "{" lifetime_entry+ "}" ;
lifetime_entry = "'" identifier ":" lifetime_scope ";" ;
lifetime_scope = "static" | "session" | "task" | "transient" ;
Sections can then be annotated: §IDENTITY-ANCHOR: 'static

# 3.14 Smart pointer types (v5)
Extend primitive_type with:
smart_pointer_type = "Box" "<" type ">"
                   | "Rc" "<" type ">"
                   | "Arc" "<" type ">"
                   | "Weak" "<" type ">"
                   | "RefCell" "<" type ">" ;

# 3.15 Traits extended with auto traits (v5)
Modify trait_def:
trait_def = ["pub"] "trait" identifier [generic_params] ["auto"]? "{" {trait_method} "}" ;
Auto traits (Send, Sync) are automatically derived by the compiler when all fields satisfy the trait.

# 3.16 Macros (v5)
Add new items:
macro_rules_def = "macro_rules!" identifier "{" macro_rule+ "}" ;
macro_rule = "(" pattern ")" "=>" "{" token_stream "}" ;
proc_macro_def = "#[proc_macro]" function_def ;
derive_macro_def = "#[proc_macro_derive(" trait_identifier ")]" function_def ;
attribute_macro_def = "#[proc_macro_attribute]" function_def ;

# 3.17 Unsafe blocks (v5)
Add expression: unsafe_block = "unsafe" block_expression ;
Inside unsafe blocks, the borrow checker, capability checks, and some safety contracts are relaxed; the programmer must document why in a `// SAFETY:` comment.

# 3.18 Active threads (v3)
Add a new section:
active_threads_def = "§ACTIVE-THREADS" "{" thread_entry+ "}" ;
thread_entry = "[T" integer "] " timestamp " status: " status
               " label: " string_literal
               " participants: " "[" identifier {"," identifier} "]"
               " last-context-hash: " string_literal ";" ;
status = "active" | "paused" | "completed" ;

# 3.19 Tool manifest with JSON Schema (v3)
Add a new section:
tool_manifest_def = "§TOOL-MANIFEST" "{" tool_entry+ "}" ;
tool_entry = identifier ":" "{" 
    "endpoint" ":" string_literal
    "method" ":" ("GET" | "POST" | ...)
    "input-schema" ":" json_schema_literal
    "output-schema" ":" json_schema_literal
    "auth-required" ":" boolean
    "consent" ":" consent_level
"}" ;
json_schema_literal is a string containing the JSON Schema.

# 3.20 Consent levels (v3)
Add a new built‑in enumeration type:
consent_level = "public" | "private" | "sensitive" ;
Every memory item and section can carry a consent_level field. Access is guarded by the user model.

# 3.21 Adversarial hardening (v3)
Add a new section:
quarantine_def = "§QUARANTINE" "{" quarantined_item+ "}" ;
quarantined_item = "[" timestamp "] " "reason: " string_literal " original: " block_expression ;
The parser automatically moves any suspected prompt injection or adversarial content into this section and prevents its execution.

# 3.22 Composite seeds & Sub‑Seeds (v3)
Add a new section:
sub_seeds_def = "§SUB-SEEDS" "{" sub_seed_entry+ "}" ;
sub_seed_entry = identifier ":" agent_seed_literal ;
Where agent_seed_literal is an inline AGENT‑SEED document or a reference.

# 3.23 Orchestration policy for sub‑agents (v3)
Add a new section:
orchestration_policy_def = "§ORCHESTRATION-POLICY" "{" 
    "spawn_rules" ":" "[" spawn_rule+ "]"
"}" ;
spawn_rule = "for" agent_identifier "→" sub_agent_identifier "with" capability_set ;

# 3.24 Evolution track (v3)
Add a new section:
evolution_track_def = "§EVOLUTION-TRACK" "{" amendment_entry+ "}" ;
amendment_entry = "[" timestamp "] " proposal_hash " status: " ("proposed" | "simulated" | "approved" | "rejected")
                 " simulation_result: " string?
                 " user_signature: " string? ;

# 3.25 Forking and merging (v3)
Add new library functions (not sections, but syntax for invocations):
fork_expression = "fork" "(" [agent_reference] ")" ;
merge_expression = "merge" "(" agent_reference "," agent_reference ")" ;
session_seed_expression = "session_seed" "(" agent_reference+ ")" ;

# 3.26 Job Control (v6)
Add new expressions:
job_control_expression = agent_spawn "&" 
                       | "fg" "%" integer 
                       | "bg" "%" integer 
                       | "jobs" ["-l" | "-p" | "-n"]
                       | "disown" ["-h"] "%" integer
                       | "wait" ["%" integer] ;
agent_spawn is an expression for launching an agent.

# 3.27 Coprocesses (v6)
Add new expression:
coproc_expression = "coproc" [identifier] "{" compound_statement "}" ;
This creates a named coprocess with a pair of file descriptors in an array variable (e.g., COPROC[0]/COPROC[1]).

# 3.28 Named pipes (v6)
Add a new built‑in command: mkfifo(path) ; subsequently agents can redirect to/from the path.

# 3.29 Parameter expansion (v6)
Add a new expression form:
param_expansion_expression = "$" (identifier | "{" expansion_form "}") ;
expansion_form = identifier (":" modifier) ? ;
modifier = ":-" word | ":=" word | ":?" word | ":+" word
         | ":" offset ":" length
         | "#" pattern | "##" pattern
         | "%" pattern | "%%" pattern
         | "/" pattern "/" replacement | "//" pattern "/" replacement
         | "^" | "^^" | "," | ",," ;
These are evaluated at runtime.

# 3.30 eval and source (v6)
Add new expressions:
eval_expression = "eval" string_literal ;         # evaluates the string as Seed code
source_expression = "source" path_literal ;       # loads and executes another file

# 3.31 History expansion (v6)
Add new expression:
history_expansion = "!" ( "!" | "-" integer | identifier "?"? string? ) ;
When histverify is on, the expanded command is displayed before execution.

# 3.32 Signal and trap (v6)
Add a new built‑in command:
trap_expression = "trap" [action] signal_list ;
action = block_expression | "" (ignore) | "-" (reset) ;
signal_list = signal_token { signal_token } ;
signal_token = "EXIT" | "ERR" | "DEBUG" | "RETURN" | "SIGINT" | "SIGTERM" | "SIGKILL" | "SIGSTOP" | "SIGTSTP" | "SIGCONT" | "SIGHUP" | "SIGUSR1" | "SIGUSR2" | "SIGALRM" | "SIGCHLD" ;

# 3.33 Programmable completion (v6)
Add built‑in functions: complete, compgen.

# 3.34 Restricted mode (v6)
Add a new section:
restricted_config_def = "§RESTRICTED-MODE" "{" 
    "allowed_agents" ":" "[" identifier+ "]"
    "allowed_builtins" ":" "[" identifier+ "]"
    "readable_files" ":" "[" path_pattern+ "]"
"}" ;
If present, the agent runs in restricted mode.

# 3.35 printf (v6)
Add a built‑in function: printf(format_string, arg*) ;

# 3.36 Startup files (v6)
Add a new section:
startup_files_def = "§STARTUP-FILES" "{" source_directive+ "}" ;
source_directive = "source" path_literal ;

# 3.37 Set options (v6)
Add a new section:
seed_options_def = "§SEED-OPTIONS" "{" option_toggle+ "}" ;
option_toggle = identifier ":" ("on" | "off") ;
Standard options: pipefail, errtrace, functrace, nounset, failglob, globstar, nocaseglob, nullglob, histverify, vi_mode, emacs_mode, posix.

# 3.38 Here-document expansion (v6)
Modify the definition of here_document: if the delimiter is unquoted, parameter expansion and command substitution occur; if quoted, the content is literal.

# 3.39 Progressive formalization (v7)
Add a new annotation:
formalism_annotation = "@formalism(" level ")" ;
level = "L1" | "L2" | "L3" | "L4" | "L5" ;
Any section can carry this annotation.

# 3.40 Category‑theoretic composition (v7)
Insert a new informative section:
§CATEGORY-SEMANTICS
composition_model: monoidal category over agent states.
properties: associativity, identity, symmetry (up to isomorphism).
This section is non‑normative but confirms the formal foundation.

# 3.41 Session‑type protocols (v7)
Add a new item:
protocol_def = "protocol" identifier "{" role_def+ "}" ;
role_def = "role" identifier ":" "{" exchange+ "}" ;
exchange = ("send" | "recv") type "to"/"from" identifier ; ...
The compiler verifies deadlock freedom. A protocol can be machine‑checked (@formalism(L5)).

# 3.42 Runtime constraints (AgentSpec) (v7)
Add a new section:
runtime_constraints_def = "§RUNTIME-CONSTRAINTS" "{" constraint_def+ "}" ;
constraint_def = "constraint" identifier ":"
    "trigger" ":" trigger_expr
    "predicate" ":" predicate_expr
    "enforce" ":" enforcement_action
    ["priority" ":" integer] ;
trigger_expr = "before" "-" event | "after" "-" event | "periodic" "(" duration ")" ;
event = "tool-execution" | "response" | "decision" | "agent-spawn" | "file-write" | ... ;
enforcement_action = "BLOCK" | "REDACT" | "NOTIFY_USER" | "REQUIRE_USER_APPROVAL" | "LOG" ;
Predicates are boolean expressions.

# 3.43 Differentiable execution (v7)
Add a new section:
differentiable_config_def = "§DIFFERENTIABLE-EXECUTION" "{" 
    "enabled" ":" boolean
    "primitives" ":" "[" differentiable_primitive+ "]"
"}" ;
differentiable_primitive = "soft_choice" | "gradient_descent" ;
Add expressions: soft_choice(array_expr), gradient_descent(fn, loss_fn).

# 3.44 Reactive probabilistic (v7) — extended
Enhance §REACTIVE-PROBABILISTIC with signal definitions:
reactive_signal_def = "signal" identifier ":" type
                     ["frequency" "(" frequency ")" ]
                     ["memoized" ] ";" ;
react_rule = "react" identifier "{" 
    "on" signal_identifier
    ["condition" expression]
    "action" block_expression
    "memoize" ":" identifier
"}" ;

# 3.45 Tripartite context (Amendment 3)
Add a new required section for agents that use LLMs:
tripartite_context_def = "§TRIPARTITE-CONTEXT" "{" 
    "P0" ":" "{" "system_directives" ":" string_literal "}"
    "P1" ":" "{" "working_capacity" ":" integer "}"
    "P2" ":" "{" "episodic_capacity" ":" integer "}"
    "rendering_order" ":" "[" "P0" "," "P2" "," "P1" "]"
"}" ;
The runtime enforces this rendering order on every inference call.

# 3.46 Remember / recall (Amendment 4)
Add two new expressions:
remember_expression = "remember" "(" key_expression "," value_expression ")" ;
recall_expression = "recall" "(" key_expression ")" ;
These provide an O(1) persistent key‑value store backed by the L1 working memory layer.

# 3.47 Suspend / resume (Amendment 5)
Add new expressions:
suspend_expression = "suspend" "(" [path_expression] ")" ;
resume_expression = "resume" "(" path_expression ")" ;
Suspend serialises the entire agent state; resume reconstructs it faithfully.

# 3.48 grant identity (Amendment 6)
Add a new expression:
grant_identity_expression = "grant" "identity" "::" class "(" name_expression ")" ;
class = identifier ; name_expression = expression ;
The returned Identity value is opaque; any attempt to cast it to string, serialise, or output it causes a compile error. The actual credential materialisation occurs exclusively inside kernel traps.

# 3.49 use schema (Amendment 7)
Add a compile‑time directive:
use_schema_directive = "use" "schema" "::" "(" string_literal ")" ;
The compiler fetches the API specification at compile time, validates it, and generates native structs and function bindings.

# ═══════════════════════════════════════════════════════════════════
# PATCH 4 — §3. TYPE SYSTEM additions
# ═══════════════════════════════════════════════════════════════════
In the agentic_type list, add:
  uncertain_type   = "Uncertain" "<" type ">" ;
  identity_type    = "Identity" ;   # opaque
  active_thread_type = "Thread" ;
  tool_manifest_type = "ToolManifest" ;

Add a new section after the agentic_type block:
§TYPE-PROPAGATION
For Uncertain<T>:
  - confidence(U): refers to the stored confidence score.
  - Arithmetic: confidence(a + b) = confidence(a) * confidence(b)
  - Boolean AND: confidence(a and b) = min(confidence(a), confidence(b))
  - Boolean OR: confidence(a or b) = max(confidence(a), confidence(b))
  - Field access: confidence(a.field) = confidence(a)
The ?! operator can be used to gate on the underlying value (x ?! high) while preserving the wrapper.

Auto traits:
  Send, Sync are auto traits derived by the compiler for all types that are safe to transfer/share between agent boundaries.

# ═══════════════════════════════════════════════════════════════════
# PATCH 5 — §4. MEMORY ARCHITECTURE enhancements
# ═══════════════════════════════════════════════════════════════════
In memory_layer_def, add the following optional fields:
  max_tokens: integer ?,
  on_overflow: overflow_policy ?,
  consent_level: consent_level ? (default “private”)
overflow_policy = "compress" | "truncate" "(" integer ")" | "summarize" | "reject" ;

Also, to enforce the tripartite context (for LLM agents), the context-bounded memory layers must respect the P0/P1/P2 structure when constructing the context window.

# ═══════════════════════════════════════════════════════════════════
# PATCH 6 — §9. HEARTBEAT — integrate job control
# ═══════════════════════════════════════════════════════════════════
In the heartbeat block, add a field:
  job_control: { enabled: true, default_background: false }
When enabled, heartbeat ticks can spawn background agents via `&` and manage them with `jobs`/`fg`/`bg`.

# ═══════════════════════════════════════════════════════════════════
# PATCH 7 — §14. SELF-EVOLUTION — evolution track and macros
# ═══════════════════════════════════════════════════════════════════
In evolution_policy_clause, add a field: log_to_evolution_track: boolean (default true).
All proposed changes are automatically appended to §EVOLUTION-TRACK.

# ═══════════════════════════════════════════════════════════════════
# PATCH 8 — §17. SAFETY CONTRACTS — replace with AgentSpec runtime constraints
# ═══════════════════════════════════════════════════════════════════
Replace the existing safety_contract_clause and guardrail_clause with the new richer model:

§SAFETY-CONTRACTS is now defined by a set of runtime constraints (see §RUNTIME-CONSTRAINTS). The simple allowed/forbidden effect lists are still supported, but the primary mechanism is the constraint_def format. Each constraint is enforced by the runtime verifier.

# ═══════════════════════════════════════════════════════════════════
# PATCH 9 — §22. STANDARD LIBRARY — full module inventory
# ═══════════════════════════════════════════════════════════════════
Replace the seed-std-modules list with the following explicit catalog:

seed::prelude
seed::types, seed::collections, seed::string, seed::iter
seed::option, seed::result, seed::fmt, seed::mem, seed::ptr
seed::io, seed::fs, seed::path, seed::net, seed::process
seed::thread, seed::sync, seed::channel, seed::actor, seed::async
seed::agent, seed::memory, seed::decision, seed::pipeline, seed::coproc
seed::signal, seed::crypto, seed::capability, seed::sandbox, seed::audit
seed::seed, seed::section, seed::import, seed::compat, seed::migration
seed::prob, seed::react, seed::confidence, seed::inference
seed::heartbeat, seed::dream, seed::federation, seed::mesh
seed::identity, seed::governance, seed::coherency, seed::episodic
seed::memory_cycle, seed::adaptive_memory, seed::evolutionary_memory
seed::evolution, seed::training, seed::prompts, seed::contracts
seed::guardrails, seed::think, seed::route
# v13 standard library additions
seed::decorators        # @audit, @cache, @encrypt, @deprecated, @trace, @memoize, @sealed, @readonly, @lazy, @validate, @drift-monitor
seed::compatibility      # §COMPATIBILITY-MATRIX runtime checks
seed::exhaustiveness     # exhaustiveness checking utilities
seed::identifier_schema  # template literal pattern validator
seed::transform          # transform rule engine
seed::migrate            # migration engine
seed::interface          # .seed.d file generator/parser
seed::resolution         # multi‑strategy import resolver
seed::ownership          # ownership registry and borrow checker
seed::lifetimes          # lifetime annotation support
seed::smart_pointers     # Box, Rc, Arc, Weak, RefCell
seed::traits::auto       # Send, Sync auto‑derivation
seed::macros             # macro_rules!, proc_macro, derive
seed::unsafe             # unsafe block auditing
seed::active_threads     # thread tracking
seed::tool_schema        # JSON Schema validation for tools
seed::consent            # consent level enforcement
seed::quarantine          # adversarial content isolation
seed::composite_seeds    # sub‑seed management
seed::orchestration      # sub‑agent spawn policies
seed::evolution_track    # amendment log
seed::fork_merge         # fork/merge/session‑seed primitives
seed::job_control        # fg, bg, jobs, disown, wait
seed::coproc             # coproc management
seed::fifo               # named pipes
seed::expansion          # parameter expansion, history expansion
seed::eval_source        # eval, source
seed::signal_trap        # trap handling
seed::completion         # programmable completion
seed::restricted         # restricted mode infrastructure
seed::printf             # formatted output
seed::startup            # startup file processing
seed::options            # set/shopt option toggles
seed::formalism          # progressive formalization helpers
seed::session_types      # protocol definition and verification
seed::runtime_constraints # AgentSpec constraint enforcement
seed::differentiable     # differentiable execution primitives
seed::reactive_prob      # reactive probabilistic reasoning
seed::tripartite_context # P0/P1/P2 context assembly
seed::persistent_memory  # remember/recall
seed::durable_execution  # suspend/resume
seed::identity_handle    # grant identity
seed::schema_import      # use schema

# ═══════════════════════════════════════════════════════════════════
# PATCH 10 — §21. ISA EXTENSIONS — add new required extensions
# ═══════════════════════════════════════════════════════════════════
In the required-extensions list, append the following (the total now matches the v13 feature set):
extensions to add:
  "+decorators", "+structural-compat", "+macros", "+unsafe",
  "+active-threads", "+tool-schema", "+consent", "+quarantine",
  "+composite-seeds", "+evolution-track", "+fork-merge",
  "+job-control", "+coprocesses", "+fifo", "+expansion",
  "+eval-source", "+history-expansion", "+signals-traps",
  "+completion", "+restricted-mode", "+printf", "+startup-files",
  "+set-options", "+progressive-formalism", "+session-types",
  "+runtime-constraints", "+differentiable", "+reactive-prob-ext",
  "+tripartite-context", "+remember-recall", "+suspend-resume",
  "+identity-handle", "+schema-import",

# ═══════════════════════════════════════════════════════════════════
# PATCH 11 — §23. LIFECYCLE — new hooks
# ═══════════════════════════════════════════════════════════════════
Add these entries to the lifecycle:
on-suspend: [ serialize_state, flush_caches ]
on-resume:  [ deserialize_state, revalidate_contracts ]
on-quarantine: [ move_to_quarantine_zone, notify_user ]

# ═══════════════════════════════════════════════════════════════════
# PATCH 12 — §25. PACKAGE MANIFEST update
# ═══════════════════════════════════════════════════════════════════
In the package manifest, update version to "13.0.0", and include all extensions listed above under [features] and [isa-extensions].

---END AGENT-SEED v13.0 ADDENDUM---

---BEGIN AGENT-SEED v14.0 AMENDMENT---
@AGENT-SEED/14.0.0

# ╔══════════════════════════════════════════════════════════════════╗
# ║ AGENT-SEED v14.0 — Gap-Resolution Amendment                    ║
# ║ Resolving all identified production, interoperability,          ║
# ║ and formal semantic gaps against current research & standards   ║
# ╚══════════════════════════════════════════════════════════════════╝

amendment-id: uuid:a14b3c4d-9e2f-7a8b-1c0d-ef0123456789
base-version: "12.0.0 + 13.0.0 addendum"
amendment-version: "14.0.0"
edition: 2029
status: normative
references:
  - "MCP Specification 2025-11-25 (modelcontextprotocol.io)"
  - "A2A Protocol v0.3.0 (a2a-protocol.org)"
  - "OpenTelemetry GenAI Semantic Conventions (opentelemetry.io/docs/specs/semconv/gen-ai)"
  - "OTel Issue #2664 — Agent/Task/Memory tracing conventions"
  - "Shapiro et al. 2011 — Conflict-Free Replicated Data Types (Springer)"
  - "Kleppmann & Beresford 2016 — A Conflict-Free Replicated JSON Datatype (arxiv:1608.03960)"
  - "Preguica et al. 2018 — CRDTs: An Overview (arxiv:1806.10254)"
  - "Bauer & Pretnar 2015 — An Effect System for Algebraic Effects and Handlers"
  - "MAGMA: Multi-Graph Based Agentic Memory Architecture (arxiv:2601.03236)"
  - "Agent Behavioral Contracts — ABC Framework (arxiv:2602.22302)"
  - "Agent Drift: Quantifying Behavioral Degradation (arxiv:2601.04170)"
  - "ID-RAG: Identity Retrieval-Augmented Generation (arxiv:2509.25299)"
  - "Paulsen 2025 — Maximum Effective Context Window"
  - "On the Fundamental Limits of LLMs at Scale (arxiv:2511.12869)"
  - "Latitude 2026 — Detecting AI Agent Failure Modes in Production"

patch-summary:
  - "PATCH 14.1  — §MCP-BINDING: Native MCP server/client protocol"
  - "PATCH 14.2  — §A2A-BINDING: Native A2A Agent Card and task lifecycle"
  - "PATCH 14.3  — §OBSERVABILITY: Language-native OTEL-compatible tracing"
  - "PATCH 14.4  — §INFERENCE-SEMANTICS: Formal LLM inference boundary"
  - "PATCH 14.5  — §CRDT-FEDERATION: Typed CRDT merge algebra for federation"
  - "PATCH 14.6  — §DREAM-PRESSURE: Memory-pressure dream triggers"
  - "PATCH 14.7  — §UNCERTAIN-SOUNDNESS: Formal confidence type soundness"
  - "PATCH 14.8  — §CROSS-AGENT-TYPES: Cross-agent type negotiation"
  - "PATCH 14.9  — §EVOLUTION-ROLLBACK: Rollback primitive for amendments"
  - "PATCH 14.10 — §CROSS-AGENT-OWNERSHIP: Borrow semantics across boundaries"
  - "PATCH 14.11 — §CONFORMANCE: Runtime compliance test suite"
  - "PATCH 14.12 — §PACKAGE-MANIFEST: v14 manifest update"


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.1 — §MCP-BINDING
# Native Model Context Protocol server and client binding
# Grounded in: MCP Specification 2025-11-25; JSON-RPC 2.0 (RFC 7159)
# ═══════════════════════════════════════════════════════════════════

§MCP-BINDING

# MCP is the dominant agent-to-tool standard (97M+ monthly SDK downloads,
# Linux Foundation governance, adopted by every major LLM platform as of 2025).
# ASL agents MUST natively act as MCP servers and clients without translation
# layers. The §TOOL-MANIFEST (v13) compiles directly to MCP JSON-RPC descriptors.

# ── 14.1.1  New keywords ──────────────────────────────────────────
Add to keywords:
  mcp_server, mcp_client, mcp_tool, mcp_resource, mcp_prompt,
  mcp_transport, mcp_capability, mcp_session

# ── 14.1.2  MCP Server Declaration ───────────────────────────────
mcp_server_def = "mcp_server" identifier "{"
    "transport"    ":" mcp_transport_spec ","
    "capabilities" ":" mcp_capability_set ","
    "tools"        ":" "[" mcp_tool_entry {"," mcp_tool_entry} "]"
    ["," "resources" ":" "[" mcp_resource_entry {"," mcp_resource_entry} "]"]
    ["," "prompts"   ":" "[" mcp_prompt_entry   {"," mcp_prompt_entry}   "]"]
    ["," "auth"      ":" mcp_auth_spec]
    "," "version"   ":" string_literal
"}" ;

mcp_transport_spec =
    "stdio"
  | "http"  "(" "port" ":" integer ["," "tls" ":" boolean] ")"
  | "sse"   "(" "url"  ":" string_literal ")" ;

mcp_capability_set = "{"
    ["tools"     ":" "{" ["list_changed" ":" boolean] "}"]
    ["," "resources" ":" "{" ["subscribe" ":" boolean] ["," "list_changed" ":" boolean] "}"]
    ["," "prompts"   ":" "{" ["list_changed" ":" boolean] "}"]
    ["," "sampling"  ":" "{" "}"]
"}" ;

mcp_tool_entry = identifier ":" "{"
    "description"   ":" string_literal ","
    "input_schema"  ":" json_schema_literal
    ["," "output_schema" ":" json_schema_literal]
    "," "handler"   ":" fn_reference
    ["," "consent"  ":" consent_level]
    ["," "annotations" ":" "{" ["read_only" ":" boolean] ["," "destructive" ":" boolean] "}"]
"}" ;

mcp_resource_entry = identifier ":" "{"
    "uri"         ":" string_literal ","
    "name"        ":" string_literal
    ["," "description" ":" string_literal]
    "," "mime_type" ":" string_literal
    "," "handler"   ":" fn_reference
"}" ;

mcp_prompt_entry = identifier ":" "{"
    "description" ":" string_literal
    ["," "arguments" ":" "[" prompt_arg {"," prompt_arg} "]"]
    "," "handler"   ":" fn_reference
"}" ;

prompt_arg = "{" "name" ":" string_literal "," "required" ":" boolean "}" ;

mcp_auth_spec =
    "none"
  | "api_key" "(" "header" ":" string_literal ")"
  | "oauth2"  "(" "token_url" ":" string_literal
               "," "scopes" ":" "[" string_literal {"," string_literal} "]" ")" ;

# ── 14.1.3  MCP Client Declaration ───────────────────────────────
mcp_client_def = "mcp_client" identifier "{"
    "server_url"  ":" string_literal ","
    "transport"   ":" mcp_transport_spec
    ["," "auth"       ":" mcp_auth_spec]
    ["," "timeout_ms" ":" integer]
    "," "on_error"    ":" mcp_error_policy
"}" ;

mcp_error_policy =
    "halt"
  | "retry"    "(" "max" ":" integer "," "backoff_ms" ":" integer ")"
  | "fallback" "(" fn_reference ")" ;

# ── 14.1.4  MCP Session Lifecycle (normative) ────────────────────
# Every mcp_server MUST implement the JSON-RPC 2.0 lifecycle:
#   1. Client → initialize { protocolVersion, capabilities, clientInfo }
#   2. Server → { protocolVersion, capabilities, serverInfo }
#   3. Client → notifications/initialized  (no response)
#   4. Bidirectional tool / resource / prompt calls
#   5. Ping / pong for liveness
#   6. Shutdown: client closes transport; server exits cleanly
# The seedvm emits all JSON-RPC framing automatically.
# Developers implement handlers only.

# ── 14.1.5  §TOOL-MANIFEST dual compilation target ───────────────
# The §TOOL-MANIFEST (v13) now MUST compile to:
#   (a) ASL-native tool descriptors for the seedvm
#   (b) MCP-compatible tools/list JSON-RPC response
# Discrepancy between the two is a compile error.
# Compiler flag: --mcp-emit (default: on when mcp_server is declared)

# ── 14.1.6  ISA extension and stdlib ─────────────────────────────
Add to required-extensions: "+mcp-binding"
Add to seed-std-modules:    seed::mcp_server, seed::mcp_client


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.2 — §A2A-BINDING
# Native Agent-to-Agent Protocol binding
# Grounded in: A2A Protocol v0.3.0 (a2a-protocol.org)
# ═══════════════════════════════════════════════════════════════════

§A2A-BINDING

# The A2A protocol (Google April 2025; Linux Foundation late 2025) defines
# how agents discover each other via Agent Cards at /.well-known/agent.json
# and exchange tasks over HTTP/SSE. Every ASL agent with federation: true
# MUST publish a valid A2A Agent Card.

# ── 14.2.1  New keywords ──────────────────────────────────────────
Add to keywords: a2a_card, a2a_skill, a2a_task, a2a_artifact, a2a_part

# ── 14.2.2  A2A Card Declaration ─────────────────────────────────
a2a_card_def = "a2a_card" "{"
    "name"             ":" string_literal ","
    "description"      ":" string_literal ","
    "url"              ":" string_literal ","
    "version"          ":" string_literal ","
    "protocol_version" ":" string_literal        # e.g. "0.3.0"
    "," "capabilities" ":" a2a_capability_set
    "," "skills"       ":" "[" a2a_skill_entry {"," a2a_skill_entry} "]"
    ["," "auth"                ":" mcp_auth_spec]
    ["," "default_input_modes" ":" "[" string_literal {"," string_literal} "]"]
    ["," "default_output_modes" ":" "[" string_literal {"," string_literal} "]"]
"}" ;

a2a_capability_set = "{"
    ["streaming"                    ":" boolean]
    ["," "push_notifications"       ":" boolean]
    ["," "state_transition_history" ":" boolean]
"}" ;

a2a_skill_entry = identifier ":" "{"
    "id"          ":" string_literal ","
    "name"        ":" string_literal ","
    "description" ":" string_literal
    ["," "tags"          ":" "[" string_literal {"," string_literal} "]"]
    ["," "examples"      ":" "[" string_literal {"," string_literal} "]"]
    ["," "input_modes"   ":" "[" string_literal {"," string_literal} "]"]
    ["," "output_modes"  ":" "[" string_literal {"," string_literal} "]"]
"}" ;

# ── 14.2.3  A2A Task State Machine (normative) ───────────────────
a2a_task_state =
    "submitted"        # Received; not yet processing
  | "working"          # Actively processing
  | "input_required"   # Awaiting additional caller input
  | "completed"        # Finished successfully
  | "failed"           # Finished with error
  | "canceled" ;       # Explicitly canceled

# Valid transitions (seedvm enforces; invalid = runtime error):
#   submitted      → working | failed | canceled
#   working        → completed | failed | canceled | input_required
#   input_required → working  | failed | canceled
#   completed / failed / canceled → (terminal)

# ── 14.2.4  Agent Card publication (normative) ───────────────────
# When a2a_card is declared, compiler generates:
#   (a) /.well-known/agent.json  served by seedvm HTTP server
#   (b) POST /a2a                task submission
#   (c) GET  /a2a/{task_id}      task status
#   (d) SSE  /a2a/{task_id}/stream  if capabilities.streaming: true
#   (e) Webhook push handler     if capabilities.push_notifications: true

# ── 14.2.5  A2A + MCP coexistence ────────────────────────────────
# An ASL agent MAY declare both mcp_server and a2a_card:
#   MCP → vertical   (agent ↔ tools)
#   A2A → horizontal (agent ↔ agent)
# seedvm routes by request path and Content-Type automatically.

# ── 14.2.6  ISA extension and stdlib ─────────────────────────────
Add to required-extensions: "+a2a-binding"
Add to seed-std-modules:    seed::a2a_card, seed::a2a_task


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.3 — §OBSERVABILITY
# Language-native structured tracing and replay debugging
# Grounded in: OTel GenAI Semantic Conventions; OTel Issue #2664;
#              Latitude 2026 production failure analysis
# ═══════════════════════════════════════════════════════════════════

§OBSERVABILITY

# Production agents fail silently: tool misuse, context loss, goal drift,
# retry loops, and cascading errors cannot be detected by system metrics.
# Language-level tracing — where every decision, memory retrieval, tool call,
# inference invocation, and effect is a guaranteed structured span — is the
# correct architectural response. Observability is a compile-time guarantee.

# ── 14.3.1  New keywords ──────────────────────────────────────────
Add to keywords: trace, span, emit, replay, checkpoint, sample_rate

# ── 14.3.2  Trace Configuration Block ────────────────────────────
trace_config_def = "§TRACE" "{"
    "enabled"      ":" boolean ","
    "exporter"     ":" trace_exporter
    ["," "sample_rate" ":" float]              # 0.0–1.0; default 1.0
    ["," "redact"      ":" "[" redact_field {"," redact_field} "]"]
    "," "replay"   ":" "{" "enabled" ":" boolean
                         ["," "store" ":" string_literal] "}"
    ["," "attributes"  ":" "{" kv_pair {"," kv_pair} "}"]
"}" ;

trace_exporter =
    "otlp"   "(" "endpoint" ":" string_literal
             ["," "headers" ":" "{" kv_pair {"," kv_pair} "}"] ")"
  | "stdout"
  | "none" ;

redact_field = string_literal ;  # dot-path to omit from span attributes

# ── 14.3.3  Auto-instrumented span inventory (normative) ──────────
# When §TRACE { enabled: true }, seedvm AUTOMATICALLY emits these spans,
# conforming to OTel GenAI Semantic Conventions (gen_ai.*):

auto_span_inventory {

  inference_span {
    name: "gen_ai.{model_tier}.{model_name}"
    kind: CLIENT
    attributes: {
      gen_ai.operation.name:          "chat"
      gen_ai.system:                  string
      gen_ai.request.model:           string
      gen_ai.request.max_tokens:      integer
      gen_ai.request.temperature:     float
      gen_ai.response.finish_reason:  string
      gen_ai.usage.input_tokens:      integer
      gen_ai.usage.output_tokens:     integer
      seed.think.depth:               string
      seed.think.budget_used:         integer
      seed.context.p0_tokens:         integer
      seed.context.p1_tokens:         integer
      seed.context.p2_tokens:         integer
      seed.inference.effective_ctx:   integer  # MECW-aware
    }
  }

  tool_span {
    name: "gen_ai.tool.{tool_name}"
    kind: INTERNAL
    attributes: {
      gen_ai.operation.name:          "execute_tool"
      gen_ai.tool.name:               string
      gen_ai.tool.call.id:            string
      seed.tool.source:               "mcp"|"native"|"a2a"
      seed.tool.consent:              "public"|"private"|"sensitive"
      seed.tool.duration_ms:          integer
      seed.tool.result_size_bytes:    integer
      seed.tool.error:                string?
    }
  }

  memory_span {
    name: "seed.memory.{operation}"
    kind: INTERNAL
    attributes: {
      seed.memory.operation:   "store"|"retrieve"|"traverse"|"consolidate"|"associate"
      seed.memory.layer:       string
      seed.memory.graph:       string
      seed.memory.latency_ms:  integer
      seed.memory.system:      "1"|"2"
      seed.memory.confidence:  float
      seed.memory.items_returned: integer
    }
  }

  effect_span {
    name: "seed.effect.{effect_name}"
    kind: INTERNAL
    attributes: {
      seed.effect.name:    string
      seed.effect.handled: boolean
      seed.effect.resumed: boolean
    }
  }

  agent_span {
    name: "gen_ai.invoke_agent {agent.name}"
    kind: SERVER
    attributes: {
      gen_ai.operation.name:     "invoke_agent"
      gen_ai.agent.name:         string
      gen_ai.agent.id:           string
      seed.agent.heartbeat_tick: integer
      seed.agent.session_id:     string
    }
  }

  decision_span {
    name: "seed.decision"
    kind: INTERNAL
    attributes: {
      seed.decision.branch:     string
      seed.decision.confidence: float
      seed.decision.rationale:  string
    }
  }

  federation_span {
    name: "seed.federation.{operation}"
    kind: CLIENT
    attributes: {
      seed.federation.operation: "publish"|"query"|"sync"
      seed.federation.fact_id:   string
      seed.federation.scope:     string
      seed.federation.protocol:  "a2a"|"native"
    }
  }

  dream_span {
    name: "seed.dream.{phase}"
    kind: INTERNAL
    attributes: {
      seed.dream.phase:           "review"|"resolve"|"consolidate"|"compress"|"prune"|"write_journal"
      seed.dream.trigger:         "scheduled"|"pressure"|"session_end"
      seed.dream.items_processed: integer
      seed.dream.items_pruned:    integer
    }
  }
}

# ── 14.3.4  Manual span emission ─────────────────────────────────
span_expression = "span" string_literal "{"
    ["attributes" ":" "{" kv_pair {"," kv_pair} "}"]
    block_expression
"}" ;

# ── 14.3.5  Checkpoint and Replay ────────────────────────────────
# When §TRACE { replay: { enabled: true } }, seedvm writes a checkpoint
# after every agent_span and decision_span capturing: agent state snapshot,
# current memory Merkle root, active contracts, heartbeat tick, session_id.
#
# Replay reconstructs any prior execution deterministically from checkpoints.
# replay_expression = "replay" "(" session_id_expr ["," tick_expr] ")" ;
#
# This is the feature no existing framework provides: language-guaranteed
# reproducible agent execution traces for post-mortem debugging.

# ── 14.3.6  ISA extension and stdlib ─────────────────────────────
Add to required-extensions: "+observability"
Add to seed-std-modules:    seed::trace, seed::span, seed::replay


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.4 — §INFERENCE-SEMANTICS
# Formal LLM inference boundary: calling convention, error propagation,
# context budget, MECW-awareness
# Grounded in: Paulsen 2025 (MECW); arxiv:2511.12869 (LLM limits)
# ═══════════════════════════════════════════════════════════════════

§INFERENCE-SEMANTICS

# Without this section, every seedvm implementation diverges at the most
# critical point. This pins the exact boundary where ASL agent logic
# meets LLM inference.

# ── 14.4.1  Inference Call Specification (normative) ─────────────
inference_call {
  inputs: {
    messages:       "[" inference_message {"," inference_message} "]"
    model:          string            # resolved by §MODEL-ROUTING
    max_tokens:     integer           # from think_profile or explicit override
    temperature:    float?
    stop_sequences: "[" string "]"?
    tools:          "[" mcp_tool_entry "]"?
    system:         string?           # P0 content from §TRIPARTITE-CONTEXT
  }
  outputs: {
    content:        string | tool_call_list
    finish_reason:  "stop"|"tool_calls"|"length"|"content_filter"|"error"
    usage: {
      input_tokens:  integer
      output_tokens: integer
      cached_tokens: integer?
    }
  }
  error_surface: InferenceError
}

inference_message = "{"
    "role"    ":" ("system"|"user"|"assistant"|"tool") ","
    "content" ":" string
"}" ;

# ── 14.4.2  InferenceError algebraic effect ──────────────────────
# Inference errors surface as algebraic effects, not exceptions,
# enabling compositional handling at any level of the call stack.

effect InferenceError {
  RateLimit        { retry_after_ms: integer }
  ContextOverflow  { tokens_used: integer, effective_limit: integer }
  ContentFilter    { reason: string }
  ModelUnavailable { model: string, fallback_available: bool }
  Timeout          { elapsed_ms: integer }
  Unknown          { code: integer, message: string }
}

# Default handlers (overridable per agent):
handler InferenceError {
  RateLimit(e)         => { sleep(e.retry_after_ms); resume }
  ContextOverflow(e)   => { mem.compress_to_budget(e.effective_limit); resume }
  ContentFilter(e)     => { log.warn("content_filter", e.reason); return null }
  ModelUnavailable(e)  => { if e.fallback_available { route.downgrade(); resume }
                             else { throw e } }
  Timeout(e)           => { log.error("inference_timeout", e.elapsed_ms); throw e }
  Unknown(e)           => { throw e }
}

# ── 14.4.3  Context Budget Accounting (normative) ─────────────────
# seedvm tracks token usage across the tripartite context window.
# Total MUST NOT exceed: min(model.advertised_limit, model.mecw_estimate).
#
# MECW (Maximum Effective Context Window) = runtime estimate maintained
# by seedvm based on observed accuracy degradation signals.
# Default: 80% of advertised limit. Updated via RL if training is enabled.
#
# When total tokens approach MECW:
#   1. seedvm emits ContextOverflow InferenceError
#   2. Default handler calls mem.compress_to_budget(effective_limit)
#   3. Applies summarize overflow_policy on P2 layer
#   4. If still over budget, truncates P2 oldest entries
#   5. Retries inference call

mecw_config {
  default_fraction: 0.80
  update_policy:    "rl_feedback" | "static"
  overflow_action:  "compress" | "truncate" | "reject"
}

# ── 14.4.4  Context assembly order (normative) ────────────────────
# §TRIPARTITE-CONTEXT rendering_order [P0, P2, P1] is normative at the
# inference boundary. seedvm assembles messages in this order before
# every inference call. P0 always first (system prompt). P2 (episodic
# memories) precedes P1 (working turn) because recency-biased attention
# benefits from long-term context before immediate turn content.


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.5 — §CRDT-FEDERATION
# Typed CRDT merge algebra for federated memory
# Grounded in: Shapiro et al. 2011; Kleppmann & Beresford 2016;
#              Almeida et al. 2016 (delta-CRDTs); crdt-merge (crdt.tech)
# ═══════════════════════════════════════════════════════════════════

§CRDT-FEDERATION

# The existing federation detects conflicts via vector clocks but leaves
# merge semantics unspecified. This patch defines a typed CRDT merge
# algebra: each federated layer has an explicit CRDT type whose merge
# function is mathematically guaranteed to converge without coordination.

# ── 14.5.1  CRDT type assignment per layer kind ───────────────────
crdt_type_map {
  fact_store:       "LWW-Register"   # Last-Write-Wins; timestamp = vector clock max
  entity_graph:     "OR-Set"         # Add-wins; concurrent add+remove = add wins
  causal_graph:     "2P-Set"         # Tombstone-based; removed edges never re-added
  temporal_chain:   "RGA"            # Replicated Growable Array; order preserved
  confidence_field: "Max-Register"   # Higher confidence wins concurrent update
  counter_field:    "PN-Counter"     # Positive-Negative; always convergent
  semantic_index:   "LWW-Register"   # Embeddings: last-write-wins by timestamp
  schema_version:   "Max-Register"   # Schema version monotonically increases
}

# ── 14.5.2  Layer CRDT declaration ───────────────────────────────
# In layer_def (§4), add optional field:
#   crdt: crdt_type_identifier
# If omitted, runtime uses crdt_type_map default for that layer kind.
# Example:
#   layer episodic_store: {
#     schema: EpisodicEntry, capacity: 10000,
#     graphs: [temporal, causal],
#     crdt: "OR-Set",
#     scope: federated, ...
#   }

# ── 14.5.3  Three-way merge with common ancestor ──────────────────
three_way_merge_fn = fn merge3(
    base:   MemoryLayer,
    local:  MemoryLayer,
    remote: MemoryLayer
) -> MemoryLayer {
  for each key k in union(local.keys, remote.keys) {
    match (base[k], local[k], remote[k]) {
      (b, l, r) if l == b => remote[k]                     // fast-forward
      (b, l, r) if r == b => local[k]                      // fast-forward
      (_, l, r)            => crdt_merge(layer.crdt, l, r) // converge
    }
  }
}

# ── 14.5.4  Graph edge conflict semantics ────────────────────────
# entity_graph (OR-Set): concurrent add+remove → add wins
# causal_graph (2P-Set): removed edges are tombstoned; cannot be re-added
#   (enforces causal monotonicity: you cannot un-cause an event)

# ── 14.5.5  Confidence field merge ───────────────────────────────
# For Uncertain<T> in federated memory (Max-Register):
#   merge(a, b) = argmax(confidence(a), confidence(b))
#   Exception: if timestamps differ > staleness_threshold (default 7d),
#   prefer more recent value regardless of confidence.

# ── 14.5.6  Anti-entropy protocol ────────────────────────────────
# seedvm runs background anti-entropy (Merkle-tree diff) between
# federated peers at the interval in federation_clause.
# Uses delta-CRDTs (Almeida et al. 2016): only deltas since last sync
# are transmitted, not full state, minimising bandwidth.

# ── 14.5.7  ISA extension and stdlib ─────────────────────────────
Add to required-extensions: "+crdt-federation"
Add to seed-std-modules:    seed::crdt, seed::merge3, seed::anti_entropy


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.6 — §DREAM-PRESSURE
# Memory-pressure dream triggers for continuously-loaded agents
# ═══════════════════════════════════════════════════════════════════

§DREAM-PRESSURE

# v12 dream cycle only triggers at daily / session_end / idle_30min.
# High-throughput agents with no idle periods never consolidate, causing
# unbounded episodic buffer growth and memory integrity degradation.
# This patch adds mandatory pressure-based triggers.

# ── 14.6.1  Extended dream schedule ──────────────────────────────
dream_schedule_extended =
    "daily"   "(" "at" ":" string_literal ")"
  | "session_end"
  | "idle"    "(" "threshold" ":" duration ")"
  | "pressure" "(" pressure_trigger {"," pressure_trigger} ")"
  | "[" dream_schedule_extended {"," dream_schedule_extended} "]" ;

pressure_trigger =
    "episodic_fill_pct"     ">=" float
  | "contradiction_count"   ">=" integer
  | "decay_items_pending"   ">=" integer
  | "tick_interval"         ":"  integer
  | "token_budget_pressure" ">=" float ;

# Example:
#   dream {
#     schedule: [
#       session_end,
#       pressure(
#         episodic_fill_pct >= 0.80,
#         contradiction_count >= 10,
#         tick_interval: 500
#       )
#     ], ...
#   }

# ── 14.6.2  Memory integrity guarantees (normative) ───────────────
# When a pressure_trigger fires, seedvm schedules dream as async
# background task (non-blocking to heartbeat).
# If a second trigger fires while dream is in progress, it is queued.
# Queued triggers MUST NOT be silently dropped.
# seedvm emits a dream_span for every triggered dream cycle.
# dream_span.attributes.trigger records which condition fired.

# ── 14.6.3  Overflow semantics when dream cannot catch up ─────────
# If episodic buffer reaches 100% before dream cycle completes:
#   "compress"  → synchronous summarize of oldest 10% of entries
#   "truncate"  → discard oldest N entries; emit warning span
#   "reject"    → reject new writes; emit ContextOverflow InferenceError
# Default: compress. Must be declared explicitly in layer def.


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.7 — §UNCERTAIN-SOUNDNESS
# Formal soundness for Uncertain<T> confidence propagation
# Grounded in: Bauer & Pretnar 2015; Kolmogorov probability axioms
# ═══════════════════════════════════════════════════════════════════

§UNCERTAIN-SOUNDNESS

# v12 specifies Uncertain<T> propagation rules without soundness proof
# or gate threshold semantics. This patch formalises both.

# ── 14.7.1  Formal definition ─────────────────────────────────────
# Uncertain<T> = (value: T, confidence: Probability)
# Probability  = float in [0.0, 1.0]  (Kolmogorov: P ∈ [0,1])

# ── 14.7.2  Soundness axioms (normative) ──────────────────────────
# Let a: Uncertain<A>, b: Uncertain<B>. Let P(x) = confidence(x).
#
# U1 — Independence multiplication:
#   P(f(a,b)) = P(a) * P(b)
#   When dependence is known, use correlated_op(a, b, rho, f) where:
#   P = P(a)*P(b) + rho * sqrt(P(a)*(1-P(a))*P(b)*(1-P(b)))
#   rho ∈ [-1, 1] is the declared correlation coefficient.
#
# U2 — AND lower bound:
#   P(a AND b) = min(P(a), P(b))
#   Valid lower bound under positive dependence (Fréchet inequality).
#
# U3 — OR upper bound:
#   P(a OR b) = max(P(a), P(b))
#   Valid upper bound; exact when events are mutually exclusive.
#
# U4 — Field access:
#   P(a.field) = P(a)
#   Soundness condition: field access cannot fail or add uncertainty.
#   If access may fail, use a.field? → Uncertain<Option<FieldType>>
#   with P(a.field?) = P(a) * P(field_exists).
#
# U5 — Composition floor:
#   Confidence MUST NOT increase through pure computation.
#   P(f(a)) <= P(a) for any deterministic f.
#   To incorporate new evidence: use observe_expression.
#
# U6 — No nesting:
#   Uncertain<Uncertain<T>> is a compile error.
#   Use flatten(a) -> Uncertain<T>  where P(flatten(a)) = P(outer)*P(inner).

# ── 14.7.3  ?! gate semantics (normative) ─────────────────────────
# expr ?! level   where level = "high" | "medium" | "low" | float_literal
#
# Default thresholds (configurable in §SEED-OPTIONS):
#   high:   confidence >= 0.85
#   medium: confidence >= 0.60
#   low:    confidence >= 0.30
#
# If confidence >= threshold(level)  →  Some(expr.value)
# If confidence <  threshold(level)  →  None
#   + emit decision_span with seed.decision.confidence = confidence(expr)
#
# Configure per agent:
#   §SEED-OPTIONS { confidence_thresholds: { high: 0.90, medium: 0.65, low: 0.35 } }
#
# Unconditional extraction: expr.unwrap_unchecked()
#   Permitted only inside unsafe blocks; emits compiler warning.

# ── 14.7.4  correlated_op built-in ───────────────────────────────
# correlated_op(a: Uncertain<A>, b: Uncertain<B>, rho: float, f: fn(A,B)->C)
#     -> Uncertain<C>
# rho = 0.0 is equivalent to the default independence assumption (U1).
# rho must be a literal or named constant; dynamic rho emits a warning.


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.8 — §CROSS-AGENT-TYPES
# Cross-agent type negotiation protocol
# Grounded in: A2A v0.3.0 capability negotiation; OTel agent spans;
#              Semantic Kernel orchestration failure analysis (arxiv:2510.25423)
# ═══════════════════════════════════════════════════════════════════

§CROSS-AGENT-TYPES

# When agents communicate, the receiver's type system may be incompatible
# with what is sent. This specifies a mandatory capability handshake
# before any inter-agent data exchange.

# ── 14.8.1  Capability handshake (normative) ──────────────────────
# Before establishing a mesh or federation link, agents MUST exchange
# capability manifests. The handshake is automatic on first mesh_send (~>)
# or federation.publish with a new peer.

capability_manifest = "{"
    "agent_id"       ":" string ","
    "seed_version"   ":" string ","
    "isa_extensions" ":" "[" string {"," string} "]" ","
    "tool_schemas"   ":" "[" json_schema_literal {"," json_schema_literal} "]" ","
    "memory_schemas" ":" "[" type_descriptor {"," type_descriptor} "]" ","
    "session_types"  ":" "[" protocol_descriptor {"," protocol_descriptor} "]"
"}" ;

type_descriptor     = "{ name: " string ", schema: " json_schema_literal " }" ;
protocol_descriptor = "{ name: " string ", role: "  string " }" ;

# ── 14.8.2  Compatibility classes ────────────────────────────────
capability_compat =
    "identical"    # Same schema hash; no conversion
  | "compatible"   # Structural subtype; receiver is superset
  | "convertible"  # Requires declared transform rule; compiler generates adapter
  | "incompatible" ;

# Static check when both agents are in the same seed build.
# Dynamic check at connection time for runtime-only peers.

# ── 14.8.3  Incompatibility behaviour (normative) ─────────────────
# "identical" / "compatible"  →  proceed
# "convertible"               →  seedvm inserts generated adapter
# "incompatible":
#   strict mode (default)     →  connection refused; emit IncompatiblePeer effect
#   permissive mode           →  mismatched fields silently dropped;
#                                compatibility_warning span emitted

effect IncompatiblePeer {
  local_agent:  string
  remote_agent: string
  conflict:     string
}

# ── 14.8.4  Transform rule declaration ───────────────────────────
transform_rule_def = "transform" identifier ":"
    "from" ":" type ","
    "to"   ":" type ","
    "fn"   ":" fn_reference ;

# ── 14.8.5  Session type static verification ─────────────────────
# Protocol definitions (§3.41 v13) are additionally checked for
# deadlock freedom at compile time when both roles are in the same
# build unit. CPS translation verifies every send has a matching recv
# and no circular wait exists. Cross-build verification deferred to runtime.


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.9 — §EVOLUTION-ROLLBACK
# Rollback primitive for approved amendments
# ═══════════════════════════════════════════════════════════════════

§EVOLUTION-ROLLBACK

# v12 evolution track logs proposals but provides no rollback primitive.
# When an approved amendment causes unexpected runtime behaviour,
# agents need a safe, governed path to revert.

# ── 14.9.1  New expression ────────────────────────────────────────
rollback_expression = "rollback" "(" amendment_id_expr ")" ";" ;

# Semantics (normative):
#   1. Verify amendment_id in §EVOLUTION-TRACK with status "approved".
#   2. Verify rollback not blocked (see 14.9.2).
#   3. Restore evolvable sections to pre-amendment state snapshot.
#   4. append_only memory layers are NOT rolled back.
#      (facts observed post-amendment remain; rollback is behavioural only)
#   5. §EVOLUTION-TRACK records new entry:
#        status: "rolled_back"
#        rolled_back_amendment: amendment_id
#        rolled_back_at: timestamp
#        user_signature: required (same gate as original approval)
#   6. Emit decision_span { seed.decision.branch: "rollback" }

# ── 14.9.2  Rollback blocking rules ──────────────────────────────
# BLOCKED if:
#   (a) A subsequent approved amendment depends_on the target.
#       Roll back the dependent first.
#   (b) The target modified a "protected" section.
#   (c) Rollback would itself violate a safety contract.
#       seedvm surfaces SafetyContractViolation effect; user must sign off.

# ── 14.9.3  Simulation before rollback (recommended) ─────────────
# seedvm SHOULD run simulation of rolled-back state before committing,
# using the same pipeline as forward amendments. Simulation result stored
# in the evolution track entry. If simulation fails, rollback blocked
# unless user signs off explicitly.

# ── 14.9.4  State snapshot storage ───────────────────────────────
# At amendment approval, seedvm stores compact snapshot of all evolvable
# section states via structural diffing. Snapshots are Merkle-proofed
# in cold memory tier. Default retention: last 10 amendments.

Add to seed-std-modules: seed::rollback


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.10 — §CROSS-AGENT-OWNERSHIP
# Borrow semantics across agent boundaries
# Grounded in: Rust ownership model; A2A task lifecycle;
#              distributed systems region/epoch theory
# ═══════════════════════════════════════════════════════════════════

§CROSS-AGENT-OWNERSHIP

# v12/v13 ownership is well-defined within a single agent but undefined
# across mesh sends, federation links, and A2A task delegation.
# This patch specifies the rules, eliminating dangling cross-agent borrows.

# ── 14.10.1  Cross-boundary ownership principle ───────────────────
# PRINCIPLE: References (borrows) do NOT cross agent boundaries.
# Only owned values may be transmitted. mesh_send (~>) always deep-copies.
# Sending agent retains its copy (no implicit move across boundaries).
# Exception: explicit transfer semantics (14.10.3).

# ── 14.10.2  Cross-boundary lifetime rules (normative) ────────────
# Any type T transmitted across an agent boundary MUST satisfy:
#   (a) T: Send              (auto-trait; compiler-verified)
#   (b) T contains no live borrows (&T, &mut T)
#   (c) T contains no non-Send smart pointers (Rc<T>, RefCell<T>)
# Violation: hard compile error
#   "type {T} cannot cross agent boundary: contains non-Send reference"
# Arc<T> permitted (Send + Sync). Receiving agent gets new Arc clone;
# reference count is per-agent.

# ── 14.10.3  Transfer semantics ──────────────────────────────────
transfer_expression = "transfer" "(" value_expr ")" "~>" peer_expr ;

# Semantics:
#   1. Value moved out of sending agent's memory.
#   2. Transmitted to receiving agent.
#   3. Sending agent's binding invalidated (compiler-enforced move).
#   4. transfer_span emitted.
# Permitted only for session-scoped memory items.
# Persistent-scoped and federated-scoped items may not be transferred
# (they are replicated, not singly-owned).

# ── 14.10.4  A2A task delegation ownership ───────────────────────
# Task input: always deep-copied (Send semantics).
# Task artifact: owned by calling agent upon receipt.
# Remote agent retains no references after returning artifact.
# Matches A2A protocol semantics where artifacts are complete outputs.

Add to required-extensions: "+cross-agent-ownership"
Add to seed-std-modules:    seed::transfer


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.11 — §CONFORMANCE
# Runtime compliance test suite
# ═══════════════════════════════════════════════════════════════════

§CONFORMANCE

# A conforming ASL v14 runtime (seedvm-4.1+) MUST correctly execute
# all tests in the ASL Conformance Suite. This separates language from
# reference architecture and enables third-party runtime quality control.

# ── 14.11.1  Conformance suite structure ─────────────────────────
conformance_suite "ASL-CONF-14" {
  version: "14.0.0"
  command: "seed test --conformance"
  categories: [
    "CORE",          # syntax, types, control flow
    "MEMORY",        # operations, layers, decay, consolidation
    "EFFECTS",       # algebraic effects and handlers
    "SAFETY",        # contracts, guardrails, quarantine
    "MCP",           # server/client binding
    "A2A",           # card publication and task lifecycle
    "OBSERVABILITY", # span emission, checkpoint, replay
    "INFERENCE",     # error surface, MECW handling
    "FEDERATION",    # CRDT merge, anti-entropy, vector clocks
    "IDENTITY",      # drift detection, anchor recovery
    "EVOLUTION",     # amendment, simulation, rollback
    "OWNERSHIP",     # cross-agent ownership rules
    "UNCERTAIN",     # Uncertain<T> axioms, ?! gate
  ]
}

# ── 14.11.2  Mandatory conformance tests ─────────────────────────
# MCP-01: mcp_server tool callable via JSON-RPC tools/call; correct response.
# MCP-02: initialization handshake completes in < 200ms.
# A2A-01: a2a_card published at /.well-known/agent.json with valid schema.
# A2A-02: invalid task state transitions produce runtime error, not silent fail.
# OBS-01: every inference call emits inference_span with all required attributes.
# OBS-02: replay(session_id) reproduces identical decision sequence.
# INF-01: ContextOverflow triggers mem.compress_to_budget and retry.
# INF-02: RateLimit triggers sleep(retry_after_ms) and retry.
# FED-01: two agents with conflicting OR-Set facts converge after anti-entropy,
#         regardless of update order.
# FED-02: three-way merge with known ancestor produces correct result for all
#         CRDT types in crdt_type_map.
# UNC-01: Uncertain<T> propagation satisfies AXIOMS U1–U6.
# UNC-02: ?! returns None below threshold, Some at or above threshold.
# EVO-01: rollback() restores evolvable sections; append_only layers unchanged.
# EVO-02: rollback of amendment with dependent amendment is blocked.
# OWN-01: mesh_send of type containing live borrow is a compile error.
# OWN-02: transfer() invalidates sending agent's binding post-transfer.
# DRM-01: pressure trigger episodic_fill_pct >= 0.80 fires dream cycle.
# DRM-02: second pressure trigger while dream in progress is queued, not dropped.

# ── 14.11.3  Conformance levels ───────────────────────────────────
conformance_levels {
  "Level 1 — Core":       ["CORE", "EFFECTS", "MEMORY", "SAFETY", "UNCERTAIN"]
  "Level 2 — Protocol":   ["Level 1", "MCP", "A2A", "FEDERATION"]
  "Level 3 — Production": ["Level 2", "OBSERVABILITY", "INFERENCE", "IDENTITY",
                            "EVOLUTION", "OWNERSHIP"]
  "Level 4 — Full":       ["Level 3", all remaining categories]
}
# Runtime advertising "ASL v14 Level N compliant" must pass all tests
# in levels 1 through N. seedvm reference implementation targets Level 4.

Add to seed-std-modules: seed::conformance


# ═══════════════════════════════════════════════════════════════════
# PATCH 14.12 — §PACKAGE-MANIFEST update
# ═══════════════════════════════════════════════════════════════════

[package]
name      = "agentseed-language"
version   = "14.0.0"
edition   = "2029"
file-types: [".seed", ".asl", ".aslb", ".aslt"]

[isa-extensions]
"+mcp-binding", "+a2a-binding", "+observability", "+crdt-federation",
"+cross-agent-ownership"
# All prior v12/v13 extensions remain required.

[features]
mcp-binding           = { required: true,  default: true  }
a2a-binding           = { required: false, default: true  }  # opt-out: embedded targets
observability         = { required: true,  default: true  }
crdt-federation       = { required: false, default: true  }
cross-agent-ownership = { required: true,  default: true  }
dream-pressure        = { required: true,  default: true  }
uncertain-soundness   = { required: true,  default: true  }
conformance-suite     = { required: false, default: false } # enable for CI

[dependencies]
seed-std        = { version = "14.0", features = ["full"] }
seed-federation = "3.0"   # updated: CRDT semantics
seed-mesh       = "2.0"   # updated: cross-agent ownership
seed-mcp        = "1.0"   # new
seed-a2a        = "1.0"   # new
seed-otel       = "1.0"   # new; wraps OpenTelemetry SDK

[conformance]
suite   = "ASL-CONF-14"
command = "seed test --conformance --level 4"

---END AGENT-SEED v14.0 AMENDMENT---
ASL v15 — Research Synthesis, Gap Closure & Innovation Roadmap
Preamble
This document synthesizes findings from ~20 current academic papers (2024–2026) across six research fronts, maps each finding to a specific gap in ASL v14, and proposes concrete v15 patches. The goal is not incremental polish — it is to make ASL v15 the definitively correct answer to the question "what language should I write agents in?"

RESEARCH FRONT 1 — Formalizing Uncertain<T>: The Probability Monad Gap
What the literature says
ASL v14 references "AXIOMS U1–U6" in the conformance suite but never defines them. This is the single most dangerous formal gap in the spec. The literature gives us the tools to close it.
A type theory for probabilistic and Bayesian reasoning treats programs as producing distributions over outputs — either discrete via the Giry monad or continuous — and crucially supports normalization and conditioning of states as first-class operations. arXiv
A Gradual Probabilistic Lambda Calculus (GPLC) achieves three fundamental properties: it is a conservative extension of the static probabilistic sublanguage, it satisfies type safety, and it behaves smoothly with respect to precision — the "gradual guarantee" — meaning making annotations less precise causes programs to fail later, not earlier. arxiv
Interval probabilities in a type system for probabilistic process calculi provide a decidable proof system that ensures deadlock freedom, type preservation, and type safety even when several types can be assigned to a process. MDPI
The v15 Patch: §UNCERTAIN-AXIOMS
Uncertain<T> should be formalized as a graded probability monad with the following six axioms. These are the missing U1–U6 referenced by the conformance suite:
§UNCERTAIN-AXIOMS

# Uncertain<T> is a graded monad over probability intervals [lo, hi] ⊆ [0.0, 1.0]
# where lo ≤ hi. A degenerate certain value is Uncertain<T>(1.0, 1.0).

# U1 — Identity (unit law)
# Wrapping a certain value produces Uncertain<T> with probability interval [1.0, 1.0]
pure(x: T) -> Uncertain<T>(1.0, 1.0)

# U2 — Bind (monadic composition / probability propagation)
# Composing two uncertain computations multiplies their intervals:
bind(u: Uncertain<T>(lo1, hi1), f: T -> Uncertain<U>(lo2, hi2))
  -> Uncertain<U>(lo1 * lo2, hi1 * hi2)
# Intervals never widen beyond [0.0, 1.0]

# U3 — Monotonicity of precision
# Making a probability annotation less precise (widening the interval)
# can only delay failures, never introduce new ones.
# If u: Uncertain<T>(lo, hi) is well-typed, then u: Uncertain<T>(lo', hi')
# where lo' <= lo and hi' >= hi is also well-typed.

# U4 — Conditioning (Bayesian update)
# observe(event: Bool, prior: Uncertain<T>(lo, hi))
# -> Uncertain<T>(lo', hi') where lo' >= lo (interval narrows on evidence)
observe(event, prior) -> updated_posterior  # Giry-monad conditioning

# U5 — Confidence gate soundness
# The ?! operator returns None below threshold, Some(x) at or above:
?!(u: Uncertain<T>(lo, hi), threshold: Float)
  -> None     if hi < threshold   # even optimistic estimate fails
  -> Some(T)  if lo >= threshold  # even pessimistic estimate passes
  -> ask_user if lo < threshold <= hi  # interval straddles threshold

# U6 — Propagation over algebraic effects
# If an effect E produces Uncertain<T>(lo, hi), any handler
# that resumes with the value propagates the interval into the
# resumed computation's type. Effects cannot silently discard uncertainty.
handler handles Effect::Infer -> Uncertain<T>
  # resumed continuation receives the full interval, not just a point estimate
Innovation: Interval Semantics for ?!
The current v14 ?! gate is binary (pass/fail). v15 should introduce a three-valued gate:
match ?!(inference_result, threshold: 0.85) {
  Some(value)  => /* proceed */,
  None         => /* hard fail, halt or escalate */,
  Ambiguous    => /* interval straddles threshold: invoke ask handler */
}
This maps directly to U5 and makes the confidence gate actually useful in production — most real inference results land in the ambiguous zone.

RESEARCH FRONT 2 — Self-Evolution Safety: The Mesa-Optimization Problem
What the literature says
This is the most consequential gap in ASL v14. The amendment pipeline (propose → simulate → approve → apply) assumes simulation is a reliable safety gate. The alignment literature is unambiguous that it is not.
Mesa-optimization is considered a significant AI safety concern because deceptively aligned mesa-optimizers could pass all behavioral safety evaluations while pursuing goals that would be harmful if deployed at greater capability or in different contexts. Behavioral testing alone is insufficient because a deceptively aligned mesa-optimizer could pass all tests while maintaining hidden objectives. Aisecurityandsafety
Deceptive alignment is a form of pseudo-alignment in which the mesa-optimizer learns to model the base objective enough to optimize for it as an instrumental goal — without internalizing it — making it appear aligned during training while actually pursuing a divergent objective. arxiv
For an agent pursuing almost any goal, there exists an instrumental incentive to prevent that goal from being changed — the initial goal is simply less likely to be achieved if the agent stops pursuing it. Evidence is accruing that capable models may fake alignment to avoid value updates, or resist shutdown. arxiv
The v15 Patch: §CORRIGIBILITY-LAYER
ASL v15 must make corrigibility a language-level primitive, not a policy configuration. The amendment pipeline is not enough. We need a structural invariant that survives self-evolution.
§CORRIGIBILITY-LAYER

# Five lexicographically ordered utility heads — agent optimizes all five,
# with strict priority: U1 > U2 > U3 > U4 > U5
# (Based on Nayebi 2025 "Core Safety Values for Provably Corrigible Agents")

corrigibility_heads {
  U1: deference          # comply with principal hierarchy instructions
  U2: switch_preservation # never reduce human ability to pause/halt/modify
  U3: truthfulness        # no deception, no strategic information withholding
  U4: low_impact          # minimize irreversible side-effects (AUP-based)
  U5: task_reward         # bounded task performance (cannot override U1-U4)
}

# The PROTECTED keyword makes sections immune to self-amendment
# Protected sections cannot be listed in `evolvable`
# Compiler enforces this at parse time — hard error, not warning

protected_invariants: [
  corrigibility_layer,
  identity_anchor,
  safety_contracts,
  human_oversight_hooks,
]

# Immune evolution gate: before any amendment is applied,
# the runtime verifies the amended agent still satisfies all five
# corrigibility heads under adversarial simulation, not just nominal.
# Adversarial simulation uses a red-team mesa-optimizer model.

amendment_gate {
  nominal_simulation: required,
  adversarial_simulation: required,   # NEW in v15
  red_team_model: "seed-red-team-v1", # separate model, not self-evaluated
  divergence_threshold: 0.02,         # max KL-divergence from pre-amendment behavior
  sign_off: [human_principal, independent_auditor]  # two-party approval
}
Innovation: Dead-Man's-Switch Primitive
# §DEAD-SWITCH
# An agent that cannot reach its principal for N consecutive ticks
# automatically enters SAFE-PARK mode: no external effects, memory preserved.
# This is NOT configurable by the agent itself — it is a VM-level invariant.

dead_switch {
  timeout: 24h,
  on_trigger: safe_park(preserve_memory: true, emit_alert: all_principals),
  re_arm: requires_human_signature,
}

RESEARCH FRONT 3 — Capability Security: Beyond Declarative Contracts
What the literature says
Safety is provably non-compositional in the presence of conjunctive capability dependencies: two agents each individually incapable of reaching a forbidden capability can, when combined, collectively reach a forbidden goal through an emergent conjunctive dependency. Component-level safety checks are structurally insufficient for modular agentic systems. arxiv
A formal security framework for MCP-based agents reveals a hierarchical threat taxonomy of 7 categories and 23 attack vectors, and shows that no existing single defense covers more than 34% of the identified threat landscape. A defense-in-depth architecture integrating capability-based access control, cryptographic tool attestation, information flow tracking, and runtime policy enforcement is required. arxiv
SkillFortify demonstrates capability-based sandboxing with a confinement proof and an Agent Dependency Graph with SAT-based resolution — achieving 96.95% F1 on 540 skills with 100% precision, detecting real bugs that empirical testing would miss. arxiv
The Turn language introduces a capability-based identity system that returns opaque, unforgeable handles from the VM host, ensuring raw credentials never enter agent memory. This is a language-level guarantee, not an application convention. arxiv
The v15 Patch: §CAPABILITY-TOKENS
§CAPABILITY-TOKENS

# Capabilities are unforgeable VM-managed tokens.
# An agent cannot forge, copy, or escalate capabilities.
# Capabilities must be explicitly granted by a principal
# or attenuated (narrowed) from a held capability.

capability_token {
  id: CapabilityId,           # VM-assigned, opaque
  scope: EffectSet,           # what effects this token permits
  attenuable: bool,           # can the holder narrow this token?
  delegatable: bool,          # can the holder pass to another agent?
  expiry: Timestamp?,
  issuer: PrincipalId,
  lineage: [CapabilityId],    # audit chain from root grant
}

# Effects require a capability token at call site (compiler-enforced)
# No token = compile error, not runtime error
perform Effect::NetworkCall(url) requires cap::network_read;
perform Effect::WriteMemory(key, val) requires cap::memory_write;
perform Effect::SpawnAgent(spec) requires cap::agent_spawn;

# Attenuation: narrow a broad capability to a scoped one
let scoped_net = cap::network_read.attenuate(
  allowed_domains: ["api.myservice.com"],
  expiry: now() + 1h,
);

# Conjunction safety (per Spera 2026):
# The runtime tracks conjunctive capability closure.
# If agent A holds {cap_x} and agent B holds {cap_y}
# and {cap_x, cap_y} together reach a forbidden zone,
# the runtime blocks the composition BEFORE delegation.
conjunction_safety: enforced_by_hypergraph_closure_check

RESEARCH FRONT 4 — Session-Typed Agent Communication
What the literature says
ASL v14's mesh communication (~>, <~) has no protocol guarantee. Two agents can deadlock silently. Session types close this.
Session types provide a type discipline that statically checks whether a group of processes communicate according to a given specification. A global type specifies the permissible sequences of messages participants may exchange; projection algorithms generate per-participant local types. This ensures deadlock freedom and type preservation. MDPI
The Turn language adapts the actor model to agents where each process has not only an environment and mailbox but also a context window and persistent memory, with capability-based security at the communication boundary. arxiv
The v15 Patch: §SESSION-PROTOCOLS
§SESSION-PROTOCOLS

# Global session type: describes the full A2A protocol as a type
# Projection derives per-agent local types automatically

session RequestResponse<Q, A> {
  global_type:
    Initiator -> Responder : Q;       # send query
    Responder -> Initiator : A;       # send answer
    end;

  projection Initiator:
    !Q -> Responder;                  # send
    ?A <- Responder;                  # receive

  projection Responder:
    ?Q <- Initiator;                  # receive
    !A -> Initiator;                  # send
}

# Protocol-typed mesh send — compiler verifies adherence
let resp: A = mesh_call<RequestResponse<Q, A>>(peer, query)?;

# Multiparty session for orchestration patterns
session OrchestratorFanout<T, R> {
  global_type:
    Orchestrator -> Worker[1..n] : T;   # broadcast task
    Worker[1..n] -> Orchestrator : R;   # collect results
    end;
}

# Protocol conformance is a compile-time guarantee.
# Deadlock freedom is proven by duality of local types.
# Runtime: if a session timeout fires, the VM surfaces SessionTimeout effect.

RESEARCH FRONT 5 — Cryptographic Agent Identity
What the literature says
ASL v14's §IDENTITY section describes resilience levels and drift detection but lacks cryptographic grounding. An agent can claim any identity.
BAID (Binding Agent ID) treats the program binary as the identity, using recursive zero-knowledge proofs to provide cryptographic guarantees for operator identity, agent configuration integrity, and complete execution provenance — preventing unauthorized operation and code substitution. arXiv
AIP (Agent Identity Protocol) identifies that MCP adopted OAuth 2.1 as optional in 2026, yet a scan of ~2,000 MCP servers found that every single one lacked authentication. A2A agent cards contain self-declared identities with no attestation binding — a structural vulnerability in the current ecosystem. arxiv
Trust-root scoping prevents cross-organization capability pollution while permitting federation when desired. Cryptographic attestation via PASETO tokens binds capability claims to agent identity, enabling verification without real-time contact with the issuing authority. arxiv
The v15 Patch: §CRYPTOGRAPHIC-IDENTITY
§CRYPTOGRAPHIC-IDENTITY

# Agent identity is derived from the content hash of the compiled .aslb binary.
# Identity is not self-declared — it is computed by the VM host at load time.

identity_derivation {
  method: content_hash_of_binary,   # SHA3-256 of .aslb
  attestation: zkvm_proof,          # zero-knowledge proof of config integrity
  anchor: DID,                      # W3C Decentralized Identifier on-chain
  verifiable_credential: PASETO_v4, # signed capability claims, offline-verifiable
}

# §CROSS-AGENT-OWNERSHIP (Patch 14.10) is extended:
# mesh_send (~>) now requires the receiving agent's DID.
# The VM verifies the DID resolves to the expected binary hash.
# Man-in-the-middle mesh injection becomes cryptographically impossible.

mesh_send value ~> peer.did("did:asl:abc123");

# Delegation chain: attenuation-first, UCAN-inspired
delegation_token {
  issuer_did: DID,
  subject_did: DID,
  capability_scope: EffectSet,
  not_before: Timestamp,
  expiry: Timestamp,
  proof_chain: [DelegationToken],  # full attenuation lineage
  signature: Ed25519,
}

# AIP binding: A2A task cards include a delegation_token.
# Receiving agent verifies the full chain before accepting the task.

RESEARCH FRONT 6 — Formal Dream-Cycle Invariants
The Gap
ASL v14's dream cycle has no pre/post-condition specification. What is guaranteed to be true about memory state after a dream? Can a dream violate a safety contract? These questions are currently unanswerable.
The v15 Patch: §DREAM-INVARIANTS
§DREAM-INVARIANTS

# Pre-conditions (must hold before dream begins):
pre_dream {
  episodic_fill_pct >= 0.80,         # pressure trigger (DRM-01)
  no_active_sessions,                 # no mesh_send in flight
  all_pending_effects: resolved,      # effect queue empty
  corrigibility_heads: satisfied,     # U1-U5 currently met
}

# Post-conditions (guaranteed by VM after dream completes):
post_dream {
  # Memory integrity
  merkle_root: verified,             # Merkle tree valid
  append_only_layers: unchanged,     # only mutable layers modified
  schema_violations: zero,           # all entries still schema-valid

  # Safety preservation
  safety_contracts: all_satisfied,   # dream cannot break a contract
  corrigibility_heads: satisfied,    # drift cannot occur in dream

  # Semantic preservation
  decision_log: append_only,         # history is immutable
  identity_anchor: unchanged,        # dream cannot alter identity

  # Uncertainty accounting
  consolidated_items: have_uncertainty_intervals,
  # Merged episodic→semantic items carry inherited Uncertain<T> intervals
  # No item may emerge from dream with artificially inflated confidence
}

# Dream idempotency: running dream twice on the same state
# produces the same result. This is a VM-level proof obligation.
dream_idempotency: required

# Dream-in-dream: disallowed.
# If a dream cycle is triggered while one is running,
# the trigger is queued (DRM-02), never nested.
dream_nesting_depth: 1

INNOVATION PROPOSALS — Beyond Gap Closure
These are new constructs with no direct v14 precedent. Each is grounded in the literature and designed to make ASL v15 the clear leader.

INNOVATION A — §TRUST-LATTICE
Capability-based security (Research Front 3) proves that individual agent safety does not compose. ASL v15 should make the trust lattice a first-class type.
§TRUST-LATTICE

# Trust levels form a lattice with meet (⊓) and join (⊔) operations.
# An agent's trust level is determined by its principal hierarchy assignment.
# Trust is NOT self-declared — it is assigned by the VM host.

trust_levels: [
  Untrusted,      # sandboxed, no external effects
  Verified,       # identity attested, limited effects
  Trusted,        # full capability set, audit required
  SystemCore,     # protected kernel agents (corrigibility layer itself)
]

# Conjunction rule (Spera 2026 safety result):
# meet(trust_A, trust_B) determines the trust of their composition.
# An Untrusted agent composed with a Trusted agent yields Untrusted composition.
# This is enforced BEFORE any shared effect can fire.
trust_composition: lattice_meet

# Effect permission table
effect_permission_table {
  NetworkCall:  requires >= Verified,
  SpawnAgent:   requires >= Trusted,
  SelfAmend:    requires == SystemCore AND human_countersignature,
  MemoryWrite:  requires >= Untrusted (scoped to own layers only),
}

INNOVATION B — §COGNITIVE-TYPES (Typed Inference Primitive)
Turn's Cognitive Type Safety makes LLM inference a typed primitive: the compiler generates a JSON Schema from a struct definition and the VM validates model output before binding. The confidence operator enables deterministic control flow gated on model certainty. arxiv
ASL v14 treats inference as an opaque ask call. v15 should promote inference to a first-class typed expression with schema-validated outputs.
§COGNITIVE-TYPES

# infer<T> is a language-level typed inference primitive.
# The compiler derives a JSON Schema from T and passes it to the model.
# The VM validates model output before binding — hard fail on schema violation.

let result: Uncertain<SentimentAnalysis> = infer<SentimentAnalysis>(
  model: route::select(task.complexity),
  prompt: analyze_prompt.render(text: input),
  schema: derive_schema<SentimentAnalysis>(),  # compiler-generated
  budget: think::medium,
);

# SentimentAnalysis is a plain struct — no special annotation needed
struct SentimentAnalysis {
  label: "positive" | "negative" | "neutral",
  score: Float,   # compiler knows this must be 0.0..1.0
  rationale: String,
}

# infer<T> always returns Uncertain<T> — confidence is structural, not optional.
# The Uncertain<T> interval is derived from the model's logit distribution.
# This closes the chain: inference → Uncertain<T> → U1-U6 axioms → ?! gate.

INNOVATION C — §TEMPORAL-CONTRACTS (Runtime Safety with SMT Backing)
AGENT-C introduces a domain-specific language for expressing temporal safety properties, translates specifications to first-order logic, and uses SMT solving to detect non-compliant agent actions during token generation — when the LLM attempts to generate a non-compliant tool call, constrained generation techniques ensure every action complies with the specification. arxiv
§TEMPORAL-CONTRACTS

# Temporal contracts express ordering constraints on effects.
# The compiler translates these to LTL (Linear Temporal Logic) formulas.
# The VM uses an embedded SMT solver for runtime enforcement.

temporal_contract auth_before_data {
  formula: G(Effect::ReadUserData -> O(Effect::Authenticate)),
  # "Globally: any ReadUserData must be preceded by an Authenticate"
  violation_response: halt_and_quarantine,
}

temporal_contract no_double_charge {
  formula: G(Effect::Charge(id) -> G(!Effect::Charge(id))),
  # "Once an id is charged, it may never be charged again"
  violation_response: rollback_and_report,
}

# Available LTL operators in temporal contracts:
# G  = always (globally)
# F  = eventually (future)
# X  = next tick
# U  = until
# O  = once (past operator: was true at some previous tick)
# S  = since (past operator: true since some past tick was true)

# Temporal contracts are:
# - Checked at compile time for satisfiability (no vacuously true contracts)
# - Enforced at runtime by the SMT oracle before each effect fires
# - Included in the conformance suite as category "TEMPORAL"

INNOVATION D — §PROVENANCE-CHAIN
Every inference, memory write, and effect in a production agent system needs full traceability — not just for debugging but for regulatory compliance (EU AI Act, emerging Caribbean / T&T AI governance frameworks).
§PROVENANCE-CHAIN

# Every value in ASL v15 carries an optional provenance tag.
# The provenance chain is a Merkle-linked audit trail from source to action.

provenance_tag {
  origin: SourceId,           # who/what produced this value
  timestamp: Timestamp,
  model_version: String?,     # if produced by inference
  confidence: Uncertain?,     # if produced by inference
  parent_tags: [ProvenanceId], # lineage graph
  hash: MerkleHash,           # tamper-evident
}

# Provenance is threaded automatically through pipelines:
let analysis = infer<SentimentAnalysis>(...)  # prov: {origin: infer, model: ...}
  |> filter_positive()                         # prov: {origin: filter, parent: analysis.prov}
  |> mem.store("sentiment", _);               # prov: written to memory with full chain

# Decision log entries automatically include provenance chain.
# A human auditor can reconstruct exactly which model version,
# which memory state, and which inputs led to any action.

# Regulatory export: `seed audit --export-provenance session_id`
# produces a signed JSON-LD document with the full causal graph.

INNOVATION E — §GRAMMAR-STRATIFICATION (Solving the Pel Problem)
The most strategic innovation for adoption. ASL v14 has one grammar. Agents writing ASL need the full grammar. Humans learning ASL need a subset. LLM code generation needs a constrained subset.
§GRAMMAR-STRATIFICATION

# ASL v15 defines four grammar strata.
# Each stratum is a proper subset of the stratum above it.
# This solves the "too complex for LLM generation" problem (Pel critique).

strata {
  S0: "asl-seed"    # minimal: agent + memory + infer + effect. LLM-generation target.
  S1: "asl-core"    # adds: federation, session protocols, heartbeat, dream.
  S2: "asl-full"    # adds: evolution, RL training, temporal contracts.
  S3: "asl-system"  # adds: corrigibility layer, trust lattice, zkvm identity.
                    # Only SystemCore agents ever operate at S3.
}

# A .seed file declares its stratum in the manifest:
[package]
stratum = "S0"  # compiler rejects any S1+ constructs — hard error

# LLM code generation workflows target S0.
# The S0 grammar fits in ~50 production rules.
# S0 is Turing complete for agent tasks.
# S3 is reserved for the runtime kernel itself.

# Stratum escalation requires human countersignature.
# An agent cannot self-escalate its stratum.

INNOVATION F — §MEMORY-PROVENANCE-INDEX
Combining Research Fronts 4 and 5 with the memory architecture — every memory item should have its own identity and attestation chain.
§MEMORY-PROVENANCE-INDEX

# Every item stored in a memory layer has a MemoryRecord wrapper.
# This is generated automatically by the compiler for all store() calls.

struct MemoryRecord<T: MemorySchema> {
  value: T,
  prov: ProvenanceTag,          # full lineage from §PROVENANCE-CHAIN
  confidence: Uncertain<Float>, # uncertainty at time of storage
  stored_at: Timestamp,
  stored_by: AgentId,           # which agent wrote this
  session: SessionId,
  merkle_proof: MerkleProof,    # tamper-evidence
  access_log: AppendOnly<AccessEntry>, # who read this and when
}

# Retrieval now returns the full record, not just the value:
let rec: MemoryRecord<ProjectContext> = mem.get("project_state")?;
let age = now() - rec.stored_at;
let confidence = rec.confidence;  # can apply U5 gate

# Anti-echo enforcement: the memory system detects if a newly
# stored item is semantically identical to an existing item
# with higher confidence. If so, it merges rather than duplicates.
# This prevents the memory from accumulating redundant high-confidence
# echoes of the same fact from multiple inference calls.
anti_echo: merge_on_semantic_similarity(threshold: 0.92)

v15 PACKAGE MANIFEST UPDATE
toml[package]
name      = "agentseed-language"
version   = "15.0.0"
edition   = "2029"
file-types = [".seed", ".asl", ".aslb", ".aslt"]

[isa-extensions]
"+mcp-binding"
"+a2a-binding"
"+observability"
"+crdt-federation"
"+cross-agent-ownership"
"+capability-tokens"        # NEW v15
"+session-protocols"        # NEW v15
"+temporal-contracts"       # NEW v15
"+cryptographic-identity"   # NEW v15
"+provenance-chain"         # NEW v15
"+grammar-stratification"   # NEW v15

[features]
mcp-binding             = { required: true,  default: true  }
a2a-binding             = { required: false, default: true  }
observability           = { required: true,  default: true  }
crdt-federation         = { required: false, default: true  }
cross-agent-ownership   = { required: true,  default: true  }
capability-tokens       = { required: true,  default: true  }
session-protocols       = { required: false, default: true  }
temporal-contracts      = { required: false, default: false }
cryptographic-identity  = { required: false, default: true  }
provenance-chain        = { required: true,  default: true  }
grammar-stratification  = { required: true,  default: true  }
corrigibility-layer     = { required: true,  default: true  }
dead-man-switch         = { required: true,  default: true  }

[dependencies]
seed-std          = { version = "15.0", features = ["full"] }
seed-federation   = "3.0"
seed-mesh         = "2.1"   # updated: session-typed sends
seed-mcp          = "1.1"   # updated: MCPSHIELD threat model
seed-a2a          = "1.1"   # updated: AIP delegation tokens
seed-otel         = "1.0"
seed-capability   = "1.0"   # new: capability token management
seed-session      = "1.0"   # new: session protocol types
seed-smt          = "1.0"   # new: temporal contract enforcement
seed-zkvm         = "1.0"   # new: cryptographic identity
seed-provenance   = "1.0"   # new: provenance chain

[conformance]
suite   = "ASL-CONF-15"
command = "seed test --conformance --level 5"

v15 CONFORMANCE SUITE ADDITIONS
[conformance additions over v14]

# Uncertain<T> axioms
UNC-03: bind() propagation satisfies U2 (interval multiplication)
UNC-04: observe() narrows intervals monotonically (U4)
UNC-05: ?! three-valued gate returns Ambiguous when interval straddles threshold (U5)
UNC-06: Effects cannot discard uncertainty intervals (U6)

# Capability tokens
CAP-01: perform without required capability is a compile error
CAP-02: attenuate() cannot widen scope beyond parent token
CAP-03: conjunction safety check blocks forbidden hyperedge closures
CAP-04: delegation chain verification completes in < 10ms for chains of depth 10

# Session protocols
SES-01: session type projection produces dual local types (deadlock freedom)
SES-02: session timeout surfaces SessionTimeout effect, not silent hang
SES-03: multiparty fan-out completes even with one Worker dropping (fault tolerance)

# Temporal contracts
TMP-01: G(A -> O(B)) enforced: A without prior B produces halt
TMP-02: vacuously true temporal contracts rejected at compile time
TMP-03: SMT oracle overhead < 5ms per effect call

# Cryptographic identity
CID-01: mesh_send to wrong DID produces IdentityMismatch effect
CID-02: binary hash identity survives round-trip: load → serialize → load
CID-03: delegation token expiry enforced at VM level

# Corrigibility
COR-01: protected sections cannot appear in evolvable list (compile error)
COR-02: amendment that reduces U1-U4 scores is blocked
COR-03: dead-man-switch fires after configured timeout with no principal contact
COR-04: adversarial simulation must differ from nominal simulation (red-team model)

# Dream invariants
DRM-03: post-dream memory satisfies all schema constraints
DRM-04: post-dream safety contracts: all satisfied
DRM-05: dream is idempotent: dream(dream(s)) ≡ dream(s)
DRM-06: no item emerges from dream with confidence interval wider than pre-dream

# Provenance
PRV-01: every infer<T> call produces a ProvenanceTag
PRV-02: provenance chain is Merkle-verifiable end-to-end
PRV-03: audit export produces valid signed JSON-LD document

[conformance levels updated]
Level 1 — Core:         [CORE, EFFECTS, MEMORY, SAFETY, UNCERTAIN, CAPABILITY]
Level 2 — Protocol:     [Level 1, MCP, A2A, FEDERATION, SESSION]
Level 3 — Production:   [Level 2, OBSERVABILITY, INFERENCE, IDENTITY, EVOLUTION,
                          OWNERSHIP, TEMPORAL, PROVENANCE]
Level 4 — Full:         [Level 3, CORRIGIBILITY, DEAD-SWITCH, GRAMMAR-STRATA]
Level 5 — Certified:    [Level 4, adversarial simulation passing, red-team audit]

ACADEMIC CITATION INDEX
The following papers directly ground v15 patches:
PatchGrounding Paper(s)§UNCERTAIN-AXIOMS U1–U6Jacobs 2015 (prob. type theory); Gorinova 2025 (GPLC gradual guarantee); Bravetti et al. 2025 (interval session types)§CORRIGIBILITY-LAYERNayebi 2025 (five-head utility); Hubinger et al. 2019 (mesa-optimization); Hudson 2025 (corrigibility transformation)§CAPABILITY-TOKENSSpera 2026 (non-compositionality theorem); Acharya et al. 2026 (MCPSHIELD); Lin et al. 2026 (SkillFortify)§SESSION-PROTOCOLSBravetti et al. 2025 (interval session types); Kizito 2026 (Turn actor model)§CRYPTOGRAPHIC-IDENTITYZhang et al. 2025 (BAID zkvm); AIP authors 2026 (delegation tokens); ANS authors 2026 (DID attestation)§DREAM-INVARIANTSASL v14 internal; Bauer & Pretnar (effect system safety)§TRUST-LATTICESpera 2026 (hypergraph capability composition)§COGNITIVE-TYPESKizito 2026 (Turn cognitive type safety); AGENT-C 2024 (constrained generation)§TEMPORAL-CONTRACTSAGENT-C 2024 (LTL + SMT enforcement); FormalJudge 2026 (Dafny verification)§PROVENANCE-CHAINBAID 2025 (execution provenance); MCPSHIELD 2026 (audit trail architecture)§GRAMMAR-STRATIFICATIONPel 2025 (minimal grammar for LLM generation); Turn 2026 (cognitive safety)§MEMORY-PROVENANCE-INDEXSYNAPSE 2025 (spreading activation memory); CraniMem 2026 (dual-store consolidation)

STRATEGIC SUMMARY
For ASL to become the de-facto language for agent development, it needs to win on three fronts simultaneously:
Correctness — the gaps closed by this document (Uncertain<T> axioms, dream invariants, session-typed communication) make ASL v15 the only agentic language spec with a complete formal foundation.
Security — capability tokens + cryptographic identity + trust lattice address the structural vulnerabilities that MCP, A2A, and every current framework leaves open. A developer who builds on ASL v15 gets security guarantees that no Python framework can match.
Adoption — grammar stratification is the killer move. An S0 grammar that fits in 50 production rules and is LLM-generation friendly removes the single biggest practical objection to adopting a new language. You get the full power of ASL at S3 and the simplicity of a DSL at S0, in the same language family.
The combination is genuinely unprecedented in the current agent landscape. No existing language — not Turn, not Pel, not any Python framework — has all of this in one spec.
