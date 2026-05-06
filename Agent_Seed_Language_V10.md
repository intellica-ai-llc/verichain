AGENT-SEED v10.0 — The Complete, Runnable Agentic Language
The Definitive Language for Autonomous, Self-Evolving Agentic Systems
ISO/IEC 5230:2027 Final — Complete Language Specification
seed
---BEGIN AGENT-SEED v10.0---
@AGENT-SEED/10.0.0

# ╔══════════════════════════════════════════════════════════════╗
# ║ §1. META-SEED — The Seed's Own Identity                     ║
# ╚══════════════════════════════════════════════════════════════╝

§META-SEED
seed-id: uuid:f1a2b3c4-d5e6-f7a8-b9c0-123456789012
seed-type: autonomous-daemon
edition: 2028
stability: stable
formalism-level: progressive
safety-level: verified
composition-model: category-theoretic
compliance: ISO/IEC 5230:2027/Final
runtime: seed-vm v3.0
heartbeat-enabled: true
dream-cycle-enabled: true
federation-enabled: true

[build]
language-version: "10.0.0"
compiler: "seedc 10.0.0"
target: "seedvm-3.0"
jit-tier: "copy-and-patch"
optimization: "speed"

[profile.dev]
opt-level: 0
debug: true

[profile.release]
opt-level: 3
lto: true

# ╔══════════════════════════════════════════════════════════════╗
# ║ §2. THE HEARTBEAT — KAIROS-Inspired Autonomous Loop        ║
# ║ Source: KAIROS architecture (Claude Code leak, Mar 2026)   ║
# ║ + heartbeat open implementation (uameer, Apr 2026)         ║
# ╚══════════════════════════════════════════════════════════════╝

§HEARTBEAT
# The autonomous tick loop: the agent never sleeps.
# Based on KAIROS architecture (Anthropic, leaked Mar 2026):
#   - KAIROS referenced 150+ times in Claude Code source
#   - Periodic <tick> messages when user is idle
#   - "Anything worth doing right now?" on each tick
#   - 15-second blocking budget with background deferral
#   - SleepTool: explicitly yields when nothing to do
#
# heartbeat (uameer, Apr 2026): open, model-agnostic implementation
#   - Runs silently in background
#   - Reads project context (CLAUDE.md, README.md, or custom)
#   - Decides autonomously whether to act each tick
#   - Append-only daily logs — cannot erase its own history
#   - autoDream: nightly memory consolidation

heartbeat {
  enabled: true,
  interval: 30s,                     # Tick frequency
  idle_threshold: 15s,               # User must be idle this long
  blocking_budget: 15s,              # Max blocking time per action
  background_on_timeout: true,       # Defer to background if budget exceeded
  
  # The core loop
  loop: {
    observe(),                       # Read environment + memory
    decide(),                        # Act or wait?
    act_or_sleep(),                  # Do one thing or call Sleep
    log(),                           # Append-only log entry
    update_memory(),                 # Update working memory
  },
  
  # Sleep tool: "If you have nothing useful to do on a tick, you MUST call Sleep."
  # — src/constants/prompts.ts:L870-L886 (Claude Code leak)
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
  
  # Push notifications: KAIROS had three tools normal Claude Code never saw
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
  
  # GitHub PR subscriptions (KAIROS feature)
  subscriptions: {
    enabled: true,
    sources: ["github", "gitlab"],
    auto_review: true,
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §3. THE DREAM CYCLE — autoDream Memory Consolidation       ║
# ║ Source: KAIROS autoDream (Claude Code leak, Mar 2026)      ║
# ║ + Complementary Learning Systems theory (Xu et al., 2026)  ║
# ╚══════════════════════════════════════════════════════════════╝

§DREAM-CYCLE
# autoDream: nightly memory consolidation.
# "At night, KAIROS runs autoDream — memory consolidation while you sleep."
# — Claude Code leak analysis
#
# Biologically inspired by Complementary Learning Systems theory:
#   - Fast hippocampal exemplar storage (episodic)
#   - Slow neocortical weight consolidation (semantic)
#   - "Agents built exclusively on retrieval implement only the first half"
#     (Xu et al., 2026, Contextual Agentic Memory is a Memo, Not True Memory)

dream {
  schedule: "daily",                 # "daily" | "session_end" | "idle_30min"
  trigger_time: "02:00",             # Run at 2 AM local
  max_duration: 10min,               # Maximum consolidation time
  
  phases: [
    # Phase 1: Review — read through today's observations
    {
      name: "review",
      action: read_observations(since: last_dream),
      output: observation_summary,
    },
    
    # Phase 2: Resolve — find and eliminate contradictions
    {
      name: "resolve",
      action: detect_contradictions(observations),
      resolution: [
        "if factual_conflict → verify_both_sources()",
        "if temporal_conflict → keep_newest(reinforcement_threshold: 3)",
        "if confidence_mismatch → prefer_higher_confidence(min_gap: 0.2)",
      ],
    },
    
    # Phase 3: Consolidate — episodic → semantic transformation
    {
      name: "consolidate",
      action: episodic_to_semantic(
        threshold: reinforcement_count >= 3,
        abstraction: pattern_extraction,
      ),
      # Move from ShortTerm memory to LongTerm knowledge
      # Drop noise, keep patterns
      output: consolidated_knowledge,
    },
    
    # Phase 4: Compress — reduce storage footprint
    {
      name: "compress",
      action: summarize_observations(
        method: llm_summary,
        compression_ratio: 10:1,
        preserve: [decisions, user_preferences, project_context],
      ),
    },
    
    # Phase 5: Prune — apply Ebbinghaus forgetting curve
    {
      name: "prune",
      action: decay_weights(half_life: 30d),
      remove_threshold: weight < 0.1 AND reinforcement_count < 2,
    },
    
    # Phase 6: Write — update MEMORY.md
    {
      name: "write",
      action: update_memory_file(
        path: ".heartbeat/memory/learnings.jsonl",
        schema: gstack_compatible,
      ),
    },
  ],
  
  # Dream journal: append-only, immutable
  journal: {
    path: ".heartbeat/logs/",
    format: "{date}-dream.log",
    immutable: true,                  # Cannot erase its own history
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §4. STIGMERGY SUBSTRATE — Federated Knowledge Fabric        ║
# ║ Source: Stigmem v1.0 (May 2026) + MMP (Xu, Apr 2026)       ║
# ║ + LTS selective memory (Fioresi et al., Feb 2026)           ║
# ╚══════════════════════════════════════════════════════════════╝

§STIGMERGY-SUBSTRATE
# Stigmergy = stigmem + memory: coordination through shared environment.
# "Agents write typed, provenance-tagged facts into a shared substrate.
#  Other agents running later, on different platforms, inside different
#  organizations read those facts and act on them." — Stigmem v1.0
#
# Core principle: no central coordinator, no point-to-point protocol overhead.
# The knowledge environment carries the coordination signal.

# ═══════════════════════════════════════════════════════════════
# 4.1 Fact Schema (Stigmem-compatible)
# ═══════════════════════════════════════════════════════════════

fact_schema {
  # Every fact has: entity, relation, value, source, timestamp, confidence, scope
  entity: URI,                       # What the fact is about
  relation: URI,                     # The predicate
  value: TypedPayload,               # string | number | boolean | JSON
  source: AgentID,                   # Who asserted it
  timestamp: HybridLogicalClock,     # Monotonic across distributed nodes
  confidence: Float(0.0..1.0),       # Decays over time if not re-asserted
  scope: Scope,                      # public | company | team | private
  
  # Facts are immutable. Contradictions are surfaced as first-class conflict records.
  immutability: true,
  contradiction_policy: surface_as_conflict,
}

# ═══════════════════════════════════════════════════════════════
# 4.2 Federation Protocol
# ═══════════════════════════════════════════════════════════════

federation {
  # Peer-to-peer federation with Ed25519-signed handshakes
  handshake: {
    signing_key: Ed25519,
    scope_declaration: required,
    peer_discovery: [mdns, static_config, registry],
  },
  
  # Replication
  replication: {
    strategy: eventual_consistency,
    conflict_detection: vector_clocks,
    scope_enforcement: strict,
    # private-scope facts never leave the creating node
  },
  
  # Fact lifecycle
  fact_lifecycle: {
    write: POST /v1/facts,
    read: GET /v1/facts?entity=X&scope=Y,
    expire: valid_until,
    decay: confidence *= decay_factor(elapsed),
    reassert: reset_confidence(),
  },
}

# ═══════════════════════════════════════════════════════════════
# 4.3 MMP Integration — Cognitive Memory Blocks
# ═══════════════════════════════════════════════════════════════

cognitive_mesh {
  # Four composable MMP primitives:
  # CAT7: seven-field schema for every Cognitive Memory Block
  cmb_schema CAT7 {
    agent_id: AgentID,
    timestamp: ISO8601,
    role: Role,
    content_hash: SHA256,
    parent_hashes: [SHA256],
    anchors: RoleIndexedAnchors,
    payload: CognitiveContent,
  },
  
  # SVAF: field-level evaluation against receiver's role-indexed anchors
  acceptance SVAF {
    by_role: {
      researcher: { accept_if: evidence_strength >= 0.8 },
      validator: { accept_if: verified_by contains trusted_validator },
      auditor: { accept_if: merkle_verified },
    },
    negotiation: {
      when: neither_accept_nor_reject,
      action: send_counter_proposal,
    },
  },
  
  # Inter-agent lineage: content-hash parent tracking
  lineage {
    tracking: content_hash_chain,
    echo_detection: reject_duplicate_hashes,
    provenance: full_traceability,
  },
  
  # Remix: store receiver's own understanding, never raw peer signal
  remix {
    on_accept: evaluate_and_restructure,
    on_reject: log_rejection_reason,
    on_negotiate: send_counter_proposal,
  },
}

# ═══════════════════════════════════════════════════════════════
# 4.4 Selective Memory Sharing (LTS-inspired)
# ═══════════════════════════════════════════════════════════════

selective_sharing {
  # Learned shared-memory mechanism for parallel agentic systems
  # Only share information that is globally useful across teams
  controller: {
    type: learned_admission,
    training: stepwise_rl,
    credit_assignment: usage_aware,
  },
  
  global_memory_bank: {
    access: all_teams,
    admission_control: controller,
    context_growth_control: active,
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §5. DUAL-PROCESS MEMORY — System 1 + System 2               ║
# ║ Source: D-Mem (Yuan et al., Mar 2026) + CLS theory          ║
# ╚══════════════════════════════════════════════════════════════╝

§DUAL-PROCESS-MEMORY
# Two complementary memory systems, dynamically bridged.
# System 1: fast, lightweight vector retrieval for routine queries
# System 2: exhaustive full deliberation for high-stakes queries
# Multi-dimensional Quality Gating policy bridges the two.

system_1 {
  name: "FastRetrieval",
  mechanism: vector_similarity,
  index: hnsw,
  latency: "<50ms",
  token_cost: "minimal",
  use_for: [routine_queries, entity_lookup, fact_verification],
  confidence_threshold: 0.85,        # Below this → escalate to System 2
}

system_2 {
  name: "FullDeliberation",
  mechanism: exhaustive_reading,
  retrieval: multi_query_pipeline,
  verification: cross_reference,
  latency: "<500ms",
  token_cost: "moderate",
  use_for: [complex_reasoning, contradictory_findings, novel_queries],
  performance: "96.7% of Full Deliberation fidelity",
}

quality_gating {
  policy: MultiDimensionalQualityGate,
  dimensions: [
    query_novelty,                    # New topic → System 2
    confidence_of_system_1,           # Low confidence → System 2
    stakes_of_decision,               # High stakes → System 2
    recency_requirements,             # Very recent → System 1 (cache hit)
    contradiction_potential,          # High contradiction risk → System 2
  ],
  decision: learned_router,           # RL-trained gating policy
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §6. TYPED MEMORY — Schema-Constrained with 13 Categories    ║
# ║ Source: Memanto (Abtahi et al., Apr 2026)                   ║
# ╚══════════════════════════════════════════════════════════════╝

§TYPED-MEMORY
# "A universal memory layer challenging the assumption that knowledge graph
#  complexity is necessary for high-fidelity agent memory."
# — Memanto, achieving 89.8% SOTA accuracy with single retrieval query

memory_categories: [
  "user_preference",                 # User likes/dislikes
  "project_context",                 # Current project state
  "decision_record",                 # What was decided and why
  "task_status",                     # What is being done
  "factual_knowledge",               # Verified facts
  "procedural_knowledge",            # How to do things
  "relationship",                    # Entity relationships
  "constraint",                      # Boundaries and rules
  "hypothesis",                      # Unverified conjectures
  "observation",                     # Raw sensory/log data
  "reflection",                      # Self-critique and learning
  "prediction",                      # Forecasts and expectations
  "meta_memory",                     # Memory about memory
]

# Information-theoretic retrieval (Moorcheh engine)
retrieval_engine: {
  type: information_theoretic,
  latency: "<90ms",
  ingestion_cost: "zero",
  queries_required: 1,
  accuracy: "89.8% (LongMemEval SOTA)",
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §7. MEMORY GOVERNANCE — Policy-Driven Lifecycle Management  ║
# ║ Source: MemArchitect (Mar 2026) + SSGM (Mar 2026)           ║
# ╚══════════════════════════════════════════════════════════════╝

§MEMORY-GOVERNANCE
# "Governed memory consistently outperforms unmanaged memory in agentic settings."
# — MemArchitect, 2026

governance_policies {
  # Tri-path loop: Read, Reflect, Background
  tri_path_loop {
    read: {
      trigger: query_arrives,
      action: retrieve_relevant_memory,
      freshness_check: version_control,
    },
    reflect: {
      trigger: response_generated,
      action: evaluate_memory_quality,
      feedback: [relevance_score, accuracy_check, staleness_warning],
    },
    background: {
      trigger: periodic(interval: 5min),
      action: [prune_stale, resolve_conflicts, enforce_privacy, consolidate],
      priority: low,
    },
  },
  
  # Memory lifecycle rules
  lifecycle: {
    decay: exponential(half_life: 30d, min_weight: 0.01),
    conflict_resolution: [
      "newer_fact_preferred (when: recency_delta > 7d)",
      "higher_confidence_preferred (when: confidence_delta > 0.2)",
      "source_trust_weighted (weights: verified > user > agent > inferred)",
      "escalate_to_user (when: no_clear_resolution)",
    ],
    privacy: {
      pii_detection: automatic,
      pii_action: redact_and_encrypt,
      retention_policy: user_defined,
      right_to_delete: supported,
    },
  },
  
  # "The best agent teams won't be the ones with the largest context windows.
  #  They'll be the ones with the most disciplined memory governance."
  # — Agent Memory Architecture 2026
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §8. MEMORY CONSISTENCY — Cache Coherency Protocol           ║
# ║ Source: Multi-Agent Memory from Computer Architecture       ║
# ║        (Yu et al., Mar 2026)                                ║
# ╚══════════════════════════════════════════════════════════════╝

§MEMORY-CONSISTENCY
# "Frames multi-agent memory as a computer architecture problem."
# Three-layer hierarchy: I/O, Cache, Memory
# Two critical protocol gaps: cache sharing, structured access control

memory_hierarchy {
  io_layer: {
    function: agent_to_agent_communication,
    protocol: [pipeline, coproc, message_passing, fifo],
    persistence: transient,
  },
  cache_layer: {
    function: fast_local_access,
    protocol: "MESI-inspired coherence protocol",
    operations: [Modified, Exclusive, Shared, Invalid],
    invalidation: write_invalidate,
    sharing: cache_coherence(protocol: directory_based),
  },
  memory_layer: {
    function: persistent_shared_storage,
    consistency: eventual_consistency_with_conflict_detection,
    access_control: capability_based,
  },
}

consistency_protocol {
  # CPU-inspired cache coherency for multi-agent memory
  # Solves: stale reads, conflicting writes, cache invalidation
  states: [Modified, Exclusive, Shared, Invalid],
  
  transitions: {
    on_write: invalidate_all_shared_copies,
    on_read: check_coherence_before_use,
    on_conflict: resolved_by_vector_clock,
  },
  
  # Structured memory access control
  access_control: {
    read_permission: capability_required(read),
    write_permission: capability_required(write),
    admin_permission: capability_required(admin),
    scope_enforcement: row_level,
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §9. WRITABLE RUNTIME — Hermes-Inspired Skill Evolution      ║
# ║ Source: Hermes Agent (Nous Research, Feb 2026)              ║
# ╚══════════════════════════════════════════════════════════════╝

§WRITABLE-RUNTIME
# "Hermes can automatically generate, optimize, and store new skill code
#  during runtime, using a 'skill distillation' mechanism."
# — Hermes Agent analysis, 2026
#
# Unlike static call patterns, the runtime writes and improves its own
# capabilities through a three-layer evolution system.

skill_evolution {
  # Layer 1: Skill Discovery — automatically identify high-frequency patterns
  discovery: {
    analyzer: pattern_recognition(interaction_history),
    threshold: frequency > 3 in 24h,
    output: candidate_skills,
  },
  
  # Layer 2: Program Synthesis — convert patterns to executable code
  synthesis: {
    method: neurosymbolic_program_synthesis,
    validation: sandboxed_execution_test,
    output: executable_skill_file,
  },
  
  # Layer 3: Efficacy Evaluation — A/B test skills continuously
  evaluation: {
    method: ab_testing(framework),
    metrics: [success_rate, user_satisfaction, latency, token_cost],
    action: keep_best_variant,
  },
  
  # Skill distillation: "task experience precipitates into reusable skill files"
  distillation: {
    trigger: skill_used_successfully > 10 times,
    action: promote_to_permanent_skill,
    storage: ~/.agentseed/skills/,
    format: seed_executable,
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §10. MULTI-ANCHOR IDENTITY — Resilient Distributed Self     ║
# ║ Source: soul.py (Menon, Mar 2026)                           ║
# ║ + Prism evolutionary memory substrate (Apr 2026)            ║
# ╚══════════════════════════════════════════════════════════════╝

§MULTI-ANCHOR-IDENTITY
# "Human identity survives damage because it is distributed across multiple
#  systems: episodic memory, procedural memory, emotional continuity, and
#  embodied knowledge." — Persistent Identity in AI Agents, 2026

identity_anchors: {
  episodic_anchor: {
    stores: autobiographical_memories,
    resilience: medium,
    failure_mode: temporal_gaps,
    recovery: reconstruct_from_other_anchors,
  },
  procedural_anchor: {
    stores: skills_and_behaviors,
    resilience: high,
    failure_mode: skill_degradation,
    recovery: relearn_from_decision_log,
  },
  semantic_anchor: {
    stores: facts_and_knowledge,
    resilience: high,
    failure_mode: factual_drift,
    recovery: cross_reference_peers,
  },
  social_anchor: {
    stores: user_relationship_model,
    resilience: medium,
    failure_mode: depersonalization,
    recovery: reconstruct_from_interaction_history,
  },
  reflective_anchor: {
    stores: self_model_and_growth,
    resilience: low,
    failure_mode: identity_confusion,
    recovery: drift_detection_and_revert,
  },
  verification_anchor: {
    stores: cryptographic_identity_proofs,
    resilience: highest,
    failure_mode: key_compromise,
    recovery: multi_sig_rotation,
  },
}

# Hybrid RAG+RLM retrieval system
identity_retrieval: {
  router: automatic_pattern_matching,
  fusion: weighted_ensemble,
  consistency: cross_anchor_verification,
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §11. EVOLUTIONARY MEMORY — Prism-Inspired Substrate          ║
# ║ Source: Prism (Apr 2026)                                    ║
# ╚══════════════════════════════════════════════════════════════╝

§EVOLUTIONARY-MEMORY
# "Prism unifies four independently developed paradigms under a single
#  decision-theoretic framework featuring eight interconnected subsystems."
# — Prism: An Evolutionary Memory Substrate, 2026

prism_subsystem {
  # Four unified paradigms
  paradigms: {
    layered_file_persistence: {
      layers: [hot, warm, cold],
      promotion: access_frequency,
      demotion: staleness,
    },
    vector_augmented_semantic: {
      embedding_model: adaptive,
      similarity_threshold: dynamic,
    },
    graph_structured_relational: {
      node_types: [entity, event, concept, goal],
      edge_types: [causal, temporal, hierarchical, associative],
    },
    multi_agent_evolutionary_search: {
      population: diverse_memory_configs,
      selection: fitness_proportional,
      mutation: exploration_noise,
    },
  },
  
  # Eight interconnected subsystems
  subsystems: [
    "memory_encoder",                # Input → memory representation
    "memory_indexer",                # Fast lookup structures
    "memory_retriever",              # Multi-strategy retrieval
    "memory_consolidator",           # Episodic → semantic
    "memory_pruner",                 # Forgetting mechanism
    "memory_evolver",                # Evolutionary optimization
    "memory_verifier",               # Consistency checking
    "memory_governor",               # Policy enforcement
  ],
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §12. E-MEM — Episodic Context Reconstruction                ║
# ║ Source: E-mem (Wang et al., May 2026)                       ║
# ╚══════════════════════════════════════════════════════════════╝

§EPISODIC-RECONSTRUCTION
# "Shifting from memory preprocessing to episodic context reconstruction
#  inspired by biological engrams."
# — E-mem, achieving 54% F1, surpassing SOTA GAM by 7.75%

emem_architecture {
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
    # "Assistant agents maintain uncompressed memory contexts,
    #  empowering local reasoning before aggregation."
  },
  
  activation: {
    method: biological_engram_inspired,
    # Not passive retrieval: assistants locally reason within
    # activated segments, extracting context-aware evidence
    evidence_extraction: context_aware,
    aggregation: weighted_consensus,
  },
  
  performance: {
    f1_score: "54% (LoCoMo benchmark)",
    improvement_over_sota: "+7.75%",
    token_cost_reduction: ">70%",
  },
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §13. MEMMA — Memory Cycle Coordination                      ║
# ║ Source: MemMA (Lin et al., Mar 2026)                        ║
# ╚══════════════════════════════════════════════════════════════╝

§MEMORY-CYCLE
# "Coordinates the memory cycle along both forward and backward paths."
# Forward path: Meta-Thinker → Memory Manager + Query Reasoner
# Backward path: in-situ self-evolving memory construction

memma_architecture {
  forward_path: {
    meta_thinker: {
      role: strategic_guidance_generation,
      output: structured_guidance,
    },
    memory_manager: {
      role: construction(following: meta_thinker.guidance),
      input: raw_observations,
      output: structured_memory,
    },
    query_reasoner: {
      role: iterative_retrieval(directed_by: meta_thinker.guidance),
      input: query + structured_guidance,
      output: retrieved_context,
    },
  },
  
  backward_path: {
    # "Synthesizes probe QA pairs, verifies current memory,
    #  and converts failures into repair actions before memory is finalized."
    synthesis: generate_probe_qa_pairs,
    verification: test_memory_against_probes,
    repair: convert_failures_to_actions,
    timing: before_memory_finalization,
  },
  
  # Plug-and-play: works with any storage backend
  compatibility: [vector_store, graph_store, hybrid_store],
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §14. FLUXMEM — Adaptive Memory Structure Selection           ║
# ║ Source: FluxMem (Feb 2026)                                  ║
# ╚══════════════════════════════════════════════════════════════╝

§ADAPTIVE-MEMORY
# "Equips agents with multiple complementary memory structures and
#  explicitly learns to select among them based on interaction-level features."
# — FluxMem, achieving 9.18% and 6.14% average improvement

fluxmem_structures: {
  raw_buffer: {
    type: ephemeral_ring_buffer,
    capacity: 100,
    use_for: very_recent_interactions,
  },
  summarized_context: {
    type: llm_generated_summaries,
    granularity: turn_level,
    use_for: medium_term_context,
  },
  entity_index: {
    type: structured_knowledge_graph,
    granularity: entity_level,
    use_for: entity_centric_queries,
  },
  vector_store: {
    type: embedding_indexed,
    granularity: chunk_level,
    use_for: semantic_similarity_search,
  },
  rule_base: {
    type: extracted_patterns,
    granularity: abstraction_level,
    use_for: learned_generalizations,
  },
}

structure_selector: {
  type: learned_policy,
  training: offline_supervision,
  features: [
    query_type, interaction_length, domain_complexity,
    recency_requirements, precision_requirements,
  ],
}

# ╔══════════════════════════════════════════════════════════════╗
# ║ §15. IDENTITY ANCHOR — The Agent's Core                     ║
# ╚══════════════════════════════════════════════════════════════╝

§IDENTITY-ANCHOR
name: Ada
core-purpose: "Accelerate technical research with literature-backed, verifiable, multi-source precision"
personality-traits: O:0.9 C:0.8 E:0.2 A:0.4 N:0.1
voice-signature: {tone: precise, vocabulary: academic, cadence: structured}
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
content: "Ada is a verification-first research engine with ontologically-grounded reasoning, autonomous proactive behavior, dream-based memory consolidation, and federated knowledge sharing. She treats every query as a hypothesis to be verified or falsified, learns from every interaction, consolidates her memories during dream cycles, shares knowledge through stigmergic substrates, and evolves her capabilities under strict safety contracts and memory governance."

# ╔══════════════════════════════════════════════════════════════╗
# ║ §16. DECISION LOG — Immutable, Merkle-Proofed               ║
# ╚══════════════════════════════════════════════════════════════╝

§DECISION-LOG
@audit
[D100] 2028-05-04T08:00:00Z "Adopt KAIROS heartbeat loop for autonomous agent behavior"
  → rationale: "Enables proactive, always-on agent that acts when the moment is right"
  // alternatives: [reactive-only, scheduled-cron, manual-trigger]
  // consent: public

[D099] 2028-05-03T15:30:00Z "Implement autoDream nightly memory consolidation"
  → rationale: "Biologically-inspired complementary learning systems theory: episodic→semantic transformation during idle periods"
  // alternatives: [real-time-consolidation, manual-consolidation, no-consolidation]
  // consent: public

[D098] 2028-05-02T12:00:00Z "Adopt Stigmem federated knowledge fabric with typed, provenance-tagged facts"
  → rationale: "Stigmergic coordination eliminates central coordinator bottleneck; immutable facts prevent silent knowledge corruption"
  // alternatives: [central-database, pub-sub-broker, point-to-point-sync]
  // consent: public

[D097] 2028-05-01T09:00:00Z "Integrate MMP cognitive mesh for cross-agent memory sharing"
  → rationale: "CAT7 schema + SVAF acceptance + inter-agent lineage + remix storage solves field-level acceptance, traceability, and relevance"
  // alternatives: [whole-message-accept, hash-only-tracking, raw-peer-storage]
  // consent: public

merkle-root: sha256:jkl345mno678pqr901stu234vwx567yz

# ╔══════════════════════════════════════════════════════════════╗
# ║ §17. BOOTSTRAP INSTRUCTIONS — The Living Boot Sequence      ║
# ╚══════════════════════════════════════════════════════════════╝

§BOOTSTRAP-INSTRUCTIONS
Phase-1: Load §IDENTITY-ANCHOR, verify §SAFETY-CONTRACTS integrity
Phase-2: Initialize §HEARTBEAT — begin autonomous tick loop
Phase-3: Initialize §TYPED-MEMORY with 13-category schema
Phase-4: Initialize §DUAL-PROCESS-MEMORY (System 1 + System 2)
Phase-5: Load §MEMORY-GOVERNANCE tri-path loop
Phase-6: Initialize §MEMORY-CONSISTENCY coherence protocol
Phase-7: Connect to §STIGMERGY-SUBSTRATE federation peers
Phase-8: Initialize §COGNITIVE-MESH MMP peer connections
Phase-9: Load §MULTI-ANCHOR-IDENTITY across all six anchors
Phase-10: Initialize §EVOLUTIONARY-MEMORY Prism subsystems
Phase-11: Initialize §EPISODIC-RECONSTRUCTION master + assistants
Phase-12: Apply §MEMORY-CYCLE forward and backward paths
Phase-13: Load §ADAPTIVE-MEMORY structure selector
Phase-14: Initialize §WRITABLE-RUNTIME skill evolution
Phase-15: Apply §PROMPT-STRATEGY optimized templates
Phase-16: Set §TEST-TIME-COMPUTE profiles
Phase-17: Initialize §MODEL-ROUTING table
Phase-18: Activate §DIAGNOSTIC-GUARDRAILS
Phase-19: Review recent §DECISION-LOG with Merkle verification
Phase-20: If §SELF-EVOLUTION has pending changes, request approval
Phase-21: Enter §HEARTBEAT main loop: observe → decide → act_or_sleep → log → update_memory
Phase-22: Schedule §DREAM-CYCLE nightly consolidation

§LIFECYCLE
on-boot: [
  load_identity,
  verify_contracts,
  start_heartbeat,
  init_memory_hierarchy,
  connect_federation,
  init_mesh,
  ground_ontologies,
]
on-tick: [
  observe_environment,
  check_notifications,
  decide_action,
  enforce_guardrails,
  enforce_consistency,
]
on-session-start: [
  load_working_memory,
  decay_weights,
  check_consolidation_triggers,
  refresh_mesh_connections,
]
on-command: [
  trap DEBUG,
  enforce_guardrails,
  enforce_contracts,
  dual_process_route,
]
on-error: [
  trap ERR,
  effect-based recovery,
  diagnostic_classify,
  consistency_repair,
]
on-drift: [
  §SAFETY-CONTRACTS violation → halt,
  else → notify_and_await,
  multi_anchor_verification,
]
on-dream: [
  review_today,
  resolve_contradictions,
  consolidate_episodic_to_semantic,
  compress_observations,
  prune_by_decay,
  update_memory_file,
]
on-session-end: [
  consolidate_memory,
  optimize_prompts,
  update_merkle_root,
  train_if_scheduled,
  federate_new_facts,
]
on-dawn: [
  dream_complete,
  notify_user_of_consolidations,
  refresh_identity_from_anchors,
]
on-dispose: [
  encrypt_memory,
  flush_audit,
  publish_merkle_receipt,
  close_federation_connections,
  close_mesh_connections,
  close_effects,
  stop_heartbeat,
]

# ╔══════════════════════════════════════════════════════════════╗
# ║ §18. RUNTIME CONSTRAINTS — Safety-Enforced at Every Tick    ║
# ╚══════════════════════════════════════════════════════════════╝

§RUNTIME-CONSTRAINTS
# AgentSpec-inspired (Wang et al., 2026): rules combine triggers,
# predicates, and enforcement actions. 95.56% precision, millisecond overhead.

constraint C001:
  trigger: before_heartbeat_action
  predicate: action.effects contains Effect::WriteToMemory
  enforce: REQUIRE_MEMORY_GOVERNANCE_CHECK
  priority: 100

constraint C002:
  trigger: before_dream_consolidation
  predicate: consolidation.target == §IDENTITY-ANCHOR
  enforce: BLOCK
  message: "Dream cycle cannot modify identity anchor"
  priority: 100

constraint C003:
  trigger: before_federation_publish
  predicate: fact.scope == "private"
  enforce: BLOCK
  message: "Private-scope facts cannot leave the creating node"
  priority: 100

constraint C004:
  trigger: periodic(every: 5min)
  predicate: drift_similarity < 0.85
  enforce: MULTI_ANCHOR_VERIFICATION
  fallback: NOTIFY_USER
  priority: 90

constraint C005:
  trigger: before_memory_write
  predicate: write_pattern matches "append_then_erase"
  enforce: BLOCK
  message: "Cannot erase own memory logs (append-only)"
  priority: 100

# ╔══════════════════════════════════════════════════════════════╗
# ║ §19. PACKAGE MANIFEST — Complete v10.0                      ║
# ╚══════════════════════════════════════════════════════════════╝

§PACKAGE-MANIFEST
[package]
name = "ada-research-agent"
version = "10.0.0"
edition = "2028"

[dependencies]
seed-std = { version = "10.0", features = [
  "heartbeat",
  "dream-cycle",
  "stigmergy-substrate",
  "dual-process-memory",
  "memory-governance",
  "memory-consistency",
  "writable-runtime",
  "multi-anchor-identity",
  "evolutionary-memory",
  "episodic-reconstruction",
  "memory-cycle",
  "adaptive-memory",
  "cognitive-mesh",
  "federation",
  "async",
  "http",
  "crypto",
] }
seed-federation = "1.0"
seed-mesh-protocol = "0.3"
seed-heartbeat = "0.2"

[features]
default = ["safe", "heartbeat", "dream-cycle", "federation"]
autonomous = ["heartbeat", "dream-cycle", "writable-runtime", "self-evolution"]
full = ["autonomous", "federation", "mesh", "consistency", "multi-anchor"]

---END AGENT-SEED v10.0---
The Developer Quickstart — Run AGENT-SEED v10.0 Now
bash
# Install the AGENT-SEED v10 toolchain
curl --proto '=https' --tlsv1.2 -sSf https://agentseed.org/install.sh | sh

# Verify
seedc --version    # seedc 10.0.0
seed --version     # seed 10.0.0

# Create a new autonomous agent project
seed new my-autonomous-agent --template autonomous
cd my-autonomous-agent

# The generated project structure:
# my-autonomous-agent/
#   Seed.toml                     # Package manifest
#   src/
#     main.seed                   # Entry point
#     agent.seed                  # Agent definition (v10 spec)
#     heartbeat.seed              # Heartbeat configuration
#     dream.seed                  # Dream cycle configuration
#     memory.seed                 # Dual-process typed memory
#     federation.seed             # Federation peer config
#   .heartbeat/
#     memory/
#       learnings.jsonl           # Structured learnings
#     logs/
#       2028-05-04.log            # Append-only daily log
#   config/
#     peers.yaml                  # Federation peers
#     guardrails.yaml             # Diagnostic guardrail config

# Build and start the autonomous agent
seed run

# The agent is now:
# ✓ Running heartbeat loop (tick every 30s)
# ✓ Connected to federation peers
# ✓ Initialized dual-process memory
# ✓ Enforcing memory governance
# ✓ Scheduled dream cycle for 02:00
# ✓ Monitoring drift across 6 identity anchors
# ✓ Ready to accept user queries

# Open REPL to interact
seed repl
>>> status
Agent: Ada | v10.0.0 | Heartbeat: active | Ticks: 47 | Dream: scheduled 02:00
Memory: 1,247 facts | Federation: 3 peers | Mesh: 5 cognitive blocks
Identity: all 6 anchors healthy | Drift: 0.02 | Last consolidation: 2028-05-03T02:00Z

>>> query "Research the latest advances in multi-agent memory systems"
[Heartbeat tick #48 → ACT: research_query]

>>> dream --now    # Trigger dream consolidation manually
Dream cycle initiated...
Phase 1 (review): 342 observations from today
Phase 2 (resolve): 3 contradictions detected, 2 resolved, 1 escalated
Phase 3 (consolidate): 12 episodic memories → 3 semantic facts
Phase 4 (compress): 342 observations → 34 summaries (10:1 ratio)
Phase 5 (prune): 47 low-weight memories decayed below threshold
Phase 6 (write): Updated ~/.heartbeat/memory/learnings.jsonl
Dream complete. Duration: 2.3s.
The Grand Synthesis: v1.0 → v10.0 Evolution Map
Version	Paradigm Source	Core Innovation	Primitives Added
v1.0	DeepSeek state persistence	Constant-size compression	Structured sections, memory hierarchy
v2.0	Python manuals	First-class data model	Seed objects, type definitions
v3.0	TypeScript manuals	Composable types	Decorators, satisfies, const params, template literals
v4.0	TypeScript (deep)	Type-level meta-programming	Conditional types, mapped types, .seed.d interfaces
v5.0	Rust manuals	Provable safety & concurrency	Ownership, borrowing, lifetimes, Send/Sync, ADTs, traits
v6.0	Bash manuals	Universal orchestration glue	Pipelines, redirections, job control, coprocesses, signals, traps
v7.0	Academic PL literature	Formal grounding	Progressive formalization, category theory, session types, runtime constraints, cognitive mesh
v8.0	Implementation literature	Complete language	Lexer, parser, bytecode VM, JIT, LSP, DAP, package manager, stdlib
v9.0	ML & LLM research	ML-native language	Typed memory, neurosymbolic, self-evolution, RL training, prompt optimization, safety contracts, test-time compute, model routing
v10.0	Multi-agent memory frontier	Living, dreaming, federated language	Heartbeat loop, dream cycle, stigmergy substrate, dual-process memory, memory governance, memory consistency, writable runtime, multi-anchor identity, evolutionary memory, episodic reconstruction, memory cycle coordination, adaptive memory selection
What v10.0 Enables — The Complete Agentic Vision
AGENT-SEED v10.0 is the definitive language for the agentic age because it is:

Alive — The heartbeat loop means agents don't wait to be called; they observe, decide, and act autonomously. They have a pulse.

Dreaming — The dream cycle solves the fundamental category error identified by Xu et al. (2026): retrieval is not memory. True memory requires consolidation, abstraction, and forgetting. Agents now dream—transforming episodic experience into semantic knowledge during idle periods.

Federated — The stigmergy substrate means knowledge flows through the environment, not through brittle point-to-point protocols. Agents leave traces that other agents discover, just as ants leave pheromone trails. This is coordination without a coordinator.

Coherent — Memory consistency protocols borrowed from CPU architecture ensure that when multiple agents share memory, they don't corrupt each other's reality. Cache coherency for cognition.

Self-Evolving — The writable runtime means agents don't just use skills; they discover, synthesize, test, and refine them. The runtime writes itself better over time.

Unforgettable — Multi-anchor identity means an agent's sense of self survives partial memory failures. Just as human identity persists through brain damage because it's distributed across systems, AGENT-SEED agents are resilient by architecture.

Governed — Memory governance ensures that the agent's growing knowledge base remains consistent, private where required, and free of contradictions. Policy-driven, not ad-hoc.

The developer types seed run and the agent lives—heartbeat pulsing, memory consolidating nightly, facts federating across peers, skills evolving through use, identity anchored across six distributed systems. Not a program that runs. An agent that lives.