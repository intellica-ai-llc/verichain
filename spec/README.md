# spec/
Language-neutral artifacts. `statement.schema.json` freezes Decision Primitive v2 fields (G1, pending final sign-off of any open item below).

Chain rule (implemented in `vc-core::chain`): a statement id is hex SHA-256 of its RFC 8785 bytes; each agent's first statement has `seq` 0 and `prev` of 64 zeros; later ones have `seq` = last+1 and `prev` = id of the previous statement of that agent; an `outcome` must reference an earlier `intent` of the same agent exactly once; intents with no outcome are reported as open (orphan policy and timeout are applied by the caller).

Merkle rules (implemented in `vc-log`): RFC 9162 hashing with 0x00/0x01 domain separation; the 8-leaf Certificate Transparency reference roots and all inclusion/consistency pairs up to size 130/70 are tested.

Open before COSE/log vectors are published: exact COSE header labels, checkpoint text format (C2SP tlog-checkpoint candidate; unverified), commitment HMAC key derivation.
