# Test vector generators

Python 3 standard library only. Written independently of the Rust crates.

Regenerate (from anywhere; paths are relative to the script):

    python3 tools/vectors/gen_merkle.py   # spec/vectors/merkle_v1.json, ct_reference_v1.json
    python3 tools/vectors/gen_jcs.py      # spec/vectors/jcs_v1.json
    python3 tools/vectors/gen_chain.py    # spec/vectors/chain_v1.json

Each script reloads its output and self-checks (positives verify, negatives fail, expected JCS output
matches an independent mini-canonicalizer, chain results match a reference validator) and exits non-zero on failure.
Output is deterministic.

## Assumptions and ambiguities

Merkle (RFC 9162 s2.1, SHA-256)
- Inclusion vectors give leaf_hash (already H(0x00||entry)), not the raw entry.
- Some wrong_size negatives are omitted: the RFC verification algorithm cannot reject every wrong size
  (e.g. a size whose right-hand structure is invisible to the proof path). Only sizes the algorithm rejects are emitted.
- truncated_proof_hash uses a 31-byte element (malformed length); implementations may reject at parse time rather than verify time. Both count as reject.
- Consistency with first == second requires an empty proof and equal roots. first == 0 is rejected (no vectors accept it).
- ct_reference_v1.json roots are computed; the recalled CT reference roots match them for sizes 1..8 but the recalled values came from memory, not an authoritative source.

JCS (RFC 8785)
- Expected values are hand-written from the RFC; number cases cross-checked against a repr()-based ES Number::toString.
- expected_canonical_hex is the authoritative byte sequence.
- 2^53+1 (9007199254740993) is expected to parse as an IEEE double and print 9007199254740992; an implementation
  that keeps big integers exactly would differ. Marked medium. 5e-324 and 1.7976931348623157e308 also marked medium (extremes).
- Not covered: lone surrogates, duplicate keys, NaN/Infinity, numbers overflowing to infinity (inputs should be rejected; spec silent here).
- The user-facing "section 3.2.2 example" is the numbers/string/literals sample; the key-sorting sample (3.2.3) is also included.

Chain
- Statement id hashes the whole statement object (no id field inside). ts is an integer; pair is {"intent_id": ...}.
- Check order is my choice: SeqGap, BadPrev, then UnknownIntent, AgentMismatch, DuplicateOutcome. The spec did not order checks;
  cases both_seq_and_prev_wrong_seqgap_wins and agent_mismatch_other_agents_closed_intent depend on this order.
- An outcome whose pair targets an outcome statement id is classed UnknownIntent (not an intent). Forward references are UnknownIntent.
- A repeated or decreasing seq is SeqGap.
- Not covered (unspecified): outcome with missing pair, unknown kinds, intents carrying pair, a statement for an agent whose id differs in case.
- open_intent_ids are ordered by log position.
