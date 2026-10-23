# VeriChain

**A platform where autonomous agents run, transact and are held accountable.** Everything an agent is authorized to do and does is recorded in a witnessed, append-only, post-quantum-ready log that anyone can verify offline. Agents run on VeriChain (strongest evidence) or beside it (gateway/signer).

> Status: **specs at production grade (v8 / MAS v3), awaiting approval; L0 code started** (`vc-core`, `vc-log`, `vc-cose` pass tests). The previous contents of this repository (an ASL toolchain copy and the v6 "agent economy" architecture) are preserved on branch `archive/pre-reset-2026-10-23`. The ASL language lives at https://github.com/agentseedlanguage-cpu/agentseed.

## Why
Agents now act with money, data and authority, and most "audit trails" are self-signed logs that can be bypassed or rewritten. VeriChain aims to provide:
- **Completeness by construction:** with ASL, no effect leaves the runtime without a receipt (C2/C3); a gateway tier (C1) serves any other agent.
- **Authorization lineage:** receipts carry the capability chain and policy hash.
- **Witnessed non-equivocation:** tile log, signed checkpoints, independent cosigners; export as SCITT (RFC 9943) receipts.
- **Batch post-quantum signing** and an honest assurance ladder (A0 signed → A3 proof).

## Documents
| File | Purpose |
|---|---|
| `docs/arc42/VERICHAIN_ARC42_v8.md` | **VeriChain full-platform spec (v8 draft; v7 = superseded L0 baseline)** |
| `docs/arc42/MAS_ARC42_v3.md` | **Agents on VeriChain: flagship tenant, strategies, capital phases (v3 draft; v2 superseded)** |
| `ARCHITECTURE.md` | Short architecture summary and pointers |
| `ROADMAP.md` | Design gates and commercial milestones |
| `DEVLOG.md` | Dated log of decisions and findings |
| `docs/addenda/` | A gap analysis · B ARC42 review · C frontier/differentiation · D revenue and flywheel · E revenue evidence and pricing · F trading research · G go-to-market answers |
| `docs/archive/` | v6 VeriChain ARC42, MAS (trading swarm) ARC42, v6 task list: historical, not current |

## License
Apache-2.0
