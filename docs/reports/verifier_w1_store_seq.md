# Verifier report W1: vc-store / vc-seq (consolidated, rounds 1-3)

Round 1 (`verifier_w1_*`, findings ST-1..ST-8, SQ-1..SQ-7), round 2 (`verifier_w1b_*`, see
`verifier_w1b_store_seq.md`: ST-9, ST-10, SQ-8..SQ-12), round 3 (`verifier_w1c_*`, below). All three rounds were
written by independent verifier agents that did not author the code. Status: every `bug_*` test now passes;
workspace tests, `--release --include-ignored`, clippy `-D warnings`, fmt and trace check are green.

## Design change that closed rounds 1-2 (commit 340713a)
`meta/commit` (size + Merkle root) is written last in every append and is the single linearisation point;
`append_batch` is transactional; `open` is O(1) and checks derived tiles against the committed root (rebuild
from entries on mismatch); full hash tiles are verified against their parent hash and entries against leaf
hashes; `scan` reads each bundle once; FsStore has unique temp names, durable dir creation, writer lock.
vc-seq: leaf-hash duplicate detection over durable entries only, size check before parsing, strict
completeness/assurance with ceilings, MAX_STATEMENT_BYTES = 65535, rollback into locals, self-healing poison.

## Round 3 findings (all fixed)
| ID | Sev | Finding | Fix |
|---|---|---|---|
| ST-11 | High | Stale second writer overwrote same-path committed partial bundle/tile before its commit CAS failed, leaving the store unopenable (also after the first writer exits and drops its flock) | Fence (compare store commit to the instance's last-seen commit) runs BEFORE any write; CAS retained at commit |
| ST-12 | Medium | Verified-tile cache pinned the verdict, not the bytes: after a tile was read once, replacing bundle+tile with a forged consistent pair was accepted | Cache keyed to SHA-256 of the verified bytes; changed bytes are re-verified against the parent |
| ST-13 | Medium | Pre-commit (legacy) stores: a failed batch whose entries landed became durable on reopen via size inference | First append to a commit-less store pins `meta/commit` before any entry is written (open stays write-free, so read-only stores still open) |
| SQ-13 | Low | A poisoned sequencer reported open intents that never became durable | On poison, speculative chain state is dropped and `open_intents()` is empty until reconciled |

## Dispositions / residual risks (recorded, not hidden)
- Interior full entry bundles are verified lazily (`entry`, `scan`, `verify_integrity`; `Sequencer::open` replays via `scan`, so a truncated or altered bundle fails sequencer start). `TileLog::open` alone does not read them (ST-5 requires O(1) open). The round-1 test `bug_short_full_entry_bundle_rejected_at_open` was corrected accordingly.
- A writer race between the fence check and the commit put is narrowed to a tiny window (flock for FsStore, CAS for others); true multi-writer needs a store with conditional put (vcd deployment concern, tracked for G3).
- Full-bundle `entry(i)` reads the whole bundle (<=16 MB); the sequencer avoids it via the in-memory leaf map. A range-read store API is a later optimisation.
- C2SP tlog-tiles layout is still implemented from the Go reference, unverified against the spec text.
