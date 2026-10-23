# spec/
Language-neutral artifacts. `statement.schema.json` freezes Decision Primitive v2 fields (G1, pending final sign-off of any open item below).

Chain rule (implemented in `vc-core::chain`): a statement id is hex SHA-256 of its RFC 8785 bytes; each agent's first statement has `seq` 0 and `prev` of 64 zeros; later ones have `seq` = last+1 and `prev` = id of the previous statement of that agent; an `outcome` must reference an earlier `intent` of the same agent exactly once; intents with no outcome are reported as open (orphan policy and timeout are applied by the caller).

Merkle rules (implemented in `vc-log`): RFC 9162 hashing with 0x00/0x01 domain separation; the 8-leaf Certificate Transparency reference roots and all inclusion/consistency pairs up to size 130/70 are tested.

Checkpoints (implemented in `vc-log::checkpoint`): C2SP tlog-checkpoint body (origin, decimal size, base64 root) inside a signed note; Ed25519 type byte 0x01; key id = SHA-256(name || 0x0A || type || pubkey)[:4]. Formats read from the C2SP specs this session. A checkpoint verifies only if the operator signed and at least N distinct trusted witnesses signed that exact text.

COSE (`vc-cose`): tagged COSE_Sign1, protected header {1: alg -19 (Ed25519), 3: content type, 4: kid}, empty external AAD, attached payload.

Open before vectors are published as files: RFC 3161 anchoring format, commitment HMAC key derivation, ML-DSA hybrid labels, final content-type string.
