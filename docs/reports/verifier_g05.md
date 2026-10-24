# Verifier report G05 (adversarial)

Scope: crates vc-core, vc-log (lib + checkpoint), vc-cose. No source files modified; only test files added.
Run everything with `cargo test --offline --no-fail-fast`. Tests named `bug_*` assert the correct behaviour and FAIL
today (17 failing tests = 15 findings). `regression_*` / `ambiguity_*` pass and are kept as labelled regression tests.

New test files:
- crates/vc-core/tests/verifier_g05_jcs.rs (4 failing, 6 passing)
- crates/vc-core/tests/verifier_g05_chain.rs (1 failing, 8 passing)
- crates/vc-log/tests/verifier_g05_merkle.rs (8 passing; includes boundary differential tests up to n=1025)
- crates/vc-log/tests/verifier_g05_checkpoint.rs (5 failing, 11 passing)
- crates/vc-cose/tests/verifier_g05_cose.rs (7 failing, 7 passing)

Existing tests (merkle, checkpoint, jcs_chain, sign1) all pass.

## Confirmed bugs

### High

**H1. Witness quorum counts distinct key NAMES, not distinct keys.**
crates/vc-log/src/checkpoint.rs:81-93 (`valid_signers` inserts `k.name`), :106 (`got = ... .len()`).
Test: `cargo test -q -p vc-log --test verifier_g05_checkpoint bug_quorum_counts_names_not_keys`
Failure: `one key signing under two names satisfied a 2-witness quorum`.
One physical key listed as "w-a" and "w-b" signs two lines and satisfies min_witnesses=2. Fix: collect the set of
`key.as_bytes()` (or (name,key) pairs deduplicated by key) and count distinct public keys; reject trust lists that contain
duplicate public keys at construction.

**H2. Operator is counted as a witness.**
crates/vc-log/src/checkpoint.rs:104-106 (witness set is not disjoint from the operator).
Tests: `bug_operator_counts_as_witness`, `bug_operator_alias_as_witness` (same file).
Failures: `operator alone satisfied min_witnesses=1`; `operator key under an alias name counted as an independent witness`.
With the operator key in the witness slice (or under an alias), the operator self-satisfies the quorum, which defeats
the split-view defence (quorum is supposed to be independent of the operator). Fix: in `verify_note`, exclude any witness whose
public key equals the operator key (and whose name equals the operator name) from the count, or return an error on overlap.

### Medium

**M1. JCS key ordering uses UTF-8 bytes, not UTF-16 code units (RFC 8785 3.2.3).**
crates/vc-core/src/canonical.rs:4-6 (delegates to serde_jcs 0.1.0).
Test: `cargo test -q -p vc-core --test verifier_g05_jcs bug_key_order_utf16_astral_vs_bmp_high`
Failure: left `{"\u{e000}":1,"😀":2}` right `{"😀":2,"\u{e000}":1}`.
U+1F600 (D83D DE00) must sort before U+E000. Statement ids/signatures differ from any conformant implementation (JS, Go)
whenever a key set mixes astral and U+E000..U+FFFF characters (possible in `ext`/`io`). Fix: implement own JCS serializer (sort by
`encode_utf16()` sequence) or pre-sort; add this as a published vector.

**M2. Integers above 2^53 are not reduced to IEEE-754 doubles (RFC 8785 3.2.2.3).**
canonical.rs:4-6. Tests: `bug_integer_above_2_53_not_reduced_to_double`, `bug_u64_max_es6_form`.
Failures: left `{"n":9007199254740993}` right `{"n":9007199254740992}`; left `[18446744073709551615]` right `[18446744073709552000]`.
Exact big integers are emitted, so ids differ from JS/Go/Python-jcs verifiers. Fix: either serialize via f64/ES6 formatting, or
(stricter, recommended for an evidence log) reject any integer with |n| > 2^53-1 in `canonicalize` (I-JSON) and require
big values as strings.

**M3. Duplicate object keys are silently accepted (last wins).**
canonical.rs:9-11 (`serde_json::from_str::<Value>`). Test: `bug_duplicate_keys_rejected`.
Failure: `duplicate keys silently accepted (last wins)`. RFC 8785 requires I-JSON input (RFC 7493 2.3 forbids duplicates). Two
different byte strings `{"a":1,"a":2}` and `{"a":2}` get one id; parsers differ on which wins. Fix: parse with a duplicate-detecting
deserializer (custom Visitor for maps) and error.

**M4. chain::validate does not enforce schema constants.**
crates/vc-core/src/chain.rs:36-50. Test: `cargo test -q -p vc-core --test verifier_g05_chain bug_schema_constants_not_enforced`
Failure: `statement with v=99, unknown kind, empty agent id, no ts accepted`.
spec/statement.schema.json requires v==2, kind in the enum, agent.id minLength 1, `ts` present, `prev` matching `^[0-9a-f]{64}$`
(prev is only compared to an expected value so that part is effectively enforced after seq 0), `pair.intent_id` pattern.
Note also arc42 5.1.1 v8 kinds (lifecycle, approval, settlement, governance) are absent from the schema enum: reconcile. Fix: validate
these in `validate` (or explicitly document that the registration policy does it and add a combined entry point).

**M5. COSE: trailing bytes after the COSE_Sign1 item are ignored.**
crates/vc-cose/src/lib.rs:49 (`from_reader` reads one item only). Test: `cargo test -q -p vc-cose --test verifier_g05_cose bug_trailing_bytes_after_sign1_accepted`
Failure: `trailing bytes accepted`. Many byte strings verify as one statement; a leaf hash over raw bytes diverges from what was verified.
Fix: decode from a cursor and require it fully consumed.

**M6. COSE: duplicate labels in the protected header accepted, first wins.**
lib.rs:55-57 (`get` = first match). Test: `bug_duplicate_protected_keys_accepted`.
Failure: `duplicate kid label accepted; first wins` (same for alg). RFC 9052 3.1 forbids duplicate labels and requires rejection; an
implementation taking the last kid selects a different key (parser differential). Fix: reject maps with duplicate keys.

**M7. COSE: `verify` instead of `verify_strict` permits small-order keys (universal forgery).**
lib.rs:60 and checkpoint.rs:89 (`k.key.verify`). Test: `bug_small_order_key_universal_forgery`.
Failure: `forged signature verified under identity key for payload [97, 110, 121, ...]`.
With the identity point as public key, R=identity, S=0 verifies for every message. Requires such a key to reach the trust lookup
(registry/config error or hostile registration) but then every statement is forgeable. S+L malleability is correctly rejected
(`regression_signature_scalar_malleability_rejected`). Fix: use `verify_strict` in vc-cose and vc-log, and reject weak keys
(`VerifyingKey::is_weak`) on KeyInfo/lookup.

### Low

**L1. Checkpoint size accepts non-canonical decimal ("+5", "0005").** checkpoint.rs:36. Test `bug_size_noncanonical_accepted`
(failure: `size "+5" accepted`). Two distinct signed texts map to one Checkpoint. Fix: require `s == size.to_string()`.

**L2. `sign_line` does not validate the key name** (newline injects a forged signature line; also spaces/`+`/empty forbidden by
signed-note). checkpoint.rs:61-65. Test `bug_sign_line_name_with_newline_injects_line` (failure: left 2, right 1 newline count).
Fix: assert/return error for names containing whitespace, `+`, or empty; validate in `KeyInfo` construction too.

**L3. COSE: trailing bytes inside the protected-header bstr ignored.** lib.rs:53. Test `bug_trailing_bytes_in_protected_accepted`
(`protected header with trailing bytes accepted`). Fix: require full consumption.

**L4. COSE: unprotected header (element 1) not required to be a map.** lib.rs:52 (`_`). Test `bug_unprotected_not_a_map_accepted`
(`unprotected header Integer(Integer(5)) accepted`). Fix: match `Value::Map(_)`.

**L5. COSE: `crit` (label 2) ignored.** lib.rs:54-57. Test `bug_crit_header_ignored` (`unknown critical header accepted`).
RFC 9052 3.1. Fix: reject if label 2 present and any listed label is not understood (or reject label 2 outright).

**L6. COSE: indefinite-length CBOR accepted** (outer array and payload bstr). lib.rs:49. Test `bug_indefinite_length_envelope_accepted`
(`indefinite-length CBOR accepted`). Fix: decode with a strict/deterministic checker (or re-encode and compare bytes with input).

## Spec ambiguities / design gaps (tests pass, behaviour documented)

- **A1 (medium) verify_note does not bind an expected origin.** `ambiguity_origin_not_checked`: a note validly signed for
  "other.example/log" is returned. API should take expected origin. checkpoint.rs:101.
- **A2** Inclusion verification does not bind `size` to the root (`ambiguity_inclusion_size_not_bound_to_root`): proof for leaf 0 of a 5-leaf
  tree verifies with claimed size 5..8 (RFC 9162 behaviour; size must come from the signed checkpoint). Callers must pass the checkpoint size.
- **A3** Note without final newline and `\r\n`-terminated signature lines are accepted (`str::lines`); signed-note requires "\n" termination. `ambiguity_note_without_final_newline_accepted`.
- **A4** Extension lines after the root are accepted and dropped by `Checkpoint::parse` (`regression_extension_lines_ignored_not_exposed`); tlog-checkpoint allows them but they are not surfaced.
- **A5** arc42 5.1.1 says the protected header carries CWT claims (iss/sub) and the content type; spec/README.md and code define only {1,3,4}, and verify1 requires neither 3 nor 15 (`ambiguity_content_type_and_cwt_claims_not_required`). Untagged COSE_Sign1 is rejected (acceptable but should be stated). arc42 v8 kinds vs schema enum mismatch (see M4).
- **A6** Chain rule "outcome references an *earlier* intent": enforced by log order only; spec/README is silent on whether an intent's `pair` may be present (ignored). Ordering across agents relies on log order, not timestamps.

## Checked and correct

vc-log Merkle (merkle.rs): empty tree root = SHA-256(""), size 1, m==n, first==0 (Range), first>second (Range), index==size,
size 0, usize::MAX sizes and indices with empty/long/200-element proofs: no panic, no overflow, typed errors. Exhaustive mutation of valid
inclusion/consistency proofs for all n<=40 (truncate, extend, prepend, bit-flip, wrong index, swapped roots) always rejected. Independent recursive-free
root differential at n in {127..129, 255..257, 511..513, 1000, 1023..1025}, inclusion at 5 indices each, consistency at 14 first-sizes each. Proof length bounded by ceil(log2 n).
Leaf/node domain separation holds.
Checkpoint: text must end in newline; CRLF rejected; empty origin, missing lines, u64 overflow, spaces around size, 31/33-byte roots,
non-canonical base64 (trailing bits), unpadded, URL-safe alphabet all rejected. Blank-line separator required; signatures bound to exact text (stale/other-size cosigns do
not count); reordered, malformed, unknown and garbage signature lines are skipped; every byte flip of keyid||sig rejected, and wrong-length payloads rejected; key names with spaces
parse correctly (rsplit on last space) and cannot be spoofed by shifting the boundary; one name with two keys counts at most once; key id equals SHA-256(name||0x0A||0x01||pubkey)[:4]; split-view requires same origin and size.
vc-cose: Sig_structure = ["Signature1", protected bstr, h'' , payload bstr] matches RFC 9052 4.4 (signature from sign1 verifies independently with verify_strict); wrong/double/missing tag, 3/5-element arrays,
nil payload, non-bstr protected header, empty protected, truncation at every offset, wrong alg type/value, alg/kid only in unprotected header, non-bstr kid all rejected; kid lookup is exact-bytes and swapping kid breaks the signature;
S+L scalar malleability rejected; deep nesting (100k) and giant declared lengths return errors without panic/OOM.
vc-core JCS: 1e21 -> 1e+21, 1e-7, 0.000001, 1.0 -> 1, -0 and -0.0 -> 0, 5e-324, f64::MAX, 1e400 and NaN rejected, lone surrogates rejected, valid pair kept, control-char escapes, depth 100 ok / 100k errors, field order independent, RFC 8785 example.
Chain: first seq != 0, uppercase-hex prev, float / negative / string / null seq, missing fields, outcome before its intent, outcome referencing an outcome id (self-reference), resubmitted statement, duplicate outcome, cross-agent outcome, interleaved agents with open intents, statement_id independent of field order and of `1.0` vs `1`, 30k-statement log validates linearly. (`want_seq = n + 1` could overflow at u64::MAX but is unreachable because seq must start at 0 and increase by 1.)

## Counts

Confirmed bugs: High 2, Medium 7, Low 6 (total 15 findings, 17 failing tests). Ambiguities: 6.

## Top three findings

1. H1/H2 - checkpoint quorum is inflatable (same key under two names; operator counted as a witness) - breaks the independent-witness guarantee that split-view detection rests on.
2. M7 - signatures verified with non-strict `verify`: an identity/small-order key makes every COSE statement and checkpoint signature forgeable.
3. M1-M3 - canonicalization is not RFC 8785 conformant (UTF-16 key order, integers over 2^53, duplicate keys), so statement ids/signatures will not interoperate with other implementations and parser differentials exist.

## Resolution log (orchestrator, 2026-10-24)
All 15 findings fixed; the 17 failing `bug_*` tests now pass and are kept as regression tests. H1/H2: quorum counts distinct keys and names and excludes the operator. M1-M3: new strict JCS serializer (UTF-16 key order, ES6 numbers, duplicate keys/lone surrogates rejected). M4: schema constants enforced (`v`, kind enum incl. v8 kinds, agent id, ts string, hex ids). M5-M7, L3-L6: strict COSE (single definite item, duplicate labels, crit, unprotected map, `verify_strict`, weak keys). L1: canonical size decimal. L2: control characters neutralized in `sign_line`; `try_sign_line` enforces signed-note name rules (verification stays lenient about spaces, per the regression test). A1: `verify_note_for_origin` added. A2-A6 documented. Independent vectors (`spec/vectors`, authored from RFCs without reading the Rust code) agree with the implementation on 880 inclusion + 860 consistency positives, ~6,000 negatives, 50 JCS and 27 chain cases (one spec ambiguity found by the vector author: chain vectors originally omitted schema constants; vectors regenerated schema-valid).
