# Verifier report W1b: vc-store / vc-seq (adversarial, second pass)

Scope: `Sequencer::duplicate_of` semantics, numeric extremes, DoS/resource behaviour, registry semantics. Complements the
first-pass files (verifier_w1_store.rs / verifier_w1_seq.rs, findings ST-1..ST-8, SQ-1..SQ-7); nothing there is repeated.
No `src/` file or existing test file was modified. New files:
- crates/vc-store/tests/verifier_w1b_store.rs (2 failing, 10 passing, 1 ignored)
- crates/vc-seq/tests/verifier_w1b_seq.rs (5 failing, 14 passing, 2 ignored)

Run: `cargo test --offline -p vc-store -p vc-seq --no-fail-fast` (add `--release` to check release-mode arithmetic; both modes give the same
7 failures). Ignored tests (debug >5 s): `cargo test --offline --release -p vc-store -p vc-seq --test verifier_w1b_store --test verifier_w1b_seq -- --include-ignored`.
`bug_*` assert correct behaviour and fail today; `holds_*` pass; `observe_*` document behaviour.

## Confirmed bugs

### High

**SQ-8. A retry after a failed append is acknowledged `duplicate: true` although nothing was made durable.**
crates/vc-seq/src/lib.rs:118-129, :223 (rollback `replay()` re-reads the non-durable in-memory `entry_edge`/advanced `size`, so `by_id`
contains the id) with :239-252 (`duplicate_of`). Root cause shared with ST-1/SQ-5; this is the client-visible acknowledgement.
Test: `cargo test --offline -p vc-seq --test verifier_w1b_seq bug_duplicate_ack_after_failed_append_is_not_durable`
Failure: `fail_after=0: retry acknowledged index 0 (duplicate=true) but a reopened log has size 0`.
Violates "never acknowledges before durable write" (ARC42 s5.1). Fix: make `TileLog::append_batch` transactional (compute new state,
write, then commit `size`/`edge`/`entry_edge`), and have `duplicate_of` require `idx < durable size`.

**SQ-9. Rollback `replay()` error is swallowed (`let _ = self.replay()`); a failed replay leaves chain state and `by_id` truncated and the log can then be forked/bricked.**
crates/vc-seq/src/lib.rs:223 (`replay`, :118, clears `chain`/`by_id` first, returns on first error).
Test: `cargo test --offline -p vc-seq --test verifier_w1b_seq bug_failed_rollback_replay_lets_log_fork_the_chain`
Failure: `old statement -> Ok(Registration { index: 301, .. duplicate: false }); next statement -> Err(.. seq 300 but expected 1); reopen -> Err("chain rule violated: statement 301: seq 0 but expected 301")`.
A transient read error during rollback makes the sequencer forget every agent: it re-registers an existing seq-0 statement as new (index 301),
rejects the genuine next statement, and the log can no longer be opened (`Sequencer::open` fails permanently). Fix: build the replacement
`ChainState`/`by_id` into locals and swap on success; on failure mark the sequencer poisoned (refuse all submits) instead of continuing.

### Medium

**SQ-10. `checkpoint()` panics after a failed append.**
crates/vc-store/src/tilelog.rs:306 (`self.edge[l as usize]` on the emptied `edge` Vec after `mem::take`), reached from vc-seq/src/lib.rs:259 (`self.log.root()?`).
Test: `cargo test --offline -p vc-seq --test verifier_w1b_seq bug_checkpoint_after_failed_append_must_not_panic_or_sign_non_durable_data`
Failure: `base=0 k=1: checkpoint() PANICKED after a failed append` (index out of bounds: len is 0 but the index is 0).
Same root cause as ST-1 (which reports `root()` panics at TileLog level); listed because it takes down the signing path of the sequencer.
Fix: as ST-1 (restore `edge` on every error path).

**SQ-11. `duplicate_of` runs before the size check and before signature verification, so an unauthenticated caller can force unbounded decode+canonicalise work.**
crates/vc-seq/src/lib.rs:193 (call) and :239-245 (peek_payload, UTF-8 check, `canonicalize_str`, SHA-256 on an unvetted payload); size check only at :150.
Test: `cargo test --offline -p vc-seq --test verifier_w1b_seq bug_oversize_unauthenticated_statement_is_refused_before_being_parsed`
Failure: `refusal of an oversize statement took 601ms` (release; 4.5 s debug) for an 8 MB statement signed by an unregistered key, final answer `TooLarge`.
CPU and several copies of the payload in memory per request (cost grows linearly with whatever the transport accepts). Fix: check
`cose.len() > MAX_STATEMENT_BYTES` first in `submit_batch` (before `duplicate_of`), and let `duplicate_of` look up by `sha256(cose)`/leaf hash
(a map from leaf hash to index) instead of re-parsing untrusted input.

**SQ-12. `Sequencer::open` replay reads every entry through `TileLog::entry`, which fetches the whole 256-entry bundle per entry (~170-220x read amplification).**
crates/vc-seq/src/lib.rs:122 (`self.log.entry(i)` in `replay`), crates/vc-store/src/tilelog.rs:264-281.
Test: `cargo test --offline -p vc-seq --test verifier_w1b_seq bug_open_replay_reads_each_bundle_once`
Failure: `replay read 546222654 B of a 3185828 B log (171x)`. With 64 KB statements a 1 GB log is read ~160 GB at every start and after every failed append (rollback replays too).
Replay I/O is linear in log size (`holds_replay_io_is_linear_in_log_size`, ignored in debug), so this is a constant factor of ~256, not quadratic.
Fix: add `TileLog::entries(range)`/an iterator that reads each bundle once, use it in `replay` and `recover`.

**ST-9. `TileLog::open` recovery (`recover()`) reads each entry bundle once per entry (257x amplification).**
crates/vc-store/src/tilelog.rs:157 (`self.entry(i)` per i) and :264-281.
Test: `cargo test --offline -p vc-store --test verifier_w1b_store bug_recover_reads_each_entry_bundle_once`
Failure: `recovery read 527014488 bytes to rebuild from 2050048 bytes of entry bundles (257x)`.
Fix: iterate bundles (see SQ-12).

### Low

**ST-10. `entry()` inside the current partial bundle clones the whole in-memory `entry_edge` (up to ~16 MB) per lookup.**
crates/vc-store/src/tilelog.rs:273 (`Some(self.entry_edge.clone())`).
Test: `cargo test --offline -p vc-store --test verifier_w1b_store bug_entry_lookup_in_partial_bundle_does_not_clone_whole_edge`
Failure: `median lookup 1.2ms (copies the whole 15 MB edge per call)` (threshold 200 us; ~10 us expected). Every duplicate check / replay step inherits this.
Fix: parse the bundle from a borrowed slice (`parse_bundle(&self.entry_edge)`), copy only the requested entry.

## Areas attacked that hold

- `duplicate_of` semantics: same payload but different COSE bytes (other content type, other registered key/kid of the same agent, whitespace / key-order
  non-canonical twins) is never a duplicate and never appended (`holds_same_payload_different_envelope_is_not_a_duplicate`); statement ids do not collide across agents and a
  foreign agent cannot register or shadow another agent's statement (`holds_statement_ids_do_not_collide_across_agents`); a 5000-element batch of duplicates is answered idempotently, quickly and writes nothing
  (`holds_huge_batch_of_duplicates_writes_nothing`). The byte-equality check against the stored entry means by_id poisoning from the log is not exploitable through `duplicate_of`.
- Numeric extremes in statements: `seq` as -0, 0.0, 0e0, 00, +0, -1, 2^53-1, 2^53, 2^53+1, u32/i64/u64 limits, u64::MAX+1, 1e999, 1.8446744073709552e19, strings, null, arrays at position 0 and 1 are all refused, no panic,
  never coerced to the expected value (`holds_seq_numeric_extremes_rejected_without_panic`); `v` spellings (`2.0`, `2e0`, `02`) refused; numbers in `ext` are accepted only in canonical ES6 form and everything accepted replays on reopen.
- TileLog public API at extreme sizes: a synthetic read-only store serves consistent logs of 2^24 and (2^24 + 2^16 + 256) leaves (and, with `--release --include-ignored`, 2^32 leaves). Roots, inclusion and consistency proofs match an independent
  computation and verify with vc-log; (index, size) / (first, second) / root_at / entry / leaf_hash_at arguments of u64::MAX, 2^63, size+1, etc. return `Range`, never panic or wrap, in debug and release.
  `MAX_LEVELS` cannot be exhausted via the public API (needs 2^64 leaves; `size` would overflow first), so the `level >= MAX_LEVELS` branch is unreachable.
- `index_path` / `hash_tile_path` / `entry_tile_path` for indices up to u64::MAX and levels up to u32::MAX round-trip, are injective, prefix-free (no file/dir clash) and usable with `FsStore`.
- DoS: JSON nesting 200..32000 (arrays) and 9000 (objects) from registered and unregistered signers inside a 2 MB-stack thread: refused, no stack overflow; accepted nesting depth is monotone and every accepted statement replays;
  kid of 40 KB / agent id of 20 KB / 3000 object members / 20k-char escaped strings under the limit: accepted and fast; 100k tiny entries in one `append_batch` linear and correct; 2000 statements from 200 agents in one `submit_batch` correct
  and the root equals the independent root (ignored in debug for signing cost); intact `open` does no writes; replay I/O is linear in log size.
- Registry: a registered weak (identity-point) key cannot forge (`holds_registered_weak_key_cannot_forge`); several kids per agent id share one chain and a kid cannot speak for another agent even with an identical key;
  empty inputs (`submit_batch(&[])`, `submit(&[])`) are harmless.

## Observations (not defects, documented by `observe_*` tests)

- `Registry::register` silently overwrites an existing kid (different agent id, key, ceiling) and returns nothing; a configuration mistake or hostile registration cannot be detected by the caller (`observe_register_overwrites_silently`). Consider returning `Option<Issuer>`/`Result`.
- The duplicate path ignores the current registry: after reopening with the issuer removed, key rotated or ceiling lowered, a retry of an already-stored statement is still answered `Ok(duplicate: true)` (`observe_duplicate_ignores_current_registry`). Reasonable for idempotent retry of a public fact, but replay on open also skips registry/agent-binding checks, so a log written under another registry is accepted as is (see SQ-6).
- `observe_single_append_write_amplification`: every single-entry append rewrites the whole partial bundle and all partial hash tiles (O(width) bytes per append).

## Counts

Confirmed: High 2 (SQ-8, SQ-9), Medium 4 (SQ-10, SQ-11, SQ-12, ST-9), Low 1 (ST-10); 7 failing tests (ST-9, ST-10, SQ-8..SQ-12), identical in debug and `--release`.
