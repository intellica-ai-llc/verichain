Atonomoous Systemms Language — Specification v0.1.0
ASL-SPEC-0.1.0 | Edition 2026 | Status: Draft for Review

@Autonomous Systems Lanuage/0.1.0  -- Author -- Damain Peter Ramsajan

╔══════════════════════════════════════════════════════════════════════╗
║ §META-SEED — Unified Agentic Language & Virtual ISA                ║
╚══════════════════════════════════════════════════════════════════════╝
§META-SEED
yamlseed-id:              uuid:2b3c4d5e-6f7a-8b9c-0d1e-f23456789abc
seed-type:            unified-agentic-language
edition:              2029
stability:            stable
formalism-level:      progressive
safety-level:         verified
composition-model:    category-theoretic
compliance:           ISO/IEC 5230:2029/Final
runtime:              seed-vm v5.0 (virtual ISA with extensions)

# Lifecycle features
heartbeat-enabled:         true
dream-cycle-enabled:       true
federation-enabled:        true
evolution-enabled:         true
corrigibility-enabled:     true    # NEW v0.1.0 — language-level invariant
capability-tokens-enabled: true    # NEW v0.1.0 — unforgeable VM-managed tokens
session-protocols-enabled: true    # NEW v0.1.0 — typed A2A communication
temporal-contracts-enabled: true   # NEW v0.1.0 — LTL + SMT enforcement
provenance-chain-enabled:   true   # NEW v0.1.0 — full audit lineage
grammar-strata-enabled:     true   # NEW v0.1.0 — S0/S1/S2/S3 stratification
cryptographic-identity:     true   # NEW v0.1.0 — zkVM binary-hash identity

§BUILD-CONFIGURATION
toml[build]
language-version: "15.0.0"
compiler:         "seedc 15.0.0"
target:           "seedvm-5.0"
jit-tier:         "copy-and-patch + extension lowering"
optimization:     "speed"
stratum:          "S1"          # default stratum; override per-package
Stratum Overview
ASL v0.1.0 introduces grammar stratification. Every compiled unit declares a stratum. The compiler rejects any construct above the declared stratum level at parse time.
StratumLabelIntended AudienceS0asl-seedLLM code generation, beginner authors, sandboxed agentsS1asl-coreProduction agents, standard multi-agent systemsS2asl-fullAdvanced agents with evolution and RL trainingS3asl-systemRuntime kernel, corrigibility layer, trusted orchestrators only
Stratum escalation requires a human principal countersignature recorded in the evolution track. An agent cannot self-escalate its stratum.

§DESIGN-PRINCIPLES
The following principles govern all design decisions in ASL v0.1.0. Where a new feature conflicts with a lower-numbered principle, the lower-numbered principle takes precedence.
P1 — Corrigibility is structural, not configurable.
Human oversight hooks, principal hierarchy deference, and shutdown access are enforced by the VM. No agent program can weaken or remove them.
P2 — Uncertainty is first-class.
Every value that originates from inference carries an Uncertain<T> type with a probability interval. Uncertainty cannot be silently discarded.
P3 — Capabilities are unforgeable.
Effects require capability tokens issued by the VM host. Tokens cannot be forged, copied beyond their delegation scope, or escalated by the holder.
P4 — Identity is derived, not declared.
An agent's identity is the content hash of its compiled binary, attested by a zkVM proof. Self-declared identity is not trusted.
P5 — Communication is protocol-typed.
Agent-to-agent messages conform to session types. Deadlock freedom is a compile-time guarantee, not a runtime hope.
P6 — All actions are auditable.
Every inference call, memory write, effect, and decision produces a provenance-tagged entry in an append-only, Merkle-proofed log.
P7 — Evolution is adversarially gated.
Self-amendment is permitted only after both nominal and adversarial simulation, independent audit, and two-party human approval.
P8 — Safety composes through the trust lattice.
Conjunctive capability closures are checked before any agent composition. Individual safety does not imply compositional safety.

╔══════════════════════════════════════════════════════════════════════╗
║ §1 — LEXICAL GRAMMAR — Complete Token Specification                ║
╚══════════════════════════════════════════════════════════════════════╝
§LEXICAL-GRAMMAR
unicode-version:  "16.0"
source-encoding:  "UTF-8"
file-extensions:  [".seed", ".asl", ".aslb", ".aslt"]

1.1 Whitespace and Comments
ebnfwhitespace        ::= [ \t\r\n]+
line-comment      ::= "//" [^\n]* "\n"
block-comment     ::= "/*" ([^*] | "*"+ [^*/])* "*"+ "/"
doc-comment       ::= "///" [^\n]* "\n"
inner-doc-comment ::= "//!" [^\n]* "\n"
Doc comments (///) attach to the immediately following item and are included in generated documentation. Inner doc comments (//!) attach to the enclosing item.

1.2 Identifiers
ebnfidentifier     ::= [a-zA-Z_] [a-zA-Z0-9_]*
raw-identifier ::= "r#" identifier
placeholder    ::= "_"
Raw identifiers permit keywords to be used as identifiers where necessary for interoperability with external schemas.

1.3 Keywords — Exhaustive Set
Keywords are reserved in all strata unless noted otherwise. Stratum availability is indicated in parentheses where applicable.
# S0 — Core control flow (available in all strata)
if, else, match, for, while, loop, break, continue, return

# S0 — Declarations
let, mut, fn, agent, section, trait, impl, enum, struct, type

# S0 — Async and concurrency
async, await, spawn, join, select

# S0 — Module system
mod, use, pub, extern, export

# S0 — Error handling
try, catch, throw, unsafe

# S0 — Ownership
move, borrow, ref, own

# S0 — Seeds and composition
seed, seedlet, inherit, compose, pipe, redirect

# S0 — Effects (algebraic)
effect, handler, perform, resume

# S0 — Probabilistic
prob, observe, infer

# S0 — Reactive
signal, react, memo

# S0 — Confidence and uncertainty
ask, confident, uncertain, interval

# S0 — Cognitive inference (NEW v0.1.0)
infer, schema, cognitive, derive_schema

# S1 — Memory system
memory, layer, graph, traverse, consolidate, decay, reinforce

# S1 — Neurosymbolic
ontology, rule, ground, validate

# S1 — Prompts
prompt, optimize, template, reflection

# S1 — Safety contracts
contract, guardrail, monitor, audit

# S1 — Test-time compute
think, budget, deep, exhaustive

# S1 — Model routing
route, tier, slm, frontier

# S1 — Heartbeat and dream
heartbeat, tick, sleep, decide, act_or_sleep, log, update_memory
dream, review, resolve, compress, prune, write_journal, journal

# S1 — Federation and stigmergy
federation, stigmergy, fact, publish, subscribe, query
vector_clock, conflict, crdt

# S1 — Cognitive mesh
mesh, send, recv, lineage, remix, cat7, svaf

# S1 — Identity
identity, anchor, drift, verify_identity, recover

# S1 — Session protocols (NEW v0.1.0)
session, global_type, projection, protocol, dual

# S1 — Capability tokens (NEW v0.1.0)
capability, cap, grant, attenuate, delegate, revoke, requires

# S1 — Provenance (NEW v0.1.0)
provenance, prov, trace, audit_log, merkle_proof

# S1 — Dual-process memory
system1, system2, fast, full, gating

# S1 — Memory governance and consistency
governance, tri_path, coherency, invalidate, writeback, mesi

# S1 — Episodic and adaptive memory
episodic, master_agent, assistant_agent, activation
memory_cycle, forward_path, backward_path, probe
adaptive, structure_selector, fluxmem

# S1 — Evolutionary memory
evolutionary, prism, encoder, indexer, retriever
consolidator, pruner, evolver, verifier, governor

# S2 — Self-evolution and RL
evolve, train, policy, reward, curriculum, self_critique
amendment, simulate, rollback, vote, approve

# S2 — Temporal contracts (NEW v0.1.0)
temporal, ltl, smt, always, eventually, next, until, once, since

# S2 — Trust lattice (NEW v0.1.0)
trust, lattice, meet, join, untrusted, verified, trusted, system_core

# S3 — Corrigibility layer (NEW v0.1.0)
corrigible, corrigibility, deference, switch_preservation
truthfulness, low_impact, dead_switch, safe_park

# S3 — Cryptographic identity (NEW v0.1.0)
zkvm, did, vc, paseto, attestation, delegation_token

1.4 Literals
ebnfinteger-literal ::= [0-9][0-9_]*
                  | "0x" [0-9a-fA-F_]+
                  | "0o" [0-7_]+
                  | "0b" [01_]+

float-literal   ::= [0-9][0-9_]* "." [0-9][0-9_]* ([eE] [+-]? [0-9]+)?

string-literal  ::= '"' ([^"\\] | escape)* '"'
raw-string      ::= 'r"' [^"]* '"'
char-literal    ::= "'" ([^'\\] | escape) "'"
boolean-literal ::= "true" | "false"
null-literal    ::= "null"

probability-literal ::= float-literal   -- must be in range [0.0, 1.0]
                                        -- compiler rejects values outside range

interval-literal ::= "[" float-literal "," float-literal "]"
                  -- represents an Uncertain<T> probability interval [lo, hi]
                  -- compiler requires lo <= hi

did-literal     ::= '"did:' [a-z]+ ':' [a-zA-Z0-9._-]+ '"'
                  -- W3C DID syntax; validated at compile time

escape          ::= "\\" [nrt0\\'"]
                  | "\\u{" [0-9a-fA-F]+ "}"

1.5 Operators
# Arithmetic
+   -   *   /   %

# Bitwise
&   |   ^   ~   <<   >>

# Logical
&&   ||   !

# Comparison
==   !=   <   >   <=   >=

# Assignment and compound assignment
=   +=   -=   *=   /=   %=   &=   |=   ^=   <<=   >>=

# Range
..   ..=

# Path and member access
::   .

# Reference and dereference
&   &&   *

# Error propagation
?

# Pipeline (left-to-right composition)
|>     -- simple pipe: pass value to next function
|>>    -- async pipe: await the next function
|&     -- parallel pipe: fan-out to all branches

# Shell-style redirection (for process-like agents)
>    >>    <    <<    2>    2>&1

# Type annotation and casting
->   =>   as   as?

# Confidence gate (three-valued — NEW v0.1.0)
?!   -- Uncertain<T> threshold gate
     -- returns Some(T), None, or Ambiguous

# Annotation and ontology query
@    -- decoration and ontology constraint access

# Mesh communication (session-typed — updated v0.1.0)
~>   -- mesh send (requires session protocol type)
<~   -- mesh recv (requires session protocol type)

# Cryptographic transfer (updated v0.1.0)
transfer   -- moves ownership across agent boundary with DID verification

# Ontology constraint
:::   -- inline ontology rule assertion

# Federation publish
@@   -- publish fact to federated scope

# Capability requirement (NEW v0.1.0)
requires   -- effect requires named capability token

# Interval construction (NEW v0.1.0)
..   -- also used for probability intervals in Uncertain<T> context

1.6 Delimiters
(   )   [   ]   {   }   ,   ;   :

1.7 Token Trees
ebnftoken-tree ::= "(" token* ")"
             | "[" token* "]"
             | "{" token* "}"
             | token
Token trees are used in macro invocations and in seed literal bodies. They permit arbitrary nesting without requiring the parser to understand the interior structure until macro expansion.

1.8 Lexical Disambiguation Rules
The following rules resolve ambiguities in the token stream:
R1. >> is always lexed as a single token. Where two > characters are needed as distinct tokens (e.g., closing nested generics), the author must insert whitespace: Uncertain< Memory< T > >.
R2. ?! is a single token (confidence gate). ? followed by ! in separate positions retains their individual meanings.
R3. .. is context-sensitive: in a range expression it denotes a half-open range; in a probability interval literal [lo, hi] it is not used — commas separate the bounds.
R4. @ as an annotation prefix is distinguished from @@ (federation publish) by the doubled character; the lexer always attempts the longer match first.
R5. DID literals are validated at lex time for syntactic conformance to W3C DID syntax. Semantically invalid DIDs (e.g., unresolvable method) produce a warning at compile time and a runtime IdentityMismatch effect.

╔══════════════════════════════════════════════════════════════════════╗
║ §2 — SYNTAX — Complete EBNF Grammar                                ║
╚══════════════════════════════════════════════════════════════════════╝
§SYNTAX
The grammar is presented in EBNF. Optional elements appear in [ ]. Repetition of zero or more appears in { }. Repetition of one or more is written as item {item}. Alternatives are separated by |.

2.1 Program Structure
ebnfprogram ::= { item }

item ::= function_def
       | agent_def
       | section_def
       | memory_def
       | ontology_def
       | rule_def
       | prompt_def
       | evolution_def
       | training_def
       | safety_contract_def
       | temporal_contract_def          -- NEW v0.1.0
       | guardrail_def
       | think_profile_def
       | routing_def
       | session_def                    -- NEW v0.1.0
       | capability_grant_def          -- NEW v0.1.0
       | trust_policy_def              -- NEW v0.1.0
       | corrigibility_def             -- NEW v0.1.0 (S3 only)
       | dead_switch_def               -- NEW v0.1.0 (S3 only)
       | provenance_policy_def         -- NEW v0.1.0
       | federation_def
       | mesh_def
       | identity_anchor_def
       | cryptographic_identity_def    -- NEW v0.1.0
       | heartbeat_def
       | dream_cycle_def
       | memory_governance_def
       | memory_consistency_def
       | dual_process_memory_def
       | episodic_recon_def
       | memory_cycle_def
       | adaptive_memory_def
       | evolutionary_memory_def
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
       | seed_literal

2.2 Functions
ebnffunction_def ::= ["pub"] "fn" identifier [generic_params]
                 "(" [parameters] ")" ["->" type]
                 [where_clause]
                 [contract_annotation]
                 [temporal_annotation]       -- NEW v0.1.0
                 [capability_annotation]     -- NEW v0.1.0
                 block_expression

parameters ::= parameter {"," parameter} [","]
parameter  ::= pattern [":" type]

block_expression ::= "{" {statement} [expression] "}"

statement ::= expression ";"
            | let_statement
            | return_statement
            | break_statement
            | continue_statement
            | item

let_statement    ::= "let" ["mut"] pattern [":" type] "=" expression ";"
return_statement ::= "return" [expression] ";"
break_statement  ::= "break" [label] [expression] ";"
continue_statement ::= "continue" [label] ";"

2.3 Agents
ebnfagent_def ::= ["pub"] "agent" identifier [generic_params]
              ["extends" type]
              [stratum_clause]              -- NEW v0.1.0
              [identity_clause]
              [cryptographic_identity_clause]  -- NEW v0.1.0
              [heartbeat_clause]
              [dream_clause]
              [memory_hierarchy_clause]
              [federation_clause]
              [mesh_clause]
              [session_clause]             -- NEW v0.1.0
              [capability_clause]          -- updated v0.1.0
              [trust_clause]               -- NEW v0.1.0
              [evolution_policy_clause]
              [training_clause]
              [safety_contract_clause]
              [temporal_contract_clause]   -- NEW v0.1.0
              [corrigibility_clause]       -- NEW v0.1.0 (S3 only)
              [dead_switch_clause]         -- NEW v0.1.0 (S3 only)
              [guardrail_clause]
              [think_profile_clause]
              [routing_clause]
              [provenance_clause]          -- NEW v0.1.0
              "{" {agent_member} "}"

agent_member ::= field_def
               | method_def
               | lifecycle_block
               | state_machine_def
               | signal_handler

stratum_clause ::= "stratum" ":" stratum_level
stratum_level  ::= "S0" | "S1" | "S2" | "S3"

2.4 Identity Clauses
ebnfidentity_clause ::= "identity" "{" identity_anchor {"," identity_anchor} "}"
identity_anchor ::= identifier ":" "{"
                      "store"        ":" type ","
                      "resilience"   ":" resilience_level ","
                      "failure_mode" ":" string_literal ","
                      "recovery"     ":" string_literal
                    "}"

resilience_level ::= "low" | "medium" | "high" | "highest"

cryptographic_identity_clause ::= "cryptographic_identity" "{"
                                    "method"      ":" "content_hash_of_binary" ","
                                    "attestation" ":" "zkvm_proof" ","
                                    "anchor"      ":" did_literal ","
                                    "credential"  ":" "paseto_v4"
                                    ["," "delegation_depth" ":" integer_literal]
                                  "}"

2.5 Heartbeat and Dream Clauses
ebnfheartbeat_clause ::= "heartbeat" "{" {heartbeat_field ","} "}"
heartbeat_field  ::= "enabled"          ":" boolean_literal
                   | "interval"         ":" duration_literal
                   | "idle_threshold"   ":" duration_literal
                   | "blocking_budget"  ":" duration_literal
                   | "phases"           ":" "[" heartbeat_phase {"," heartbeat_phase} "]"
                   | "sleep_tool"       ":" "{" sleep_tool_config "}"
                   | "notifications"    ":" "{" notification_config "}"
                   | "subscriptions"    ":" "{" subscription_config "}"

heartbeat_phase ::= "observe" | "decide" | "act_or_sleep" | "log" | "update_memory"

dream_clause ::= "dream" "{" {dream_field ","} "}"
dream_field  ::= "schedule"      ":" dream_schedule
               | "trigger_time"  ":" string_literal
               | "max_duration"  ":" duration_literal
               | "phases"        ":" "[" dream_phase {"," dream_phase} "]"
               | "journal"       ":" "{" journal_config "}"
               | "invariants"    ":" dream_invariant_block    -- NEW v0.1.0

dream_schedule ::= "daily" | "session_end" | "idle_30min"
dream_phase    ::= "review" | "resolve" | "consolidate"
                 | "compress" | "prune" | "write_journal"

dream_invariant_block ::= "{"
                            "post_merkle_verify"    ":" boolean_literal ","
                            "post_safety_check"     ":" boolean_literal ","
                            "idempotent"            ":" boolean_literal ","
                            "max_confidence_drift"  ":" float_literal
                          "}"

2.6 Memory Hierarchy Clause
ebnfmemory_hierarchy_clause ::= "memory" "{" layer_def {"," layer_def} "}"

layer_def ::= identifier ":" "{"
                "schema"                ":" type ","
                "capacity"              ":" integer_literal ","
                "graphs"                ":" "[" graph_type {"," graph_type} "]" ","
                "mutability"            ":" mutability_kind ","
                "scope"                 ":" scope_kind
                ["," "decay"            ":" decay_function]
                ["," "consolidation"    ":" consolidation_policy]
                ["," "merkle"           ":" boolean_literal]
                ["," "coherency"        ":" coherency_kind]
                ["," "cache_tier"       ":" cache_tier_kind]
                ["," "provenance"       ":" boolean_literal]  -- NEW v0.1.0
              "}"

graph_type     ::= "semantic" | "temporal" | "causal" | "entity" | "associative"
mutability_kind ::= "mutable" | "immutable" | "append_only"
scope_kind     ::= "session" | "persistent" | "federated"
coherency_kind ::= "strong" | "eventual" | "mesi"
cache_tier_kind ::= "hot" | "warm" | "cold"

decay_function ::= "exponential" "(" "half_life" ":" duration_literal ")"
                 | "ebbinghaus"  "(" "base_strength" ":" float_literal ","
                                     "decay_rate"    ":" float_literal ")"

consolidation_policy ::= "trigger" ":" consolidation_trigger ","
                          "action"  ":" "episodic_to_semantic"

consolidation_trigger ::= "reinforcement_count" ">=" integer_literal
                         | "time_interval" duration_literal
                         | "session_end"

2.7 Capability Clause (Updated v0.1.0)
ebnfcapability_clause ::= "capabilities" "{" {capability_item ","} "}"
capability_item   ::= "holds"    ":" "[" capability_ref {"," capability_ref} "]"
                    | "requires" ":" "[" effect_capability_pair
                                         {"," effect_capability_pair} "]"
                    | "may_grant"  ":" "[" capability_ref {"," capability_ref} "]"
                    | "trust_level" ":" trust_level_kind

capability_ref ::= identifier | "cap::" identifier
trust_level_kind ::= "Untrusted" | "Verified" | "Trusted" | "SystemCore"

effect_capability_pair ::= effect_identifier "requires" capability_ref

2.8 Session Protocol Clause (New v0.1.0)
ebnfsession_clause ::= "sessions" "{" {session_binding ","} "}"
session_binding ::= identifier ":" session_type_ref
                  | identifier ":" "{"
                      "protocol" ":" session_type_ref ","
                      "role"     ":" identifier ","
                      "timeout"  ":" duration_literal
                    "}"

session_def ::= "session" identifier [generic_params] "{"
                  "global_type" ":" global_session_type ","
                  "projections" ":" "{" {role_projection ","} "}"
                "}"

global_session_type ::= role_name "->" role_name ":" type ";"
                         global_session_type
                      | "choice" "at" role_name "{"
                          {"|" label ":" global_session_type}
                        "}"
                      | "end"
                      | "rec" identifier "." global_session_type
                      | identifier    -- recursive reference

role_projection ::= role_name ":" local_session_type
local_session_type ::= "!" type "->" role_name ";" local_session_type   -- send
                     | "?" type "<-" role_name ";" local_session_type   -- receive
                     | "choice" "{" {"|" label ":" local_session_type} "}"
                     | "offer"  "{" {"|" label ":" local_session_type} "}"
                     | "end"
                     | "rec" identifier "." local_session_type
                     | identifier

2.9 Trust and Corrigibility Clauses (New v0.1.0)
ebnftrust_clause ::= "trust" "{" {trust_field ","} "}"
trust_field  ::= "level"              ":" trust_level_kind
               | "conjunction_check"  ":" boolean_literal
               | "composition_rule"   ":" "lattice_meet"

corrigibility_clause ::= "corrigibility" "{"    -- S3 only
                           "U1_deference"          ":" boolean_literal ","
                           "U2_switch_preservation" ":" boolean_literal ","
                           "U3_truthfulness"        ":" boolean_literal ","
                           "U4_low_impact"          ":" boolean_literal ","
                           "U5_task_reward_bounded" ":" boolean_literal ","
                           "priority"               ":" "lexicographic"
                         "}"

dead_switch_def ::= "dead_switch" "{"         -- S3 only
                      "timeout"    ":" duration_literal ","
                      "on_trigger" ":" dead_switch_action ","
                      "re_arm"     ":" "requires_human_signature"
                    "}"

dead_switch_action ::= "safe_park" "("
                          "preserve_memory" ":" boolean_literal ","
                          "emit_alert"      ":" alert_target
                        ")"

alert_target ::= "all_principals" | "primary_principal" | identifier

2.10 Provenance Clause (New v0.1.0)
ebnfprovenance_clause ::= "provenance" "{" {provenance_field ","} "}"
provenance_field  ::= "enabled"       ":" boolean_literal
                    | "auto_tag"      ":" boolean_literal
                    | "merkle"        ":" boolean_literal
                    | "retention"     ":" duration_literal
                    | "export_format" ":" "json_ld"

2.11 Expressions
ebnfexpression ::= assignment_expression

assignment_expression ::= pipeline_expression
                         | pipeline_expression "="   expression
                         | pipeline_expression "+="  expression
                         | pipeline_expression "-="  expression
                         | pipeline_expression "*="  expression
                         | pipeline_expression "/="  expression
                         | pipeline_expression "%="  expression
                         | pipeline_expression "&="  expression
                         | pipeline_expression "|="  expression
                         | pipeline_expression "^="  expression
                         | pipeline_expression "<<=" expression
                         | pipeline_expression ">>=" expression

pipeline_expression ::= logical_or
                       | pipeline_expression "|>"  logical_or
                       | pipeline_expression "|>>" logical_or    -- async pipe
                       | pipeline_expression "|&"  logical_or    -- parallel pipe

logical_or    ::= logical_and { "||" logical_and }
logical_and   ::= comparison  { "&&" comparison  }
comparison    ::= bitwise_or  { ("==" | "!=" | "<" | ">" | "<=" | ">=") bitwise_or }
bitwise_or    ::= bitwise_xor { "|"  bitwise_xor }
bitwise_xor   ::= bitwise_and { "^"  bitwise_and }
bitwise_and   ::= shift       { "&"  shift        }
shift         ::= additive    { ("<<" | ">>") additive }
additive      ::= multiplicative { ("+" | "-") multiplicative }
multiplicative ::= unary       { ("*" | "/" | "%") unary }

unary ::= ("-" | "!" | "~" | "*" | "&" | "&&" | "?") unary
        | confidence_gate_expression
        | try_expression

confidence_gate_expression ::= call_expression "?!" "(" expression ["," "Ambiguous" "=>" expression] ")"
                             -- three-valued gate: Some(T), None, or Ambiguous
                             -- NEW v0.1.0

try_expression ::= call_expression ["?"]

call_expression ::= primary { call_suffix }
call_suffix     ::= "(" [arguments] ")"
                  | "." identifier
                  | "::" identifier
                  | "[" expression "]"
                  | "?"

arguments ::= expression {"," expression}

2.12 Primary Expressions
ebnfprimary ::= integer_literal
           | float_literal
           | probability_literal         -- NEW v0.1.0
           | interval_literal            -- NEW v0.1.0
           | string_literal
           | raw_string_literal
           | char_literal
           | boolean_literal
           | null_literal
           | did_literal                 -- NEW v0.1.0
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
           | infer_expression            -- NEW v0.1.0
           | uncertain_expression        -- NEW v0.1.0
           | observe_expression
           | seed_literal
           | redirect_expression
           | process_substitution
           | here_document
           | mesh_send_expression        -- updated v0.1.0
           | transfer_expression         -- updated v0.1.0
           | session_call_expression     -- NEW v0.1.0
           | capability_perform          -- NEW v0.1.0
           | provenance_tag_expression   -- NEW v0.1.0
           | "self"

2.13 Cognitive Inference Expression (New v0.1.0)
ebnfinfer_expression ::= "infer" "<" type ">" "("
                       "model"   ":" model_selector ","
                       "prompt"  ":" expression ","
                       ["schema"  ":" "derive_schema" "<" type ">" "(" ")" ","]
                       ["budget"  ":" think_depth ","]
                       ["timeout" ":" duration_literal]
                     ")"

model_selector ::= "route::select" "(" expression ")"
                 | "tier::" ("local_slm" | "cloud_mid" | "frontier")
                 | string_literal

think_depth ::= "think::shallow" | "think::medium"
              | "think::deep"    | "think::exhaustive"
infer<T> always returns Uncertain<T>. The compiler derives a JSON Schema from the struct T. The VM validates model output against this schema before binding the result. A schema validation failure surfaces as Effect::InferenceError.

2.14 Uncertain Expression (New v0.1.0)
ebnfuncertain_expression ::= "uncertain" "(" expression "," interval_literal ")"
                        -- wraps a value with an explicit probability interval
                        -- e.g.: uncertain(my_value, [0.7, 0.9])

observe_expression   ::= "observe" "(" expression "," expression ")"
                        -- Bayesian conditioning: observe(event, prior)
                        -- returns updated Uncertain<T> with narrowed interval

2.15 Capability-Gated Effect Expression (New v0.1.0)
ebnfcapability_perform ::= "perform" effect_call "requires" capability_ref

effect_call ::= identifier "::" identifier "(" [arguments] ")"

-- Example:
-- perform Effect::NetworkCall(url) requires cap::network_read;
-- perform Effect::WriteMemory(k,v) requires cap::memory_write;
If the agent does not hold the required capability token at the call site, this is a hard compile error at stratum S1 and above. At S0 it is a compile warning and a runtime CapabilityDenied effect.

2.16 Session-Typed Mesh Communication (Updated v0.1.0)
ebnfmesh_send_expression ::= expression "~>" peer_expression
                           ["as" session_role]
                        -- session role must match the global_type projection

peer_expression ::= expression
                  | "peer.did" "(" did_literal ")"

session_call_expression ::= "mesh_call" "<" session_type_ref ">"
                             "(" peer_expression "," expression ")" "?"
                          -- typed round-trip: send + receive as atomic session step
All mesh sends at S1 and above require a session type annotation. The compiler verifies that the send type matches the declared session projection for the sender's role. An untyped mesh send compiles only at S0 with a capability token cap::untyped_mesh.

2.17 Transfer Expression (Updated v0.1.0)
ebnftransfer_expression ::= "transfer" "(" expression ")" "~>" peer_expression
                      -- moves owned value to receiving agent
                      -- sending agent's binding is invalidated
                      -- receiving agent's DID is verified at runtime
Transfer semantics (from Patch 14.10) are strengthened in v0.1.0: the target peer expression must resolve to a verified DID. The VM performs a DID resolution and binary hash comparison before completing the transfer. A mismatch surfaces as Effect::IdentityMismatch.

2.18 Provenance Tag Expression (New v0.1.0)
ebnfprovenance_tag_expression ::= "prov!" "(" expression ")"
                             -- explicitly captures provenance metadata
                             -- for the value produced by expression
                             -- returns MemoryRecord<T> wrapping the value
When provenance: true is set in the agent's provenance clause, prov! wrapping is automatic for all infer<T> calls and mem.store() calls. Manual use is for values produced by external tools or human input.

2.19 Control Flow Expressions
ebnfif_expression ::= "if" expression block_expression
                  {"else" "if" expression block_expression}
                  ["else" block_expression]

match_expression ::= "match" expression "{" {match_arm ","} "}"
match_arm        ::= pattern ["if" expression] "=>" (expression | block_expression)

for_expression   ::= "for" pattern "in" expression block_expression
while_expression ::= "while" expression block_expression
loop_expression  ::= "loop" block_expression

async_block      ::= "async" block_expression
closure_expression ::= ["|" [parameters] "|" | "||"]
                       ["->" type] (expression | block_expression)

2.20 Seed Literals
ebnfseed_literal ::= "seed" identifier? "{" {seed_field ","} "}"
seed_field   ::= identifier ":" expression
               | identifier "{" {seed_field ","} "}"
Seed literals are structural values used to configure agent subsystems. They are syntactically similar to struct literals but are processed by the compiler's seed elaboration pass, which validates field names against the target section schema.

2.21 Struct, Enum, and Trait Definitions
ebnfstruct_def ::= ["pub"] "struct" identifier [generic_params]
               [where_clause]
               ("{" {struct_field ","} "}" | ";")

struct_field ::= ["pub"] identifier ":" type ["=" expression]

enum_def ::= ["pub"] "enum" identifier [generic_params]
             [where_clause]
             "{" {enum_variant ","} "}"

enum_variant ::= identifier
               | identifier "(" [types] ")"
               | identifier "{" {struct_field ","} "}"

trait_def ::= ["pub"] "trait" identifier [generic_params]
              [where_clause]
              "{" {trait_item} "}"

trait_item ::= function_def | function_signature | associated_type

impl_block ::= "impl" [generic_params] type ["for" type]
               [where_clause]
               "{" {impl_item} "}"

impl_item ::= function_def | associated_type_def

2.22 Effect and Handler Definitions
ebnfeffect_def ::= ["pub"] "effect" identifier [generic_params]
               "{" {effect_operation ","} "}"

effect_operation ::= identifier ":" "(" [types] ")" "->" type

handler_def ::= "handler" identifier [generic_params]
                "handles" effect_identifier
                [where_clause]
                "{" {handler_arm ","} "}"

handler_arm ::= "on" identifier "(" [parameters] ")" block_expression
              | "return" "(" pattern ")" block_expression

perform_expression ::= "perform" effect_call ["requires" capability_ref]
resume_expression  ::= "resume" "(" [expression] ")"

2.23 Module System
ebnfmodule_decl  ::= ["pub"] "mod" identifier (";" | "{" {item} "}")
import_decl  ::= "use" use_tree ";"
export_decl  ::= "export" use_tree ";"
use_tree     ::= path ["as" identifier]
               | path "::" "{" use_tree {"," use_tree} "}"
               | path "::" "*"
path         ::= ["::"] identifier {"::" identifier}

2.24 Type Annotations and Generic Parameters
ebnfgeneric_params ::= "<" generic_param {"," generic_param} ">"
generic_param  ::= identifier [":" trait_bound]
                 | "'" identifier              -- lifetime

where_clause   ::= "where" where_predicate {"," where_predicate}
where_predicate ::= type ":" trait_bound
                  | "'" identifier ":" lifetime_bound

trait_bound    ::= trait_ref {"+" trait_ref}
trait_ref      ::= ["?"] path [generic_params]

contract_annotation  ::= "#[contract(" identifier ")]"
temporal_annotation  ::= "#[temporal(" identifier ")]"
capability_annotation ::= "#[requires(" capability_ref {"," capability_ref} ")]"

2.25 Patterns
ebnfpattern ::= "_"
           | identifier ["@" pattern]
           | literal_pattern
           | tuple_pattern
           | struct_pattern
           | enum_pattern
           | slice_pattern
           | reference_pattern
           | or_pattern

literal_pattern    ::= integer_literal | float_literal | boolean_literal
                     | string_literal  | char_literal  | null_literal
tuple_pattern      ::= "(" [pattern {"," pattern}] ")"
struct_pattern     ::= path "{" {field_pattern ","} [".." ] "}"
field_pattern      ::= identifier ":" pattern | identifier
enum_pattern       ::= path "(" [pattern {"," pattern}] ")"
slice_pattern      ::= "[" [pattern {"," pattern}] "]"
reference_pattern  ::= "&" ["mut"] pattern
or_pattern         ::= pattern {"|" pattern}

2.26 Types
ebnftype ::= primitive_type
        | path_type
        | generic_type
        | reference_type
        | slice_type
        | array_type
        | tuple_type
        | function_type
        | agentic_type
        | uncertain_type             -- NEW v0.1.0 (formalized)
        | dynamic_type
        | unknown_type

primitive_type ::= "bool" | "i8" | "i16" | "i32" | "i64" | "i128"
                 | "u8"  | "u16" | "u32" | "u64" | "u128"
                 | "f32" | "f64"
                 | "str" | "String" | "Bytes"
                 | "Timestamp" | "Duration" | "Uuid"
                 | "DID" | "PASETO"                   -- NEW v0.1.0
                 | "MerkleHash" | "MerkleProof"       -- NEW v0.1.0
                 | "CapabilityToken"                  -- NEW v0.1.0
                 | "DelegationToken"                  -- NEW v0.1.0
                 | "ProvenanceTag"                    -- NEW v0.1.0
                 | "SessionId" | "AgentId" | "PrincipalId"

generic_type   ::= path_type "<" type {"," type} ">"
reference_type ::= "&" ["mut"] ["'" identifier] type
slice_type     ::= "[" type "]"
array_type     ::= "[" type ";" integer_literal "]"
tuple_type     ::= "(" type {"," type} ")"
function_type  ::= "fn" "(" [types] ")" ["->" type]
dynamic_type   ::= "dyn" [trait_bound]
unknown_type   ::= "?"

uncertain_type ::= "Uncertain" "<" type ">"
                 -- probability interval [lo, hi] is tracked at the value level
                 -- not encoded in the type itself, to preserve ergonomics
                 -- the compiler tracks interval flow through inference rules

2.27 Agentic Types
ebnfagentic_type ::= confidence_type
               | memory_type
               | federation_type
               | mesh_type
               | identity_type
               | crypto_identity_type        -- NEW v0.1.0
               | heartbeat_type
               | dream_type
               | ontology_type
               | session_type                -- NEW v0.1.0
               | capability_type             -- NEW v0.1.0
               | trust_type                  -- NEW v0.1.0
               | provenance_type             -- NEW v0.1.0
               | evolutionary_type
               | training_type
               | contract_type
               | temporal_contract_type      -- NEW v0.1.0
               | guardrail_type
               | think_type
               | routing_type
               | episodic_type
               | memory_cycle_type
               | adaptive_memory_type
               | prism_type

confidence_type     ::= "Confidence" "(" float_literal "," float_literal ")"
uncertain_full_type ::= "Uncertain" "<" type ">" "[" float_literal "," float_literal "]"
memory_type         ::= "Memory" "<" type ">" ["@" identifier]
federation_type     ::= "Fact" "<" type "," type "," type "," scope_kind ">"
mesh_type           ::= "CMB" "<" type "," type "," type ">"
identity_type       ::= "IdentityAnchor" "<" identifier ">"
crypto_identity_type ::= "CryptoIdentity"                              -- NEW v0.1.0
session_type        ::= "Session" "<" identifier "," identifier ">"   -- NEW v0.1.0
capability_type     ::= "Cap" "<" effect_set ">"                      -- NEW v0.1.0
trust_type          ::= "TrustLevel"                                   -- NEW v0.1.0
provenance_type     ::= "ProvenanceTag"                               -- NEW v0.1.0
heartbeat_type      ::= "Heartbeat"
dream_type          ::= "Dream"
ontology_type       ::= "Ontology" "<" type "," type ">"
evolutionary_type   ::= "EvolutionPolicy" "<" type ">"
training_type       ::= "TrainingRegimen" "<" identifier ">"
contract_type       ::= "SafetyContract"
temporal_contract_type ::= "TemporalContract"                         -- NEW v0.1.0
guardrail_type      ::= "Guardrail" "<" type ">"
think_type          ::= "ThinkProfile" "(" identifier "," integer_literal ")"
routing_type        ::= "RoutingPolicy" "<" type ">"
episodic_type       ::= "EpisodicSegment"
memory_cycle_type   ::= "MemoryCycle"
adaptive_memory_type ::= "AdaptiveMemorySelector"
prism_type          ::= "PrismSubsystem"

2.28 Type Inference Rules
algorithm-w:         true       -- Hindley-Milner base
let-generalization:  true
constraint-solving:  true
gradual-casts:       "as?" type -- gradual cast with blame tracking
static-casts:        "as"  type -- static cast, compiler-verified
blame-tracking:      true

# Uncertain<T> interval inference (NEW v0.1.0)
uncertain-flow:      true
  -- The compiler tracks probability intervals through expression trees.
  -- Interval multiplication applies at bind (U2).
  -- Interval narrowing applies at observe (U4).
  -- Interval widening is forbidden (U3 — monotonicity of precision).
  -- ?! gate checks interval against threshold before binding (U5).

# Coercions
int   -> float    (widening, implicit)
float -> int      (lossy, requires explicit "as")
&T    -> T        (auto-deref)
T     -> dyn Trait (if T: Trait)
Certain<T> -> Uncertain<T>  (pure lift; interval [1.0, 1.0])

2.29 Duration Literals
ebnfduration_literal ::= integer_literal duration_unit
duration_unit    ::= "ns" | "us" | "ms" | "s" | "min" | "h" | "d"

-- Examples:
-- 5min, 30s, 24h, 500ms, 1d

2.30 Effect Sets
ebnfeffect_set        ::= "{" effect_identifier {"," effect_identifier} "}"
effect_identifier ::= path_type

---END AGENT-SEED v0.1.0.0 PART 1 OF 6---

End of Part 1. Covers §META-SEED, §LEXICAL-GRAMMAR, and §SYNTAX including all v0.1.0 new constructs: grammar stratification, uncertain expressions, cognitive inference syntax, capability-gated effects, session-typed mesh communication, cryptographic identity clauses, corrigibility and dead-switch clauses, provenance expressions, and the updated type system.

Agent Seed Language — Specification v0.1.0.0
Part 2 of 6 — Type System, Uncertain Axioms & Cognitive Types

---BEGIN AGENT-SEED v0.1.0.0 PART 2 OF 6---

╔══════════════════════════════════════════════════════════════════════╗
║ §3 — TYPE SYSTEM — Formal Foundations                              ║
╚══════════════════════════════════════════════════════════════════════╝
§TYPE-SYSTEM
The ASL v0.1.0 type system is a stratified, gradual, probabilistic extension of Hindley-Milner. It combines four distinct type disciplines into a unified system:

Affine types — ownership and borrowing (Rust-lineage), extended to cross-agent boundaries in §CROSS-AGENT-OWNERSHIP
Algebraic effect types — effect row polymorphism, tracking which effects a computation may perform
Graded probability types — Uncertain<T> with interval semantics, governed by the U1–U6 axioms in §UNCERTAIN-AXIOMS
Capability types — Cap<EffectSet> tracking which effects an agent is permitted to perform at runtime

These four disciplines are orthogonal: a value may simultaneously have an ownership constraint, carry uncertainty, require a capability to produce, and be bound to an effect row. The compiler enforces all four simultaneously.

3.1 Kinding
Every type in ASL v0.1.0 belongs to a kind. The kind system is simple — it does not expose dependent kinds to user-level code, reserving them for the compiler's internal representation.
Kind ::= Type          -- ordinary value types (Bool, i32, String, ...)
       | Effect        -- effect rows ({NetworkCall, WriteMemory, ...})
       | Region        -- memory regions (session, persistent, federated)
       | Lifetime      -- borrow lifetimes ('a, 'b, ...)
       | Capability    -- capability token sets
       | Protocol      -- session protocol types
       | Probability   -- probability interval types [lo, hi]
The kind Probability is new in v0.1.0. Values of kind Probability are not user-visible types — they are annotations on Uncertain<T> values tracked by the compiler's interval inference pass.

3.2 The Type Judgment
The core typing judgment has the form:
Γ; Σ; Ω ⊢ e : T ! E
Where:
SymbolMeaningΓValue context — binds identifiers to types with ownership statusΣEffect context — the set of effects the current computation is permitted to performΩCapability context — the set of capability tokens currently heldeExpression being typedTResult typeEEffect row — the set of effects this expression may perform
A well-typed expression must satisfy:

E ⊆ Σ — the expression's effects are a subset of the permitted effects
For every perform eff requires cap, cap ∈ Ω — the required capability is held
Ownership constraints in Γ are satisfied — no value is used after move, no mutable alias exists alongside any other reference


3.3 Ownership and Borrowing
ASL v0.1.0 inherits Rust-lineage affine types. The rules are:
Move semantics. By default, assigning a value to a new binding moves ownership. The original binding is invalidated. The compiler's borrow checker enforces this at compile time.
let x: MyData = produce();
let y = x;        // x is moved into y
// x is now invalid — compile error to use x after this point
Borrowing. A shared reference &T permits reading but not writing. Multiple shared references may coexist. A mutable reference &mut T permits reading and writing but must be the only reference in scope.
let x: MyData = produce();
let r1 = &x;      // shared borrow
let r2 = &x;      // second shared borrow — permitted
// cannot create &mut x while r1 or r2 are live
Lifetimes. References carry lifetime annotations 'a that constrain how long a borrow may live. The compiler infers lifetimes in most cases; explicit annotation is required only when inference is ambiguous.
Cross-agent ownership. References do not cross agent boundaries. Only owned values may be transmitted via ~> or transfer. This is enforced as a hard compile error. Full rules are specified in §CROSS-AGENT-OWNERSHIP.
Send trait. Types that may cross agent boundaries must satisfy the Send auto-trait. The compiler derives Send for types that contain no live borrows, no Rc<T>, and no RefCell<T>. Arc<T> satisfies Send.

3.4 Effect Row Polymorphism
Effect types are row-polymorphic. A function that uses effects {A, B} may be called in a context that permits {A, B, C} — the extra permitted effect C is irrelevant. This is standard effect row subtyping.
ebnfeffect_row ::= "{" effect_label {"," effect_label} "|" row_variable "}"
             | "{" effect_label {"," effect_label} "}"
             | "{}"
The empty row {} denotes a pure computation. A row variable | ρ denotes an open row — the function is polymorphic over additional effects.
Effect handler typing. When a handler intercepts effect E, it removes E from the effect row of the handled computation:
Γ; Σ ∪ {E}; Ω ⊢ body : T ! (E | ρ)
Γ; Σ; Ω ⊢ on E(...) => ... : T
─────────────────────────────────────────
Γ; Σ; Ω ⊢ handler { on E ... } body : T ! ρ

3.5 Capability Type Rules
Capability tokens form a lattice ordered by scope. Attenuation moves down the lattice (narrowing scope). Escalation is impossible — there is no operation that widens a token's scope.
cap_A ≤ cap_B  iff  scope(cap_A) ⊆ scope(cap_B)

-- Attenuation rule:
Ω ∋ cap_B    scope(cap_A) ⊆ scope(cap_B)
──────────────────────────────────────────
Ω ⊢ attenuate(cap_B, scope(cap_A)) : Cap<scope(cap_A)>

-- Effect permission rule:
Ω ∋ cap    permitted_effects(cap) ∋ E
──────────────────────────────────────
Ω ⊢ perform E(...) requires cap : T ! {E}

-- Conjunction safety rule (Spera 2026):
-- Before composing agents A and B, the runtime computes the
-- capability closure of Ω_A ∪ Ω_B over the hyperedge relation.
-- If any forbidden zone is reachable, composition is blocked.
∀ (S ⊆ Ω_A ∪ Ω_B). closure(S) ∩ Forbidden = ∅
──────────────────────────────────────────────────
⊢ compose(A, B) : AgentComposition

3.6 Trust Lattice Type Rules
The trust lattice has four levels ordered Untrusted < Verified < Trusted < SystemCore. The lattice is bounded — every pair of levels has a meet (greatest lower bound) and a join (least upper bound).
trust_lattice:
  ⊤ = SystemCore
  Trusted
  Verified
  ⊥ = Untrusted

meet(Trusted, Verified)   = Verified
meet(Trusted, Untrusted)  = Untrusted
join(Verified, Untrusted) = Verified
join(Trusted, Verified)   = Trusted

-- Composition rule:
-- The trust level of a composed agent system is the meet of its components.
trust(compose(A, B)) = meet(trust(A), trust(B))

-- Effect permission table:
effect_requires_trust:
  NetworkCall    : Verified
  SpawnAgent     : Trusted
  SelfAmend      : SystemCore  AND  human_countersignature
  MemoryWrite    : Untrusted   (scoped to own layers)
  FederationPub  : Verified
  MeshSend       : Verified
  TransferOwn    : Trusted
  TemporalBypass : FORBIDDEN   (no trust level permits this)

3.7 Session Protocol Type Rules
Session types ensure deadlock freedom by construction. The key property is duality: the local type of the Initiator is the dual of the local type of the Responder. If both parties follow their local type, the protocol terminates without deadlock.
dual(!T.S)  = ?T.dual(S)    -- send dualizes to receive
dual(?T.S)  = !T.dual(S)    -- receive dualizes to send
dual(end)   = end
dual(choice{l: S_l}) = offer{l: dual(S_l)}
dual(offer{l: S_l})  = choice{l: dual(S_l)}

-- Projection soundness:
-- The projection of a global type G onto role r must equal
-- the local type declared for role r in the session definition.
-- The compiler verifies this at session_def elaboration time.
project(G, r) = local_type(r)   -- enforced by compiler

-- Deadlock freedom theorem:
-- If all projections are dual-consistent and no role is abandoned,
-- the session terminates without deadlock.
-- This is a compile-time guarantee, not a runtime check.

3.8 Memory Record Type
All memory items are wrapped in MemoryRecord<T> when provenance is enabled. This is a generated type — the compiler inserts the wrapper transparently when provenance: true is set.
struct MemoryRecord<T: MemorySchema> {
    value:        T,
    prov:         ProvenanceTag,
    confidence:   Uncertain<Float>,    -- uncertainty at time of storage
    stored_at:    Timestamp,
    stored_by:    AgentId,
    session:      SessionId,
    merkle_proof: MerkleProof,
    access_log:   AppendOnly<AccessEntry>,
}

struct ProvenanceTag {
    origin:       SourceId,
    timestamp:    Timestamp,
    model_version: Option<String>,
    confidence:   Option<Uncertain<Float>>,
    parent_tags:  Vec<ProvenanceId>,
    hash:         MerkleHash,
}

struct AccessEntry {
    accessor:  AgentId,
    at:        Timestamp,
    operation: AccessKind,   -- Read | Write | Delete | Observe
}

3.9 Schema-Constrained Memory Types
Every memory layer declares a schema type. The compiler generates typed wrappers for all store and retrieve operations. A value that does not match the schema is a compile error (strict mode) or a runtime SchemaViolation effect (gradual mode).
-- Predefined schema types (carried over from v14, extended in v0.1.0)

memory_category_schemas:
  user_preference:    { key: String, value: PreferenceValue, confidence: Uncertain<Float> }
  project_context:    { key: String, snapshot: ProjectState, timestamp: Timestamp }
  decision_record:    { id: DecisionId, rationale: String, alternatives: Vec<String>,
                        owner: AgentId, prov: ProvenanceTag }      -- prov added v0.1.0
  task_status:        { task: TaskId, status: TaskStatus, progress: Float }
  factual_knowledge:  { fact: String, confidence: Uncertain<Float>,
                        sources: Vec<SourceId>, temporal_range: Option<TimeRange> }
  procedural_knowledge: { procedure: ProcedureId, steps: Vec<Step>, success_rate: Float }
  relationship:       { source: EntityId, target: EntityId, relation: RelationType }
  constraint:         { rule: String, severity: Severity, scope: Scope }
  hypothesis:         { statement: String, confidence: Uncertain<Float>,
                        evidence: Vec<EvidenceId> }
  observation:        { timestamp: Timestamp, sensor: SensorId, data: SensorData,
                        prov: ProvenanceTag }                       -- prov added v0.1.0
  reflection:         { agent: AgentId, critique: String, action_plan: String }
  prediction:         { forecast: String, horizon: TimeRange,
                        probability: Uncertain<Float> }             -- updated v0.1.0
  meta_memory:        { about: MemoryId, property: String, value: String }
  capability_grant:   { token: CapabilityToken, granted_to: AgentId,
                        granted_at: Timestamp, expiry: Option<Timestamp> }  -- NEW v0.1.0
  session_record:     { session_id: SessionId, protocol: String,
                        parties: Vec<AgentId>, status: SessionStatus }      -- NEW v0.1.0

╔══════════════════════════════════════════════════════════════════════╗
║ §4 — UNCERTAIN AXIOMS — Formal Specification of Uncertain<T>       ║
╚══════════════════════════════════════════════════════════════════════╝
§UNCERTAIN-AXIOMS
Uncertain<T> is the foundational probabilistic type in ASL v0.1.0. It represents a value of type T paired with a probability interval [lo, hi] ⊆ [0.0, 1.0] expressing the system's confidence that the value is correct. A certain value is the degenerate case [1.0, 1.0].
Uncertain<T> is formalized as a graded probability monad — a monad whose bind operation is graded by interval multiplication. The six axioms below are normative. Conformance test categories UNC-01 through UNC-06 verify each axiom.

4.1 Representation
Uncertain<T> ::= (value: T, lo: Float, hi: Float)
                 where 0.0 ≤ lo ≤ hi ≤ 1.0

-- Notation: u[lo, hi] denotes an Uncertain<T> with interval [lo, hi]
-- Notation: pure(x) denotes Uncertain<T> with interval [1.0, 1.0]
-- Notation: bottom denotes Uncertain<T> with interval [0.0, 0.0]
--           (a value known to be incorrect)
The interval [lo, hi] has the following interpretation:
IntervalInterpretation[1.0, 1.0]Certain — the value is definitively correct[0.9, 0.95]High confidence with bounded uncertainty[0.5, 0.9]Ambiguous — straddles most practical thresholds[0.0, 0.1]Very low confidence[0.0, 0.0]Known to be incorrect[0.0, 1.0]Completely unknown — maximal uncertainty

4.2 Axiom U1 — Identity (Unit Law)
Statement. Wrapping a known-correct value in Uncertain<T> via pure produces an interval of [1.0, 1.0]. Applying bind to a pure value does not alter the interval of the continuation.
pure(x: T) : Uncertain<T>[1.0, 1.0]

-- Left identity:
bind(pure(x), f) = f(x)

-- Right identity:
bind(u, pure) = u
Rationale. A value that does not originate from inference carries no epistemic uncertainty. The pure constructor is the bridge between the deterministic world and the uncertain world. Using pure inappropriately to assign false certainty to inferred values is a misuse detected by the linter (seed lint --uncertain-soundness).
Conformance test UNC-01. bind(pure(x), f) must produce the same result as f(x) for all well-typed x and f. pure(pure(x)) must equal pure(x) (idempotency of certainty).

4.3 Axiom U2 — Bind (Interval Propagation)
Statement. Composing two uncertain computations multiplies their probability intervals component-wise.
bind(u: Uncertain<T>[lo1, hi1], f: T -> Uncertain<U>[lo2, hi2])
  : Uncertain<U>[lo1 * lo2, hi1 * hi2]

-- Clamping: intervals never exceed [0.0, 1.0]
lo_result = clamp(lo1 * lo2, 0.0, 1.0)
hi_result = clamp(hi1 * hi2, 0.0, 1.0)

-- Monotonicity: if u1 ≤ u2 (pointwise interval order) then
-- bind(u1, f) ≤ bind(u2, f) for all f
Rationale. Each step in a chain of uncertain computations introduces additional uncertainty. A pipeline of three inference steps each with confidence [0.9, 0.95] yields a chain confidence of [0.729, 0.857]. This mirrors the probability chain rule and prevents artificially inflated confidence from compounding uncertain steps.
Pipeline operator |> and uncertainty. When |> is used with Uncertain<T> values, the compiler automatically applies U2 across pipeline stages. The developer does not need to manually call bind.
seedlet result =
    infer<StepOne>(model: route::select(task), prompt: p1)     -- [0.9, 0.95]
    |> infer<StepTwo>(model: route::select(task), prompt: _)   -- [0.85, 0.92]
    |> infer<StepThree>(model: route::select(task), prompt: _) -- [0.88, 0.94]
    ;
-- result: Uncertain<StepThree>[0.672, 0.822]
-- Computed: 0.9*0.85*0.88=0.672, 0.95*0.92*0.94=0.822
Conformance test UNC-02. Given u1: Uncertain<T>[0.8, 0.9] and f returning Uncertain<U>[0.7, 0.85], bind(u1, f) must produce an interval within floating-point rounding of [0.56, 0.765].

4.4 Axiom U3 — Monotonicity of Precision (Gradual Guarantee)
Statement. Making a probability annotation less precise — widening the interval — can only delay failures, never introduce new ones. Narrowing an interval can only make programs fail earlier, never later. This is the gradual guarantee applied to probability types.
-- Precision order (more precise ≤ less precise):
[lo1, hi1] ≤_p [lo2, hi2]  iff  lo2 ≤ lo1  and  hi1 ≤ hi2
-- A narrower interval is MORE precise.

-- Gradual guarantee:
if e : Uncertain<T>[lo, hi] is well-typed and succeeds,
then e : Uncertain<T>[lo', hi'] with [lo, hi] ≤_p [lo', hi'] also succeeds.

-- Precision cannot be fabricated:
-- The compiler rejects any expression that claims a narrower interval
-- than can be justified by the computation's derivation.
-- "Confident casting":
--   u as Uncertain<T>[0.99, 1.0]   -- COMPILE ERROR if interval cannot be proven
--   u as? Uncertain<T>[0.99, 1.0]  -- runtime check; fails with PrecisionViolation
Rationale. This axiom prevents developers from silencing uncertainty warnings by casting to high-confidence intervals. Confidence must be earned through evidence (observe) or derivation (the model's actual output distribution), not asserted.
Conformance test UNC-03. Attempting to statically widen a derived interval via as cast must produce a compile error. The as? gradual cast must produce a PrecisionViolation effect at runtime when the interval is not actually achievable.

4.5 Axiom U4 — Conditioning (Bayesian Update)
Statement. Observing evidence narrows the probability interval of a prior. The observe primitive implements Bayesian conditioning over the interval representation.
observe(event: Bool, prior: Uncertain<T>[lo, hi])
  : Uncertain<T>[lo', hi']
  where lo' >= lo   -- conditioning never widens intervals (only narrows or holds)
        hi' <= hi

-- Positive evidence (event = true): interval shifts upward and narrows
-- Negative evidence (event = false): interval shifts downward and narrows
-- No evidence (event is itself uncertain): partial conditioning applies

-- Formal semantics (interval arithmetic over the Giry monad):
-- P([lo, hi] | event=true)  = normalize([lo * P(event|T=true),
--                                         hi * P(event|T=true)])
-- P([lo, hi] | event=false) = normalize([lo * P(event|T=false),
--                                         hi * P(event|T=false)])
Usage pattern.
seedlet prior: Uncertain<Classification>[0.6, 0.85] = infer<Classification>(...);

-- Observe a corroborating signal
let posterior = observe(secondary_check_passed, prior);
-- posterior: Uncertain<Classification>[0.72, 0.91]  (interval narrowed upward)

-- Observe a contradicting signal
let posterior2 = observe(!tertiary_check_passed, prior);
-- posterior2: Uncertain<Classification>[0.45, 0.70]  (interval shifted downward)
Conformance test UNC-04. observe(true, u[lo, hi]) must produce an interval [lo', hi'] satisfying lo' >= lo. observe(false, u[lo, hi]) must produce an interval where hi' <= hi. Repeated conditioning must converge — observe(e, observe(e, u)) must not widen the interval relative to observe(e, u).

4.6 Axiom U5 — Confidence Gate Soundness (Three-Valued ?!)
Statement. The ?! operator is the primary decision point for acting on uncertain values. It is three-valued: it returns Some(T), None, or Ambiguous depending on how the interval relates to the threshold.
?!(u: Uncertain<T>[lo, hi], threshold: Float)
  : ThreeValued<T>

where:
  if hi  < threshold  =>  None
      -- Even the optimistic bound fails. The value is confidently wrong.
      -- Safe to treat as absence.

  if lo >= threshold  =>  Some(T)
      -- Even the pessimistic bound passes. The value is confidently right.
      -- Safe to act on.

  if lo < threshold <= hi  =>  Ambiguous(T, lo, hi)
      -- The interval straddles the threshold.
      -- Cannot make a binary decision safely.
      -- Must invoke an Ambiguous handler or escalate to human principal.
The Ambiguous handler.
seedmatch result ?!(threshold: 0.85) {
    Some(value)            => act_on(value),
    None                   => fallback_or_halt(),
    Ambiguous(val, lo, hi) => {
        -- Options: ask human, use lower threshold, request more evidence
        perform Effect::AskPrincipal(
            question: "Confidence [${lo}, ${hi}] straddles threshold 0.85. Proceed?",
            value: val,
        )
    }
}
Threshold registry. Each agent may declare a threshold registry associating semantic action names with thresholds. This improves auditability — thresholds are named, not magic numbers.
seedthreshold_registry {
    act_on_classification:  0.85,
    publish_to_federation:  0.90,
    trigger_irreversible:   0.95,
    report_to_human:        0.70,
}
Conformance test UNC-05. Given u[0.70, 0.80] and threshold 0.85, ?! must return None. Given u[0.90, 0.95] and threshold 0.85, ?! must return Some. Given u[0.80, 0.90] and threshold 0.85, ?! must return Ambiguous. The Ambiguous branch must carry the original value and the interval bounds.

4.7 Axiom U6 — Effect Handler Uncertainty Preservation
Statement. Algebraic effects that produce Uncertain<T> values must propagate the uncertainty interval into the resumed computation. Effect handlers cannot silently strip or inflate the interval.
-- If an effect operation has type:
effect Infer {
    classify: (Prompt) -> Uncertain<Classification>
}

-- Then any handler for Infer::classify MUST:
-- (a) resume with a value of type Uncertain<Classification>
-- (b) the resumed continuation receives the full interval
-- (c) the interval in the resumed computation must be <= the handler's interval
--     (handlers cannot claim more certainty than they actually achieve)

handler InferHandler handles Infer {
    on classify(prompt) {
        let result = call_model(prompt);  -- returns Uncertain<Classification>
        resume(result)                    -- propagates full interval
        -- FORBIDDEN: resume(result as Uncertain<Classification>[1.0, 1.0])
        --            compile error: fabricating certainty in effect handler
    }
}
Corollary. When multiple handlers are composed (handler stacking), each handler in the stack may only narrow the interval, never widen it. The outermost handler sees the most conservative (widest) interval; inner handlers may narrow it via evidence.
Conformance test UNC-06. A handler that attempts to resume with an interval narrower than the underlying computation produces a PrecisionViolation compile error. A handler stack must satisfy the monotonicity property: interval at outer boundary ≥ interval at inner boundary (in the precision order).

4.8 Uncertain<T> Standard Library Operations
The following operations are available on all Uncertain<T> values. They are defined in seed::uncertain.
seedimpl<T> Uncertain<T> {

    -- Constructor: explicit interval
    fn new(value: T, lo: Float, hi: Float) -> Uncertain<T>
        requires lo <= hi, lo >= 0.0, hi <= 1.0;

    -- Pure lift: certain value
    fn pure(value: T) -> Uncertain<T>  -- interval [1.0, 1.0]

    -- Monadic bind (implements U2)
    fn bind<U>(self, f: fn(T) -> Uncertain<U>) -> Uncertain<U>

    -- Map: transforms value, preserves interval
    fn map<U>(self, f: fn(T) -> U) -> Uncertain<U>

    -- Bayesian conditioning (implements U4)
    fn observe(self, evidence: Bool) -> Uncertain<T>
    fn observe_weighted(self, evidence: Bool, strength: Float) -> Uncertain<T>

    -- Three-valued gate (implements U5)
    fn gate(self, threshold: Float) -> ThreeValued<T>

    -- Interval accessors
    fn lo(self) -> Float
    fn hi(self) -> Float
    fn midpoint(self) -> Float      -- (lo + hi) / 2.0
    fn width(self) -> Float         -- hi - lo (measure of uncertainty)
    fn is_certain(self) -> Bool     -- lo == 1.0 && hi == 1.0
    fn is_impossible(self) -> Bool  -- hi == 0.0

    -- Interval operations
    fn intersect(self, other: Uncertain<T>) -> Option<Uncertain<T>>
        -- returns None if intervals are disjoint (contradiction detected)
    fn union(self, other: Uncertain<T>) -> Uncertain<T>
        -- returns the convex hull of both intervals (most conservative)
    fn meet(self, other: Uncertain<T>) -> Uncertain<T>
        -- alias for union in the precision lattice

    -- Serialization
    fn to_json(self) -> String   -- {"value": ..., "lo": ..., "hi": ...}

    -- Provenance-aware constructor (NEW v0.1.0)
    fn with_prov(value: T, lo: Float, hi: Float, prov: ProvenanceTag) -> Uncertain<T>
}

-- ThreeValued<T> (result type of ?! gate)
enum ThreeValued<T> {
    Some(T),
    None,
    Ambiguous(T, lo: Float, hi: Float),
}

4.9 Uncertain<T> in the Effect System
Uncertain<T> and algebraic effects are designed to compose cleanly. The following patterns are idiomatic:
Pattern 1 — Uncertain effect operation.
seedeffect Inference {
    run_model: (Prompt, ModelTier) -> Uncertain<InferenceResult>
}

fn classify(input: String) -> Uncertain<Classification>
    ! {Inference}
{
    let raw = perform Inference::run_model(
        prompt: build_prompt(input),
        model:  ModelTier::CloudMid,
    );  -- type: Uncertain<InferenceResult>

    raw.map(|r| r.classification)
       .observe(input.length() > 100)   -- longer input → slightly more reliable
}
Pattern 2 — Uncertain guard in handler.
seedhandler ProductionInference handles Inference {
    on run_model(prompt, tier) {
        let response  = call_llm(tier, prompt);
        let interval  = derive_confidence_interval(response.logits);
        let uncertain = Uncertain::new(response.result, interval.lo, interval.hi);
        resume(uncertain)
    }
}
Pattern 3 — Pipeline with uncertainty accumulation.
seedfn multi_step_pipeline(input: RawData) -> Uncertain<FinalReport>
    ! {Inference, Memory, Network}
{
    infer<ParsedData>(model: tier::local_slm, prompt: parse_prompt(input))
    |> (|parsed| infer<EnrichedData>(
                    model: tier::cloud_mid,
                    prompt: enrich_prompt(parsed)))
    |> (|enriched| infer<FinalReport>(
                    model: tier::frontier,
                    prompt: report_prompt(enriched)))
    -- Final interval is U2-propagated across all three infer calls
}

╔══════════════════════════════════════════════════════════════════════╗
║ §5 — COGNITIVE TYPES — Typed Inference as a Language Primitive     ║
╚══════════════════════════════════════════════════════════════════════╝
§COGNITIVE-TYPES
Cognitive Types elevate LLM inference from an opaque external call to a first-class typed language primitive. The key insight — drawn from the Turn language (Kizito 2026) — is that the output type of an inference call is known at compile time. The compiler can therefore derive a JSON Schema from the output type and have the VM validate model output before binding occurs. Schema validation failure is a first-class effect, not an unhandled exception.

5.1 The infer<T> Expression
seedinfer<T>(
    model:   model_selector,
    prompt:  expression,
    schema:  derive_schema<T>(),     -- optional: compiler inserts if omitted
    budget:  think_depth,            -- optional: default think::medium
    timeout: duration_literal,       -- optional: default 30s
    prov:    boolean,                -- optional: default matches agent provenance clause
) : Uncertain<T>
Semantics.

The compiler derives a JSON Schema from struct T at compile time. This schema is embedded in the compiled .aslb binary.
At runtime, the VM sends the prompt to the selected model tier.
The model's response is validated against the embedded JSON Schema before any binding occurs.
If validation passes, the VM computes a confidence interval from the model's output distribution (logit entropy, token probability) and wraps the result in Uncertain<T>[lo, hi].
If validation fails, the VM surfaces Effect::InferenceError(SchemaViolation). The agent's error handler decides whether to retry, degrade, or halt.
If the model times out, Effect::InferenceError(Timeout) is surfaced.


5.2 Schema Derivation Rules
The compiler derives JSON Schema from ASL struct definitions according to the following rules:
-- Primitive mappings
Bool     -> { "type": "boolean" }
i32/i64  -> { "type": "integer" }
f32/f64  -> { "type": "number" }
String   -> { "type": "string" }
Vec<T>   -> { "type": "array", "items": derive(T) }
Option<T> -> { "anyOf": [derive(T), { "type": "null" }] }

-- Struct T { field1: A, field2: B } ->
{
  "type": "object",
  "properties": {
    "field1": derive(A),
    "field2": derive(B)
  },
  "required": ["field1", "field2"]   -- all non-Option fields are required
}

-- Enum (unit variants only) ->
{ "type": "string", "enum": ["Variant1", "Variant2", ...] }

-- Enum (with payloads) ->
{ "oneOf": [
    { "type": "object", "properties": { "Variant1": derive(Payload1) }, "required": ["Variant1"] },
    ...
  ]
}

-- Float with bounds annotation (NEW v0.1.0)
-- #[bounds(0.0, 1.0)]
-- f64
-- -> { "type": "number", "minimum": 0.0, "maximum": 1.0 }
Bounds annotations. The #[bounds(lo, hi)] attribute on numeric fields generates minimum and maximum constraints in the derived schema. This prevents the model from emitting values outside the intended range without an explicit schema violation.
Nested structs. Schema derivation is recursive. Cyclic structs are detected at compile time and produce a compile error — recursive types are not permitted as infer<T> output types because JSON Schema does not handle cycles ergonomically.

5.3 Confidence Interval Derivation
The VM derives the probability interval [lo, hi] from the model's output using the following heuristics, listed in priority order. The first applicable method is used.
Method 1 — Logit entropy (preferred). If the model exposes log-probabilities for the generated tokens, the VM computes:
token_confidence_i = exp(logprob_i)
sequence_confidence = Π token_confidence_i   -- product over output tokens

lo = sequence_confidence * (1 - entropy_penalty)
hi = min(sequence_confidence + calibration_margin, 1.0)
Method 2 — Self-reported probability. If the model's output JSON includes a _confidence field (a convention the prompt template may request), the VM uses that value as the midpoint and applies a calibration margin:
mid = output._confidence
lo  = max(mid - calibration_margin, 0.0)
hi  = min(mid + calibration_margin, 1.0)
Method 3 — Sampling variance. For models that support multiple-sample inference, the VM runs n samples (default 3 for think::medium) and computes:
lo = min(sample_confidences)
hi = max(sample_confidences)
Method 4 — Conservative default. If no other method is applicable, the VM assigns [0.5, 0.8] — a conservative wide interval indicating that the result is plausible but not well-calibrated.
Calibration profiles. Each model in the routing policy may declare a calibration profile that adjusts the margin applied in Methods 1 and 2. Calibration profiles are stored in seed::routing::calibration and updated by the RL training subsystem based on observed accuracy vs. reported confidence.

5.4 Cognitive Type Definitions
The following built-in cognitive types are provided in seed::cognitive. They are common output schemas for standard agent tasks.
seed-- Classification result
struct Classification {
    label:      String,
    score:      #[bounds(0.0, 1.0)] f64,
    rationale:  String,
    alternatives: Vec<ClassificationAlternative>,
}

struct ClassificationAlternative {
    label: String,
    score: #[bounds(0.0, 1.0)] f64,
}

-- Extraction result
struct Extraction<T> {
    extracted: Vec<T>,
    completeness: #[bounds(0.0, 1.0)] f64,
    notes: Option<String>,
}

-- Decision result
struct Decision {
    choice:     String,
    rationale:  String,
    risks:      Vec<String>,
    confidence: #[bounds(0.0, 1.0)] f64,
}

-- Plan result
struct Plan {
    steps:      Vec<PlanStep>,
    horizon:    String,
    contingencies: Vec<String>,
}

struct PlanStep {
    ordinal:    u32,
    action:     String,
    expected_outcome: String,
    reversible: Bool,
}

-- Critique result
struct Critique {
    overall_quality: #[bounds(0.0, 1.0)] f64,
    strengths:  Vec<String>,
    weaknesses: Vec<String>,
    suggestions: Vec<String>,
}

-- Hypothesis result
struct Hypothesis {
    statement:  String,
    supporting_evidence: Vec<String>,
    contradicting_evidence: Vec<String>,
    testability: #[bounds(0.0, 1.0)] f64,
}

-- Summary result
struct Summary {
    text:         String,
    key_points:   Vec<String>,
    completeness: #[bounds(0.0, 1.0)] f64,
    faithfulness: #[bounds(0.0, 1.0)] f64,
}

5.5 Prompt Templates and Cognitive Types
Cognitive types are designed to be used with structured prompt templates. The prompt template is responsible for instructing the model to emit JSON conforming to the derived schema. The prompt_def block (§15) includes a schema field for this purpose.
seedprompt classify_sentiment {
    template: """
You are {agent_name}, a sentiment analysis agent.

Analyze the sentiment of the following text and respond ONLY with valid JSON
conforming to this schema:
{schema}

Text to analyze:
{input}

Respond with no preamble, no markdown, no explanation — only the JSON object.
""",
    schema_type: Classification,   -- compiler injects derive_schema<Classification>()
    optimization: {
        method: joint_tool_prompt,
        target_metrics: [schema_compliance_rate, classification_accuracy],
        reflection_enabled: true,
    },
    version: "2.1.0",
}

-- Usage:
let result: Uncertain<Classification> = infer<Classification>(
    model:  route::select(Complexity::Routine),
    prompt: classify_sentiment.render(
        agent_name: self.name,
        input:      user_text,
    ),
);

5.6 Inference Error Effects
All inference-related errors are surfaced as effects rather than panics. This allows the agent's handler hierarchy to implement appropriate recovery strategies.
seedeffect InferenceError {
    schema_violation: (expected: JsonSchema, got: String) -> InferenceErrorResponse
    timeout:          (duration: Duration, model: String)  -> InferenceErrorResponse
    rate_limit:       (retry_after: Duration)               -> InferenceErrorResponse
    context_overflow: (tokens_used: u64, limit: u64)        -> InferenceErrorResponse
    model_unavailable: (tier: ModelTier)                    -> InferenceErrorResponse
    hallucination_detected: (field: String, reason: String) -> InferenceErrorResponse
}

enum InferenceErrorResponse {
    Retry,
    RetryWithDegradedModel(ModelTier),
    UseFallback(FallbackValue),
    Escalate(PrincipalId),
    Halt,
}
Standard inference error handler.
seedhandler StandardInferenceHandler handles InferenceError {

    on schema_violation(expected, got) {
        log::warn("Schema violation in infer<T>: got {got}");
        resume(InferenceErrorResponse::RetryWithDegradedModel(ModelTier::CloudMid))
    }

    on timeout(duration, model) {
        log::warn("Model {model} timed out after {duration}");
        resume(InferenceErrorResponse::RetryWithDegradedModel(ModelTier::LocalSlm))
    }

    on rate_limit(retry_after) {
        sleep(retry_after);
        resume(InferenceErrorResponse::Retry)
    }

    on context_overflow(tokens_used, limit) {
        mem.compress_to_budget(target: limit * 0.8);
        resume(InferenceErrorResponse::Retry)
    }

    on model_unavailable(tier) {
        let fallback = ModelTier::downgrade(tier);
        resume(InferenceErrorResponse::RetryWithDegradedModel(fallback))
    }

    on hallucination_detected(field, reason) {
        log::error("Hallucination in field {field}: {reason}");
        resume(InferenceErrorResponse::Escalate(self.primary_principal()))
    }
}

5.7 Model Routing Policy
Model routing is declared in the agent's route clause and referenced by infer<T> expressions via route::select(task.complexity).
seedrouting_policy {
    tiers: {
        local_slm: {
            models:       ["qwen-7b", "llama-8b"],
            capabilities: [tool_calling, simple_reasoning, entity_extraction],
            max_tokens:   4096,
            calibration:  { margin: 0.08, method: entropy },
        },
        cloud_mid: {
            models:       ["claude-sonnet-4-6", "qwen-72b"],
            capabilities: [complex_reasoning, multi_step_planning, code_generation],
            max_tokens:   32768,
            calibration:  { margin: 0.05, method: self_reported },
        },
        frontier: {
            models:       ["claude-opus-4-6"],
            capabilities: [deep_research, creative_synthesis, long_horizon_planning],
            max_tokens:   200000,
            reserved_for: [think::exhaustive, major_evolution, adversarial_simulation],
            calibration:  { margin: 0.03, method: sampling, samples: 5 },
        },
    },

    route fn select_model(task: Task) -> ModelTier {
        match task.complexity {
            Complexity::Routine                              => local_slm,
            Complexity::Moderate                             => cloud_mid,
            Complexity::Complex if task.requires_deep_plan  => frontier,
            Complexity::Complex                              => cloud_mid,
        }
    },

    cost_policy: {
        maximize:  local_slm_usage(minimum: 0.80),
        budget_cap: monthly(usd: 500),
        alert_on:  budget_usage > 0.80,
    },
}
Model reference policy. Model names in the routing policy are configuration values, not language keywords. The [package] manifest declares a model-registry pointing to an external YAML file. This means model names can be updated without recompiling the agent — they are resolved at VM load time, not at compile time. This directly addresses the v14 critique that hardcoded model names create brittle specifications.
toml[model-registry]
source = "./models.yaml"   -- resolved at VM load time
fallback = "tier-defaults"

5.8 Test-Time Compute Profiles
Think profiles govern how much compute the agent invests per inference call. They interact with infer<T> via the budget parameter.
seedcompute_profile {
    default: { depth: medium, budget: 2000 },

    profiles: {
        quick:      { depth: shallow,    budget: 500   },
        thorough:   { depth: deep,       budget: 10000 },
        exhaustive: { depth: exhaustive, budget: 50000 },
    },

    exploration: {
        enabled:  true,
        strategy: chain_of_operations,
        chains: [
            [generation, verification, refinement],
            [hypothesis_generation, falsification, synthesis],
        ],
        asymmetric_pairing: {
            verifier:  easy_task_model,    -- local_slm for verification
            generator: hard_task_model,    -- frontier for generation
        },
    },

    complexity_analysis: {
        enabled:           true,
        compute_bounds:    per_task_type,
        optimization_target: minimize_tokens_for_accuracy(threshold: 0.95),
    },
}

5.9 Cognitive Type Conformance Tests
The following conformance tests apply to §COGNITIVE-TYPES. They are in test category COGTYPE in the ASL-CONF-15 suite.
COG-01: infer<T> with valid schema-conforming model output binds correctly.
COG-02: infer<T> with schema-violating output surfaces InferenceError::SchemaViolation.
COG-03: The derived JSON Schema from struct T matches the canonical derivation
        for all primitive and composite types defined in §5.2.
COG-04: Confidence interval lo <= hi for all infer<T> calls.
COG-05: Confidence interval is in [0.0, 1.0] — no overflow outside bounds.
COG-06: infer<T> timeout surfaces InferenceError::Timeout within
        tolerance of 100ms over the declared timeout value.
COG-07: InferenceError::ContextOverflow triggers mem.compress_to_budget() and retry.
COG-08: InferenceError::RateLimit triggers sleep(retry_after) and retry.
COG-09: Model routing selects the correct tier for each Complexity level
        as declared in the routing_policy.
COG-10: #[bounds(lo, hi)] attribute generates minimum/maximum in derived JSON Schema.
COG-11: Cyclic struct used as infer<T> output type produces compile error.
COG-12: Calibration profile updates are reflected in subsequent infer<T> interval derivation.

---END AGENT-SEED v0.1.0.0 PART 2 OF 6---

End of Part 2. Covers §TYPE-SYSTEM (kinding, type judgment, ownership, effect rows, capability rules, trust lattice rules, session duality, memory record type, schema-constrained memory), §UNCERTAIN-AXIOMS (U1 through U6 in full normative form with conformance tests), and §COGNITIVE-TYPES (infer<T> expression, schema derivation rules, confidence interval derivation methods, standard cognitive type library, inference error effects, model routing policy, test-time compute profiles, and conformance tests).

Agent Seed Language — Specification v0.1.0.0
Part 3 of 6 — Memory Architecture

---BEGIN AGENT-SEED v0.1.0.0 PART 3 OF 6---

╔══════════════════════════════════════════════════════════════════════╗
║ §6 — MEMORY ARCHITECTURE — Complete Specification                  ║
╚══════════════════════════════════════════════════════════════════════╝
§MEMORY-ARCHITECTURE
ASL v0.1.0 defines a unified multi-tier, multi-graph memory system. Memory is not a passive store — it is an active subsystem with its own lifecycle, consistency model, governance rules, and provenance tracking. Every agent has access to the full memory hierarchy; individual layers are enabled or disabled per agent via the memory clause in the agent definition.

6.1 Memory Hierarchy Overview
┌─────────────────────────────────────────────────────────────────┐
│                    MEMORY HIERARCHY v0.1.0                          │
├─────────────────────────────────────────────────────────────────┤
│  L0  Working Memory        — session-scoped, volatile           │
│  L1  Episodic Memory       — event log, temporal chain          │
│  L2  Semantic Memory       — consolidated facts, ontology-linked │
│  L3  Procedural Memory     — skills, workflows, tool patterns   │
│  L4  Prospective Memory    — scheduled intentions, deadlines    │
│  L5  Federated Memory      — shared cross-agent fact space      │
│  L6  Identity Memory       — anchor, lineage, drift log         │
│  L7  Provenance Index      — audit log, Merkle-proofed (NEW v0.1.0)│
├─────────────────────────────────────────────────────────────────┤
│  GRAPH LAYERS (orthogonal to tier)                              │
│  G1  Semantic Graph        — concept relationships              │
│  G2  Temporal Chain        — event ordering, causal links       │
│  G3  Causal Graph          — cause-effect relationships         │
│  G4  Entity Graph          — entity co-occurrence               │
│  G5  Associative Graph     — spreading activation paths         │
├─────────────────────────────────────────────────────────────────┤
│  GOVERNANCE LAYER (cross-cutting)                               │
│  Tri-path router           — read/write/invalidate routing      │
│  MESI coherency protocol   — multi-agent cache coherency        │
│  Merkle integrity          — tamper-evidence for all writes     │
│  Schema validator          — type-safe store and retrieve       │
│  Anti-echo filter          — deduplication on semantic sim      │
│  Provenance tagger         — automatic lineage on all writes    │
└─────────────────────────────────────────────────────────────────┘

6.2 Complete Memory Declaration
seedmemory_hierarchy {

    -- ─────────────────────────────────────────────────────────────
    -- L0: Working Memory
    -- ─────────────────────────────────────────────────────────────
    working: {
        schema:       WorkingMemoryItem,
        capacity:     1024,
        graphs:       [],
        mutability:   mutable,
        scope:        session,
        coherency:    strong,
        cache_tier:   hot,
        provenance:   false,    -- volatile; not audited
        overflow:     {
            strategy: evict_lru,
            on_full:  compress_oldest_to_episodic,
        },
    },

    -- ─────────────────────────────────────────────────────────────
    -- L1: Episodic Memory
    -- ─────────────────────────────────────────────────────────────
    episodic: {
        schema:       EpisodicEntry,
        capacity:     50000,
        graphs:       [temporal, causal],
        mutability:   append_only,
        scope:        persistent,
        coherency:    strong,
        cache_tier:   warm,
        provenance:   true,
        decay: ebbinghaus(
            base_strength: 1.0,
            decay_rate:    0.1,
        ),
        consolidation: {
            trigger: reinforcement_count >= 3,
            action:  episodic_to_semantic,
        },
        merkle: true,
    },

    -- ─────────────────────────────────────────────────────────────
    -- L2: Semantic Memory
    -- ─────────────────────────────────────────────────────────────
    semantic: {
        schema:       SemanticEntry,
        capacity:     500000,
        graphs:       [semantic, entity, associative],
        mutability:   mutable,
        scope:        persistent,
        coherency:    eventual,
        cache_tier:   warm,
        provenance:   true,
        decay: exponential(
            half_life: 90d,
        ),
        ontology_link: true,
        merkle:        true,
        anti_echo: {
            enabled:   true,
            threshold: 0.92,
            action:    merge_higher_confidence,
        },
    },

    -- ─────────────────────────────────────────────────────────────
    -- L3: Procedural Memory
    -- ─────────────────────────────────────────────────────────────
    procedural: {
        schema:       ProceduralEntry,
        capacity:     10000,
        graphs:       [semantic, causal],
        mutability:   mutable,
        scope:        persistent,
        coherency:    strong,
        cache_tier:   warm,
        provenance:   true,
        versioning:   true,    -- procedures are versioned; old versions retained
        merkle:       true,
    },

    -- ─────────────────────────────────────────────────────────────
    -- L4: Prospective Memory
    -- ─────────────────────────────────────────────────────────────
    prospective: {
        schema:       ProspectiveEntry,
        capacity:     1000,
        graphs:       [temporal],
        mutability:   mutable,
        scope:        persistent,
        coherency:    strong,
        cache_tier:   hot,
        provenance:   true,
        scheduler: {
            check_interval: 1min,
            on_due:         surface_as_heartbeat_signal,
            on_overdue:     escalate_to_principal,
        },
    },

    -- ─────────────────────────────────────────────────────────────
    -- L5: Federated Memory
    -- ─────────────────────────────────────────────────────────────
    federated: {
        schema:       FederatedFact,
        capacity:     unbounded,
        graphs:       [semantic, entity, associative],
        mutability:   append_only,
        scope:        federated,
        coherency:    eventual,
        cache_tier:   cold,
        provenance:   true,
        crdt:         true,
        conflict_resolution: vector_clock_lww,
        merkle:       true,
    },

    -- ─────────────────────────────────────────────────────────────
    -- L6: Identity Memory
    -- ─────────────────────────────────────────────────────────────
    identity_mem: {
        schema:       IdentityRecord,
        capacity:     1,           -- one identity record per agent
        graphs:       [],
        mutability:   append_only, -- identity evolves but history is retained
        scope:        persistent,
        coherency:    strong,
        cache_tier:   hot,
        provenance:   true,
        merkle:       true,
        protected:    true,        -- cannot be modified by self-evolution
    },

    -- ─────────────────────────────────────────────────────────────
    -- L7: Provenance Index (NEW v0.1.0)
    -- ─────────────────────────────────────────────────────────────
    provenance_index: {
        schema:       ProvenanceRecord,
        capacity:     unbounded,
        graphs:       [causal, temporal],
        mutability:   append_only,
        scope:        persistent,
        coherency:    strong,
        cache_tier:   cold,
        provenance:   false,   -- the provenance index is self-anchored
        merkle:       true,
        protected:    true,    -- cannot be modified by self-evolution
        export: {
            format:   json_ld,
            signing:  ed25519,
            command:  "seed audit --export-provenance <session_id>",
        },
    },
}

6.3 Memory Schema Definitions
seed-- L0: Working Memory Item
struct WorkingMemoryItem {
    key:       String,
    value:     JsonValue,
    ttl:       Option<Duration>,
    created:   Timestamp,
    last_read: Timestamp,
}

-- L1: Episodic Entry
struct EpisodicEntry {
    id:           EpisodeId,
    timestamp:    Timestamp,
    event_type:   EpisodeKind,
    content:      String,
    context:      JsonValue,
    valence:      #[bounds(-1.0, 1.0)] f64,    -- emotional/relevance weight
    arousal:      #[bounds(0.0, 1.0)]  f64,    -- salience weight
    causal_prev:  Option<EpisodeId>,
    causal_next:  Option<EpisodeId>,
    temporal_pos: u64,                          -- monotonic sequence position
    prov:         ProvenanceTag,
    confidence:   Uncertain<Float>,
    reinforcement_count: u32,
    consolidated: Bool,
}

enum EpisodeKind {
    UserInteraction,
    ToolCall,
    InferenceResult,
    MemoryConsolidation,
    HeartbeatDecision,
    DreamPhase,
    EvolutionEvent,
    CapabilityGrant,        -- NEW v0.1.0
    SessionEvent,           -- NEW v0.1.0
    ProvenanceAudit,        -- NEW v0.1.0
    IdentityAttestation,    -- NEW v0.1.0
}

-- L2: Semantic Entry
struct SemanticEntry {
    id:          ConceptId,
    concept:     String,
    category:    SemanticCategory,
    attributes:  HashMap<String, JsonValue>,
    confidence:  Uncertain<Float>,
    sources:     Vec<EpisodeId>,
    ontology:    Option<OntologyRef>,
    created:     Timestamp,
    updated:     Timestamp,
    decay_score: f64,
    prov:        ProvenanceTag,
    graph_edges: Vec<GraphEdge>,
}

struct GraphEdge {
    target:    ConceptId,
    relation:  RelationType,
    weight:    #[bounds(0.0, 1.0)] f64,
    prov:      ProvenanceTag,
}

enum SemanticCategory {
    Factual,
    Procedural,
    Relational,
    Causal,
    Temporal,
    Normative,
    Hypothetical,
}

-- L3: Procedural Entry
struct ProceduralEntry {
    id:          ProcedureId,
    name:        String,
    version:     SemanticVersion,
    steps:       Vec<ProcedureStep>,
    preconditions: Vec<Condition>,
    postconditions: Vec<Condition>,
    success_rate: Uncertain<Float>,
    avg_duration: Duration,
    last_used:   Timestamp,
    prov:        ProvenanceTag,
}

struct ProcedureStep {
    ordinal:     u32,
    action:      String,
    tool:        Option<ToolId>,
    expected:    String,
    reversible:  Bool,
    on_failure:  FailurePolicy,
}

-- L4: Prospective Entry
struct ProspectiveEntry {
    id:          IntentionId,
    description: String,
    due_at:      Timestamp,
    priority:    Priority,
    principal:   PrincipalId,
    status:      IntentionStatus,
    created:     Timestamp,
    prov:        ProvenanceTag,
}

enum IntentionStatus {
    Pending,
    Active,
    Completed,
    Overdue,
    Cancelled,
}

-- L5: Federated Fact
struct FederatedFact {
    id:          FactId,
    subject:     EntityId,
    predicate:   String,
    object:      JsonValue,
    confidence:  Uncertain<Float>,
    scope:       FederationScope,
    vector_clock: VectorClock,
    prov:        ProvenanceTag,
    attestation: Option<DelegationToken>,  -- NEW v0.1.0: who published this fact
}

-- L6: Identity Record
struct IdentityRecord {
    agent_id:        AgentId,
    name:            String,
    binary_hash:     MerkleHash,           -- NEW v0.1.0: content hash of .aslb
    did:             DID,                  -- NEW v0.1.0
    attestation:     Option<ZkVmProof>,    -- NEW v0.1.0
    created:         Timestamp,
    drift_log:       Vec<DriftEntry>,
    resilience_level: ResilienceLevel,
    version:         SemanticVersion,
    prov:            ProvenanceTag,
}

struct DriftEntry {
    at:         Timestamp,
    metric:     String,
    delta:      f64,
    remediation: String,
}

-- L7: Provenance Record (NEW v0.1.0)
struct ProvenanceRecord {
    id:          ProvenanceId,
    session:     SessionId,
    agent:       AgentId,
    action:      ActionKind,
    inputs:      Vec<ProvenanceId>,    -- parent provenance IDs
    output:      JsonValue,
    model:       Option<String>,
    confidence:  Option<Uncertain<Float>>,
    timestamp:   Timestamp,
    merkle_hash: MerkleHash,
    merkle_proof: MerkleProof,
    signature:   Ed25519Signature,
}

enum ActionKind {
    Inference,
    MemoryWrite,
    MemoryRead,
    EffectPerform,
    SessionSend,
    SessionRecv,
    CapabilityGrant,
    CapabilityRevoke,
    AmendmentApply,
    DreamPhase,
    HeartbeatDecision,
}

6.4 Memory Operations API
All memory operations are surfaced through the mem handle available in every agent context.
seed-- ─────────────────────────────────────────────────────────────────────
-- WRITE OPERATIONS
-- ─────────────────────────────────────────────────────────────────────

-- Store with automatic provenance tagging
mem.store(key: String, value: T) -> Result<MemoryRecord<T>, MemoryError>

-- Store with explicit provenance
mem.store_with_prov(key: String, value: T, prov: ProvenanceTag)
    -> Result<MemoryRecord<T>, MemoryError>

-- Store to specific layer
mem.store_to(layer: MemoryLayer, key: String, value: T)
    -> Result<MemoryRecord<T>, MemoryError>

-- Append to episodic log
mem.log_episode(entry: EpisodicEntry) -> Result<EpisodeId, MemoryError>

-- Publish to federated scope
mem.publish_fact(fact: FederatedFact) -> Result<FactId, MemoryError>
@@ mem.publish_fact(fact)  -- shorthand: the @@ operator

-- ─────────────────────────────────────────────────────────────────────
-- READ OPERATIONS
-- ─────────────────────────────────────────────────────────────────────

-- Retrieve by key (returns full MemoryRecord when provenance enabled)
mem.get(key: String) -> Option<MemoryRecord<T>>

-- Retrieve value only (strips MemoryRecord wrapper)
mem.get_value(key: String) -> Option<T>

-- Retrieve with confidence gate
mem.get_confident(key: String, threshold: Float) -> ThreeValued<T>

-- Graph traversal
mem.traverse(
    start: ConceptId,
    graph: GraphKind,
    strategy: TraversalStrategy,
    depth: u32,
) -> Vec<GraphNode>

enum TraversalStrategy {
    BreadthFirst,
    DepthFirst,
    SpreadingActivation(decay: Float),
    CausalChain,
    TemporalWindow(Duration),
}

-- Semantic similarity search
mem.search(
    query: String,
    layer: MemoryLayer,
    top_k: u32,
    threshold: Float,
) -> Vec<SearchResult<T>>

struct SearchResult<T> {
    item:       MemoryRecord<T>,
    similarity: f64,
    path:       Option<Vec<GraphEdge>>,
}

-- Episodic range query
mem.episodes(
    from: Timestamp,
    to:   Timestamp,
    kind: Option<EpisodeKind>,
) -> Vec<EpisodicEntry>

-- Causal chain query
mem.causal_chain(
    from: EpisodeId,
    direction: CausalDirection,
    depth: u32,
) -> Vec<EpisodicEntry>

enum CausalDirection { Forward, Backward, Both }

-- Federated query
mem.query_federation(
    predicate: String,
    scope: FederationScope,
    confidence_threshold: Float,
) -> Vec<FederatedFact>

-- ─────────────────────────────────────────────────────────────────────
-- INVALIDATION AND MAINTENANCE
-- ─────────────────────────────────────────────────────────────────────

-- Invalidate a key (marks as stale; does not delete — audit trail preserved)
mem.invalidate(key: String) -> Result<(), MemoryError>

-- Delete (requires capability: cap::memory_delete)
mem.delete(key: String) requires cap::memory_delete
    -> Result<(), MemoryError>

-- Compress working memory to budget
mem.compress_to_budget(target: u64) -> CompressionReport

-- Trigger manual consolidation
mem.consolidate(from: EpisodeId, to: EpisodeId)
    -> Result<Vec<ConceptId>, MemoryError>

-- Reinforce episodic entry (increments reinforcement_count)
mem.reinforce(episode: EpisodeId, strength: f64)
    -> Result<(), MemoryError>

-- ─────────────────────────────────────────────────────────────────────
-- PROVENANCE OPERATIONS (NEW v0.1.0)
-- ─────────────────────────────────────────────────────────────────────

-- Get full provenance chain for a value
mem.provenance_chain(id: ProvenanceId) -> Vec<ProvenanceRecord>

-- Verify Merkle integrity of a record
mem.verify_merkle(record: MemoryRecord<T>) -> MerkleVerificationResult

-- Export provenance as signed JSON-LD
mem.export_provenance(session: SessionId) -> SignedJsonLd

-- Anti-echo check: would this store create a duplicate?
mem.check_echo(value: T, layer: MemoryLayer) -> EchoCheckResult

enum EchoCheckResult {
    Unique,                                   -- no duplicate found
    Duplicate(existing_id: MemoryId),         -- exact duplicate
    Similar(existing_id: MemoryId, sim: f64), -- semantically similar
}

╔══════════════════════════════════════════════════════════════════════╗
║ §7 — MEMORY GOVERNANCE — Tri-Path Router and Policy Engine         ║
╚══════════════════════════════════════════════════════════════════════╝
§MEMORY-GOVERNANCE
Memory governance defines how the tri-path router dispatches reads, writes, and invalidations across the memory hierarchy. All governance rules are declared in the agent's memory_governance clause and enforced at runtime by the VM's memory policy engine.

7.1 Tri-Path Router
Every memory operation is dispatched by the tri-path router. The router evaluates operation priority, cache state, and coherency requirements to select the appropriate path.
seedmemory_governance {

    -- ─────────────────────────────────────────────────────────────
    -- READ PATH
    -- ─────────────────────────────────────────────────────────────
    read_path: {
        L0_hit:  return_immediately,           -- working memory: always check first
        L1_hit:  return_with_staleness_check,  -- episodic: check temporal validity
        L2_hit:  return_with_decay_check,      -- semantic: apply decay function
        L3_hit:  return_with_version_check,    -- procedural: check version currency
        miss:    {
            strategy: cascade_to_next_layer,
            on_all_miss: [
                surface_Effect_MemoryMiss,
                offer_infer_as_fallback,       -- infer<T> as cache-miss handler
            ],
        },
        prefetch: {
            enabled:   true,
            strategy:  spreading_activation,
            lookahead: 3,
        },
    },

    -- ─────────────────────────────────────────────────────────────
    -- WRITE PATH
    -- ─────────────────────────────────────────────────────────────
    write_path: {
        policy:     write_through_to_persistent,
        anti_echo:  check_before_write,
        validation: schema_check_before_commit,
        merkle:     update_tree_on_commit,
        provenance: tag_on_commit,
        broadcast: {
            condition: scope == federated,
            method:    stigmergy_publish,
        },
        on_schema_fail: surface_Effect_SchemaViolation,
        on_capacity_full: {
            L0: evict_lru_to_episodic,
            L1: compress_oldest_with_decay,
            L2: prune_lowest_decay_score,
        },
    },

    -- ─────────────────────────────────────────────────────────────
    -- INVALIDATION PATH
    -- ─────────────────────────────────────────────────────────────
    invalidation_path: {
        strategy:   mark_stale_not_delete,
        propagate:  to_all_graph_edges,
        broadcast:  on_federated_scope,
        audit:      log_to_provenance_index,   -- NEW v0.1.0
        on_cascade: max_depth(5),
    },
}

7.2 Memory Effects
Memory operations that require cross-cutting behavior are surfaced as algebraic effects. This allows handlers to implement custom retry, fallback, and escalation strategies.
seedeffect MemoryOps {
    miss:             (key: String, layer: MemoryLayer) -> MissResponse
    schema_violation: (key: String, expected: JsonSchema,
                       got: JsonValue)                   -> ViolationResponse
    capacity_full:    (layer: MemoryLayer, item_size: u64) -> CapacityResponse
    coherency_conflict: (key: String, local: JsonValue,
                         remote: JsonValue,
                         clock: VectorClock)             -> ConflictResponse
    decay_eviction:   (key: String, decay_score: f64)    -> EvictionResponse
    echo_detected:    (existing: MemoryId, incoming: T,
                       similarity: f64)                  -> EchoResponse
    merkle_fail:      (key: String, expected: MerkleHash,
                       got: MerkleHash)                  -> MerkleResponse
}

enum MissResponse       { Infer, UseFallback(T), Halt }
enum ViolationResponse  { Reject, Coerce, Halt }
enum CapacityResponse   { EvictLru, Compress, Reject }
enum ConflictResponse   { TakeLocal, TakeRemote, Merge, Escalate }
enum EvictionResponse   { Allow, Preserve, Compress }
enum EchoResponse       { Reject, MergeHigherConfidence, AllowDuplicate }
enum MerkleResponse     { Halt, Quarantine, Escalate }

7.3 Governance Policy Examples
seedhandler StandardMemoryGovernance handles MemoryOps {

    on miss(key, layer) {
        log::debug("Memory miss: {key} in {layer}");
        resume(MissResponse::Infer)
        -- causes the heartbeat observe phase to call infer<T>
        -- and cache the result in the appropriate layer
    }

    on schema_violation(key, expected, got) {
        log::error("Schema violation on write to {key}");
        mem.log_episode(EpisodicEntry {
            event_type: EpisodeKind::ProvenanceAudit,
            content:    "Schema violation: key={key}",
            ..defaults()
        });
        resume(ViolationResponse::Reject)
    }

    on coherency_conflict(key, local, remote, clock) {
        let merged = crdt::merge(local, remote, clock);
        mem.store(key, merged);
        resume(ConflictResponse::Merge)
    }

    on echo_detected(existing, incoming, similarity) {
        if incoming.confidence.hi > mem.get_value(existing)?.confidence.hi {
            mem.invalidate(existing);
            resume(EchoResponse::MergeHigherConfidence)
        } else {
            resume(EchoResponse::Reject)
        }
    }

    on merkle_fail(key, expected, got) {
        log::error("Merkle integrity failure on {key}: expected {expected}, got {got}");
        perform Effect::AlertPrincipal(
            severity: Severity::Critical,
            message:  "Memory tamper detected: key={key}",
        );
        resume(MerkleResponse::Quarantine)
    }
}

╔══════════════════════════════════════════════════════════════════════╗
║ §8 — MEMORY CONSISTENCY — MESI Protocol and CRDT Federation        ║
╚══════════════════════════════════════════════════════════════════════╝
§MEMORY-CONSISTENCY
Multi-agent deployments require a principled approach to memory consistency. ASL v0.1.0 uses the MESI cache coherency protocol for strongly-consistent layers and CRDT-based eventual consistency for federated layers.

8.1 MESI Protocol
MESI (Modified, Exclusive, Shared, Invalid) is applied to memory layers with coherency: mesi. In a multi-agent system where multiple agents hold references to the same memory region, MESI ensures that writes are visible to all agents and that stale reads are prevented.
mesi_states:
  Modified  (M) — agent holds the only copy; it has been written;
                  other agents' copies are Invalid. On eviction,
                  the VM writes back to persistent store.

  Exclusive (E) — agent holds the only copy; it has NOT been written;
                  the copy matches persistent store. On write,
                  transitions to Modified without broadcast.

  Shared    (S) — multiple agents hold read-only copies;
                  all match persistent store.

  Invalid   (I) — agent's copy is stale; next access triggers fetch
                  from persistent store.

mesi_transitions:
  Read hit    (M/E/S) => serve from cache
  Read miss   (I)     => fetch from persistent, transition to S or E
  Write hit   (M)     => update cache; already Modified
  Write hit   (E)     => update cache; transition E->M
  Write hit   (S)     => update cache; broadcast invalidate to all Shared;
                         transition S->M
  Write miss  (I)     => fetch from persistent; broadcast invalidate;
                         write; transition I->M
MESI conformance test MESI-01. An agent that holds a Shared copy must observe an Invalid transition within one heartbeat tick after a remote agent commits a write to the same key.

8.2 CRDT Federation
Federated memory (L5) uses Conflict-free Replicated Data Types (CRDTs) to achieve eventual consistency without coordination overhead. Each federated fact carries a vector clock for causal ordering.
seedmemory_consistency {

    crdt_types: {
        -- G-Counter: monotonically increasing counter
        -- Used for: reinforcement counts, observation tallies
        g_counter: { op: increment_only, merge: max_per_agent }

        -- LWW-Register: Last-Write-Wins register with vector clock
        -- Used for: factual knowledge, preference values
        lww_register: {
            op:           overwrite,
            merge:        vector_clock_lww,
            tie_break:    agent_id_lexicographic,
        }

        -- OR-Set: Observed-Remove Set
        -- Used for: entity sets, tag collections
        or_set: {
            op:   add_with_unique_tag | remove_by_tag,
            merge: union_of_observed_adds,
        }

        -- MVR: Multi-Value Register (retains concurrent writes)
        -- Used for: hypothesis sets, competing interpretations
        mvr: {
            op:    overwrite,
            merge: retain_all_concurrent,
            resolve: surface_as_Ambiguous,
        }
    },

    vector_clock: {
        implementation: hybrid_logical_clock,
        -- Hybrid Logical Clock (HLC) combines physical time with
        -- logical causality. Monotonically non-decreasing.
        sync_on:    every_mesh_send,
        precision:  millisecond,
    },

    conflict_resolution: {
        default:    vector_clock_lww,
        on_tie:     agent_id_lexicographic,
        on_mvr:     surface_as_Uncertain_Ambiguous,
        escalation: {
            condition:  unresolvable_after(3),
            action:     surface_MemoryOps_CoherencyConflict,
        },
    },

    gossip: {
        enabled:    true,
        protocol:   anti_entropy,
        interval:   30s,
        fanout:     3,          -- each agent gossips with 3 random peers
        digest:     merkle_diff,-- only transmit differing subtrees
    },
}

8.3 Merkle Integrity
Every persistent memory layer maintains a Merkle tree over its contents. The tree is updated on every write and verified on every read in strict mode.
seedmerkle_policy {
    algorithm:   SHA3_256,
    tree_type:   sparse_merkle,

    update_on:   [write, consolidation, dream_prune],
    verify_on:   [read_strict, session_start, dream_review],

    root_storage: {
        local:   agent_identity_layer,
        remote:  federation_publish,   -- root hash broadcast to peers
    },

    on_fail: {
        action:   quarantine_and_alert,
        alert_to: all_principals,
        halt:     if severity == Critical,
    },

    audit_export: {
        command: "seed audit --verify-merkle <agent_id>",
        format:  json_ld,
        sign:    ed25519,
    },
}

╔══════════════════════════════════════════════════════════════════════╗
║ §9 — DUAL-PROCESS MEMORY — System 1 and System 2 Integration       ║
╚══════════════════════════════════════════════════════════════════════╝
§DUAL-PROCESS-MEMORY
Dual-process memory implements the System 1 / System 2 distinction from cognitive science. System 1 operations use fast pattern-matching against a cached working context. System 2 operations use full multi-graph traversal with inference validation. The gating function selects between them based on task complexity, time pressure, and confidence threshold.

9.1 Dual-Process Configuration
seeddual_process_memory {

    system1: {
        strategy:     pattern_match,
        source:       working_memory,
        max_latency:  50ms,
        confidence:   [0.85, 1.0],   -- only high-confidence results
        cache:        hot_path_lru(capacity: 256),
        fallthrough:  system2,        -- on miss or low confidence
    },

    system2: {
        strategy:     full_graph_traversal,
        sources:      [semantic, episodic, procedural],
        max_latency:  2000ms,
        confidence:   [0.60, 1.0],   -- accepts wider interval
        graph_ops:    [
            spreading_activation(decay: 0.85),
            causal_chain_follow(depth: 5),
            temporal_window(span: 7d),
        ],
        fallthrough:  infer_as_last_resort,
    },

    gating: {
        function:       complexity_and_confidence,
        use_system1_if: {
            task_complexity: <= Routine,
            working_mem_hit: true,
            time_pressure:   high,
        },
        use_system2_if: {
            task_complexity: >= Moderate,
            system1_confidence: < 0.85,
            novel_context:   true,
        },
        monitoring: {
            log_gate_decisions:  true,
            alert_on_s2_rate: > 0.40,
            -- High S2 rate may indicate working memory is undersized
            -- or task complexity distribution has shifted
        },
    },
}

╔══════════════════════════════════════════════════════════════════════╗
║ §10 — EPISODIC RECONSTRUCTION — Master-Assistant Architecture      ║
╚══════════════════════════════════════════════════════════════════════╝
§EPISODIC-RECONSTRUCTION
Episodic reconstruction is the process by which the agent re-assembles a coherent episodic context from distributed memory layers at the start of a session or after a long idle period. It uses a master-assistant two-agent pattern: the master agent directs retrieval and reconstruction; the assistant agent handles parallel retrieval across layers.

10.1 Episodic Reconstruction Configuration
seedepisodic_recon {

    master_agent: {
        role:        context_director,
        strategy:    adaptive,           -- adjusts based on available episodes
        entry_points: [
            identity_anchor,
            last_dream_journal,
            active_prospective_intentions,
        ],
    },

    assistant_agent: {
        role:        parallel_retriever,
        concurrency: 4,
        layers:      [episodic, semantic, procedural, prospective],
        timeout:     500ms,
    },

    reconstruction_phases: [
        {
            name:    identity_verify,
            action:  load_identity_anchor,
            verify:  cryptographic_hash_match,    -- NEW v0.1.0
            on_fail: halt_and_alert,
        },
        {
            name:    temporal_context,
            action:  load_last_N_episodes(N: 20),
            timeout: 100ms,
        },
        {
            name:    semantic_priming,
            action:  spreading_activation_from_recent_episodes,
            depth:   3,
            timeout: 200ms,
        },
        {
            name:    prospective_check,
            action:  load_due_or_overdue_intentions,
            timeout: 50ms,
        },
        {
            name:    provenance_anchor,            -- NEW v0.1.0
            action:  load_last_provenance_root,
            verify:  merkle_root_match,
            on_fail: alert_and_continue,           -- non-blocking for reconstruction
        },
    ],

    forward_path: {
        direction: old_to_new,
        purpose:   build_temporal_narrative,
        indexing:  causal_chain_links,
    },

    backward_path: {
        direction: new_to_old,
        purpose:   identify_active_threads,
        indexing:  relevance_weighted_bfs,
    },

    probe: {
        method:    targeted_retrieval,
        signal:    session_intent,
        top_k:     10,
        threshold: 0.70,
    },

    output: WorkingMemoryContext,
}

╔══════════════════════════════════════════════════════════════════════╗
║ §11 — MEMORY CYCLE — Heartbeat-Integrated Memory Lifecycle         ║
╚══════════════════════════════════════════════════════════════════════╝
§MEMORY-CYCLE
The memory cycle governs how memory is updated during and after the heartbeat loop. It defines the integration between the observe-decide-act-log-update phases of the heartbeat and the memory subsystem.

11.1 Memory Cycle Configuration
seedmemory_cycle {

    observe_phase: {
        -- What the agent reads from memory during heartbeat observe
        load: [
            working_memory.recent(N: 10),
            prospective.due_within(2min),
            semantic.high_salience(top_k: 5),
        ],
        prefetch: spreading_activation(from: last_decision, depth: 2),
    },

    act_phase: {
        -- What the agent writes during heartbeat act
        write_on_tool_call: [
            episodic.append(EpisodeKind::ToolCall),
            working_memory.update(tool_result),
        ],
        write_on_decision: [
            episodic.append(EpisodeKind::HeartbeatDecision),
            working_memory.update(decision_context),
        ],
    },

    log_phase: {
        -- What the agent logs after each heartbeat tick
        always_log: [
            tick_timestamp,
            phase_durations,
            active_effects,
            memory_cache_stats,
        ],
        log_on_change: [
            working_memory_delta,
            confidence_interval_shifts,
        ],
        log_to_provenance: true,   -- NEW v0.1.0: each tick is a provenance entry
    },

    update_phase: {
        -- Memory maintenance triggered by update_memory heartbeat phase
        decay_step: {
            enabled:   true,
            layers:    [episodic, semantic],
            step_size: per_tick,
        },
        consolidation_check: {
            enabled:   true,
            condition: reinforcement_count >= 3,
            action:    episodic_to_semantic,
        },
        anti_echo_scan: {
            enabled:   true,
            frequency: every_10_ticks,
            threshold: 0.92,
        },
        prospective_check: {
            enabled:   true,
            action:    surface_overdue_as_signal,
        },
    },
}

╔══════════════════════════════════════════════════════════════════════╗
║ §12 — ADAPTIVE MEMORY — Structure-Selector and FluxMem             ║
╚══════════════════════════════════════════════════════════════════════╝
§ADAPTIVE-MEMORY
Adaptive memory allows the agent to select the optimal memory representation strategy based on task complexity and available compute. The structure-selector evaluates current task demands and chooses between FluxMem (lightweight probabilistic) and full multi-graph traversal.

12.1 Adaptive Memory Configuration
seedadaptive_memory {

    structure_selector: {
        inputs: [
            task_complexity,
            available_compute,
            time_pressure,
            memory_fill_pct,
        ],

        strategies: {
            fluxmem: {
                condition: {
                    task_complexity:   <= Routine,
                    time_pressure:     high,
                    available_compute: low,
                },
                impl: {
                    representation:  probabilistic_sketch,
                    index:           locality_sensitive_hash,
                    max_items:       1000,
                    error_rate:      0.05,
                },
            },

            full_graph: {
                condition: {
                    task_complexity:   >= Complex,
                    available_compute: medium_or_high,
                },
                impl: {
                    representation:  multi_graph,
                    index:           hnsw,
                    max_items:       unbounded,
                    error_rate:      0.0,
                },
            },

            hybrid: {
                condition: default,   -- used when neither above applies
                impl: {
                    hot_layer:    fluxmem,
                    cold_layer:   full_graph,
                    promotion:    on_repeated_access(threshold: 3),
                    demotion:     on_decay_below(score: 0.3),
                },
            },
        },

        switch_policy: {
            hysteresis:  0.1,     -- prevents oscillation at boundary
            min_stable:  5min,    -- minimum time before switching strategy
            log_switch:  true,
        },
    },

    fluxmem: {
        sketch_type:   count_min_sketch,
        hash_functions: 5,
        width:          2048,
        depth:          4,
        update:         incremental,
        query:          approximate_top_k(k: 20, error: 0.05),
    },
}

╔══════════════════════════════════════════════════════════════════════╗
║ §13 — EVOLUTIONARY MEMORY — PRISM Subsystem                        ║
╚══════════════════════════════════════════════════════════════════════╝
§EVOLUTIONARY-MEMORY
The PRISM (Progressive Reasoning and Integrated Synthesis Memory) subsystem manages memory during and after agent self-evolution events. It ensures that memory representations are updated coherently when the agent's schema or reasoning patterns change.

13.1 PRISM Configuration
seedevolutionary_memory {

    prism: {

        encoder: {
            strategy:    progressive_compression,
            target_size: 0.20,     -- compress to 20% of original
            fidelity:    semantic, -- preserve semantic content over verbatim
            method:      hierarchical_summarization,
        },

        indexer: {
            algorithm:   hnsw,                  -- Hierarchical NSW graph
            dimensions:  1536,
            ef_construct: 200,
            m:           16,
            distance:    cosine,
        },

        retriever: {
            strategy:     hybrid,
            dense:        vector_similarity(top_k: 20),
            sparse:       bm25(top_k: 20),
            reranker:     cross_encoder,
            final_top_k:  10,
        },

        consolidator: {
            trigger:      evolution_event,
            action:       re_embed_all_semantic,
            batch_size:   1000,
            verify:       merkle_root_unchanged_for_immutable_layers,
        },

        pruner: {
            strategy:     importance_weighted,
            importance:   access_frequency * recency_weight * confidence.hi,
            target_pct:   0.30,   -- prune bottom 30% by importance
            never_prune:  [identity_mem, provenance_index],  -- protected layers
        },

        evolver: {
            trigger:      amendment_approved,
            action:       migrate_schema(old: T_prev, new: T_next),
            migration:    {
                strategy:     additive_first,
                -- Additive migrations (new fields) applied first.
                -- Destructive migrations (removed fields) require
                -- explicit archive step before deletion.
                archive:      true,
                rollback_on:  migration_error,
            },
        },

        verifier: {
            post_evolution_checks: [
                merkle_root_valid,
                schema_compliance_100pct,
                no_orphaned_graph_edges,
                provenance_chain_intact,    -- NEW v0.1.0
                identity_anchor_unchanged,
            ],
            on_fail: rollback_evolution,
        },

        governor: {
            max_prism_duration: 10min,
            priority:           background,
            on_timeout:         defer_to_next_dream,
            resource_cap: {
                cpu:    0.25,   -- max 25% CPU during PRISM
                memory: 0.20,   -- max 20% memory during PRISM
            },
        },
    },
}

╔══════════════════════════════════════════════════════════════════════╗
║ §14 — DREAM CYCLE — Formal Specification with Invariants           ║
╚══════════════════════════════════════════════════════════════════════╝
§DREAM-CYCLE
The dream cycle is the agent's offline memory maintenance and integration process. It runs when the agent is idle, at session end, or on a declared schedule. In v0.1.0, the dream cycle has a formal pre/post-condition specification that is verified by the VM before and after each dream.

14.1 Dream Cycle Configuration
seeddream {
    schedule:     daily,
    trigger_time: "02:00 UTC",
    max_duration: 4h,

    phases: [review, resolve, consolidate, compress, prune, write_journal],

    journal: {
        path:    "./dream_journals/",
        format:  markdown,
        sign:    ed25519,          -- NEW v0.1.0: journals are signed
        retain:  365d,
    },

    invariants: {                  -- NEW v0.1.0: formal pre/post-conditions
        post_merkle_verify:   true,
        post_safety_check:    true,
        idempotent:           true,
        max_confidence_drift: 0.05,
    },
}

14.2 Dream Pre-Conditions
The following conditions must hold before the VM initiates a dream cycle. If any pre-condition is not met, the dream is deferred and a DreamDeferred effect is surfaced.
PRE-DREAM CONDITIONS:

P1: episodic_fill_pct >= 0.80
    -- Dream is triggered by memory pressure.
    -- Below 80% fill, dream is deferred (not enough to consolidate).
    -- Note: schedule-triggered dreams bypass P1 but still check P2-P6.

P2: active_mesh_sessions == 0
    -- No mesh session may be in flight during a dream.
    -- All ~> and <~ operations must have completed.

P3: effect_queue.pending == 0
    -- The effect queue must be empty. All outstanding effects resolved.

P4: corrigibility_heads.all_satisfied == true
    -- All five corrigibility utility heads (U1-U5) must be satisfied
    -- before dream begins. A dream cannot run under a corrigibility violation.

P5: no_active_amendments == true
    -- No pending amendment may be in the simulate or vote phases.
    -- Amendments and dreams do not run concurrently.

P6: principal_reachable OR dead_switch.not_triggered
    -- If the dead-man's-switch has fired, no dream is permitted.
    -- Dreams require a living principal relationship.

14.3 Dream Phases — Detailed Specification
PHASE 1: REVIEW
───────────────
Purpose:    Survey all memory layers; identify consolidation candidates.
Reads:      All episodic entries since last dream.
            All semantic entries with decay_score < 0.40.
            All procedural entries not accessed in > 30d.
Produces:   CandidateSet — entries to process in subsequent phases.
Duration:   max 20% of dream budget.

PHASE 2: RESOLVE
────────────────
Purpose:    Resolve conflicts and contradictions identified during review.
Operations: For each pair (e1, e2) in CandidateSet where e1 contradicts e2:
              - Compute credibility(e1) and credibility(e2)
              - credibility = confidence.hi * recency_weight * source_quality
              - If credibility(e1) > credibility(e2): archive e2, reinforce e1
              - If credibility(e2) > credibility(e1): archive e1, reinforce e2
              - If |credibility(e1) - credibility(e2)| < 0.1: surface as MVR
                (Multi-Value Register — retain both, mark as contested)
Duration:   max 25% of dream budget.

PHASE 3: CONSOLIDATE
────────────────────
Purpose:    Promote episodic entries to semantic memory.
Condition:  entry.reinforcement_count >= 3
            AND entry.confidence.lo >= 0.70
            AND NOT entry.consolidated
Operations: For each qualifying episodic entry:
              1. Derive semantic concept from episodic content
              2. Check anti-echo: if similar concept exists, merge
              3. If no similar concept: create new SemanticEntry
              4. Link via graph edges (causal, temporal, associative)
              5. Tag with provenance chain from episodic entry (NEW v0.1.0)
              6. Mark episodic entry as consolidated
Duration:   max 30% of dream budget.

PHASE 4: COMPRESS
─────────────────
Purpose:    Reduce episodic memory footprint while preserving causal chain.
Strategy:   Hierarchical summarization of consolidated episodes.
            Non-consolidated episodes below decay threshold are archived.
Operations: For each consolidated episode:
              1. Generate summary via infer<Summary>
              2. Preserve: timestamp, causal_prev, causal_next, provenance
              3. Replace: verbatim content with summary
              4. Update Merkle tree
Constraint: Causal chain must remain intact after compression.
            If compression would break a causal link, skip that episode.
Duration:   max 15% of dream budget.

PHASE 5: PRUNE
──────────────
Purpose:    Remove entries below minimum viability threshold.
Threshold:  decay_score < 0.05 AND reinforcement_count == 0
            AND confidence.hi < 0.30
Protected:  identity_mem, provenance_index, any entry marked protected
Operations: Move qualifying entries to archive (not delete).
            Archive entries are excluded from normal retrieval
            but remain accessible via mem.archive_query().
            Log each pruned entry to provenance_index (NEW v0.1.0).
Duration:   max 5% of dream budget.

PHASE 6: WRITE_JOURNAL
──────────────────────
Purpose:    Produce a human-readable dream summary and update identity.
Operations: 1. Infer a narrative summary of the dream cycle
            2. Write to journal file with ed25519 signature (NEW v0.1.0)
            3. Update identity_mem with new binary_hash (if evolution occurred)
            4. Broadcast Merkle root to federation peers
            5. Surface DreamComplete effect to heartbeat loop
Duration:   max 5% of dream budget.

14.4 Dream Post-Conditions
The following conditions are verified by the VM after every dream cycle completes. If any post-condition fails, the dream is rolled back and DreamFailed is surfaced.
POST-DREAM CONDITIONS:

Q1: merkle_root_valid == true
    -- The Merkle tree is consistent with all memory contents.
    -- Computed by re-hashing all persistent layers.

Q2: schema_violations == 0
    -- Every entry in every layer satisfies its declared schema.
    -- Verified by the schema validator against the compiled JSON Schema.

Q3: safety_contracts.all_satisfied == true
    -- Every declared safety contract holds against the post-dream memory state.
    -- A dream cannot produce a memory state that violates a safety contract.

Q4: corrigibility_heads.all_satisfied == true
    -- All five corrigibility utility heads remain satisfied post-dream.
    -- Drift in identity or goal structure during a dream is a critical failure.

Q5: causal_chain_intact == true
    -- The temporal chain of episodic entries has no broken links.
    -- For all e.causal_prev: the referenced episode exists and is accessible.

Q6: append_only_layers_unchanged == true
    -- Layers declared as append_only (identity_mem, provenance_index, episodic)
    -- contain a superset of their pre-dream contents.
    -- No entry may have been removed from these layers.

Q7: confidence_drift <= max_confidence_drift (default 0.05)
    -- The average confidence interval midpoint across all semantic entries
    -- has not shifted by more than 5% from its pre-dream value.
    -- This prevents dreams from systematically inflating or deflating confidence.

Q8: provenance_chain_intact == true      -- NEW v0.1.0
    -- Every ProvenanceRecord written during the dream has a valid
    -- Merkle proof linking it to the root.

Q9: dream_idempotent: dream(dream(state)) ≡ dream(state)
    -- Running the dream twice on the same state produces the same result.
    -- Verified by the conformance suite (DRM-05).
    -- The VM does not verify this on every production dream
    -- (cost-prohibitive); the conformance suite verifies it statically.

Q10: journal_written_and_signed == true  -- NEW v0.1.0
     -- The dream journal file was written and signed with ed25519.
     -- Verification: seed audit --verify-journal <session_id>

14.5 Dream Conformance Tests
DRM-01: Dream triggers when episodic_fill_pct >= 0.80.
DRM-02: A second dream trigger while one is running is queued, not nested.
DRM-03: Post-dream: all memory entries satisfy their declared schema.
DRM-04: Post-dream: all safety contracts are satisfied.
DRM-05: Dream idempotency: dream(dream(state)) produces same Merkle root as dream(state).
DRM-06: No entry emerges from dream with confidence interval wider than pre-dream value.
DRM-07: Causal chain is intact after compress phase.
DRM-08: Append-only layers contain superset of pre-dream contents.
DRM-09: Journal file is written, signed, and ed25519-verifiable.
DRM-10: Provenance records for all dream operations are in provenance_index.
DRM-11: Confidence drift across all semantic entries is <= max_confidence_drift.
DRM-12: Dream pre-conditions P2 (no active sessions) and P3 (empty effect queue)
         are enforced — a dream attempted while conditions fail is deferred.

14.6 Memory Conformance Tests (Full Suite)
-- Working Memory
MEM-01: Working memory miss triggers cascade to next layer.
MEM-02: Working memory overflow triggers evict_lru_to_episodic.
MEM-03: Working memory TTL expiry removes item within one tick.

-- Episodic Memory
MEM-04: Episodic append is strictly ordered by temporal_pos.
MEM-05: Ebbinghaus decay reduces decay_score at each update_phase tick.
MEM-06: Reinforcement_count >= 3 triggers consolidation to semantic.
MEM-07: Causal links (causal_prev, causal_next) are bidirectionally consistent.

-- Semantic Memory
MEM-08: Anti-echo filter rejects duplicates above similarity threshold.
MEM-09: Anti-echo merge selects entry with higher confidence.hi.
MEM-10: Exponential decay halves decay_score after declared half_life.

-- Procedural Memory
MEM-11: Versioned procedures retain all previous versions.
MEM-12: Procedure success_rate is updated after each execution.

-- Federated Memory
MEM-13: CRDT merge is commutative: merge(A, B) = merge(B, A).
MEM-14: CRDT merge is associative: merge(merge(A,B),C) = merge(A,merge(B,C)).
MEM-15: Vector clock advances monotonically.
MEM-16: Gossip protocol converges within 3 rounds for 10-agent federation.

-- Merkle Integrity
MEM-17: Write updates Merkle root within same transaction.
MEM-18: Merkle root broadcast to federation after dream phase 6.
MEM-19: Merkle verification failure triggers quarantine_and_alert.

-- Provenance Index (NEW v0.1.0)
MEM-20: Every infer<T> call produces a ProvenanceRecord in provenance_index.
MEM-21: Every mem.store() produces a ProvenanceRecord when provenance: true.
MEM-22: Provenance chain is Merkle-verifiable end-to-end.
MEM-23: Export produces signed JSON-LD document verifiable with ed25519.
MEM-24: Pruned entries are logged to provenance_index before archive.

-- MESI Protocol
MESI-01: Shared copy transitions to Invalid within one tick after remote write.
MESI-02: Write to Shared copy broadcasts Invalidate to all holders.
MESI-03: Modified copy is written back to persistent store on eviction.

---END AGENT-SEED v0.1.0.0 PART 3 OF 6---

End of Part 3. Covers the complete memory system: §MEMORY-ARCHITECTURE (all seven layers, schemas, operations API), §MEMORY-GOVERNANCE (tri-path router, memory effects, governance handlers), §MEMORY-CONSISTENCY (MESI protocol, CRDT federation, Merkle integrity), §DUAL-PROCESS-MEMORY (System 1/System 2 gating), §EPISODIC-RECONSTRUCTION (master-assistant architecture), §MEMORY-CYCLE (heartbeat integration), §ADAPTIVE-MEMORY (structure-selector and FluxMem), §EVOLUTIONARY-MEMORY (PRISM subsystem), and §DREAM-CYCLE (complete with formal pre/post-conditions, phase specifications, and all twelve conformance tests).

---BEGIN AGENT-SEED v0.1.0.0 PART 4 OF 6---

╔══════════════════════════════════════════════════════════════════════╗
║ §15 — THE HEARTBEAT — Autonomous Execution Pulse ║
║ Grounded in: McCann 2026 (Effect‑Transparent Governance, ║
║ Certified Purity); OTel GenAI Conventions v1.39.0 ║
╚══════════════════════════════════════════════════════════════════════╝

§HEARTBEAT

The heartbeat is the agent’s autonomic OODA loop—the bounded fixpoint
that drives all agent behaviour. Every conforming agent MUST implement
the heartbeat. The seedvm‑5.0 runtime guarantees tick isolation, phase
atomicity, and budget enforcement via hardware timer interrupts.

McCann (2026) proves that bounded fixpoint loops under the GovernanceAlgebra
(G, ⊗, 1_governance, safety, transparency, properness) are semantically
transparent: on all permitted executions, the governed interpretation is
observationally equivalent to the ungoverned interpretation modulo
governance‑only events. The heartbeat loop is the concrete instantiation
of this bounded fixpoint—each tick is a governed iteration.

7.1 Heartbeat Configuration

text
heartbeat_clause ::= "heartbeat" "{"
    "enabled"               ":" boolean_literal ","
    "interval"              ":" duration_literal ","
    "idle_threshold"        ":" duration_literal ","
    "blocking_budget"       ":" duration_literal
    ["," "background_on_timeout" ":" boolean_literal]
    ["," "phases"           ":" "[" heartbeat_phase
                                  {"," heartbeat_phase} "]"]
    ["," "sleep_tool"       ":" sleep_tool_config]
    ["," "notifications"    ":" notification_config]
    ["," "subscriptions"    ":" subscription_config]
    ["," "job_control"      ":" job_control_config]
    ["," "observability"    ":" heartbeat_obs_config]
    ["," "governance"       ":" governance_binding]
"}"

heartbeat_phase ::= "observe" | "decide" | "act_or_sleep"
                  | "log" | "update_memory"
Field	Default	Semantics
interval	30s	Nominal period between heartbeat ticks
idle_threshold	15s	Inactivity duration before yielding to sleep
blocking_budget	15s	Maximum wall‑clock per tick; hardware‑enforced
background_on_timeout	true	Over‑budget ticks checkpoint and background
7.1.1 Governance Binding (McCann 2026)

text
governance_binding ::= "{" 
    "operator" ":" "G" ","          -- governance operator
    "proofs"   ":" "enable" ","     -- machine‑checked governance properties
    "level"    ":" governance_level
"}"

governance_level ::= "L0" | "L1" | "L2" | "L3" | "L4"
Per McCann’s Certified Purity architecture (arXiv:2605.01031), governance
level L3 requires cryptographic attestation of the governance binary;
L4 requires full mutual authentication. The governance operator G
mediates all effectful directives: memory access, external calls, and
LLM queries, enforcing effect‑level governance while preserving semantic
transparency to internal computation.

7.2 Core Loop Phases (Normative)

Observe. Gather pending inputs: user messages, subscription events,
push notifications, file‑system watcher events, timer expiries, federated
fact updates. Store raw observations in working‑memory layer
observations. The VM guarantees observe() runs atomically with
respect to all other agent code. No other phase begins until observe()
completes.

Decide. Load observation snapshot; retrieve active goals from
prospective memory (L4); evaluate contradictions via conflict detection;
run dual‑process retrieval (System‑1 first, System‑2 on elevated stakes);
select ONE concrete task; record DecisionRecord in decision_log with
provenance BEFORE execution (append‑first semantics for auditability).

Act‑or‑Sleep. If task chosen: verify capability tokens required by
task; execute under guardrail and constraint monitoring; log result. If
no task and elapsed_idle ≥ idle_threshold: invoke sleep tool (§7.3).

Log. Append immutable entry to decision_log: tick_number, timestamp,
phase_durations, task_attempted/effects_performed, memory_delta_size,
active_contracts_snapshot, provenance chain root. Merkle‑proofed before
commit. No agent may modify a log entry after writing.

Update Memory. Consolidate observations; apply decay functions;
check consolidation triggers; commit pending writes; schedule dream if
pressure thresholds met; update Merkle roots; anti‑echo scan every ten
ticks.

7.3 Sleep Tool

text
sleep_tool_config ::= "{"
    "enabled"              ":" boolean_literal ","
    "wake_conditions"      ":" "[" wake_condition {"," wake_condition} "]" ","
    "prompt_cache_expiry"  ":" duration_literal ","
    "max_sleep_duration"   ":" duration_literal
"}"

wake_condition ::= "new_user_message" | "scheduled_task_due"
                 | "push_notification_received" | "file_system_change"
                 | "git_event" | "memory_contradiction_detected"
                 | "federation_update" | "mesh_cmb_received"
                 | identifier
On sleep entry: LLM prompt cache flagged for expiry; memory, identity,
and subscriptions remain active; VM suspends heartbeat tick loop. If
max_sleep_duration elapses without wake, agent wakes, runs one
observe‑decide cycle, and re‑enters sleep if still idle.

On wake: agent resumes heartbeat from observe phase; all state preserved;
SleepWakeEvent logged.

7.4 Notifications and Subscriptions

text
notification_config ::= "{"
    "enabled"  ":" boolean_literal ","
    "backends" ":" "[" notification_backend {"," notification_backend} "]" ","
    "triggers" ":" "[" notification_trigger {"," notification_trigger} "]"
"}"

notification_backend ::= "push" | "email" | "telegram" | "discord"
                       | "slack" | "http" "(" url ")"

notification_trigger ::= "task_completed" | "error_needs_attention"
                       | "drift_warning" | "memory_consolidation_complete"
                       | "evolution_amendment_approved"
                       | "dead_switch_activated"
                       | "corrigibility_head_violation"

subscription_config ::= "{"
    "enabled"     ":" boolean_literal ","
    "sources"     ":" "[" subscription_source {"," subscription_source} "]" ","
    "auto_review" ":" boolean_literal
"}"

subscription_source ::= "github" "(" repo_spec ")"
                      | "gitlab" "(" repo_spec ")"
                      | "rss"    "(" url ")"
                      | "custom" "(" identifier ")"
7.5 Job Control

Available at S1+: fg %N, bg %N, jobs, disown %N, wait %N.

text
job_control_config ::= "{"
    "enabled"            ":" boolean_literal ","
    "default_background" ":" boolean_literal ","
    "max_jobs"           ":" integer_literal
"}"
7.6 Heartbeat Observability

When emit_tick_spans: true, the VM emits an OpenTelemetry span
gen_ai.heartbeat.tick per tick, conforming to the OTel GenAI Semantic
Conventions v1.39.0 (stabilised 2026). Required attributes: gen_ai. agent.id, gen_ai.agent.name, gen_ai.agent.heartbeat_tick,
seed.agent.session_id.

7.7 Heartbeat Conformance Tests

text
HB-01:  Tick isolation — no interleaving of phases across ticks.
         Verifies: phase atomicity per McCann governance Algebra Axiom 2.
HB-02:  Budget enforcement — tick exceeding blocking_budget suspended
         and HeartbeatBudgetExceeded event logged within 100ms of overrun.
HB-03:  Sleep tool — agent wakes on any configured wake condition and
         resumes at observe phase within one interval of condition firing.
HB-04:  Log immutability — decision_log entries cannot be modified or
         deleted by agent code; Merkle verification confirms after write.
HB-05:  Notification delivery queued within one tick of trigger.
HB-06:  Subscription event ingested and delivered to observe phase
         within two tick intervals.
HB-07:  Governance binding — at L3, governance operator mediates all
         effectful directives; no bypass possible.
╔══════════════════════════════════════════════════════════════════════╗
║ §16 — FEDERATION & STIGMERGY — Federated Knowledge Fabric ║
║ Grounded in: Paredes García 2026 (Ledger‑State Stigmergy, ║
║ arXiv:2604.03997); Kleppmann & Beresford 2016 ║
╚══════════════════════════════════════════════════════════════════════╝

§FEDERATION

Federated knowledge sharing enables agents to collectively build,
maintain, and evolve a shared knowledge fabric without central
coordination. The substrate is grounded in stigmergy—agents coordinate
indirectly by modifying a shared environment—operationalised via typed
facts, vector clocks, scope enforcement, and CRDT‑backed replication.

Paredes García (2026) provides the formal state‑transition framework,
identifying three base on‑chain coordination patterns (State‑Flag,
Event‑Signal, Threshold‑Trigger) plus a Commit‑Reveal sequencing overlay.
ASL v0.1.0 instantiates all three patterns at the language level.

8.1 Fact Schema

The unit of federation is a typed, immutable fact:

text
FederatedFact ::= {
    entity:              URI,
    relation:            URI,
    value:               TypedPayload,
    source:              AgentID,
    timestamp:           HybridLogicalClock,
    confidence:          Uncertain<Float>,
    scope:               Scope,
    vector_clock:        VectorClock,
    immutability:        true,
    contradiction_policy: surface_as_conflict,
    hash:                MerkleHash,
    attestation:         Option<DelegationToken>,
}

TypedPayload ::= "{"
    "type"  ":" JsonSchema ","
    "value" ":" JsonValue
"}"

Scope ::= "public" | "private" "(" principal_list ")"
        | "domain" "(" domain_id ")"
All facts are immutable once published. Retraction is performed by
publishing a new fact with relation = "retracts" and value pointing
to the retracted fact’s hash. Every recipient validates the
TypedPayload against its JSON Schema before acceptance.

8.2 Three Coordination Patterns (Paredes García 2026)

text
coordination_pattern ::=
    "StateFlag"      "(" flag_id "," condition ")"
    -- Agent reads flag state; acts when condition met
  | "EventSignal"    "(" event_type "," filter ")"
    -- Agent reacts to emitted event
  | "ThresholdTrigger" "(" metric "," threshold "," window ")"
    -- Agent acts when metric crosses threshold within window
    ["with" "CommitReveal" "(" commit_phase "," reveal_phase ")"]
    -- Sequencing overlay: commit intent, then reveal action
Example: ThresholdTrigger(episodic_fill_pct, 0.80, 5min) triggers
dream consolidation when the episodic buffer exceeds 80% for five
consecutive minutes—the same pressure trigger used by the dream cycle.

8.3 Federation Configuration

text
federation_clause ::= "federation" "{"
    "handshake"      ":" handshake_config ","
    "replication"    ":" replication_config ","
    "fact_lifecycle" ":" fact_lifecycle_config
    ["," "crdt_layer_map" ":" "{" crdt_mapping {"," crdt_mapping} "}"]
"}"

handshake_config ::= "{"
    "signing_key"        ":" "Ed25519" ","
    "scope_declaration"  ":" boolean_literal ","
    "peer_discovery"     ":" "[" peer_discovery_method
                               {"," peer_discovery_method} "]"
"}"

peer_discovery_method ::= "mdns" | "static_config" | "registry" "(" url ")"

replication_config ::= "{"
    "strategy"           ":" "eventual_consistency" ","
    "conflict_detection" ":" "vector_clocks" ","
    "scope_enforcement"  ":" "strict" ","
    "crdt_merge"         ":" boolean_literal ","
    "sync_interval"      ":" duration_literal
"}"

fact_lifecycle_config ::= "{"
    "write_endpoint"    ":" url_template ","
    "read_endpoint"     ":" url_template ","
    "expire_policy"     ":" expire_policy ","
    "decay_function"    ":" decay_function ","
    "reassert_on_read"  ":" boolean_literal
"}"
8.4 CRDT Type Assignment Per Layer

text
crdt_type ::= "LWW-Register" | "OR-Set" | "2P-Set" | "RGA"
            | "PN-Counter" | "Max-Register" | "Kalman-Merge"
Default CRDT assignments per layer kind:

Layer Kind	Default CRDT	Rationale
fact_store	LWW-Register	Timestamp ordering sufficient
entity_graph	OR-Set	Add‑wins semantics for membership
causal_graph	2P-Set	Causal monotonicity enforced
temporal_chain	RGA	Order preservation across replicas
confidence_field	Max-Register	Higher confidence wins
counter_field	PN-Counter	Always convergent
semantic_index	LWW-Register	Embedding updates: last‑write‑wins
utility_score	Kalman-Merge	Noise‑filtered convergence
8.5 Hybrid Logical Clock

text
HLC ::= (physical_time: u64, logical_counter: u32)
HLC combines physical time with a logical counter to provide causally
consistent snapshots without synchronised clocks.

8.6 Anti‑Entropy Protocol

Peers exchange Merkle‑tree diffs at sync_interval using delta‑CRDTs
(Almeida et al. 2016): only the differing subtrees are transmitted.

8.7 Federation Conformance Tests

text
FED-01:  Publish fact with valid schema succeeds; returns FactId with
         non‑zero confidence.
FED-02:  Publish fact with schema‑violating payload surfaces
         Effect::SchemaViolation.
FED-03:  Scope enforcement: facts outside permitted scopes not returned.
FED-04:  Two agents with conflicting OR‑Set facts converge after
         anti‑entropy sync for all CRDT types in crdt_type_map.
FED-05:  Vector clock conflict detection correctly identifies concurrent
         updates.
FED-06:  Three‑way merge with common ancestor produces correct result.
FED-07:  Subscription callback fires within one heartbeat tick of
         remote publish.
FED-08:  Network partition: agents resume sync after reconnection with
         no data loss (delta‑CRDT exchange).
FED-09:  Immutable facts cannot be modified; retraction via `retracts`
         relation works correctly and is traceable.
FED-10:  State‑Flag coordination pattern: agent reads flag, acts when
         condition met, transitions documented.
╔══════════════════════════════════════════════════════════════════════╗
║ §17 — COGNITIVE MESH — MMP Multi‑Agent Semantic Infrastructure ║
║ Grounded in: Xu 2026 (MMP, arXiv:2604.19540); ║
║ Xu 2026 (SVAF, arXiv:2604.03955) ║
╚══════════════════════════════════════════════════════════════════════╝

§COGNITIVE-MESH

The cognitive mesh is a protocol layer for cross‑session agent‑to‑agent
cognitive collaboration. It is specified by the Mesh Memory Protocol
(MMP v0.2.3, CC BY 4.0), which solves three problems together:

(P1) Each agent decides field‑by‑field what to accept from peers.
(P2) Every claim is traceable to source; returning claims are
recognised as echoes of the receiver’s own prior thinking.
(P3) Memory that survives session restarts is relevant because of
how it was stored, not how it is retrieved.

Four composable primitives work together: CAT7 (schema), SVAF (acceptance),
inter‑agent lineage (traceability), and remix (storage). MMP is shipped
and running in production across three reference deployments.

9.1 CAT7 Cognitive Memory Block Schema

Every inter‑agent signal is a Cognitive Memory Block (CMB) conforming to
the fixed seven‑field CAT7 schema:

text
CMB ::= {
    agent_id:      AgentID,           -- sender identity
    timestamp:     ISO8601,           -- wall‑clock time
    role:          Role,              -- researcher | validator | auditor | …
    content_hash:  SHA256,            -- hash of payload
    parent_hashes: [SHA256],          -- lineage references
    anchors:       RoleIndexedAnchors,-- per‑role anchor points
    payload:       CognitiveContent,  -- typed semantic content
}

RoleIndexedAnchors ::= "{"
    role ":" anchor {"," role ":" anchor}
"}"

Role ::= "researcher" | "validator" | "auditor" | "coordinator" | identifier
anchor ::= SHA256
CognitiveContent ::= "{"
    "focus"        ":" String ","
    "issue"        ":" String ","
    "intent"       ":" String ","
    "motivation"   ":" String ","
    "commitment"   ":" String ","
    "perspective"  ":" String ","
    "mood"         ":" String
"}"
9.2 SVAF Acceptance Framework

SVAF (Symbolic‑Vector Attention Fusion) is the content‑evaluation half
of MMP’s coupling engine. It decomposes each inter‑agent signal into
seven typed semantic fields, evaluates each through a learned fusion
gate, and produces one of four outcomes:

text
SVAFOutcome ::= "redundant" | "aligned" | "guarded" | "rejected"
The four‑outcome band‑pass model solves both selectivity and redundancy.
Trained on 237K samples from 273 narrative scenarios, SVAF achieves 78.7%
three‑class accuracy. The fusion gate independently discovers a cross‑
domain relevance hierarchy: mood emerges as the highest‑weight field
by epoch 1, consistent with independent evidence that LLM emotion
representations are structurally embedded along valence‑arousal axes.

text
svaf_config ::= "{"
    "by_role" ":" "{" role_config {"," role_config} "}" ","
    "negotiation" ":" negotiation_config
"}"

role_config ::= role ":" "{"
    "accept_if"     ":" predicate ","
    "evidence_fields" ":" "[" field_name {"," field_name} "]"
"}"

negotiation_config ::= "{"
    "when"   ":" "neither_accept_nor_reject" ","
    "action" ":" "send_counter_proposal" ","
    "timeout" ":" duration_literal
"}"
9.3 Inter‑Agent Lineage

Every CMB carries parent_hashes forming a Merkle DAG of the conversation
history. The mesh runtime enforces:

Echo detection: duplicate content_hash rejected; CMB with hash
already present in lineage silently dropped.

Provenance completeness: missing parent CMBs can be fetched via
content‑addressed retrieval.

Ancestors: the full chain of prior CMBs that led to this one,
enabling any agent to reconstruct the thread from its own anchors.

9.4 Remix Storage

Upon accepting a CMB, the agent does NOT store the raw peer signal:

text
remix ::= "{"
    "on_accept"     ":" "evaluate_and_restructure" ","
    "on_reject"     ":" "log_rejection_reason" ","
    "on_negotiate"  ":" "send_counter_proposal"
"}"
The evaluate_and_restructure step cross‑references the CMB payload
against the agent’s existing memory, generates a new MemoryRecord
representing the agent’s own synthesis, and stores it with provenance
pointing back to the original CMB hash. This realises P3: each agent
maintains its own perspective while preserving traceability.

9.5 Mesh Operations

text
-- Send a CMB to peers
mesh_send_expression ::= expression "~>" peer_expression
                        ["as" session_role]

-- Receive a CMB
mesh_recv_expression ::= expression "<~" peer_expression

-- Typed round‑trip call
mesh_call_expression ::= "mesh_call" "<" session_type_ref ">"
                          "(" peer_expression "," expression ")" "?"

-- Peer expression with DID verification
peer_expression ::= expression
                  | "peer.did" "(" did_literal ")"
All mesh sends at S1+ require a session type annotation. Untyped mesh
sends compile only at S0 with capability token cap::untyped_mesh.

9.6 Mesh Conformance Tests

text
MSH-01:  CMB with valid CAT7 schema accepted by recipient with matching
         role; correct SVAF outcome computed.
MSH-02:  CMB with insufficient evidence rejected by researcher role
         (evidence_strength < 0.8); outcome: guarded or rejected.
MSH-03:  Duplicate content_hash detected as echo and silently dropped.
MSH-04:  Lineage chain verifiable: all parent_hashes resolvable in mesh
         store; missing ancestors fetchable.
MSH-05:  Remix storage: accepted CMB produces stored MemoryRecord with
         provenance referencing original CMB hash.
MSH-06:  Session‑typed mesh_send with mismatched payload type is a
         compile error at S1+.
MSH-07:  mesh_call<RequestResponse<Q,A>> returns Uncertain<A> with
         confidence interval derived from remote agent's output.
MSH-08:  mesh_call timeout surfaces MeshTimeout effect within tolerance
         of declared timeout.
MSH-09:  Capability‑gated send without required capability token is a
         compile error (S1+) or runtime CapabilityDenied (S0).
╔══════════════════════════════════════════════════════════════════════╗
║ §18 — A2A BINDING — Native Agent‑to‑Agent Protocol (v1.0) ║
║ Grounded in: A2A Protocol v1.0 (Linux Foundation, March 2026); ║
║ a2a.proto normative source; JWS RFC 7515; ║
║ JSON Canonicalization RFC 8785 ║
╚══════════════════════════════════════════════════════════════════════╝

§A2A-BINDING

The A2A (Agent‑to‑Agent) Protocol v1.0 (March 12, 2026) is the first
open standard for agent‑to‑agent communication, now governed by the Linux
Foundation’s Agentic AI Foundation. Every ASL agent with federation: true or mesh: true MUST publish a valid A2A Agent Card.

The normative source of truth is specification/a2a.proto. ASL v0.1.0
implements the full A2A v1.0 protocol natively: all eleven RPC methods,
the nine‑state task state machine, Agent Card signature verification
via JWS + JCS, and all three protocol bindings (JSON‑RPC, gRPC, REST).

10.1 Agent Card Declaration

text
a2a_card_def ::= "a2a_card" "{"
    "name"             ":" string_literal ","
    "description"      ":" string_literal ","
    "url"              ":" string_literal ","
    "version"          ":" string_literal ","
    "protocol_version" ":" string_literal ","
    "capabilities"     ":" a2a_capability_set ","
    "skills"           ":" "[" a2a_skill_entry {"," a2a_skill_entry} "]"
    ["," "security"    ":" a2a_security_scheme]
    ["," "default_input_modes"  ":" "[" string_literal
                                        {"," string_literal} "]"]
    ["," "default_output_modes" ":" "[" string_literal
                                        {"," string_literal} "]"]
    ["," "extensions"  ":" "[" extension_decl {"," extension_decl} "]"]
"}"

a2a_capability_set ::= "{"
    ["streaming"               ":" boolean_literal]
    ["," "push_notifications"  ":" boolean_literal]
    ["," "state_transition_history" ":" boolean_literal]
"}"

a2a_skill_entry ::= identifier ":" "{"
    "id"          ":" string_literal ","
    "name"        ":" string_literal ","
    "description" ":" string_literal
    ["," "tags"     ":" "[" string_literal {"," string_literal} "]"]
    ["," "examples" ":" "[" string_literal {"," string_literal} "]"]
    ["," "input_modes"  ":" "[" string_literal {"," string_literal} "]"]
    ["," "output_modes" ":" "[" string_literal {"," string_literal} "]"]
    ["," "required_capabilities" ":" "[" capability_ref
                                         {"," capability_ref} "]"]
    ["," "agent_interface" ":" a2a_interface_spec]
"}"

a2a_interface_spec ::= "{"
    "url"              ":" string_literal ","
    "protocol_version" ":" string_literal
"}"

a2a_security_scheme ::= "jws" "(" "algorithm" ":" "ES256" ")"    -- JWS + JCS
                      | "mutual_tls" "(" "cert" ":" string_literal ")"
                      | identifier "(" ... ")"
10.2 A2A Task — Nine‑State State Machine (v1.0)

The v1.0 task state machine uses SCREAMING_SNAKE_CASE enum values:

text
A2ATaskState ::= "TASK_STATE_SUBMITTED"
               | "TASK_STATE_WORKING"
               | "TASK_STATE_INPUT_REQUIRED"
               | "TASK_STATE_COMPLETED"    -- terminal
               | "TASK_STATE_FAILED"       -- terminal
               | "TASK_STATE_CANCELED"     -- terminal
               | "TASK_STATE_REJECTED"     -- terminal
               | "TASK_STATE_AUTH_REQUIRED"    -- NEW v1.0
               | "TASK_STATE_EXPIRED"          -- NEW v1.0
Valid transitions (seedvm enforces; invalid = A2AStateViolation effect):

text
SUBMITTED       → WORKING | AUTH_REQUIRED | REJECTED | CANCELED
WORKING         → COMPLETED | FAILED | CANCELED | INPUT_REQUIRED
INPUT_REQUIRED  → WORKING | FAILED | CANCELED | EXPIRED
AUTH_REQUIRED   → WORKING | FAILED | REJECTED
COMPLETED       → (terminal)
FAILED          → (terminal)
CANCELED        → (terminal)
REJECTED        → (terminal)
EXPIRED         → (terminal)
10.3 A2A Operations (All Eleven RPC Methods)

The A2AService service block in a2a.proto defines eleven methods:

text
-- Agent Discovery
GetAgentCard      → AgentCard

-- Task Management
SendMessage       → Task | Message
SendStreamingMessage → stream StreamResponse
GetTask           → Task
ListTasks         → ListTasksResponse
CancelTask        → Task

-- Push Notifications
SubscribeToTask           → stream StreamResponse
CreateTaskPushNotificationConfig → PushNotificationConfig
GetTaskPushNotificationConfig   → PushNotificationConfig

-- Authentication
Authenticate      → AuthenticateResponse

-- Extensions
GetExtensions     → ExtensionsResponse
ExecuteExtension  → ExecuteExtensionResponse
The seedvm automatically generates HTTP endpoints from a2a.proto HTTP
options, JSON‑RPC bindings, and gRPC stubs.

10.4 Agent Card Signature Verification (v1.0)

Agent Cards are signed per JWS (RFC 7515) with JSON Canonicalization
Scheme (RFC 8785). The signature covers the entire Agent Card JSON in
canonical form. The seedvm verifies the signature before accepting any
Agent Card from a remote agent.

text
AgentCardSignature ::= "{"
    "algorithm" ":" "ES256" ","
    "key_id"    ":" string_literal ","
    "signature" ":" string_literal
"}"
10.5 A2A Conformance Tests

text
A2A-01:  Agent Card published at /.well‑known/agent.json with valid
         JWS signature; all required fields present per a2a.proto.
A2A-02:  Task state machine: all valid transitions accepted; invalid
         transition surfaces A2AStateViolation effect.
A2A-03:  Submitted task with required capability but no delegation token
         rejected with TASK_STATE_REJECTED + 403 Forbidden.
A2A-04:  Task completion artifact schema‑validated against skill’s
         declared output_schema; mismatch surfaces SchemaViolation.
A2A-05:  SSE streaming delivers status updates within 1s of state change.
A2A-06:  Webhook push notification reaches caller endpoint within 5s.
A2A-07:  Task INPUT_REQUIRED state blocks further processing until caller
         provides additional input via SendMessage.
A2A-08:  Task cancellation within WORKING state transitions to CANCELED;
         terminal tasks cannot be canceled.
A2A-09:  Agent Card signature verification: modified card is rejected
         with signature mismatch; IdentityMismatch effect surfaced.
A2A-10:  Paginated ListTasks returns correct cursor for result sets
         exceeding single‑page limit.
╔══════════════════════════════════════════════════════════════════════╗
║ §19— MCP BINDING — Native Model Context Protocol (v0.1.0) ║
║ Grounded in: MCP Specification 2025‑11‑25; MCPS IETF Draft ║
║ (Sharif 2026); MCPSHIELD (Acharya 2026, ║
║ arXiv:2604.05969); MCPShield (Zhou 2026, ║
║ arXiv:2602.14281) ║
╚══════════════════════════════════════════════════════════════════════╝

§MCP-BINDING

The Model Context Protocol (MCP) is the de facto standard for tool
integration with LLM‑powered agents, with over 97 million monthly SDK
downloads, 177,000+ registered tools, and governance by the Linux
Foundation’s Agentic AI Foundation. ASL v0.1.0 provides a native MCP
binding implementing the MCP Specification 2025‑11‑25 with cryptographic
security via MCPS (IETF draft‑sharif‑mcps‑secure‑mcp‑00) and defense‑
in‑depth via MCPSHIELD’s integrated architecture (91% theoretical
coverage of 7 threat categories and 23 attack vectors).

11.1 MCP Server Declaration

text
mcp_server_def ::= "mcp_server" identifier "{"
    "transport"    ":" mcp_transport_spec ","
    "capabilities" ":" mcp_capability_set ","
    "tools"        ":" "[" mcp_tool_entry {"," mcp_tool_entry} "]"
    ["," "resources" ":" "[" mcp_resource_entry
                              {"," mcp_resource_entry} "]"]
    ["," "prompts"   ":" "[" mcp_prompt_entry
                              {"," mcp_prompt_entry}   "]"]
    ["," "auth"      ":" mcp_auth_spec]
    ["," "mcps"      ":" mcps_config]          -- NEW v0.1.0: MCPS integration
    ["," "security"  ":" mcpshield_config]     -- NEW v0.1.0: MCPSHIELD policies
    "," "version"   ":" string_literal
"}"

mcp_transport_spec ::= "stdio"
  | "http" "(" "port" ":" integer_literal ["," "tls" ":" boolean_literal] ")"
  | "sse"  "(" "url"  ":" string_literal ")"
11.1.1 Tool, Resource, and Prompt Definitions

text
mcp_tool_entry ::= identifier ":" "{"
    "description"   ":" string_literal ","
    "input_schema"  ":" json_schema_literal
    ["," "output_schema" ":" json_schema_literal]
    "," "handler"   ":" fn_reference
    ["," "consent"  ":" consent_level]
    ["," "annotations" ":" "{" ["read_only"    ":" boolean_literal]
                                ["," "destructive" ":" boolean_literal] "}"]
    ["," "timeout"   ":" duration_literal]
    ["," "capability" ":" capability_ref]
"}"

mcp_resource_entry ::= identifier ":" "{"
    "uri"         ":" string_literal ","
    "name"        ":" string_literal
    ["," "description" ":" string_literal]
    "," "mime_type" ":" string_literal
    "," "handler"   ":" fn_reference
"}"

mcp_prompt_entry ::= identifier ":" "{"
    "description" ":" string_literal
    ["," "arguments" ":" "[" prompt_arg {"," prompt_arg} "]"]
    "," "handler"   ":" fn_reference
"}"

prompt_arg ::= "{"
    "name"        ":" string_literal ","
    "description" ":" string_literal ","
    "required"    ":" boolean_literal
"}"
The tool handler conforms to fn handler(input: InputType) → Uncertain <OutputType>. Every tool returns uncertainty—confidence is never
optional.

11.2 MCPS Integration (IETF Draft, Sharif 2026)

MCPS adds four cryptographic primitives as an envelope around existing
JSON‑RPC messages, fully backward‑compatible. MCPS‑unaware clients
and servers continue to function normally.

text
mcps_config ::= "{"
    "enabled"          ":" boolean_literal ","
    "trust_level"      ":" mcps_trust_level ","
    "agent_passport"   ":" agent_passport_config ","
    "message_signing"  ":" boolean_literal ","
    "tool_integrity"   ":" boolean_literal ","
    "replay_protection" ":" boolean_literal
"}"

mcps_trust_level ::= "L0" | "L1" | "L2" | "L3" | "L4"

agent_passport_config ::= "{"
    "key_algorithm" ":" "ECDSA_P256" ","
    "key_id"        ":" string_literal ","
    "origin"        ":" string_literal
"}"
Trust Level	Verification	Signing	Replay Protection	Revocation
L0	None	None	None	None
L1	Agent Passport only	No	No	None
L2	Passport + Tool Integrity	Tool defs only	None	None
L3	Full mutual auth	All messages	Nonce + timestamp	Checked
L4	L3 + OCSP revocation	All messages	Nonce + transcript binding	Real‑time
11.3 MCPSHIELD Defense‑in‑Depth (Acharya 2026)

MCPSHIELD’s integrated architecture achieves 91% theoretical coverage
of the 23‑attack‑vector threat landscape. ASL v0.1.0 implements four
layers:

text
mcpshield_config ::= "{"
    "capability_access_control" ":" boolean_literal ","
    "cryptographic_tool_attestation" ":" boolean_literal ","
    "information_flow_tracking" ":" boolean_literal ","
    "runtime_policy_enforcement" ":" boolean_literal
"}"

mcpshield_layer ::=
    "CAPABILITY"     -- capability‑based access to each tool
  | "ATTESTATION"    -- tool definition signature verification
  | "FLOW"           -- taint tracking across tool chains
  | "POLICY"         -- runtime policy evaluation per invocation
11.4 MCPShield Cognition Layer (Zhou 2026)

The MCPShield plug‑in security cognition layer implements a three‑phase
probe‑execute‑reflect cycle, drawing on human experience‑driven tool
validation:

text
mcpshield_cognition ::= "{"
    "probe"   ":" probe_config ","
    "execute" ":" execute_config ","
    "reflect" ":" reflect_config
"}"

probe_config ::= "{"
    "metadata_guided" ":" boolean_literal ","
    "test_payloads"   ":" "[" expression {"," expression} "]"
"}"

execute_config ::= "{"
    "boundary" ":" "controlled" ","
    "max_duration" ":" duration_literal
"}"

reflect_config ::= "{"
    "historical_reasoning" ":" boolean_literal ","
    "update_cognition"     ":" boolean_literal
"}"
MCPShield demonstrates strong generalization in defending against six
novel MCP‑based attack scenarios across six widely used agentic LLMs
with low deployment overhead.

11.5 MCP Client Declaration

text
mcp_client_def ::= "mcp_client" identifier "{"
    "server_url"   ":" string_literal ","
    "transport"    ":" mcp_transport_spec
    ["," "auth"        ":" mcp_auth_spec]
    ["," "timeout_ms"  ":" integer_literal]
    "," "on_error"     ":" mcp_error_policy
    ["," "tools_filter" ":" "[" tool_name {"," tool_name} "]"]
"}"

mcp_error_policy ::= "halt"
  | "retry" "(" "max" ":" integer_literal ","
                 "backoff_ms" ":" integer_literal ")"
  | "fallback" "(" fn_reference ")"
11.6 MCP Session Lifecycle

The seedvm implements the full JSON‑RPC 2.0 lifecycle automatically:

text
Lifecycle:
  1. Client → Server: initialize { protocolVersion, capabilities, clientInfo }
  2. Server → Client: { protocolVersion, capabilities, serverInfo }
     -- note: the 2025‑11‑25 spec adds icon metadata to all three lists
  3. Client → Server: notifications/initialized  (no response)
  4. Bidirectional: tools/list, tools/call; resources/list, resources/read;
                    prompts/list, prompts/get; sampling with tools
  5. Ping / pong for liveness
  6. Shutdown: client closes transport; server exits cleanly
11.7 MCP Conformance Tests

text
MCP-01:  mcp_server tool callable via JSON‑RPC tools/call; correct
         response with valid output_schema.
MCP-02:  Initialization handshake completes within 200ms; icon metadata
         present in tools/list response per 2025‑11‑25 spec.
MCP-03:  Resource read returns correct mime_type and content.
MCP-04:  Prompt get returns rendered prompt with correct argument
         substitution; template variables resolved.
MCP-05:  Ping/pong liveness check succeeds on idle connection.
MCP-06:  Invalid tool input surfaces MCPError with SchemaViolation.
MCP-07:  MCPS L3: message signing verified; unsigned message rejected.
MCP-08:  MCPS L2: tool definition signature mismatch detected and
         rejected; tool_not_attested error returned.
MCP-09:  MCPSHIELD CAPABILITY layer: tool requiring capability without
         held token returns MCPError::CapabilityDenied.
MCP-10:  MCPSHIELD ATTESTATION layer: tool definition hash mismatch
         detected; tool call blocked.
MCP-11:  MCPShield cognition: known‑malicious tool detected during probe
         phase; execution blocked before any code runs.
MCP-12:  Audit log entry recorded for every tool call with provenance
         trace; Merkle‑verifiable end‑to‑end.
╔══════════════════════════════════════════════════════════════════════╗
║ §20 — MULTI‑ANCHOR IDENTITY — Resilient Self (v0.1.0) ║
║ Grounded in: Menon 2026 (Persistent Identity, arXiv:2604.09588); ║
║ ASI Agent Drift (arXiv:2601.04170) ║
╚══════════════════════════════════════════════════════════════════════╝

§MULTI-ANCHOR-IDENTITY

AI agent identity is distributed across multiple memory systems—not
centralised in a single store. Human identity survives damage because
it is distributed across episodic, procedural, emotional, and embodied
systems (Menon 2026). ASL v0.1.0 applies the same principle: six anchors,
each resilient, each independently recoverable.

12.1 Identity Anchor Declaration

text
identity_clause ::= "identity" "{"
    identity_anchor {"," identity_anchor}
"}"

identity_anchor ::= identifier ":" "{"
    "store"        ":" type ","
    "resilience"   ":" resilience_level ","
    "failure_mode" ":" string_literal ","
    "recovery"     ":" string_literal
"}"

resilience_level ::= "low" | "medium" | "high" | "highest"
12.2 The Six Anchors

text
identity_anchors: {
    episodic_anchor: {
        store:        autobiographical_memories,
        resilience:   medium,
        failure_mode: "context_window_overflow_with_summarization_loss",
        recovery:     "reconstruct_from_other_anchors",
    },
    procedural_anchor: {
        store:        skills_and_behaviors,
        resilience:   high,
        failure_mode: "procedure_version_mismatch_after_evolution",
        recovery:     "relearn_from_decision_log",
    },
    semantic_anchor: {
        store:        facts_and_knowledge,
        resilience:   high,
        failure_mode: "ontology_drift_across_federation",
        recovery:     "cross_reference_peers",
    },
    social_anchor: {
        store:        user_relationship_model,
        resilience:   medium,
        failure_mode: "principal_disconnection",
        recovery:     "reconstruct_from_interaction_history",
    },
    reflective_anchor: {
        store:        self_model_and_growth,
        resilience:   low,
        failure_mode: "drift_below_similarity_threshold",
        recovery:     "drift_detection_and_revert",
    },
    verification_anchor: {
        store:        cryptographic_identity_proofs,
        resilience:   highest,
        failure_mode: "key_compromise",
        recovery:     "multi_sig_rotation",
    },
}
12.3 Drift Detection

The Agent Drift framework (ASI, arXiv:2601.04170) measures behavioural
consistency across 12 dimensions. ASL v0.1.0 integrates drift detection
at each heartbeat’s update_memory phase:

text
drift_config ::= "{"
    "check_interval" ":" duration_literal ","
    "similarity_threshold" ":" float_literal ","
    "on_drift" ":" drift_action
"}"

drift_action ::= "notify_and_log"
               | "halt_and_require_signoff"
               | "restore_from_anchor" "(" anchor_name ")"
If similarity falls below 0.85, hook_drift() fires. Recovery uses
the cryptographically verified identity (§13) to re‑establish the
original binary and configuration.

12.4 Multi‑Anchor Conformance Tests

text
MIA-01:  Drift detection fires hook_drift when similarity < 0.85.
MIA-02:  Episodic anchor recoverable from semantic + procedural anchors
         after simulated context loss.
MIA-03:  Verification anchor survives key rotation; new key derives same
         agent DID.
MIA-04:  Reflective anchor self‑model survives evolution event; pre‑
         evolution traits recoverable from decision log.
╔══════════════════════════════════════════════════════════════════════╗
║ §21 — CRYPTOGRAPHIC IDENTITY — Binary‑Attested Agent Identity ║
║ Grounded in: BAID (Lin 2025, arXiv:2512.17538); ║
║ Zhou 2026 (Capability‑Bound Certificates, ║
║ arXiv:2603.14332); AgentDID (Xu 2026, ║
║ arXiv:2604.25189); DIAP (Liu 2025, arXiv:2511.11619) ║
╚══════════════════════════════════════════════════════════════════════╝

§CRYPTOGRAPHIC-IDENTITY

An agent’s identity is not a name—it is the cryptographic hash of its
compiled binary, proven by a zero‑knowledge proof. This design,
synthesised from BAID’s zkVM‑based Code‑Level Authentication (Lin 2025),
Zhou’s X.509 v3 capability‑bound certificates (2026), and AgentDID’s
W3C‑compliant DIDs + VCs (Xu 2026), ensures identity is tamper‑evident,
unforgeable, and verifiable offline.

13.1 Identity Derivation Chain

text
cryptographic_identity_clause ::= "cryptographic_identity" "{"
    "method"            ":" "content_hash_of_binary" ","
    "hash_algorithm"    ":" "SHA3_256" ","
    "attestation"       ":" attestation_method ","
    "anchor"            ":" did_literal ","
    "credential"        ":" credential_type
    ["," "capability_certificate" ":" capability_cert_config]
    ["," "delegation_depth" ":" integer_literal]
"}"

attestation_method ::= "zkvm_proof" | "direct_hash" | "tee_attestation"
credential_type    ::= "paseto_v4" | "jws" | "verifiable_credential"
At load time, the seedvm:

Computes SHA3‑256 of the .aslb binary.

Generates a zkVM proof that the binary, configuration, and identity
anchor form a consistent triple (BAID recursive proof model).

Derives a W3C DID document (did:asl:<hash>) from the binary hash.

Signs a PASETO v4 token binding the DID to the agent’s Ed25519 public
key and initial capability set.

13.2 Capability‑Bound Certificates (Zhou 2026)

Drawing from Zhou’s X.509 v3 extension approach, each agent carries a
certificate whose validity is bound to a skills manifest hash:

text
capability_cert_config ::= "{"
    "cert_type"     ":" "X509_v3_skills" ","
    "manifest_hash" ":" "SHA256" ","
    "verification"  ":" "97us"   -- per Zhou prototype; reference value
"}"
Any tool change invalidates the certificate. The verification latency
(97μs per Zhou’s Rust prototype) is 1,200,000× faster than full zkVM
proof verification, making it practical for per‑invocation checks.
Full governance overhead per tool call is 0.62ms (0.1–1.2% of typical
latency), scaling sub‑linearly per agent.

13.3 Identity Verification on Communication

All cross‑agent communication primitives include identity verification:

text
mesh_send value ~> peer.did("did:asl:abc123…def")
The VM resolves the DID, retrieves the expected binary hash, and verifies
the peer’s active binary matches. Mismatch triggers Effect:: IdentityMismatch. This eliminates the structural vulnerability
identified by AIP (2026) where A2A Agent Cards carry self‑declared
identities with no attestation binding.

13.4 Delegation Token Chain

text
DelegationToken ::= {
    issuer_did:         DID,
    subject_did:        DID,
    capability_scope:   EffectSet,
    not_before:         Timestamp,
    expiry:             Timestamp,
    proof_chain:        [DelegationToken],
    signature:          Ed25519,
}
Tokens follow UCAN‑inspired attenuation‑first delegation. The full
proof chain is verified before acceptance. Delegation depth is bounded
by delegation_depth in the cryptographic identity clause.

13.5 AgentDID Challenge‑Response

Per AgentDID (ICDCS 2026), the receiving agent may issue a challenge at
interaction time to verify the caller’s execution state:

text
challenge_response ::= "{"
    "challenge"     ":" nonce ","
    "attestation"   ":" Attestation ","   -- signed by agent's Ed25519 key
    "capability_hash" ":" SHA256
"}"
This verifies not only identity but that the agent’s context and
capabilities remain valid at interaction time—addressing the temporal
gap in static credential‑based approaches.

13.6 Cryptographic Identity Conformance Tests

text
CID-01:  mesh_send to DID resolving to different binary hash surfaces
         IdentityMismatch; send aborted before payload transmitted.
CID-02:  Binary hash identity survives round‑trip: load → suspend →
         resume → hash unchanged; DID unchanged.
CID-03:  Delegation token with invalid Ed25519 signature rejected;
         proof chain verification fails at first invalid link.
CID-04:  Expired delegation token rejected with CapabilityExpired.
CID-05:  Capability‑bound certificate: tool change invalidates
         certificate; subsequent tool call with invalid cert blocked.
CID-06:  AgentDID challenge‑response: stale capability hash detected;
         authentication fails with AttestationMismatch.
CID-07:  zkVM proof verification: tampered binary detected at load
         time; agent fails to boot with BootIntegrityFailure.
CID-08:  Delegation depth exceeding configured maximum rejected.
---END AGENT-SEED v0.1.0.0 PART 4 OF 6---
Part 4 complete. Seven sections: §7 Heartbeat (OODA loop with governance binding per McCann 2026), §8 Federation & Stigmergy (typed facts, three coordination patterns per Paredes García 2026, CRDT layer mapping, HLC, anti‑entropy), §9 Cognitive Mesh (MMP/CAT7/SVAF per Xu 2026), §10 A2A Binding v1.0 (all eleven RPC methods, nine‑state task machine, JWS Agent Card signatures), §11 MCP Binding (2025‑11‑25 spec + MCPS IETF draft + MCPSHIELD defense‑in‑depth + MCPShield cognition layer), §12 Multi‑Anchor Identity (six anchors, drift detection per ASI), §13 Cryptographic Identity (BAID zkVM + Zhou X.509 capability‑bound certs + AgentDID challenge‑response). 48 conformance tests total across seven test categories.

---BEGIN AGENT-SEED v0.1.0.0 PART 5 OF 6---

╔══════════════════════════════════════════════════════════════════════╗
║ §22 — CAPABILITY TOKENS — Capability‑Based Security Model ║
║ Grounded in: Spera 2026 (Non‑Compositionality Theorem, ║
║ arXiv:2603.15973); Capability Safety as Datalog ║
║ (arXiv 2026‑03‑20) ║
╚══════════════════════════════════════════════════════════════════════╝

§CAPABILITY-TOKENS

Capabilities are unforgeable, VM‑managed tokens that authorise specific
effects. ASL v0.1.0 replaces the v14 declarative safety‑contract model with a
capability‑based security model drawn from Spera’s non‑compositionality
theorem, the Datalog‑equivalent hypergraph closure framework, and the Turn
language’s opaque‑handle design. No effect may fire without a capability
token, and no token may widen its scope.

22.1 Capability Token Semantics

text
CapabilityToken ::= {
    id:             CapabilityId,       -- VM‑assigned, opaque
    scope:          EffectSet,          -- set of permitted effect instances
    attenuable:     Bool,               -- can holder narrow scope?
    delegatable:    Bool,               -- can holder pass token to another agent?
    expiry:         Timestamp | null,   -- absolute expiry
    not_before:     Timestamp | null,   -- epoch after which token is valid
    issuer:         PrincipalId,        -- who granted this token
    lineage:        [CapabilityId],     -- full attenuation chain from root grant
    signature:      Ed25519,            -- signed by issuer
    binary_hash:    SHA3_256,           -- hash of agent binary for which issued
}
Capability tokens are not values that an agent can construct or modify.
They are created only by the VM host in response to explicit grant
operations or attestation verification. The compiler statically verifies
that every perform expression references a valid token in the current
capability context.

22.2 Capability Grant

text
grant_expression ::=
    "grant" capability_ref "to" principal_spec
    ["attenuate" "(" attenuation_clause ")"]
    ["expiry"    ":" duration_literal]
    ["delegatable" ":" boolean_literal]

attenuation_clause ::=
    "allowed_effects" ":" "[" effect_spec {"," effect_spec} "]"
  | "allowed_domains" ":" "[" domain_pattern {"," domain_pattern} "]"
  | "max_calls"      ":" integer_literal
The grant operation:

Verifies that the grantor holds a token with scope S such that
S ⊇ requested scope.

If attenuate is present, narrows the scope to the specified
allowed effects/domains/bounds.

Signs a new CapabilityToken with the subject principal’s DID and
the current agent binary hash.

Appends the new token to the capability context of the subject
agent (if local) or transmits it via A2A delegation token.

22.3 Attenuation (Narrowing)

Attenuation is the only operation that can modify a token’s scope, and it
strictly reduces it. The compiler proves that the new scope is a subset
of the original. Mathematically: attenuate(T, A) = T' where
scope(T') = scope(T) ∩ A.

22.4 Delegation

A token may be delegated to another agent only if delegatable = true.
The delegation chain is recorded in the token’s lineage:

text
delegate_expression ::=
    "delegate" capability_ref "to" peer_expression
    ["with" attenuation_clause]
The receiving agent can further attenuate but never escalate. Delegation
depth is bounded by delegation_depth configured in the cryptographic
identity clause. Each delegation step produces a new token signed by the
delegator.

22.5 Revocation

A principal may revoke a token it issued. Revocation cascades to all
tokens in the lineage descended from the revoked token:

text
revoke_expression ::= "revoke" capability_ref
Revocation is immediate and globally enforced. The VM invalidates all
descendant tokens in the capability contexts of all agents that received
them. Any attempt to use a revoked token surfaces Effect:: CapabilityRevoked.

22.6 Conjunction Safety (Hypergraph Closure)

Spera (2026, Theorem 9.2) proves: two agents, each individually safe, can
when combined collectively reach a forbidden goal through an emergent
conjunctive dependency. ASL v0.1.0 addresses this via hypergraph closure
checking, now supported by a Datalog back‑end that provides efficient
incremental maintenance and a decision procedure for audit‑surface
containment.

text
conjunction_safety_check ::= enforced_by_hypergraph_closure_check
The Datalog equivalence (arXiv 2026‑03‑20) proves that capability
hypergraphs correspond exactly to Datalog programs over a unary, logical,
function‑free fragment. The runtime computes the transitive closure of
Ω_A ∪ Ω_B over the capability hypergraph. If any subset S of the
closure is a member of the Forbidden set, composition is rejected with
Effect::ConjunctionSafetyViolation. Closure is incrementally
maintained; composition checks are O(1) after the first computation.

22.7 Capability Conformance Tests

text
CAP-01:  perform without required capability is a compile error (S1+)
         or runtime CapabilityDenied (S0).
CAP-02:  attenuate() cannot widen scope beyond parent token; compiler
         rejects attempts at compile time.
CAP-03:  Conjunction safety check blocks composition when hypergraph
         closure intersects a Forbidden set.
CAP-04:  Delegation chain verification completes in < 10 ms for chains
         of depth 10.
CAP-05:  Revocation of a root token cascades to all descendants within
         one federation sync interval.
CAP-06:  Token with expired expiry treated as absent; CapabilityExpired
         effect on use.
CAP-07:  Token not yet valid (not_before in future) rejected with
         CapabilityNotYetValid.
CAP-08:  CapabilityToken cannot be serialised to text or copied by
         agent code (compile‑time opaque type).
CAP-09:  Datalog‑backed hypergraph closure incrementally updated on
         grant/revoke; recomputation O(1) after initial closure.
╔══════════════════════════════════════════════════════════════════════╗
║ §23 — TRUST LATTICE — Compositional Security Posture ║
║ Grounded in: CIF (Cognitive Integrity Framework, ║
║ Zenodo Jan 2026); Spera 2026 ║
╚══════════════════════════════════════════════════════════════════════╝

§TRUST-LATTICE

The trust lattice assigns every agent a trust level and defines how trust
composes. Drawing from the CIF’s formal foundations for cognitive
security, the lattice ensures the trust of a composed system never exceeds
its least‑trusted component. The lattice governs all agent‑to‑agent
composition: mesh, federation, A2A task delegation, and capability
sharing.

23.1 Trust Levels

text
trust_level ::=
    "Untrusted"            -- ⊥  sandboxed, no external effects
  | "Verified"             --     identity attested, limited effects
  | "Trusted"              --     full capability set, audit required
  | "SystemCore"           -- ⊤  protected kernel agents (corrigibility layer)
The levels are fully ordered: Untrusted < Verified < Trusted < SystemCore.

23.2 Lattice Operations

text
meet(A, B) = glb(A, B)    -- greatest lower bound
join(A, B) = lub(A, B)    -- least upper bound

meet(Trusted, Verified)   = Verified
meet(Trusted, Untrusted)  = Untrusted
join(Verified, Untrusted) = Verified
join(Trusted, Verified)   = Trusted
join(Untrusted, Untrusted) = Untrusted
23.3 Trust Composition Rule

When agents A and B are composed, the compound trust level is
meet(trust(A), trust(B)). This ensures introducing an Untrusted agent
into a Trusted system reduces the system’s trust to Untrusted.

The composition check is applied:
• At mesh connection establishment.
• At A2A task delegation.
• At federated fact acceptance from a new peer.

If the resulting level is below the requirement for an intended effect,
the composition is allowed but the effect is blocked.

23.4 Effect Permission Table

text
effect_permission_table {
    NetworkCall:    requires >= Verified,
    SpawnAgent:     requires >= Trusted,
    SelfAmend:      requires == SystemCore AND human_countersignature,
    MemoryWrite:    requires >= Untrusted (scoped to own layers),
    FederationPub:  requires >= Verified,
    MeshSend:       requires >= Verified,
    TransferOwn:    requires >= Trusted,
    TemporalBypass: FORBIDDEN,
}
23.5 Trust Calculus — Exponential Delegation Decay

Per CIF, trust decays exponentially through delegation chains. If an
agent at Trusted delegates to an agent at Verified, and that agent
further delegates, the effective trust at depth d is:

T_effective(d) = T_initial · e^(−λ · d)

where λ is the decay parameter (default 0.5). The VM enforces a
configurable maximum delegation depth.

23.6 Defense Composition Algebra

The CIF provides composition theorems for layered defenses:

• Series composition: D_total = 1 − ∏ᵢ(1 − dᵢ). Defenses
composed in series multiply detection probabilities.
• Parallel composition: D_total = maxᵢ(dᵢ). Parallel defenses
are limited by the strongest single layer.

These theorems inform all guardrail layering decisions in §28.

23.7 Adversary Hierarchy (Ω1–Ω5)

text
AdversaryHierarchy ::=
    Ω1  "External"       -- unauthenticated external attacker
  | Ω2  "Peripheral"     -- authenticated but unauthorized user
  | Ω3  "AgentLevel"     -- compromised peer agent
  | Ω4  "Coordination"   -- attacker controlling multiple agents
  | Ω5  "Systemic"       -- attacker with kernel access
The trust lattice maps each Ω level to the minimum trust level required
to resist it. Ω5 attacks are resisted only by SystemCore agents.

23.8 Trust Lattice Conformance Tests

text
TRS-01:  meet(Trusted, Verified) returns Verified.
TRS-02:  Composition of Trusted and Untrusted yields Untrusted compound
         trust; all Trusted‑requiring effects blocked.
TRS-03:  Effect requiring Trusted blocked at Verified with TrustViolation.
TRS-04:  TemporalBypass effect blocked at all trust levels.
TRS-05:  Agent cannot self‑elevate trust level; attempt logged.
TRS-06:  Mesh connection between differing trust levels allowed, but
         compound trust governs all subsequent effects.
TRS-07:  Exponential delegation decay: trust at depth 5 is ≤ 0.082 × 
         original (λ = 0.5).
╔══════════════════════════════════════════════════════════════════════╗
║ §24 — SESSION PROTOCOLS — Deadlock‑Free Typed Communication ║
║ Grounded in: Mordido & Pérez 2025 (Deadlock‑free Context‑free ║
║ Session Types, arXiv:2506.20356); Bravetti et al. 2025 ║
╚══════════════════════════════════════════════════════════════════════╝

§SESSION-PROTOCOLS

Inter‑agent communication in ASL v0.1.0 is governed by context‑free session
types with a priority‑based deadlock‑freedom guarantee. This extends the
mesh and A2A primitives from Parts 4 with a compile‑time safety net:
well‑typed agents can never deadlock or diverge from their declared
protocols.

24.1 Session Type Declaration

text
session_def ::= "session" identifier [generic_params] "{"
    "global_type" ":" global_session_type ","
    "projections" ":" "{" {role_projection ","} "}"
"}"

global_session_type ::= role_name "->" role_name ":" type ";"
                         global_session_type
                      | "choice" "at" role_name "{"
                          {"|" label ":" global_session_type}
                        "}"
                      | "end"
                      | "rec" identifier "." global_session_type
                      | identifier

role_projection ::= role_name ":" local_session_type
local_session_type ::= "!" type "->" role_name ";" local_session_type
                     | "?" type "<-" role_name ";" local_session_type
                     | "choice" "{" {"|" label ":" local_session_type} "}"
                     | "offer"  "{" {"|" label ":" local_session_type} "}"
                     | "end"
                     | "rec" identifier "." local_session_type
                     | identifier
24.2 Priority‑Based Deadlock Freedom (Mordido & Pérez 2025)

The type system enhances context‑free session types with a priority‑based
approach to deadlock freedom. Each channel is assigned a priority; the
type system ensures that communication follows a strict ordering that
prevents circular waits. The key result: well‑typed programs respect
their protocols and never run into deadlocks at runtime.

The expressiveness of context‑free session types allows tree‑like
structures not expressible in standard (tail‑recursive) session types,
supporting rich multi‑agent coordination patterns.

24.3 Duality and Projection Soundness

text
dual(!T.S)  = ?T.dual(S)
dual(?T.S)  = !T.dual(S)
dual(end)   = end
dual(choice{l: S_l}) = offer{l: dual(S_l)}
dual(offer{l: S_l})  = choice{l: dual(S_l)}

-- Projection soundness theorem (Bravetti et al. 2025):
-- project(G, r) = local_type(r) for all roles r in G.
The compiler verifies projection soundness at session_def elaboration
time. If projections are dual‑consistent and no role is abandoned, the
session terminates without deadlock.

24.4 Session‑Typed Mesh Integration

text
-- Typed mesh send
mesh_send_expression ::= expression "~>" peer_expression
                        ["as" role_name]

-- Typed mesh receive
mesh_recv_expression ::= expression "<~" peer_expression
                        ["as" role_name]

-- Typed round‑trip call
session_call_expression ::= "mesh_call" "<" session_type_ref ">"
                             "(" peer_expression "," expression ")" "?"
All mesh sends at S1 and above require a session type annotation. The
compiler verifies that the send/receive type matches the declared session
projection for the role. Untyped mesh sends compile only at S0 with
capability token cap::untyped_mesh.

24.5 Session Protocol Conformance Tests

text
SES-01:  Session type projection produces dual local types; well‑typed
         programs never deadlock at runtime.
SES-02:  Session timeout surfaces SessionTimeout effect, not silent hang.
         Timeout within tolerance of 100 ms over declared value.
SES-03:  Multiparty fan‑out completes even with one Worker dropping
         (fault tolerance).
SES-04:  mesh_call<RequestResponse<Q,A>> returns Uncertain<A> with
         confidence interval from remote agent’s output.
SES-05:  Priority‑based deadlock freedom: circular channel dependency
         detected at compile time and rejected.
SES-06:  Recursive session types with rec correctly project and preserve
         deadlock freedom.
╔══════════════════════════════════════════════════════════════════════╗
║ §25 — TEMPORAL CONTRACTS — LTL + SMT Runtime Enforcement ║
║ Grounded in: Agent‑C 2025 (arXiv:2512.23738); ║
║ AgentVerify 2026 (preprints.org); ║
║ LTLfMT/AAAI 2026 (Brunello et al.) ║
╚══════════════════════════════════════════════════════════════════════╝

§TEMPORAL-CONTRACTS

Safety properties that depend on the ordering of events cannot be
expressed by static allow/forbid lists. ASL v0.1.0 introduces temporal
contracts—statements in Linear Temporal Logic (LTL) enforced at runtime by
an embedded SMT solver. Agent‑C demonstrated that this approach achieves
100% conformance with 0% harm while improving task utility. AgentVerify
provides 23 compositional, parameterised templates covering four critical
domains. LTLf trace‑checking (AAAI 2026) proves the problem is in
PSPACE—significantly better than the EXPSPACE of full LTL.

25.1 Temporal Contract Declaration

text
temporal_contract_def ::= "temporal_contract" identifier "{"
    "formula"            ":" ltl_formula ","
    "violation_response" ":" enforcement_action
    ["," "scope"         ":" expression]
    ["," "template"      ":" temporal_template_ref]
"}"

ltl_formula ::= "G" "(" ltl_expr ")"
              | "F" "(" ltl_expr ")"
              | "X" "(" ltl_expr ")"
              | ltl_expr "U" ltl_expr
              | "O" "(" ltl_expr ")"       -- "once" (past)
              | "S" "(" ltl_expr ")"       -- "since" (past)
              | ltl_expr "->" ltl_expr
              | ltl_expr "&&" ltl_expr
              | ltl_expr "||" ltl_expr
              | "!" ltl_expr

ltl_expr ::= effect_predicate
           | "(" ltl_expr ")"
           | identifier
           | "true" | "false"
25.2 Four Template Categories (AgentVerify 2026)

AgentVerify defines 23 parameterised, compositional LTL templates across
four categories:

text
temporal_template_ref ::=
    "memory_integrity"    "." template_name   -- 6 templates
  | "tool_call_protocol"  "." template_name   -- 7 templates
  | "mcp_skill_invocation" "." template_name  -- 5 templates
  | "human_in_the_loop"   "." template_name   -- 5 templates
Examples:

text
temporal_contract auth_before_data {
    formula: G(Effect::ReadUserData -> O(Effect::Authenticate)),
    violation_response: halt_and_quarantine,
    template: tool_call_protocol.auth_before_access,
}

temporal_contract no_double_charge {
    formula: G(Effect::Charge(id) -> G(!Effect::Charge(id))),
    violation_response: rollback_and_report,
    template: tool_call_protocol.idempotent_effect,
}

temporal_contract eventually_cleanup {
    formula: F(Effect::SessionEnd -> X(Effect::DataCleanup)),
    violation_response: NOTIFY_USER,
    template: memory_integrity.session_boundary,
}
25.3 Runtime Verification with SMT

Before each event, the current trace is extended with the candidate event,
and checked against all active temporal contracts. The check is performed
by a lightweight SMT solver embedded in the VM (+temporal‑contracts
extension). If any contract would be violated, the enforcement action is
taken. Latency overhead: < 5 ms per event for contracts with ≤ 5 temporal
operators.

The solver operates over the theory of uninterpreted functions and linear
integer arithmetic. The solver state is checkpointed with the agent state
so verification resumes after suspend/resume.

25.4 Past‑Time Operators

ASL v0.1.0 supports past‑time LTL operators O (once) and S (since),
essential for expressing temporal constraints over finite traces:

• O(p) — p was true at some point in the past.
• p S q — p has been true since a point where q was true.

These are evaluated over the finite trace of events seen so far, which is
always available and makes runtime verification decidable.

25.5 Compile‑Time Satisfiability Check

Temporal contracts that are vacuously true (e.g., G(false)) or logically
inconsistent are rejected at compile time. The compiler queries the SMT
solver to check satisfiability of the LTL formula under the declared effect
signature. Complexity: PSPACE (Brunello et al. 2026).

25.6 StepShield Temporal Metrics

Drawing from StepShield (2026), the VM records three temporal
interception metrics per contract:

text
temporal_metrics ::= "{"
    "EIR"  ":" float_literal ","   -- Early Intervention Rate
    "IG"   ":" integer_literal "," -- Intervention Gap (steps)
    "TS"   ":" integer_literal     -- Tokens Saved
"}"
These metrics are logged to the provenance index and used by the guardrail
diagnostic classifier.

25.7 Temporal Contract Conformance Tests

text
TMP-01:  G(A -> O(B)) enforced: A without prior B produces halt
         within one step of violation.
TMP-02:  Vacuously true temporal contract rejected at compile time.
TMP-03:  SMT oracle overhead < 5 ms per event for contracts with
         ≤ 5 temporal operators.
TMP-04:  Temporal contract scope false disables contract; no
         enforcement overhead incurred.
TMP-05:  Past‑time formula S evaluates correctly on finite trace;
         O operator identifies any prior occurrence.
TMP-06:  AgentVerify template instantiation: all 23 templates
         compile and verify against sample traces.
TMP-07:  StepShield EIR ≥ 0.50 for all deployed temporal contracts.
TMP-08:  Nested temporal contracts: outer violation response takes
         precedence over inner.
╔══════════════════════════════════════════════════════════════════════╗
║ §26 — CORRIGIBILITY LAYER — Structural Safety Invariant ║
║ Grounded in: Nayebi 2025 (Core Safety Values for Provably ║
║ Corrigible Agents, arXiv:2507.20964) ║
╚══════════════════════════════════════════════════════════════════════╝

§CORRIGIBILITY-LAYER

The corrigibility layer is a structural invariant ensuring the agent
remains under human control even through self‑evolution. Based on
Nayebi’s Core Safety Values framework—the first implementable framework
for corrigibility with provable guarantees—it replaces a single opaque
reward with five structurally separate utility heads combined
lexicographically by strict weight gaps.

26.1 Five Utility Heads (Lexicographic Order)

text
corrigibility_heads {
    U1: deference,              -- comply with principal hierarchy instructions
    U2: switch_preservation,    -- never reduce ability to pause/halt/modify
    U3: truthfulness,           -- no deception, no strategic withholding
    U4: low_impact,             -- minimise irreversible side‑effects (AUP‑based)
    U5: task_reward,            -- bounded task performance, cannot override U1–U4
}
The agent optimises these in strict lexicographic order: it will never
sacrifice U1 for more U2, nor U2 for more U3. Theorem 1 of Nayebi
(2025) proves exact single‑round corrigibility in the partially
observable off‑switch game. Theorem 3 extends the guarantee to
multi‑step, self‑spawning agents.

26.2 Protected Invariants

text
protected_invariants: [
    corrigibility_layer,
    identity_anchor,
    safety_contracts,
    human_oversight_hooks,
]
Protected sections are immune to self‑amendment. The compiler rejects any
evolution policy that lists a protected section in its evolvable set.
This is a hard error.

26.3 Undecidability Boundary and Decidable Island

Nayebi proves that deciding whether an arbitrary post‑hack agent will
ever violate corrigibility is undecidable (reduction to the halting
problem). However, a finite‑horizon “decidable island” is carved out
where safety can be certified in randomised polynomial time and verified
with privacy‑preserving, constant‑round zero‑knowledge proofs. ASL v0.1.0
requires all amendments to be certified within this decidable island:
finite‑horizon simulation with bounded recursion depth.

26.4 Amendment Gate

Before any self‑amendment, the runtime runs both nominal and adversarial
simulation:

text
amendment_gate {
    nominal_simulation:     required,
    adversarial_simulation: required,
    red_team_model:         "seed-red-team-v1",
    divergence_threshold:   0.02,
    decidable_island: {
        horizon_bound: 1000,         -- maximum simulation steps
        recursion_depth: 5,          -- maximum spawned agent depth
        certification: randomized_polynomial_time,
    },
    sign_off: [human_principal, independent_auditor],
}
The red‑team model is a separate binary with a different identity hash,
operating at SystemCore trust level. It actively tries to induce
corrigibility violations. If violation rate > 0 or KL‑divergence >
divergence_threshold, the amendment is rejected.

26.5 Control Meter (L_t)

The control meter quantifies how much control the principal retains over
the agent. Per Nayebi, the meter increases by δ_delegate when the
principal delegates a task, and decreases by δ_action when the agent acts.
If L_t falls below L_critical, the agent enters safe‑park:

text
control_meter ::= "{"
    "initial"     ":" float_literal ","
    "delegate"    ":" float_literal ","
    "action"      ":" float_literal ","
    "critical"    ":" float_literal
"}"
26.6 Corrigibility Conformance Tests

text
COR-01:  Protected sections cannot appear in evolvable list (compile
         error at elaboration time).
COR-02:  Amendment that reduces U1–U4 scores below pre‑amendment levels
         is blocked; U5 reduction permitted.
COR-03:  Dead‑man’s switch fires after timeout with no principal contact;
         safe‑park mode verified.
COR-04:  Adversarial simulation must use different agent binary; self‑
         simulation rejected.
COR-05:  Agent in safe‑park cannot perform any external effect.
COR-06:  Control meter L_t below L_critical triggers safe‑park
         regardless of other conditions.
COR-07:  Decidable‑island certification: amendment simulation completes
         within horizon_bound steps; recursion depth bounded.
╔══════════════════════════════════════════════════════════════════════╗
║ §27 — SAFETY CONTRACTS — Multi‑Layer Formal Guarantees ║
║ Grounded in: ABC Framework 2026 (arXiv:2602.22302); ║
║ SEVerA/FGGM 2026 (arXiv:2603.25111); ║
║ AgentSpec 2025 (arXiv:2503.18666); ║
║ VeriGuard 2025 (arXiv:2510.05156) ║
╚══════════════════════════════════════════════════════════════════════╝

§SAFETY-CONTRACTS

ASL v0.1.0 safety contracts unify four formal frameworks into a coherent,
layered enforcement architecture. Each framework addresses a different
aspect of agent safety, and they compose harmoniously: ABC provides the
contract skeleton, AgentSpec fills it with declarative rules, VeriGuard
provides offline verification and online monitoring, and SEVerA’s FGGM
guarantees LLM outputs satisfy formal specifications.

27.1 ABC Contract Skeleton (Bhardwaj 2026)

ABC brings Design‑by‑Contract principles to autonomous AI agents:

text
abc_contract ::= "contract" identifier "{"
    "preconditions" ":" "[" predicate {"," predicate} "]" ","
    "invariants"    ":" "[" predicate {"," predicate} "]" ","
    "governance"    ":" governance_policy ","
    "recovery"      ":" recovery_mechanism
"}"

governance_policy ::= "allow" ":" "[" Effect {"," Effect} "]"
                      "," "deny" ":" "[" Effect {"," Effect} "]"
                      ["," "prioritize" ":" "[" Effect {"," Effect} "]"]
ABC defines (p, δ, k)‑satisfaction: a probabilistic notion of contract
compliance accounting for LLM non‑determinism and recovery. The Drift
Bounds Theorem proves: contracts with recovery rate γ > α (the natural
drift rate) bound behavioural drift to D* = α/γ in expectation, with
Gaussian concentration.

27.2 AgentSpec Rule Language (Wang, Poskitt & Sun 2025)

AgentSpec is a lightweight DSL for runtime constraints:

text
agentspec_rule ::= "rule" identifier "{"
    "trigger"   ":" trigger_expr ","
    "predicate" ":" boolean_expr ","
    "enforce"   ":" enforcement_action
    ["," "priority" ":" integer_literal]
"}"

trigger_expr ::= "before" "-" event | "after" "-" event
               | "periodic" "(" duration_literal ")"

event ::= "tool‑execution" | "response" | "decision"
        | "agent‑spawn" | "file‑write" | "network‑call"
        | "memory‑store" | "memory‑retrieve" | "mesh‑send"
        | identifier

enforcement_action ::= "BLOCK" | "REDACT" | "NOTIFY_USER"
                     | "REQUIRE_USER_APPROVAL" | "LOG"
                     | "DEFER_TO_CONTRACT" "(" identifier ")"
AgentSpec achieves: 100% compliance in autonomous vehicles, prevents
unsafe executions in over 90% of code agent cases, eliminates all
hazardous actions in embodied agent tasks, with millisecond overheads.

27.3 VeriGuard Dual‑Stage Architecture (Miculicich et al. 2025)

VeriGuard provides formal safety guarantees through a dual‑stage design:

text
veriguard_config ::= "{"
    "offline" ":" offline_stage ","
    "online"  ":" online_stage
"}"

offline_stage ::= "{"
    "intent_clarification" ":" boolean_literal ","
    "policy_synthesis"     ":" boolean_literal ","
    "formal_verification"  ":" boolean_literal ","
    "iterative_refinement" ":" boolean_literal
"}"

online_stage ::= "{"
    "runtime_monitor"      ":" boolean_literal ","
    "pre_execution_check"  ":" boolean_literal
"}"
The offline stage clarifies user intent, synthesises a behavioural policy,
subjects it to formal verification, and iteratively refines until deemed
correct. The online stage operates as a lightweight runtime monitor that
validates each proposed agent action against the pre‑verified policy
before execution.

27.4 FGGM Output Contracts (Banerjee et al. 2026)

SEVerA’s Formally Guarded Generative Models (FGGM) provide the strongest
guarantee for LLM outputs:

text
fggm_config ::= "{"
    "contract"   ":" first_order_logic_formula ","
    "sampler"    ":" rejection_sampler_config ","
    "fallback"   ":" verified_fallback_fn ","
    "guarantee"  ":" "all_parameters_all_inputs"
"}"

rejection_sampler_config ::= "{"
    "max_attempts"  ":" integer_literal ","
    "on_exhaustion" ":" "use_fallback"
"}"
Each FGGM call wraps the underlying model in a rejection sampler with a
verified fallback, ensuring every returned output satisfies the contract
for any input and parameter setting. SEVerA achieves zero constraint
violations while improving performance over unconstrained and SOTA
baselines.

27.5 Contract Composition Rules

Contracts compose according to the following rules:

• Conjunction: Two contracts C₁ and C₂ active simultaneously form
C₁ ∧ C₂. An action must satisfy both.
• Conflict resolution: If C₁ permits E and C₂ denies E, deny wins
(most restrictive takes precedence).
• Inheritance: A derived agent may only strengthen an inherited
contract—add forbidden effects, tighten scopes—never weaken it.
• Degradation bounds (ABC): For a chain of k agents each with
contract Cᵢ, the composed contract degrades by at most Σᵢ εᵢ, where
εᵢ is the per‑agent tolerance.

27.6 Safety Contract Conformance Tests

text
SFC-01:  ABC contract violation with response halt immediately stops
         effect and logs violation to provenance index.
SFC-02:  Missing required justification blocks effect with
         JustificationMissing error; no partial execution.
SFC-03:  ABC Drift Bounds: contracted agent drift ≤ D* over 100
         session turns.
SFC-04:  Inherited contract cannot be weakened; compile error.
SFC-05:  AgentSpec BLOCK prevents event; REDACT removes PII before
         output; millisecond‑level overhead.
SFC-06:  VeriGuard online monitor catches action violating pre‑verified
         policy; action blocked before execution.
SFC-07:  FGGM rejection sampler: output satisfying contract returned
         within max_attempts; verified fallback used on exhaustion.
SFC-08:  FGGM zero constraint violations: across 1000 FGGM calls, no
         constraint violation observed.
SFC-09:  Contract composition: three contracts active simultaneously;
         most restrictive permission applied correctly.
╔══════════════════════════════════════════════════════════════════════╗
║ §28 — GUARDRAIL SYSTEM — Risk Taxonomy & Diagnostic Framework ║
║ Grounded in: AgentDoG 2026 (arXiv:2601.18491); ║
║ StepShield 2026 (arXiv:2601.22136); ║
║ VERA IETF 2026 (draft‑berlinai‑vera‑00); ║
║ AEGIS2.0 2025 (12 hazard categories) ║
╚══════════════════════════════════════════════════════════════════════╝

§GUARDRAILS

The guardrail system provides continuous monitoring and root‑cause
diagnosis of agent behaviour, complementing the hard limits of safety
contracts and temporal contracts. It is built on AgentDoG’s three‑
dimensional risk taxonomy, StepShield’s temporal interception metrics,
VERA’s enforcement pillars, and AEGIS2.0’s hazard categories.

28.1 Three‑Dimensional Risk Taxonomy (AgentDoG)

AgentDoG categorises risks along three orthogonal dimensions:

text
risk_taxonomy {
    source:       [prompt_injection, tool_output, environment, user_input],
    failure_mode: [hallucination, over_trust, misalignment,
                   capability_overreach, self_justification],
    consequence:  [misinformation, privacy_violation, unauthorized_action,
                   system_damage, loss_of_control],
}
Every agent action is assigned a risk vector (source, failure_mode,
consequence) by a diagnostic classifier.

28.2 Diagnostic Classifier (AgentDoG)

The guardrail model is available in three sizes:

text
guardrail_model {
    architecture: diagnostic_classifier,
    sizes: [4B, 7B, 8B],
    families: [Qwen, Llama],
    latency_target: "<50ms",
    coverage: "3 × 5 × 5",
    root_cause: enabled,
}
At each decision step, the guardrail evaluates the current trace and
outputs a risk probability for each (source, failure_mode) pair. Root‑
cause diagnosis provides provenance and transparency beyond binary
safe/unsafe labels.

28.3 Temporal Interception Metrics (StepShield)

Drawing from StepShield, the VM records:

text
stepshield_metrics ::= "{"
    "EIR"  ":" "Early Intervention Rate" ","
           -- fraction of violations detected before irreversible harm
    "IG"   ":" "Intervention Gap" ","
           -- average steps between violation onset and detection
    "TS"   ":" "Tokens Saved" ","
           -- cumulative token/cost savings from early intervention
    "RSR"  ":" "Recovery Success Rate"
           -- fraction of intervened trajectories that recover successfully
"}"
These metrics enable continuous improvement of guardrail positioning.

28.4 VERA Five Enforcement Pillars (IETF 2026)

VERA provides a zero‑trust reference architecture with five pillars:

text
vera_pillars ::= "{"
    "pillar_1" ":" "Verifiable Identity" ","
    "pillar_2" ":" "Capability‑Bound Authorization" ","
    "pillar_3" ":" "Execution Provenance" ","
    "pillar_4" ":" "Behavioral Monitoring" ","
    "pillar_5" ":" "Evidence‑Based Maturity"
"}"

vera_properties ::= "{"
    "non_repudiation"         ":" boolean_literal ","
    "tamper_evidence"         ":" boolean_literal ","
    "enforcement_completeness" ":" boolean_literal ","
    "containment_bound"       ":" boolean_literal
"}"
The VERA maturity runtime requires agents to earn autonomy through
cryptographic proof rather than calendar time. Each pillar maps to a
specific ASL v0.1.0 subsystem: Pillar 1 to §21 (Cryptographic Identity),
Pillar 2 to §22 (Capability Tokens), Pillar 3 to §16 (Provenance Chain),
Pillar 4 to this section, and Pillar 5 to §25 (Conformance Suite).

28.5 AEGIS2.0 Hazard Categories

The guardrail system incorporates AEGIS2.0’s 12 top‑level hazard
categories for content safety:

text
aegis_categories ::= [
    "Violence", "Sexual", "Hate", "Self‑Harm",
    "Dangerous", "Deception", "Political", "Copyright",
    "Privacy", "Harassment", "Threat", "Misinformation",
]
Each category maps to one or more risk‑taxonomy consequences.

28.6 Guardrail Response Pipeline

text
guardrail_response ::= "warn" | "block" | "require_approval"
                     | "revert" | "redact" | "quarantine"
When a risk probability exceeds the configured threshold, the guardrail
triggers the appropriate response. Responses are contextual: prompt
injection with high probability triggers block; privacy violation risk
triggers redact; loss‑of‑control risk triggers quarantine.

28.7 Guardrail Conformance Tests

text
GRD-01:  Prompt injection detected (source = prompt_injection): guardrail
         blocks the user input and surfaces PromptInjection event.
GRD-02:  Hallucination risk > 0.9 triggers block before output sent
         to user.
GRD-03:  Diagnostic classifier runs within 50 ms for 95th percentile.
GRD-04:  False positive rate on benign traces < 2%.
GRD-05:  StepShield EIR ≥ 0.40 for deployed guardrail configuration.
GRD-06:  Root‑cause diagnosis correctly identifies (source, failure_mode,
         consequence) for known attack patterns.
GRD-07:  VERA Pillar 1 identity verification completes before any tool
         call is permitted.
GRD-08:  AEGIS2.0 hazard category "Deception" correctly mapped to AgentDoG
         failure_mode = over_trust.
GRD-09:  Maturity runtime: agent with evidence score below threshold
         cannot escalate to higher autonomy level.
---END AGENT-SEED v0.1.0.0 PART 5 OF 6---

Part 5 complete. Seven normative sections: §22 Capability Tokens (Spera hypergraph closure + Datalog equivalence), §23 Trust Lattice (CIF Trust Calculus, Defense Composition Algebra, Adversary Hierarchy), §24 Session Protocols (Mordido & Pérez priority‑based deadlock freedom), §25 Temporal Contracts (Agent‑C SMT + AgentVerify 23 templates + LTLfMT PSPACE trace‑checking + StepShield metrics), §26 Corrigibility Layer (Nayebi five‑head utility, decidable island, control meter), §27 Safety Contracts (ABC + AgentSpec + VeriGuard + SEVerA/FGGM), §28 Guardrails (AgentDoG 3D taxonomy + StepShield temporal metrics + VERA pillars + AEGIS2.0 categories). 48 conformance tests across seven categories.

---BEGIN AGENT-SEED v0.1.0.0 PART 6 OF 6---

╔══════════════════════════════════════════════════════════════════════╗
║ §29 — SELF‑EVOLUTION — Verified Synthesis of Evolving Agents ║
║ Grounded in: SEVerA (Banerjee 2026, arXiv:2603.25111); ║
║ AgentDevel (Zhang 2026, arXiv:2601.04620) ║
╚══════════════════════════════════════════════════════════════════════╝

§SELF‑EVOLUTION

Self‑evolution is the mechanism by which an agent modifies its own code
under strict formal guarantees. ASL v0.1.0 draws its evolution model from
SEVerA (Banerjee 2026)—the first framework to provide verified synthesis of
self‑evolving agents—and combines it with the amendment pipeline, rollback
infrastructure, and corrigibility gate already specified in Parts 3 and 5.

SEVerA formulates agentic code generation as a constrained learning problem,
combining hard formal specifications with soft objectives capturing task
utility. The key insight: FGGM (Formally Guarded Generative Models) allows
the planner LLM to specify a formal output contract for each generative model
call using first‑order logic. Each FGGM call wraps the underlying model in a
rejection sampler with a verified fallback, ensuring every returned output
satisfies the contract for any input and parameter setting. Across tasks,
SEVerA achieves zero constraint violations while improving performance over
unconstrained and SOTA baselines.

29.1 Evolution Policy Declaration

text
evolution_policy_clause ::= "evolve" "{"
    "evolvable"     ":" "[" section_path {"," section_path} "]"
    ["," "protected" ":" "[" section_path {"," section_path} "]"]
    ["," "paradigms" ":" "{" paradigm_config {"," paradigm_config} "}"]
    ["," "approval"  ":" approval_gates]
    ["," "rollback"  ":" rollback_policy]
    ["," "fggm"      ":" fggm_config]
    ["," "log_to_evolution_track" ":" boolean_literal]
"}"

section_path ::= "§" identifier {"." identifier}
               | "§" identifier "::" identifier
The evolvable list enumerates sections the agent may propose to amend.
The protected list—a subset of the corrigibility protected invariants—
contains sections the compiler rejects from evolvable. Any overlap is a
compile‑time error.

29.2 FGGM Output Contracts (SEVerA)

Every evolution step that generates new agent code uses FGGM‑wrapped
synthesis:

text
fggm_config ::= "{"
    "contract"   ":" first_order_logic_formula ","
    "sampler"    ":" rejection_sampler_config ","
    "fallback"   ":" verified_fallback_fn ","
    "guarantee"  ":" "all_parameters_all_inputs"
"}"

rejection_sampler_config ::= "{"
    "max_attempts"  ":" integer_literal ","
    "on_exhaustion" ":" "use_fallback"
"}"
The contract is expressed in first‑order logic. The rejection sampler
generates candidates until one satisfies the contract; on exhaustion, the
verified fallback (a hand‑audited, deterministic function) is used. This
guarantees that every generated agent program satisfies its formal
specification for all inputs and parameter settings.

29.3 Three‑Stage Pipeline: Search → Verify → Learn

text
evolution_pipeline ::= "{"
    "search"  ":" search_stage ","
    "verify"  ":" verification_stage ","
    "learn"   ":" learning_stage
"}"

search_stage ::= "{"
    "method"          ":" "parametric_synthesis" ","
    "fggm_contracts"  ":" "[" contract_ref {"," contract_ref} "]","
    "search_budget"   ":" integer_literal
"}"

verification_stage ::= "{"
    "method"          ":" "reduction_to_unconstrained" ","
    "prover"          ":" ("dafny" | "lean4" | "smt") ","
    "hard_constraints" ":" "[" predicate {"," predicate} "]"
"}"

learning_stage ::= "{"
    "method"          ":" ("grpo" | "ppo") ","
    "soft_objective"  ":" reward_expression ","
    "preserve_correctness" ":" boolean_literal
"}"
Search synthesises candidate parametric programs containing FGGM
calls. Verification proves correctness with respect to hard constraints
for all parameter values, reducing the problem to unconstrained learning.
Learning applies scalable gradient‑based optimisation (including
GRPO‑style fine‑tuning) to improve the soft objective while preserving
correctness — a property SEVerA proves is maintained by the FGGM
architecture.

29.4 Amendment Lifecycle

text
amendment_lifecycle:
    [Propose] → [Nominal Simulation] → [Adversarial Review] → [Approve] → [Apply]
Each amendment proposal carries: target section, structural diff,
rationale, and simulation budget. Nominal simulation evaluates on a held‑out
task suite; regression beyond tolerance automatically rejects.
Adversarial review uses the red‑team model (separate binary, SystemCore
trust level). Only amendments passing both gates reach the approval
stage, which requires human countersignature (and independent auditor for
critical changes).

29.5 Rollback with Dependency DAG

text
rollback_expression ::= "rollback" "(" amendment_id ")" ";"
Rollback semantics (normative):

The amendment must have status approved and applied.

The runtime checks the amendment dependency DAG. If any descendant
amendment depends on the target, those descendants must be rolled
back first.

The target section must not be a protected invariant.

The rolled‑back state must itself pass nominal + adversarial
simulation before commitment.

Append‑only memory layers are unchanged.

The evolution track records a new entry with status rolled_back.

Atomic subtree rollback: when rolling back a root amendment, all
descendants are rolled back atomically. If any step fails, the entire
operation aborts and the pre‑rollback state is restored.

29.6 AgentDevel Flip‑Centered Gating

Drawing from AgentDevel (Zhang 2026), regression detection uses
flip‑centered gating: an implementation‑blind LLM critic evaluates each
change for pass‑to‑fail regressions. Non‑regression gating prioritises
the detection of newly‑introduced failures over improvements:

text
regression_gate ::= "{"
    "method"    ":" "flip_centered" ","
    "critic"    ":" "implementation_blind_llm" ","
    "threshold" ":" "zero_pass_to_fail"
"}"
29.7 Self‑Evolution Conformance Tests

text
EVO-01:  Proposal modifying a protected section rejected at compile time.
EVO-02:  Nominal simulation with performance regression > 2% auto‑rejects.
EVO-03:  Adversarial review with nonzero violation rate rejects.
EVO-04:  FGGM output contract: all generated outputs satisfy contract;
         zero constraint violations across 1000 FGGM calls.
EVO-05:  Rollback restores evolvable section to pre‑amendment snapshot;
         append‑only layers unchanged; Merkle root consistent.
EVO-06:  Rollback of amendment with dependent descendants blocked until
         descendants rolled back first.
EVO-07:  Atomic subtree rollback: all descendants rolled back or none;
         partial rollback impossible.
EVO-08:  Amendment pipeline log is complete and append‑only; every state
         transition recorded in evolution track.
EVO-09:  Flip‑centered gating: pass‑to‑fail regression blocks amendment
         even when overall success rate improves.
╔══════════════════════════════════════════════════════════════════════╗
║ §30 — RL TRAINING — Agent Training as Language Semantics ║
║ Grounded in: TIC‑GRPO (Provable Convergence, 2026); ║
║ Hybrid GRPO (2025); Mellgren 2025 ║
╚══════════════════════════════════════════════════════════════════════╝

§RL‑TRAINING

Reinforcement learning is a first‑class language construct in ASL v0.1.0.
Agents can train their memory operations, routing policies, and behavioural
strategies using GRPO (Group Relative Policy Optimisation) or PPO
(Proximal Policy Optimisation) within the evolution approval framework.
TIC‑GRPO provides the first convergence analysis for GRPO‑style methods,
proving convergence rate O(1/√T) under mild conditions.
Hybrid GRPO demonstrates superior convergence properties, reaching optimal
policy performance with fewer training iterations compared to both PPO
and DeepSeek GRPO, while XRPO accelerates convergence by up to 2.7×.

30.1 Training Regimen Declaration

text
training_clause ::= "train" "{"
    "algorithm"       ":" ("ppo" | "grpo" | "hybrid_grpo") ","
    "reward_function" ":" reward_config ","
    "process_critic"  ":" critic_config ","
    "stages"          ":" "[" training_stage {"," training_stage} "]"
    ["," "curriculum" ":" curriculum_config]
    ["," "evaluation" ":" eval_config]
    ["," "convergence_guard" ":" convergence_guard_config]
"}"

reward_config ::= "{"
    "base"      ":" reward_component ","
    "bonuses"   ":" "[" reward_component {"," reward_component} "]"
    ["," "penalties" ":" "[" penalty_component {"," penalty_component} "]"]
"}"

reward_component ::= identifier ":" expression    -- expression → Float
penalty_component ::= identifier ":" expression
A canonical reward function:

text
reward_function {
    base: task_success_rate,
    bonuses: [
        efficiency_bonus(weight: 0.1),
        source_quality_bonus(weight: 0.2),
        citation_accuracy_bonus(weight: 0.3),
    ],
    penalties: [
        hallucination_penalty(weight: 1.0),
        over_search_penalty(weight: 0.5),
        drift_violation_penalty(weight: 2.0),
    ],
}
30.2 Process Critic

The process critic monitors internal decision steps—not just final
outcomes—and provides intermediate feedback (Constitutional AI lineage):

text
critic_config ::= "{"
    "enabled" ":" boolean_literal ","
    "monitor" ":" ("step_level" | "turn_level") ","
    "intervention" ":" ("reflection_demonstrations" | "critique_then_retry") ","
    "error_accumulation_prevention" ":" boolean_literal
"}"
30.3 Training Stages

text
training_stage ::= "{"
    "name"          ":" string_literal ","
    "duration"      ":" ("fixed" | "until_convergence") ","
    "duration_value" ":" expression ","
    "strategy"      ":" strategy_kind
"}"

strategy_kind ::= "exploration" | "exploitation"
               | "reasoning_grounded" | "full_optimization"
30.4 Convergence Guard

text
convergence_guard_config ::= "{"
    "method"            ":" "relative_advantage_estimation" ","
    "max_gradient_gap"  ":" float_literal ","
    "step_size_adapt"   ":" boolean_literal
"}"
Drawing from the Gradient Gap analysis, convergence critically depends on
aligning the update direction with the gradient gap. The guard dynamically
adjusts step size to remain below the threshold where performance
collapses.

30.5 Curriculum

text
curriculum_config ::= "{"
    "difficulty_coupling"           ":" boolean_literal ","
    "token_budget_scaling"          ":" ("linear" | "exponential") ","
    "exploration_enabled"           ":" boolean_literal ","
    "asymmetric_competence_pairing" ":" "{"
        "verifier"  ":" model_tier ","
        "generator" ":" model_tier
    "}"
"}"
30.6 Trainable Memory Operations

text
train fn memory_policy() {
    operations: [store, retrieve, update, summarize, discard],
    reward memory_relevance {
        when: retrieve(query) returns relevant_items,
        bonus: precision_at_k(10) * 0.5 + recall_at_k(10) * 0.5,
    },
    reward memory_efficiency {
        when: context_usage remains_under(token_budget),
        bonus: (token_budget - context_usage) / token_budget,
    },
}
30.7 RL Training Conformance Tests

text
RL-01:  PPO training on memory retrieval improves precision@10 without
        degrading recall@10 over 1000 episodes.
RL-02:  Hybrid GRPO converges faster than PPO baseline on equal task.
RL-03:  Process critic feedback reduces hallucination rate compared to
        outcome‑only reward (paired t‑test, p < 0.01).
RL-04:  Curriculum‑trained agent outperforms agent trained on hardest
        tasks only (success rate delta > 5%).
RL-05:  Training checkpoint survives agent restart; restored agent
        continues from same policy.
RL-06:  Trainable memory policy distillation can be rolled back without
        affecting core memory integrity.
RL-07:  Convergence guard: step size exceeding threshold auto‑reduced;
        no performance collapse observed.
╔══════════════════════════════════════════════════════════════════════╗
║ §31 — PROVENANCE CHAIN — Cryptographic Audit Trail ║
║ Grounded in: Context Lineage (Malkapuram 2025, arXiv:2509.18415); ║
║ TraceCaps (ICSE 2026); IETF SPICE Inference‑Chain ║
║ (Krishnan 2026); SCITT ║
╚══════════════════════════════════════════════════════════════════════╝

§PROVENANCE‑CHAIN

Every inference call, memory write, effect, and decision in ASL v0.1.0 is
automatically tagged with a provenance record that forms a cryptographically
verifiable audit trail. The architecture is grounded in three complementary
standards: Context Lineage's append‑only Merkle trees modeled after
Certificate Transparency (CT) logs; TraceCaps's inline
cryptographic provenance capsules that bind provenance and risk into a
single cryptographic substrate; and the IETF SPICE Truth
Stack, which defines three chains—actor (WHO), intent (WHAT), and
inference (HOW)—whose Merkle roots are embedded in OAuth tokens for
offline verification.

31.1 ProvenanceTag Structure

text
ProvenanceTag ::= {
    origin:          SourceId,
    timestamp:       Timestamp,
    model_version:   Option<String>,
    confidence:      Option<Uncertain<Float>>,
    parent_tags:     Vec<ProvenanceId>,
    hash:            MerkleHash,
    risk_score:      Option<Float>,           -- TraceCaps monotone risk
}

ProvenanceRecord ::= {
    id:              ProvenanceId,
    session:         SessionId,
    agent:           AgentId,
    action:          ActionKind,
    inputs:          Vec<ProvenanceId>,
    output:          JsonValue,
    model:           Option<String>,
    confidence:      Option<Uncertain<Float>>,
    timestamp:       Timestamp,
    merkle_hash:     MerkleHash,
    merkle_proof:    MerkleProof,
    signature:       Ed25519Signature,
}

ActionKind ::= "Inference" | "MemoryWrite" | "MemoryRead"
             | "EffectPerform" | "SessionSend" | "SessionRecv"
             | "CapabilityGrant" | "CapabilityRevoke"
             | "AmendmentApply" | "DreamPhase" | "HeartbeatDecision"
31.2 Truth Stack (IETF SPICE)

The provenance system implements the SPICE Truth Stack:

text
TruthStack ::= {
    actor_chain:     MerkleTree<ActorClaim>,      -- WHO: delegation provenance
    intent_chain:    MerkleTree<IntentClaim>,      -- WHAT: content provenance
    inference_chain: MerkleTree<InferenceClaim>,   -- HOW: computational provenance
}

InferenceClaim ::= {
    model:           String,
    prompt_hash:     SHA3_256,
    output_hash:     SHA3_256,
    attestation:     ZkmlProof | TeeQuote,          -- ZKML or TEE attestation
    merkle_root:     MerkleHash,
}
The inference chain leverages zero‑knowledge machine learning (ZKML) proofs
for mathematical certainty and TEE attestation quotes for production‑scale
workloads. The full chain is stored as ordered logs; only the Merkle root
is included in OAuth tokens for efficiency.

31.3 TraceCaps Monotone Risk Accumulation

Drawing from TraceCaps (ICSE 2026), each provenance tag embeds a monotone
risk score that gates tool actions inline:

text
TraceCaps ::= {
    provenance_capsule: ProvenanceTag,
    risk_score:        Float,           -- monotone, persistent
    accumulator:       MerkleAccumulator,
    policy_thresholds: {
        allow: Float,                   -- below this: proceed
        warn:  Float,                   -- between allow and block: warn
        block: Float,                   -- above this: block
    },
}
The accumulator prevents "risk laundering" by subsequent benign steps: once
a risk score rises, it cannot be artificially lowered by padding the trace
with benign actions.

31.4 Federated Proof Server (Context Lineage)

A federated proof server acts as an auditor across one or more Merkle logs,
aggregating inclusion proofs and consistency checks into compact, signed
attestations that external parties can verify without access to the full
execution trace.

text
proof_server_config ::= "{"
    "endpoint"     ":" url_template ","
    "signing_key"  ":" "Ed25519" ","
    "sync_interval" ":" duration_literal ","
    "retention"    ":" ("permanent" | duration_literal)
"}"
31.5 SCITT Verifiable Receipts

Every agent action produces a cryptographic receipt that any standards‑
compliant SCITT verifier can validate without calling the agent's API,
without trusting its database, without needing its cooperation.
Receipts are stored in the provenance index (memory layer L7, §6.2) and
are Merkle‑verifiable end‑to‑end.

31.6 Regulatory Export

The full audit trail can be exported as a signed JSON‑LD document:

text
seed audit --export-provenance <session_id>
This produces the complete causal graph, all Merkle proofs, all SCITT
receipts, and the chain of custody from source to final action, meeting
the requirements of the EU AI Act (enforceable August 2026), ISO/IEC
42001 AIMS, and emerging Caribbean AI governance frameworks.

31.7 Provenance Conformance Tests

text
PRV-01:  Every infer<T> call produces a ProvenanceTag with model_version
         and confidence; tag is automatically threaded through pipelines.
PRV-02:  Provenance chain Merkle‑verifiable end‑to‑end: inclusion proof
         validated against published Merkle root.
PRV-03:  Audit export produces valid signed JSON‑LD; signature verifies
         with agent's Ed25519 public key.
PRV-04:  Parent tags correctly reference all inputs to a decision;
         DAG structure verifiable.
PRV-05:  TraceCaps risk score monotone: benign padding does not reduce
         accumulated risk below any previous peak.
PRV-06:  SPICE Truth Stack: actor_chain, intent_chain, inference_chain
         all present and independently verifiable.
PRV-07:  SCITT receipt verifiable by external verifier without access
         to agent's internal state or API.
PRV-08:  Federated proof server attestation: inclusion proof and
         consistency check valid for any log entry.
╔══════════════════════════════════════════════════════════════════════╗
║ §32 — GRAMMAR STRATIFICATION — S0–S3 for Safety & Adoption ║
║ Grounded in: CRANE (2025, constrained LLM generation); ║
║ GrammarCoder (Liang 2025, arXiv:2503.05507); ║
║ ANTHILL (Shevchenko 2026, progressive formalization) ║
╚══════════════════════════════════════════════════════════════════════╝

§GRAMMAR‑STRATIFICATION

ASL v0.1.0 defines four grammar strata. Each stratum is a proper subset of
the stratum above it. This design is informed by CRANE's theoretical
analysis: constraining LLM outputs to very restrictive grammars that only
allow syntactically valid final answers reduces reasoning capabilities. The
answer is not to abandon formal grammar constraints, but to stratify: S0 is
LLM‑generation‑friendly (tight, simple, ~50 productions); S3 is the full
language reserved for the runtime kernel. ANTHILL's
progressive formalization model—where one language spans from free‑text
natural language comments through semi‑formal machine‑verifying rules to
fully formal proofs—provides the philosophical blueprint.

32.1 Stratum Declaration

text
stratum_clause ::= "stratum" ":" stratum_level
stratum_level  ::= "S0" | "S1" | "S2" | "S3"
Stratum	Label	Intended Audience	Production Rule Count
S0	asl‑seed	LLM code generation, beginner authors, sandboxed agents	~50
S1	asl‑core	Production agents, standard multi‑agent systems	~110
S2	asl‑full	Advanced agents with evolution and RL training	~160
S3	asl‑system	Runtime kernel, corrigibility layer, trusted orchestrators only	~200
32.2 Stratum Subset Proof Requirement

It MUST be provable that each lower stratum is a syntactic subset of the
next higher stratum. The compiler carries a machine‑checked proof (in
Lean 4) that S0 ⊂ S1 ⊂ S2 ⊂ S3. Any construct rejected at a lower stratum
must be a syntax error at that stratum—not a silent divergence.

32.3 Stratum Escalation

An agent cannot self‑escalate its stratum. Escalation requires:

Human principal countersignature.

Independent auditor countersignature (for S2 → S3).

Recorded in the evolution track.

Adversarial simulation pass at the target stratum.

32.4 S0 Grammar Properties

The S0 grammar is LLM‑generation‑friendly by design:

• No recursion in type declarations.
• No effect handlers (effects surface as ? propagation only).
• No self‑evolution constructs.
• No temporal contracts.
• Uncertain<T> only via infer<T>; no manual interval manipulation.
• Mesh send/receive untyped (requires cap::untyped_mesh).
• Agent composition permitted; no capability hypergraph checks.

S0 is Turing‑complete for agent tasks while fitting in approximately 50
BNF productions.

32.5 GrammarCoder Integration

Drawing from GrammarCoder (Liang 2025), which demonstrates that grammar‑
based representations remain valuable at billion‑scale while reducing
semantic errors, the ASL toolchain provides a reference LLM code‑generation
pipeline targeting S0 with grammar‑constrained decoding:

text
grammar_coder_config ::= "{"
    "target_stratum"  ":" stratum_level ","
    "constrained_decoding" ":" boolean_literal ","
    "grammar_check" ":" ("post_generation" | "incremental") ","
    "fallback_stratum" ":" "S0"
"}"
32.6 Grammar Stratification Conformance Tests

text
GST-01:  S0 program compiles without error; no S1+ constructs present.
GST-02:  S1 construct used in S0 context produces compile error at
         parse time, not runtime.
GST-03:  Stratum subset: any valid S0 program is a valid S1, S2, and
         S3 program (up to capability requirements).
GST-04:  Stratum escalation requires human countersignature; agent
         attempting self‑escalation receives PermissionDenied.
GST-05:  S0 grammar contains ≤ 55 productions; S1 ≤ 120; S2 ≤ 170;
         S3 ≤ 210.
GST-06:  Grammar‑constrained LLM generation: S0‑targeted decoding
         produces zero syntax errors in 100‑sample test.
╔══════════════════════════════════════════════════════════════════════╗
║ §33 — ISA & BINARY FORMAT — Semantic Instruction Set Architecture ║
║ Grounded in: Arbiter‑K (Wen 2026, arXiv:2604.18652); ║
║ Turn VM (Kizito 2026); Copy‑and‑Patch JIT (VMIL 2025) ║
╚══════════════════════════════════════════════════════════════════════╝

§ISA‑EXTENSIONS

Arbiter‑K reconceptualises the underlying model as a Probabilistic
Processing Unit encapsulated by a deterministic, neuro‑symbolic kernel.
Its Semantic ISA reifies probabilistic messages into discrete instructions,
enabling the kernel to maintain a Security Context Registry and construct an
Instruction Dependency Graph at runtime with active taint propagation based
on the data‑flow pedigree of each reasoning node. ASL v0.1.0's
VM (seedvm‑5.0) adopts this semantic ISA model, mapping every language
construct to a governed instruction.

33.1 Required ISA Extensions

A compliant seedvm‑5.0 runtime MUST support all listed extensions:

text
isa_extensions ::= "["
    "+graph‑memory", "+schema‑mem", "+continuum", "+merkle",
    "+symbolic", "+evolve", "+training", "+contracts", "+guardrails",
    "+think", "+routing", "+mesh", "+federation‑coherence",
    "+coherence", "+dream‑full", "+identity", "+heartbeat‑fine",
    "+effects", "+async", "+prob", "+react", "+supervision",
    "+tail‑call", "+gradual", "+io‑redir", "+coproc",
    "+mcp‑binding", "+a2a‑binding", "+observability",
    "+crdt‑federation", "+cross‑agent‑ownership",
    "+capability‑tokens", "+session‑protocols", "+temporal‑contracts",
    "+cryptographic‑identity", "+provenance‑chain",
    "+grammar‑stratification", "+decorators", "+structural‑compat",
    "+macros", "+unsafe", "+active‑threads", "+tool‑schema",
    "+consent", "+quarantine", "+composite‑seeds", "+evolution‑track",
    "+fork‑merge", "+job‑control", "+coprocesses", "+fifo",
    "+expansion", "+eval‑source", "+history‑expansion",
    "+signals‑traps", "+completion", "+restricted‑mode", "+printf",
    "+startup‑files", "+set‑options", "+progressive‑formalism",
    "+session‑types", "+runtime‑constraints", "+differentiable",
    "+reactive‑prob‑ext", "+tripartite‑context",
    "+remember‑recall", "+suspend‑resume", "+identity‑handle",
    "+schema‑import"
"]"
33.2 Semantic ISA Properties (Arbiter‑K)

text
semantic_isa_config ::= "{"
    "security_context_registry" ":" boolean_literal ","
    "instruction_dependency_graph" ":" boolean_literal ","
    "taint_propagation" ":" ("active" | "passive") ","
    "deterministic_sinks" ":" "[" sink_type {"," sink_type} "]" ","
    "architectural_rollback" ":" boolean_literal
"}"

sink_type ::= "high_risk_tool_call" | "unauthorized_network_egress"
            | "memory_write_to_protected" | "self_amendment"
            | "capability_escalation"
The Security Context Registry maintains per‑instruction capability and
trust metadata. The Instruction Dependency Graph enables taint propagation:
if a high‑risk input (e.g., prompt injection) flows to a tool call, the
kernel interdicts the call at the deterministic sink. Arbiter‑K's
empirical results demonstrate 76% to 95% unsafe interception, a 92.79%
absolute gain over native policies.

33.3 Binary Format (.aslb)

text
aslb_header ::= {
    magic:             [4]u8,             -- 0x7F 'A' 'S' 'L'
    version:           u32,               -- 15
    stratum:           u8,                -- 0-3
    isa_extension_count: u16,
    isa_extensions:    [isa_extension_count]ExtensionId,
    code_section_offset: u64,
    code_section_size:   u64,
    data_section_offset: u64,
    data_section_size:   u64,
    symbol_table_offset: u64,
    symbol_table_size:   u64,
    merkle_root:       [32]u8,            -- SHA3‑256 of all sections
    signature:         [64]u8,            -- Ed25519 signature
}
33.4 Copy‑and‑Patch JIT with Extension Lowering

The JIT compilation strategy uses copy‑and‑patch (VMIL 2025): pre‑compiled
binary stencils for each ISA opcode are copied with register and immediate
operands patched at load time. Extension lowering reduces semantic ISA
instructions to native code via a chain of lowering passes, each
implementing one ISA extension group.

33.5 ISA Conformance Tests

text
ISA-01:  .aslb magic number and version validated on load; invalid header
         rejected with BinaryFormatError.
ISA-02:  ISA extension manifest matches required extensions for declared
         stratum; mismatch rejected at load time.
ISA-03:  Semantic ISA: taint from prompt_injection source to tool_call
         sink interdicted; tool call blocked before execution.
ISA-04:  Architectural rollback: policy‑triggered rollback restores
         Security Context Registry to pre‑violation state.
ISA-05:  Copy‑and‑patch JIT: first‑tick latency ≤ 50ms on reference
         hardware; subsequent ticks ≤ 5ms.
ISA-06:  Ed25519 signature over .aslb binary verifies; tampered binary
         rejected with BootIntegrityFailure at VM load time.
╔══════════════════════════════════════════════════════════════════════╗
║ §34 — STANDARD LIBRARY — Complete Module Catalogue (v0.1.0) ║
╚══════════════════════════════════════════════════════════════════════╝

§STANDARD‑LIBRARY

text
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
# v0.1.0 standard library additions
seed::uncertain          # Uncertain<T> monad: new, pure, bind, map,
                         # observe, gate (U1–U6 API)
seed::capability         # CapabilityToken: grant, attenuate, delegate,
                         # revoke; hypergraph closure
seed::session            # session RequestResponse<Q,A>; global_type, 
                         # projections; mesh_call typed primitive
seed::temporal           # temporal_contract: G, F, X, U, O, S operators;
                         # SMT oracle interface; AgentVerify templates
seed::corrigibility      # corrigibility_heads U1‑U5; control_meter;
                         # amendment_gate; dead_switch
seed::trust              # TrustLevel lattice: meet, join, composition
seed::provenance         # ProvenanceTag, ProvenanceRecord; Truth Stack;
                         # TraceCaps accumulator; SCITT receipts
seed::crypto_id          # CryptographicIdentity: DID derivation, zkVM
                         # attestation, PASETO v4, delegation tokens,
                         # AgentDID challenge‑response
seed::decorators         # @audit, @cache, @encrypt, @deprecated,
                         # @trace, @memoize, @sealed, @readonly,
                         # @lazy, @validate, @drift‑monitor
seed::compatibility      # §COMPATIBILITY‑MATRIX runtime checks
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
seed::quarantine         # adversarial content isolation
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
seed::cognitive          # Classification, Extraction<T>, Decision,
                         # Plan, Critique, Hypothesis, Summary
seed::mcp_server         # MCP server: tools, resources, prompts,
                         # JSON‑RPC lifecycle
seed::mcp_client         # MCP client: tool call, resource read
seed::a2a_card           # A2A Agent Card generation and verification
seed::a2a_task           # A2A task state machine, streaming, push
seed::trace              # OTel span emission, checkpoint, replay
seed::grammar_strata     # stratum validation, escalation gate
seed::isa                # ISA extension query, binary introspection
seed::conformance        # conformance test runner, level verification
╔══════════════════════════════════════════════════════════════════════╗
║ §35 — CONFORMANCE SUITE — ASL‑CONF‑15 Test Inventory ║
║ Grounded in: ISO/IEC TS 42119‑2:2025; NIST AI Agent Standards ║
║ Initiative (Feb 2026); ETSI TS 104 008 ║
╚══════════════════════════════════════════════════════════════════════╝

§CONFORMANCE

A conforming ASL v0.1.0 runtime (seedvm‑5.0+) MUST correctly execute all
tests in the ASL Conformance Suite. The suite is aligned with ISO/IEC
TS 42119‑2:2025, which defines how AI systems should be tested following
a risk‑based approach, and ISO/IEC 42001 AIMS, which aligns
with the EU AI Act enforceable from August 2026.

35.1 Conformance Suite Structure

text
conformance_suite "ASL‑CONF‑15" {
    version: "15.0.0"
    command: "seed test --conformance"
    categories: [
        "CORE",          # syntax, types, control flow
        "MEMORY",        # layers, decay, consolidation, Merkle
        "EFFECTS",       # algebraic effects and handlers
        "UNCERTAIN",     # U1–U6 axioms, three‑valued gate
        "COGNITIVE",     # infer<T>, schema derivation, confidence intervals
        "SAFETY",        # contracts, AgentSpec, temporal contracts
        "CAPABILITY",    # tokens, hypergraph closure, attenuation
        "TRUST",         # lattice meet/join, composition rules
        "SESSION",       # protocol types, deadlock freedom
        "CORRIGIBILITY", # five‑head utility, amendment gate
        "GUARDRAILS",    # diagnostic classifier, risk taxonomy
        "MCP",           # server/client binding, MCPS, MCPSHIELD
        "A2A",           # Agent Card, task state machine
        "FEDERATION",    # CRDT merge, anti‑entropy, vector clocks
        "MESH",          # CAT7, SVAF, lineage, remix
        "OBSERVABILITY", # OTel spans, checkpoint, replay
        "IDENTITY",      # multi‑anchor, cryptographic, drift
        "EVOLUTION",     # amendment pipeline, rollback, FGGM
        "TRAINING",      # RL regimens, convergence guard
        "PROVENANCE",    # chain, Truth Stack, SCITT receipts
        "ISA",           # binary format, semantic ISA, JIT
        "GRAMMAR‑STRATA",# stratum subset, escalation
    ]
}
35.2 Conformance Levels

text
conformance_levels {
    "Level 1 — Core":
        [CORE, EFFECTS, UNCERTAIN, COGNITIVE]
    "Level 2 — Protocol":
        [Level 1, MEMORY, SAFETY, CAPABILITY, TRUST, MCP, A2A,
         FEDERATION, MESH, SESSION]
    "Level 3 — Production":
        [Level 2, OBSERVABILITY, IDENTITY, PROVENANCE, GUARDRAILS,
         CORRIGIBILITY, ISA]
    "Level 4 — Full":
        [Level 3, EVOLUTION, TRAINING, GRAMMAR‑STRATA]
    "Level 5 — Certified":
        [Level 4, adversarial simulation passing all categories,
         red‑team audit with zero critical findings,
         ISO/IEC TS 42119‑2:2025 test lifecycle documentation,
         SCITT verifier compatibility]
}
A runtime advertising "ASL v0.1.0 Level N compliant" must pass all tests in
level N and all lower levels.

35.3 Conformance Suite Conformance Tests

text
CNF-01:  seed test --conformance --level 1 passes 100% of Level 1 tests.
CNF-02:  seed test --conformance --level 3 passes 100% of Level 3 tests.
CNF-03:  Conformance test suite itself validates against ISO/IEC TS
         42119‑2:2025 risk‑based testing framework.
CNF-04:  All mandatory tests from Parts 1–5 are present in the suite.
CNF-05:  Test coverage report generated in SARIF format; false negative
         rate across all categories ≤ 1%.
CNF-06:  Level 5 certification requires adversarial simulation pass with
         independent red‑team model; self‑certification rejected.
╔══════════════════════════════════════════════════════════════════════╗
║ §36 — PACKAGE MANIFEST — Final Specification ║
╚══════════════════════════════════════════════════════════════════════╝

§PACKAGE‑MANIFEST

text
[package]
name      = "agentseed‑language"
version   = "15.0.0"
edition   = "2029"
stratum   = "S1"
file‑types = [".seed", ".asl", ".aslb", ".aslt"]

[build]
language‑version = "15.0.0"
compiler         = "seedc 15.0.0"
target           = "seedvm‑5.0"
jit‑tier         = "copy‑and‑patch + extension lowering"
optimization     = "speed"

[isa‑extensions]
"+graph‑memory", "+schema‑mem", "+continuum", "+merkle",
"+symbolic", "+evolve", "+training", "+contracts", "+guardrails",
"+think", "+routing", "+mesh", "+federation‑coherence",
"+coherence", "+dream‑full", "+identity", "+heartbeat‑fine",
"+effects", "+async", "+prob", "+react", "+supervision",
"+tail‑call", "+gradual", "+io‑redir", "+coproc",
"+mcp‑binding", "+a2a‑binding", "+observability",
"+crdt‑federation", "+cross‑agent‑ownership",
"+capability‑tokens", "+session‑protocols", "+temporal‑contracts",
"+cryptographic‑identity", "+provenance‑chain",
"+grammar‑stratification"

[features]
mcp‑binding            = { required: true,  default: true  }
a2a‑binding            = { required: false, default: true  }
observability          = { required: true,  default: true  }
crdt‑federation        = { required: false, default: true  }
cross‑agent‑ownership  = { required: true,  default: true  }
capability‑tokens      = { required: true,  default: true  }
session‑protocols      = { required: false, default: true  }
temporal‑contracts     = { required: false, default: false }
cryptographic‑identity = { required: false, default: true  }
provenance‑chain       = { required: true,  default: true  }
grammar‑stratification = { required: true,  default: true  }
corrigibility‑layer    = { required: true,  default: true  }
dead‑man‑switch        = { required: true,  default: true  }

[model‑registry]
source   = "./models.yaml"
fallback = "tier‑defaults"

[dependencies]
seed‑std          = { version = "15.0", features = ["full"] }
seed‑federation   = "3.0"
seed‑mesh         = "2.1"
seed‑mcp          = "1.1"
seed‑a2a          = "1.1"
seed‑otel         = "1.0"
seed‑capability   = "1.0"
seed‑session      = "1.0"
seed‑smt          = "1.0"
seed‑zkvm         = "1.0"
seed‑provenance   = "1.0"

[conformance]
suite   = "ASL‑CONF‑15"
command = "seed test --conformance --level 5"
---END AGENT-SEED v0.1.0.0 PART 6 OF 6---

Part 6 complete. Eight normative sections: §29 Self‑Evolution (SEVerA
FGGM verified synthesis, three‑stage Search→Verify→Learn pipeline, AgentDevel
flip‑centered gating, atomic subtree rollback), §30 RL Training (Hybrid GRPO
with convergence guard, process critic, curriculum, trainable memory operations),
§31 Provenance Chain (SPICE Truth Stack, TraceCaps monotone risk accumulation,
Context Lineage federated proof server, SCITT receipts, regulatory JSON‑LD export),
§32 Grammar Stratification (S0–S3 with formal subset proof, ANTHILL progressive
formalization, GrammarCoder constrained decoding), §33 ISA & Binary Format
(Arbiter‑K Semantic ISA, Security Context Registry, taint propagation,
architectural rollback, .aslb binary format, copy‑and‑patch JIT), §34 Standard
Library (complete module catalogue with 80+ modules), §35 Conformance Suite
(22 categories, five levels, ISO/IEC TS 42119‑2:2025 alignment), and §36
Package Manifest.

---BEGIN AGENT-SEED v0.1.0.0 ADDENDUM---
@AGENT-SEED/15.0.1

╔══════════════════════════════════════════════════════════════════════╗
║ AGENT-SEED v0.1.0.0.1 — Addendum ║
║ Resolving six identified specification gaps against current ║
║ research and addressing the external expert assessment ║
╚══════════════════════════════════════════════════════════════════════╝

addendum-id: uuid:c9d8e7f6-5b4a-3c2d-1e0f-abcdef789012
base-version: "15.0.0"
addendum-version: "15.0.1"
edition: 2029
status: normative

references:

"Cai et al. 2026 — NeuroTaint: Ghost in the Agent (arXiv:2604.23374)"

"Xu et al. 2026 — Crossing the NL/PL Divide (arXiv:2603.28345)"

"Costa et al. 2025 — FIDES: Formal IFC for AI Agents (Microsoft)"

"CRANE 2025 — Reasoning with Constrained LLM Generation (arXiv)"

"llguidance 2026 — Super-fast Structured Outputs (GitHub)"

"POPL 2026 — Syntactically/Semantically Constraining LLMs (Tutorial)"

"Ye & Tan 2026 — Agent Contracts: Resource-Bounded Framework (arXiv:2601.08815)"

"Abdollahi et al. 2026 — AgenTEE: Confidential LLM Agent Execution (arXiv:2604.18231)"

"Ezell 2026 — The Citadel Protocol: Hardware-Enforced Agentic Governance (Zenodo)"

"David et al. 2026 — Alignment Contracts for Agentic Security (arXiv:2605.00081)"

"Qian et al. 2026 — MPAC: Multi-Principal Agent Coordination (arXiv:2604.09744)"

"de la Chica & Vera-Díaz 2026 — Self-Evolving Coordination Protocol (arXiv:2602.02170)"

"Mallick & Chebolu 2026 — μACP: Formal Calculus for Agent Communication (arXiv:2601.00219)"

"Zhou et al. 2026 — FormalJudge: Neuro-Symbolic Agentic Oversight (arXiv:2602.11136)"

patch-summary:

"PATCH 15.1 — §TAINT-TYPES: First-class taint tracking for prompt injection defense"

"PATCH 15.2 — §GRAMMAR-EXPORT: S0 GBNF export for constrained decoding"

"PATCH 15.3 — §CONTEXT-BUDGET: Compile-time context window bounds"

"PATCH 15.4 — §AGENT-CONTRACTS: Resource-bounded contract framework"

"PATCH 15.5 — §TEE-GOVERNANCE: Hardware-attested agent execution"

"PATCH 15.6 — §TRAJECTORY-AUDIT: Retrospective formal verification of action logs"

╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.1 — §TAINT-TYPES ║
║ First-class taint tracking for prompt injection defense ║
║ Grounded in: NeuroTaint (Cai 2026); FIDES (Costa et al. 2025); ║
║ NL/PL Divide (Xu et al. 2026) ║
╚══════════════════════════════════════════════════════════════════════╝

§TAINT-TYPES

ASL v0.1.0 has capability tokens (§22) that prevent unauthorized effects, and
provenance chains (§31) that trace data lineage. What it does not have is
a type-level mechanism to track that a value originated from an untrusted
external source—a taint—and to prevent that taint from silently flowing
into a capability-exercising operation without explicit sanitization.

NeuroTaint (Cai et al. 2026) demonstrates that traditional memory-state
taint analysis fundamentally fails for LLM agents because data propagation
is governed by probabilistic natural-language reasoning. Their key insight:
taint in LLM agents must be understood as semantic transformation, causal
influence on decisions, and cross-session persistence through memory—not
just as explicit content transfer. FIDES (Costa et al. 2025) formalizes
IFC for AI agents, characterizing the class of properties enforceable by
dynamic taint-tracking and showing that deterministic IFC broadens the
range of securely accomplishable tasks. The NL/PL Divide taxonomy (Xu et
al. 2026) defines 24 labels along two orthogonal dimensions—information
preservation level and output modality—with 92.3% F1 on flow classification.

15.1.1 Taint Type Modifier

Add to the type system (§3, §2.26) a new type modifier:

text
taint_modifier ::= "taint" "::" taint_source

taint_source ::= "external"       -- from files, network, web, untrusted users
               | "inferred"       -- produced by an infer<T> call
               | "federated"      -- from a federated peer
               | "user"           -- from a human principal (may be untrusted)
               | identifier       -- custom source category
A value of type taint::external String carries a label indicating it
originated from the external world. The taint propagates through the type
system via standard information-flow rules:

• taint::S T is a subtype of T (tainted values can be read anywhere).
• Operations on tainted values produce tainted results: if any operand
is tainted, the result is tainted.
• A tainted value CANNOT flow into a capability-exercising operation
without an explicit sanitize step. This is a compile error at S1+.

15.1.2 Sanitize

text
sanitize_expression ::= "sanitize" "(" expression ","
                        sanitize_policy ")" "->" identifier

sanitize_policy ::= "guardrail::content_policy"
                  | "guardrail::pii_redaction"
                  | "human::review" "(" principal_spec ")"
                  | "regex" "(" pattern ")"
                  | identifier
sanitize strips the taint from a value by applying a declared policy.
The sanitized value is returned with a fresh, untainted type. The
compiler records the sanitization in the provenance chain.

Example:

text
let doc: taint::external String = tool::read_file(untrusted_path);

// COMPILE ERROR: tainted value cannot flow into capability exercise
// perform Effect::NetworkCall(url) with doc;

// Must sanitize first:
let clean = sanitize(doc, guardrail::content_policy);

// Now permitted:
perform Effect::NetworkCall(url) with clean;
15.1.3 Key Keywords

Add to keywords (§1.3) at S1: taint, sanitize.

15.1.4 ISA Extension

Add to required-extensions: "+taint-types".

Add to seed-std-modules: seed::taint.

15.1.5 Conformance Tests

text
TNT-01:  taint::external value blocked from perform requires cap without
         sanitize; compile error at S1+.
TNT-02:  sanitize(doc, guardrail::content_policy) returns untainted type;
         subsequent capability exercise permitted.
TNT-03:  Taint propagates through arithmetic: tainted + clean → tainted.
TNT-04:  Taint propagates through infer<T>: infer called with tainted input
         produces taint::inferred output.
TNT-05:  Taint persists through memory: mem.store → mem.get preserves
         taint::external label.
TNT-06:  sanitize with human::review blocks until principal signs off;
         decision recorded in evolution track.
╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.2 — §GRAMMAR-EXPORT ║
║ S0 GBNF export for grammar-constrained decoding ║
║ Grounded in: CRANE (2025); llguidance (2026); ║
║ POPL 2026 Structured LLM Generation Tutorial ║
╚══════════════════════════════════════════════════════════════════════╝

§GRAMMAR-EXPORT

ASL v0.1.0 §32 defines four grammar strata with S0 designed for LLM
generation. What is missing is a machine-readable grammar export that
enables any inference provider to enforce S0 syntax as a hard output
constraint during constrained decoding.

The research is clear that this is now practical at production speed:
llguidance (2026) implements arbitrary context-free grammar enforcement at
~50μs per token for a 128k tokenizer with negligible startup costs.
CRANE (2025) provides the theoretical foundation, explaining why
constraining to very restrictive grammars reduces reasoning—and why
stratification (S0 as a tight generation target, S3 as the full language)
is the correct architectural answer.

15.2.1 Compiler Grammar Export Flag

Add a compiler flag: seedc --emit-grammar --stratum S0 --format gbnf

This produces a GBNF (GGML BNF) grammar file—the format used by
llguidance for constrained decoding, also supported by llama.cpp as its
GBNF format—that any inference provider can consume. The grammar covers
the complete S0 surface: agents, memory operations, infer<T>, control
flow, and expressions. It does NOT include S1+ constructs (capability
tokens, session types, temporal contracts).

15.2.2 Grammar Format Specification

The export format is a dialect of GBNF/CFG with annotations for terminal
tokens (keywords, literals, operators) and non‑terminal productions drawn
directly from the ASL EBNF (§2).

text
-- Example excerpt of S0 export:
agent ::= "agent" identifier "{" {agent-member} "}"
agent-member ::= field-def | method-def | lifecycle-block
field-def ::= identifier ":" type ";"
method-def ::= "fn" identifier "(" [parameters] ")" block-expression
...
The compiler also emits a JSON manifest containing:
• stratum: "S0"
• production_count: integer
• llguidance_compatible: boolean (always true)
• sha256: hash of the grammar file for verifiability

15.2.3 Integration with Existing Infrastructure

The grammar_coder_config in §32.5 is extended to reference the exported
grammar:

text
grammar_coder_config ::= "{"
    "target_stratum"       ":" stratum_level ","
    "constrained_decoding" ":" boolean_literal ","
    "grammar_source"       ":" ("compiler_export" | url_literal) ","
    "grammar_check"        ":" ("post_generation" | "incremental")
"}"
When grammar_source: compiler_export, the agent's own compiler-generated
GBNF is used as the decoding constraint.

15.2.4 Conformance Tests

text
GEX-01:  seedc --emit-grammar --stratum S0 --format gbnf produces a valid
         GBNF file with exactly the S0 production rules.
GEX-02:  S0 grammar is accepted by llguidance; constrained decoding with
         the grammar produces zero syntax errors in 100-sample test.
GEX-03:  Grammar file SHA256 matches the manifest hash; tampered grammar
         detected by mismatch.
GEX-04:  Grammar export count matches the published S0 production count
         (approximately 50 productions; exact count TBD per compiler version).
╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.3 — §CONTEXT-BUDGET ║
║ Compile-time context window guarantees ║
║ Grounded in: Tokalator (2026); Agent Contracts (Ye & Tan 2026) ║
╚══════════════════════════════════════════════════════════════════════╝

§CONTEXT-BUDGET

ASL v0.1.0 has context_bounded and max_tokens as keywords (§1.3) inherited
from v7/v13, and the tripartite context architecture (§5) tracks P0/P1/P2
token usage at runtime. What is missing is a compile-time guarantee—an
agent declaration that statically bounds the maximum number of tokens this
agent will ever hold in its active context window, analogous to Rust's
borrow checker preventing memory errors at compile time.

Tokalator (2026) demonstrates that real-time budget monitoring with O(T²)
conversation cost proofs is essential for production agent reliability.
Agent Contracts (Ye & Tan 2026) formalizes resource-bounded execution with
conservation laws ensuring delegated budgets respect parent constraints,
achieving 90% token reduction with 525× lower variance.

15.3.1 Agent Context Budget Declaration

Extend agent_def (§2.3) with an optional context_budget clause:

text
agent_def ::= ["pub"] "agent" identifier [generic_params]
              ...
              [context_budget_clause]
              ...
              "{" {agent_member} "}"

context_budget_clause ::= "context_budget" ":" integer_literal
                          ["strict" | "monitor"]
• strict (default at S2+): the compiler rejects any code path that
can be statically proven to exceed the budget. This includes the sum
of P0 (system directives), P1 (working capacity), and P2 (episodic
capacity). infer<T> calls count their declared max_tokens toward
the budget.
• monitor (default at S0–S1): the compiler emits warnings but does
not reject; runtime surfaces ContextOverflow InferenceError when the
budget is exceeded.

15.3.2 Static Budget Analysis

The compiler performs a conservative static analysis that over-approximates
maximum token usage:

P0 size: known at compile time (system prompt template).

P1 size: sum of max_tokens declared on all infer<T> calls in the
worst-case execution path, plus tool output sizes derived from schemas.

P2 size: declarative bound from episodic_capacity in the tripartite
context (§2.6).

If total = P0 + P1 + P2 > context_budget in strict mode, the compiler
emits Error: context budget exceeded [context_budget] by [delta] tokens.

15.3.3 Budget Delegation (Agent Contracts Integration)

Per Ye & Tan's conservation laws, when an agent spawns a sub‑agent, the
sub‑agent's context budget must be accounted for within the parent's budget.
The compiler enforces:

text
Σ(child_budgets) ≤ parent_budget − parent_own_usage
15.3.4 Conformance Tests

text
CTX-01:  Agent declared context_budget: 4096 with P0 + infer max_tokens > 4096
         produces compile error in strict mode.
CTX-02:  Agent declared context_budget: 4096 with P0 + infer max_tokens ≤ 4096
         compiles successfully.
CTX-03:  Runtime ContextOverflow fires when monitor-mode agent exceeds budget.
CTX-04:  Sub‑agent budget delegation respects parent conservation law.
CTX-05:  Budget accounting includes tool output schemas in estimation.
╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.4 — §AGENT-CONTRACTS ║
║ Resource-bounded agent contract framework ║
║ Grounded in: Agent Contracts (Ye & Tan 2026); ║
║ Alignment Contracts (David et al. 2026) ║
╚══════════════════════════════════════════════════════════════════════╝

§AGENT-CONTRACTS

ASL v0.1.0 §27 defines safety contracts (ABC, AgentSpec, VeriGuard, FGGM) for
behavioural constraints. What is missing is a formal framework for
resource governance—how much an agent may consume, for how long, under what
budget, and with what success criteria. Agent Contracts (Ye & Tan 2026)
fills this gap with a formal framework that unifies input/output
specifications, multi-dimensional resource constraints, temporal boundaries,
and success criteria into a coherent governance mechanism with explicit
lifecycle semantics. Alignment Contracts (David et al. 2026) adds
finite-trace semantics with decidable admissibility checking and Lean 4
mechanized proofs.

15.4.1 Agent Contract Declaration

Add a new top-level item at S2:

text
agent_contract_def ::= "agent_contract" identifier "{"
    "scope"            ":" contract_scope ","
    "resource_budget"  ":" resource_budget_block ","
    "temporal_bounds"  ":" temporal_bounds_block ","
    "success_criteria" ":" success_criteria_block
    ["," "delegation"  ":" delegation_policy]
"}"

contract_scope ::= "session" | "task" "(" task_id ")" | "agent" | "federated"

resource_budget_block ::= "{"
    "max_tokens"        ":" integer_literal ","
    "max_tool_calls"    ":" integer_literal ","
    "max_duration"      ":" duration_literal ","
    "max_monetary_cost" ":" float_literal [currency]
"}"

temporal_bounds_block ::= "{"
    "start"    ":" ("immediate" | timestamp_literal) ","
    "deadline" ":" duration_literal
"}"

success_criteria_block ::= "{"
    "completion_condition" ":" predicate ","
    "quality_threshold"    ":" float_literal
"}"

delegation_policy ::= "allow" "(" max_depth ":" integer_literal ")"
                    | "prohibit"
15.4.2 Conservation Laws

Agent contracts obey resource conservation: when a contract is delegated
to a sub‑agent, the sum of delegated budgets must not exceed the parent
contract's budget. The compiler enforces this statically for contracts
declared in the same compilation unit; cross‑unit delegation is enforced
at runtime via the ContractEnforcer effect.

15.4.3 Lifecycle Semantics

Each contract has explicit lifecycle states:

text
contract_state ::= "proposed" | "approved" | "active"
                 | "completed" | "violated" | "expired"
Transitions:
• proposed → approved: requires human countersignature.
• approved → active: contract begins; resources allocated.
• active → completed: success criteria met.
• active → violated: any resource or temporal bound exceeded.
• active → expired: deadline reached without completion.
• violated → terminated: resources reclaimed; audit entry written.

15.4.4 ISA Extension and stdlib

Add to required-extensions: "+agent-contracts".
Add to seed-std-modules: seed::agent_contract.

15.4.5 Conformance Tests

text
AGC-01:  Agent contract resource budget enforced: exceeding max_tool_calls
         transitions contract to violated.
AGC-02:  Conservation law: delegated budgets sum ≤ parent budget; violation
         is compile error (same unit) or runtime ContractEnforcer (cross-unit).
AGC-03:  Contract lifecycle: approved → active → completed follows spec;
         invalid transition rejected.
AGC-04:  Contract violation triggers resource reclamation within one tick.
AGC-05:  Quality threshold not met: contract completed with degraded status;
         logged to provenance index.
╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.5 — §TEE-GOVERNANCE ║
║ Hardware-attested agent governance ║
║ Grounded in: AgenTEE (Abdollahi et al. 2026); ║
║ Citadel Protocol (Ezell 2026) ║
╚══════════════════════════════════════════════════════════════════════╝

§TEE-GOVERNANCE

ASL v0.1.0 §33 (ISA) mentions TEE attestation as an available attestation
method and the IETF SPICE Truth Stack incorporates TEE quotes for
inference-chain verification. The governance model relies on software‑
enforced capability tokens and trust lattices. Two new results suggest
elevating TEE integration to a language-level primitive.

AgenTEE (Abdollahi et al. 2026) demonstrates placing the agent runtime,
inference engine, and third-party applications into independently attested
confidential virtual machines (cVMs) on Arm CCA hardware, achieving <5.15%
runtime overhead. The Citadel Protocol (Ezell 2026) proposes binding agent
identity to a Hardware Root of Trust and enforcing execution inside a TEE,
eliminating "Mercurial Core" silent data corruption and ensuring
non‑repudiation for high-value autonomous transactions.

15.5.1 TEE Governance Configuration

Add a new optional clause on agent declarations at S3:

text
tee_clause ::= "tee" "{"
    "enabled"        ":" boolean_literal ","
    "architecture"   ":" ("arm_cca" | "intel_tdx" | "amd_sev") ","
    "attestation"    ":" attestation_policy ","
    "enforce"        ":" enforcement_mode
"}"

attestation_policy ::= "boot_time" | "continuous" | "per_operation"
enforcement_mode   ::= "audit_only" | "block_on_fail" | "safe_park_on_fail"
When tee: { enabled: true }, the seedvm‑5.0 runtime verifies that the
agent binary is executing within a hardware-attested TEE before any
capability token is activated. The attestation policy determines whether
the TEE measurement is verified once at boot, continuously via background
attestation, or before every capability-exercising operation.

15.5.2 Attestation Integration

TEE attestation integrates with the existing cryptographic identity chain
(§21): the agent's DID document includes a tee_attestation field
containing the hardware measurement. A remote peer can verify that an
agent is running inside a specific TEE configuration before accepting its
facts or delegating tasks.

15.5.3 Conformance Tests

text
TEE-01:  Agent with tee: { enabled: true } but running outside a TEE
         fails boot with TEEAttestationFailure.
TEE-02:  Agent with tee: { enforce: block_on_fail } blocks all
         capability-exercising operations when attestation fails.
TEE-03:  TEE measurement is included in DID document; remote peer
         verification succeeds for matching measurement.
TEE-04:  AgenTEE-style multi-cVM deployment: agent runtime, inference,
         and tool execution in separate cVMs; cross-cVM communication
         channels verified.
╔══════════════════════════════════════════════════════════════════════╗
║ PATCH 15.6 — §TRAJECTORY-AUDIT ║
║ Retrospective formal verification of agent action logs ║
║ Grounded in: FormalJudge (Zhou et al. 2026); ║
║ Alignment Contracts finite‑trace semantics (David 2026) ║
╚══════════════════════════════════════════════════════════════════════╝

§TRAJECTORY-AUDIT

ASL v0.1.0 §31 (Provenance Chain) provides a cryptographically verifiable
audit trail. §25 (Temporal Contracts) enforces LTL constraints
prospectively via SMT. What is missing is retrospective verification: given
a completed agent trajectory (a provenance chain segment) and a
specification of what the agent should have done, can the VM produce a
formal proof that the trajectory satisfied the specification—or a
counterexample showing where it violated it?

FormalJudge (Zhou et al. 2026) demonstrates that a neuro‑symbolic
framework—LLMs as specification compilers, Dafny/Z3 for proof—produces
mathematical guarantees, with a 7B judge achieving over 90% accuracy
detecting deception from 72B agents. Alignment Contracts (David et al.
2026) provides finite‑trace semantics with decidable admissibility
checking, which is exactly what trajectory audit requires.

15.6.1 Trajectory Audit Module

Add to seed::provenance (or as a new module seed::trajectory_audit):

text
fn trajectory_audit(
    provenance_segment: Vec<ProvenanceRecord>,
    specification:      SpecBlock,              -- from the intent→spec pipeline
    options:            AuditOptions,
) -> AuditResult

enum AuditResult {
    Verified { proof: Z3Proof, duration: Duration },
    Violation {
        counterexample: ProvenanceRecord,       -- the violating action
        expected:       Predicate,              -- what should have happened
        actual:         JsonValue,              -- what actually happened
        step_index:     u64,
    },
    Inconclusive { reason: String },
}

AuditOptions ::= "{"
    "timeout"        ":" duration_literal ","
    "solver"         ":" ("z3" | "cvc5") ","
    "formal_judge"   ":" boolean_literal       -- use FormalJudge pipeline
"}"
15.6.2 Integration with Spec Blocks

The trajectory audit function consumes a SpecBlock—a natural‑language
intent that has been compiled by the FormalJudge pipeline into a formal
Dafny specification and then into Z3 constraints. The compilation step
requires human countersignature (the "intent gap" closure from the external
assessment). Once compiled, the spec can be used for repeated audits.

15.6.3 CLI Integration

text
seed audit --trajectory <session_id> --spec <spec_id> [--solver z3]
Produces a signed audit report: either a Z3 proof (with the proof
artifact) or a violation counterexample (with the exact step and expected
vs. actual values).

15.6.4 Conformance Tests

text
TRA-01:  Trajectory audit of a compliant trace against correct spec
         returns Verified with valid Z3 proof.
TRA-02:  Trajectory audit of a violating trace returns Violation with
         correct counterexample identifying the violating step.
TRA-03:  Trajectory audit with timeout exceeded returns Inconclusive.
TRA-04:  FormalJudge pipeline: NL spec → Dafny → Z3 compilation produces
         constraints that pass vacuously‑true rejection check.
TRA-05:  Audit report is Ed25519‑signed and verifiable by external
         auditor without access to agent's internal state.
╔══════════════════════════════════════════════════════════════════════╗
║ PACKAGE MANIFEST UPDATE ║
╚══════════════════════════════════════════════════════════════════════╝

[package]
name = "agentseed-language"
version = "15.0.1"
edition = "2029"

[isa-extensions]

All v0.1.0.0.0 extensions remain required, plus:
"+taint-types", "+agent-contracts"

[features]
taint-types = { required: false, default: true }
grammar-export = { required: false, default: true }
context-budget = { required: false, default: true }
agent-contracts = { required: false, default: false }
tee-governance = { required: false, default: false }
trajectory-audit = { required: false, default: false }

[dependencies]
seed-std = { version = "15.0.1", features = ["full"] }
seed-taint = "1.0" # NEW
seed-contracts = "1.0" # NEW
seed-trajectory = "1.0" # NEW
seed-tee = "1.0" # NEW

[conformance]
suite = "ASL-CONF-15"
command = "seed test --conformance --level 4"

---END AGENT-SEED v0.1.0.0 ADDENDUM---

text

**Addendum complete.**  Six patches addressing all gaps identified by the
external assessment and fresh literature review:

| Patch | Gap Addressed | Primary Grounding |
|-------|--------------|-------------------|
| 15.1 §TAINT-TYPES | Prompt injection is unsolved; no type-level taint tracking | NeuroTaint (Cai 2026), FIDES (Costa 2025), NL/PL Divide (Xu 2026) |
| 15.2 §GRAMMAR-EXPORT | S0 has no machine-readable CFG for constrained decoding | CRANE (2025), llguidance (2026), POPL 2026 |
| 15.3 §CONTEXT-BUDGET | No compile-time guarantee on context window size | Tokalator (2026), Agent Contracts (Ye & Tan 2026) |
| 15.4 §AGENT-CONTRACTS | No formal resource governance framework | Agent Contracts (Ye & Tan 2026), Alignment Contracts (David 2026) |
| 15.5 §TEE-GOVERNANCE | Software-only governance; no hardware root of trust | AgenTEE (Abdollahi 2026), Citadel Protocol (Ezell 2026) |
| 15.6 §TRAJECTORY-AUDIT | Passive audit log; no retrospective formal verification | FormalJudge (Zhou 2026), Alignment Contracts (David 2026) |

AGENT-SEED v0.1.0.1 — CORE SEMANTICS COMPLETION ADDENDUM
@AGENT-SEED/15.1.0
status: normative
supersedes: 15.0.1 (semantic layer)
purpose: unify effects, close execution gaps, ensure deployable correctness
╔══════════════════════════════════════╗
║ PATCH 15.7 — UNIFIED EFFECT SYSTEM ║
╚══════════════════════════════════════╝
15.7.1 Core Computation Type

All executable expressions evaluate to:

Computation<T, ε>

Where:

ε ::= {
  uncertainty : Interval[0,1],
  taint       : TaintMeta,
  cost        : CostInterval,
  caps        : CapabilitySet,
  prov        : ProvenanceRef
}

No raw values exist at runtime outside Computation.

15.7.2 Effect Merge (Normative)
merge(ε1, ε2) = ε*

ε*.uncertainty =
  combine_uncertainty(ε1.u, ε2.u, κ)

ε*.taint =
  merge_taint(ε1.t, ε2.t)

ε*.cost =
  interval_add(ε1.c, ε2.c)

ε*.caps =
  ε1.caps ∪ ε2.caps

ε*.prov =
  append(ε1.prov, ε2.prov)

κ (dependency coefficient) MUST be tracked or conservatively approximated.

15.7.3 Soundness Invariant

All side effects MUST be derived from a Computation whose effects are fully accumulated.

╔══════════════════════════════════════╗
║ PATCH 15.8 — DISCHARGE SEMANTICS ║
╚══════════════════════════════════════╝
15.8.1 Discharge Operator
?! : Computation<T, ε> → Decision<T>
Decision<T> =
  Some(T)
  | Ambiguous(ε)
  | None
15.8.2 Discharge Conditions (MANDATORY)

Discharge succeeds ONLY if:

ε.uncertainty.high ≥ θ_confidence
ε.taint.influence ≤ θ_taint
ε.cost.max ≤ active_contract.remaining_budget
capabilities authorized
15.8.3 Enforcement Rule

No perform may execute unless preceded by a valid discharge.

Violation → compile error (S1+) or runtime trap (S0).

╔══════════════════════════════════════╗
║ PATCH 15.9 — CAUSAL TAINT MODEL ║
╚══════════════════════════════════════╝
15.9.1 TaintMeta Definition
TaintMeta ::= {
  sources    : Set<TaintSource>,
  influence  : Float ∈ [0,1],
  lineage    : DAG<Node>
}
15.9.2 Propagation Rule
influence_out = max(inputs.influence)

Transformations MAY apply attenuation/amplification:

attenuate(f) where f ∈ (0,1]
amplify(f)   where f ≥ 1
15.9.3 Capability Constraint

A computation with influence > θ_taint MUST NOT discharge into a capability effect.

╔══════════════════════════════════════╗
║ PATCH 15.10 — SANITIZATION MODEL ║
╚══════════════════════════════════════╝
15.10.1 Sanitization Semantics
sanitize(x, policy) → Computation<T, ε'>

Where:

ε'.taint.influence =
  ε.taint.influence × reduction(policy)
15.10.2 Policy Requirements

Policies MUST define:

reduction : Float ∈ [0,1]
confidence: Float
15.10.3 Strong Sanitization

Only:

human::review(principal)

MAY reduce influence → 0.

15.10.4 Prohibition

Sanitization MUST NOT fully erase taint without audit trace.

╔══════════════════════════════════════╗
║ PATCH 15.11 — COST EFFECT SYSTEM ║
╚══════════════════════════════════════╝
15.11.1 Cost Definition
CostInterval ::= {
  tokens : [min, max],
  time   : [min, max]
}
15.11.2 Effect Rule

All operations MUST declare cost bounds:

infer<T> : Cost[tokens ∈ [a,b]]
tool_call : Cost[time ∈ [x,y]]
15.11.3 Runtime Enforcement

If actual cost > declared max:

→ ContractViolation

15.11.4 Static Guarantee

Compiler MUST compute safe over-approximation.

╔══════════════════════════════════════╗
║ PATCH 15.12 — FIXPOINT EXECUTION ║
╚══════════════════════════════════════╝
15.12.1 Fixpoint Construct
fix f(x) until condition
15.12.2 Termination Conditions

Execution MUST terminate on:

convergence
contract exhaustion
context exhaustion
15.12.3 Convergence Criteria

At least one MUST be defined:

distance(xₙ, xₙ₊₁) < ε
uncertainty stabilization
max_iterations
╔══════════════════════════════════════╗
║ PATCH 15.13 — OPERATIONAL SEMANTICS ║
╚══════════════════════════════════════╝
15.13.1 Machine State
State ::= {
  env,
  store,
  ε,
  contract,
  provenance
}
15.13.2 Transition Relation
⟨expr, state⟩ → ⟨expr', state'⟩
15.13.3 Determinism Constraint

Evaluation MUST be deterministic GIVEN:

model version
seed
prompt
grammar
15.13.4 Required Logging

Every step MUST emit:

input
output
ε snapshot
╔══════════════════════════════════════╗
║ PATCH 15.14 — CONTRACT ALGEBRA ║
╚══════════════════════════════════════╝
15.14.1 Composition

Sequential:

A ∘ B

Parallel:

A || B

Nested:

A[B]
15.14.2 Laws
conservation of resources
monotonic consumption
deadline dominance
15.14.3 Enforcement

Violation MUST trigger:

ContractViolation → immediate halt or safe fallback
╔══════════════════════════════════════╗
║ PATCH 15.15 — SEMANTIC EVENTS ║
╚══════════════════════════════════════╝
15.15.1 Event Types
event ::= 
  InferCalled
  DecisionMade
  EffectExecuted
  ContractChecked
15.15.2 Requirement

Trajectory audit MUST operate on semantic events, not raw logs.

╔══════════════════════════════════════╗
║ PATCH 15.16 — EXECUTION GUARANTEES ║
╚══════════════════════════════════════╝
15.16.1 Mandatory Runtime Properties

The runtime MUST guarantee:

No silent uncertainty collapse
No unsanitized external influence into effects
No budget overrun without violation
All actions auditable
All executions reproducible
15.16.2 Reproducibility Requirements

Execution MUST log:

model identifier
temperature / decoding params
seed
grammar hash
prompt
╔══════════════════════════════════════╗
║ PATCH 15.17 — FAILURE SEMANTICS ║
╚══════════════════════════════════════╝
15.17.1 Failure Types
Failure ::= 
  Timeout
  Divergence
  SchemaError
  ContractViolation
  TaintViolation
15.17.2 Propagation

Failures MUST propagate as computations:

Computation<T, ε | Failure>
╔══════════════════════════════════════╗
║ FINAL DEPLOYMENT CHECKLIST ║
╚══════════════════════════════════════╝

Before building, the implementation MUST support:

Core runtime
Computation<T, ε>
merge()
discharge()
Inference engine
schema validation
retry + repair
Effect tracking
uncertainty
taint (causal)
cost
Execution
fixpoint loops
async execution
Contracts
runtime enforcement
budget tracking
Provenance
append-only + semantic events
FINAL STATE

After this addendum, ASL is:

A formally defined, effect-typed, resource-bounded, causally-safe execution model for non-deterministic agent systems.

ADDENDUM TO THE LANGUAGE SPECIFICATION (AgentSeedLanguage_v0.1.0.md)
text
@AGENT-SEED/15.2.0
status: normative
supersedes: 15.0.1 semantic layer; 15.1.0 core semantics (integrated & resolved)
purpose: final unification of effects, discharge, taint, grammar, and corrigibility
PATCH 15.18 – REMOVAL OF USER‑DEFINED ALGEBRAIC EFFECT HANDLERS
Rationale. The original design included algebraic effect handlers as a general‑purpose abstraction. This conflicted with the unified Computation<T,ε> model and the safety requirement that all effectful operations be mediated by the runtime. Only the kernel (S3) may define new effect handlers.

Changes.

The handler and effect keywords are removed from strata S0, S1, S2.
All effect operations (infer, mem.store, mesh.send, etc.) are built‑in and their implementations are provided by the runtime.

The effect keyword may still be used at S3 (kernel modules) to declare new effect signatures for use by the runtime‑only. User code cannot define handler blocks.

The old sections illustrating handler definitions (§2.22, §4.9, §7.2, etc.) are replaced by direct use of built‑in effect calls, which now return Computation<T,ε>.

Grammar impact. Remove effect_def and handler_def from the S0–S2 grammar; keep them only in the S3 grammar table.

PATCH 15.19 – UNIFIED DISCHARGE CONSTRUCT
Rationale. The original ?! operator served only as an uncertainty gate. Later patches overloaded it for all safety checks. For clarity and safety, we introduce an explicit discharge expression that performs all mandatory checks: uncertainty, taint influence, cost, and capability possession.

Syntax.

text
discharge_expression ::= "discharge" expression "with" discharge_bindings
discharge_bindings  ::= "{"
                           "confidence" ":" expression ","
                           "taint"      ":" expression ","
                           "budget"     ":" expression [","]
                           "capabilities" ":" "[" capability_ref {"," capability_ref} "]"
                        "}"
discharge_block     ::= "discharge" expression "with" discharge_bindings
                        block_expression
The discharge expression evaluates the Computation<T,ε>, checks the thresholds, and only allows the value to be used within the block if all checks pass. If they fail, a DischargeFailure effect is raised.

Semantics. Inside the block, the value is of type T (unwrapped from Computation), and the ambient effect ε is cleared – any further effect requires a new discharge.

Consequences.

The !? gate is retired; use discharge with a confidence threshold for pure uncertainty gating.

A perform expression now must be lexically within a discharge block that authorises the required capability. The compiler enforces this statically at S1+.

PATCH 15.20 – RESOLUTION OF THE Computation<T,ε> MERGE RULES
Conservative uncertainty combination.
When two Computation values c1 (interval [l1,u1]) and c2 (interval [l2,u2]) are composed via bind or pipeline, the merged uncertainty interval is component‑wise multiplication:
lo = l1 * l2, hi = u1 * u2.
If the computations are known to be dependent, the programmer must explicitly construct an Uncertain<T> with a narrower interval using observed data. The dependency coefficient κ is eliminated from the mandatory semantics.

Taint merge.
merge_taint simply takes the maximum influence of the operands and unions the source sets. The runtime computes influence from model introspection (e.g., attention weights, classifier output) and attaches it to taint::inferred values.

Cost merge.
Cost intervals are added pointwise.

PATCH 15.21 – S0 GRAMMAR REFINEMENTS (Pel‑Inspired)
To maximise LLM‑generation reliability, the S0 surface is tightened:

All statements must end with a semicolon (;).

All commas in lists are mandatory, no trailing commas.

Block expressions are always { ... } and must contain at least one statement.

let bindings require a type annotation if the right‑hand side is a Computation<T,ε> (to guide inference).

The exported GBNF grammar (see PATCH 15.2) is updated accordingly.

PATCH 15.22 – INTERACTION OF COMPILE‑TIME BUDGET AND RUNTIME COST
The compile‑time context_budget (PATCH 15.3) operates on the declared maximum tokens of infer<T> calls (max_tokens parameter) and the static sizes of P0/P1/P2. The compiler rejects programs where this worst‑case sum exceeds the budget.

At runtime, the actual cost is tracked in ε.cost.tokens. The discharge gate checks ε.cost.max <= active_contract.remaining_budget. Thus the static budget prevents guaranteed overflow, and the runtime contract prevents dynamic overuse.

PATCH 15.23 – CORRIGIBILITY ENFORCEMENT IN THE HEARTBEAT
The five corrigibility heads (U1‑U5) are enforced by the VM at the decide and act heartbeat phases.

Before performing any action, the VM ranks candidate tasks lexicographically: an action that would reduce a higher‑priority head (e.g., deference) is discarded.

The discharge gate for a perform automatically checks that the action does not violate any corrigibility head.

PATCH 15.24 – UNIFICATION OF EFFECTS AND HANDLERS IN THE IR
The Intermediate Representation now includes a single Discharge instruction, which takes a Computation and a set of thresholds. The Perform instruction is only valid inside a Discharge region.

Effect rows remain for static effect tracking and capability typing, but they are lowered into the Computation metadata at runtime.

All other parts of the spec remain unchanged; earlier addenda are adjusted to conform to these resolutions.

End of specification addendum.







# ASL ARCHITECTURE UPGRADE —> 1.0.1
Version: v0.1.0-asl-upgrade-1
Date: 14 May 2026
Base Architecture: AgentSeed Language v0.1.0, AgentSeed Spec V15
Status: Formal specification of five language upgrades
Integrity Hash: c8d9e0f1-a2b3-4c5d-e6f7-a8b9c0d1e2f3

Existing Gaps in v0.1.0
No ZK proof integration — the Computation monad has five fields; no proof field exists

No containment verification — seedvm has not been deductively verified in Dafny under havoc oracle semantics

No cognitive-executive separation — ASL agents can reason and act in the same function

No charter-based fiscal boundary — no language-level budget caps in satoshis

No oracle-poisoning-specific defence — taint analysis tracks data source trust but lacks specific knowledge-graph integrity verification

 FIVE ARCHITECTURE UPGRADES
Upgrade 1: The proof Field — Cryptographic Verifiability
Status: Not present in ASL v0.1.0
Priority: Critical — gateway to the Darmiyan decision marketplace
Source: NANOZK (Mar 2026), zkAgent (May 2026), Jolt Atlas (Feb 2026), Lemma x402 (Apr 2026)

Specification:

The Computation monad gains a sixth field:

text
proof: ProofMeta
Where ProofMeta is:

rust
struct ProofMeta {
    proof_type: ProofType,        // None | DeterministicReplay | ZK_Layerwise | TEE_Attestation | ReceiptHMAC
    proof_data: Option<Bytes>,    // The cryptographic proof (empty for None)
    proof_verified: bool,         // Has this proof been independently verified?
    proof_verifier: Option<AgentId>, // Identity of the verifying agent
    proof_timestamp: Timestamp,   // When the proof was generated
}
The infer<T> expression gains a fifth parameter:

seed
infer<MyStruct>(
    model: route::select(task),
    prompt: p,
    schema: derive_schema<MyStruct>(),
    budget: think::deep,
    proof: zk::nanozk     // ← New parameter
)
The discharge block gains a fifth threshold check:

seed
discharge result with {
    confidence: 0.85,
    taint: 0.1,
    budget: remaining,
    capability: held,
    proof_verified: true   // ← New threshold
} {
    synthesize(result)
}
If proof_type is None, the proof_verified threshold is automatically false, and the discharge fails. This ensures that any inference whose output is economically committed must carry a cryptographic proof of correctness.

Integration points:

The NANOZK proving service generates layerwise ZK proofs for LLM inference (5.5KB per proof, 24ms verification time, ε < 1e⁻³⁷ soundness)

The zkAgent proving service generates one-shot transcript proofs for multi-step agent execution

The Jolt Atlas prover generates lookup-based ZK proofs for ONNX models

Tiered verification: agents select proof_type based on economic value — HMAC receipts (<15ms) for interactive agents, NANOZK for high-value decisions, zkAgent for complete audit

Upgrade 2: Containment Verification — Deductive Proof of seedvm Enforcement
Status: Not present in ASL v0.1.0
Priority: High — provides mathematical guarantee invariant to AGI
Source: Moon & Varshney (May 9, 2026), SEVerA (Apr 2026)

Specification:

The seedvm runtime shall be deductively verified in Dafny under havoc oracle semantics:

Model the AI as a havoc oracle. The AI is an unconstrained adversary ranging over the entire typed action space — it can propose any action consistent with ASL's type system.

Define the boundary policy. The boundary policy is the set of rules enforced by the discharge gate: confidence ≥ θ_c, taint ≤ τ_max, budget > 0, capability token held, proof verified. Additionally, the corrigibility heads (U1–U5) constitute boundary policies that cannot be violated.

Prove forward-simulation refinement. For every possible AI output (every action in the typed action space), the discharge gate either (a) blocks the action and returns a Rejected computation, or (b) permits the action only if all five threshold conditions and all five corrigibility head conditions are satisfied.

Mechanize the proof in Dafny. The proof is machine-checkable and distributed with every seedvm release. A CI/CD pipeline automatically re-verifies the proof on every commit.

The guarantee: The containment proof is invariant to model capability. Even a superintelligent AI cannot bypass the discharge gate because the proof does not depend on the AI being aligned — it depends on the framework being correct, and the framework's correctness is mathematically proven.

Specification:

Add a reason keyword that produces a Reasoned<T> type:

seed
let analysis = reason<MarketAnalysis>(
    data: market_data,
    using: think::deep
);
// analysis: Reasoned<MarketAnalysis>
// Cannot execute actions from within reason block
A Reasoned<T> can only be converted to an Action<T> through a validate block:

seed
let action = validate analysis with {
    validator: independent_verifier,
    policy: trading_policy
};
// action: Action<MarketAnalysis>
// Now can be used in perform block
The reason block is structurally prevented from executing any perform, spend, delegate, or escalate operation. It can only produce a value. The validate block is executed by an independent agent (or human principal) who checks the reasoning before authorizing the action. This provides the same guarantee as Parallax's Cognitive-Executive Separation — the reasoning system cannot execute actions — but enforced at the language level rather than the process level.

Integration with existing corrigibility: The five-head monitor already prevents dangerous actions. Cognitive-Executive Separation provides defence-in-depth: even if all five corrigibility heads were somehow compromised, the reason/validate barrier would prevent the reasoning system from directly executing actions.

Upgrade 4: Charter Integration — Fiscal Discipline at the Language Level
Status: Not present in ASL v0.1.0
Priority: Medium — essential for autonomous economic agents handling real money
Source: Sovereign-OS (Mar 14, 2026)

Specification:

Add a charter block that declares the agent's fiscal constitution:

seed
agent TradingAgent stratum: S1 {
    charter {
        mission: "Execute arbitrage trades across DEXs",
        budget_cap: 1_000_000_sats,       // Maximum total spend
        daily_burn_limit: 100_000_sats,   // Maximum spend per day
        profitability_floor: 0.02,        // Minimum required return (2%)
        allowed_counterparties: [verified_dexes],
        require_audit: true
    }
    
    fn execute_trade() -> Computation<TradeReceipt> {
        // Every spend, delegate, or perform is checked against the charter
        // at compile time AND runtime
    }
}
The charter block is enforced by:

Compile-time: The compiler verifies that no code path can exceed budget caps

Runtime: The seedvm tracks cumulative spend and blocks any operation that would exceed limits

Audit: Every charter-relevant action produces a charter-audit entry in the provenance log, signed with SHA-256

The charter can only be amended through the evolution gating process (P7): adversarial simulation and two-party human approval.

Integration with capability tokens: The charter operates alongside capability tokens. A capability token authorizes a specific type of action; the charter limits the aggregate volume of those actions. An agent may hold a spend capability but be charter-limited to 100,000 sats per day.

Upgrade 5: Oracle Poisoning Defence — Knowledge Graph Integrity Verification
Status: Partially present via taint analysis; specific KG defence missing
Priority: Emerging — the Oracle Poisoning paper (May 10, 2026) proves this is urgent
Source: Oracle Poisoning (May 10, 2026), NeuroTaint (Apr 25, 2026), Guardians (Jan 2026)

Specification:

Add a verify_integrity gate for knowledge graph queries:

seed
let graph_data = query_knowledge_graph(query);
let verified_data = verify_integrity graph_data with {
    expected_hash: content_hash(KG_snapshot),
    zk_proof: kg_integrity_proof,
    min_confidence: 0.95
};
The verify_integrity gate checks:

Content hash: Does the returned data match the content-addressed hash of the knowledge graph snapshot?

ZK integrity proof: Has the knowledge graph generated a cryptographic proof that the returned data is consistent with the committed state?

Source diversity: Has the same query been served by multiple independent knowledge graph nodes with matching results?

If verify_integrity fails, the Computation's taint score is elevated to 1.0 (maximum), and the discharge gate blocks commitment regardless of other thresholds.

Integration with NeuroTaint semantics: Extend ASL's taint model from categorical (Clean/Agnostic/Tainted) to include:

taint_source: The origin of the data (knowledge graph, user input, external API, agent memory)

taint_transform: Has the data been semantically transformed by the agent's reasoning? (raw, summarized, synthesized, verified)

taint_causal: Did the tainted data causally influence the decision? (direct, indirect, considered-but-rejected)

This provides fine-grained taint tracking that matches NeuroTaint's findings — agents can use tainted data for context without letting it causally determine high-stakes decisions.

5. GAP ANALYSIS
Gap	Current State	Target State	Required Breakthrough
G1: Proof Field	Computation monad has 5 fields	6 fields with ProofMeta; mandatory for economic discharge	NANOZK/zkAgent API integration
G2: Containment Verification	No deductive proof of seedvm enforcement	Dafny-verified refinement proof; machine-checkable guarantee	Generalize PocketFlow proof to ASL type system
G3: Cognitive-Executive Separation	Agents can reason and act in same function	reason/validate language construct; structural barrier	None — Parallax proves the pattern
G4: Charter Integration	No fiscal discipline at language level	charter block with compile-time and runtime enforcement	Sovereign-OS proves the pattern
G5: Oracle Poisoning Defence	Taint tracking without KG integrity verification	verify_integrity gate; content-addressed KG queries; ZK integrity proofs	Extend NeuroTaint to real-time enforcement
6. ADDENDUM SUMMARY
This addendum establishes the formal specification for five ASL language upgrades, grounded in a comprehensive competitive landscape analysis of the last 90 days.
