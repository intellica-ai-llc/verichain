#!/usr/bin/env python3
"""Generate statement-chain vectors -> spec/vectors/chain_v1.json.  Stdlib only.

Statement id = lowercase hex SHA-256 of the RFC 8785 canonical bytes of the statement object
(the statement has no id field).  Statements here contain only strings and non-negative
integers, so a minimal JCS (UTF-16-sorted keys, no whitespace, integer digits) is exact.
Validation order per statement (first failure wins, scanning in log order):
  1 SeqGap   : seq != (last seq of that agent + 1), or != 0 for the agent's first statement
  2 BadPrev  : prev != id of that agent's previous statement (64 zeros for the first)
  3 for kind=outcome, with I = pair.intent_id:
       UnknownIntent   : I is not the id of any EARLIER statement of kind intent
       AgentMismatch   : that intent belongs to a different agent
       DuplicateOutcome: an earlier outcome already closed I
Result ok: open = ids of intents with no outcome, in log order.
"""
import hashlib, json, os, sys
OUT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "spec", "vectors"))
ZERO = "0" * 64

def jcs(x):
    if isinstance(x, str): return json.dumps(x, ensure_ascii=False)  # ASCII-only ids/names here: no escapes needed
    if isinstance(x, int): return str(x)
    if isinstance(x, dict): return "{" + ",".join(jcs(k) + ":" + jcs(x[k]) for k in sorted(x, key=lambda s: s.encode("utf-16-be"))) + "}"
    raise TypeError(x)
import datetime
def ts_str(t): return datetime.datetime.fromtimestamp(t, datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
def sid(st): return hashlib.sha256(jcs(st).encode("utf-8")).hexdigest()

def check(stmts):
    """Reference validator. Returns ('ok', open_ids) or (err, index)."""
    last = {}; kinds = {}; closed = set(); order = []
    for i, s in enumerate(stmts):
        a = s["agent"]["id"]
        want_seq = last[a][0] + 1 if a in last else 0
        want_prev = last[a][1] if a in last else ZERO
        if s["seq"] != want_seq: return ("SeqGap", i)
        if s["prev"] != want_prev: return ("BadPrev", i)
        if s["kind"] == "outcome":
            t = s["pair"]["intent_id"]
            if t not in kinds or kinds[t][0] != "intent": return ("UnknownIntent", i)
            if kinds[t][1] != a: return ("AgentMismatch", i)
            if t in closed: return ("DuplicateOutcome", i)
            closed.add(t)
        h = sid(s); last[a] = (s["seq"], h); kinds[h] = (s["kind"], a)
        if s["kind"] == "intent": order.append(h)
    return ("ok", [h for h in order if h not in closed])

class Log:
    """Builds statements, tracking per-agent chain state."""
    def __init__(self): self.s = []; self.last = {}; self.t = 1700000000; self.ids = []
    def add(self, agent, kind, pair=None, **ov):
        seq, prev = self.last.get(agent, (-1, ZERO))
        st = dict(v=2, kind=kind, agent=dict(id=agent), seq=seq + 1, prev=prev, ts=ts_str(self.t))
        self.t += 7
        if pair is not None: st["pair"] = dict(intent_id=pair)
        for k, val in ov.items(): st[k] = val
        self.s.append(st); h = sid(st); self.ids.append(h)
        self.last[agent] = (st["seq"], h)
        return h

cases = []
def case(name, desc, log, expect, index=None, open_ids=None):
    r = check(log.s)
    if expect == "ok":
        assert r == ("ok", open_ids), (name, r, open_ids)
        e = dict(expected="ok", open_intent_ids=open_ids)
    else:
        assert r == (expect, index), (name, r, expect, index)
        e = dict(expected="error", error=expect, index=index)
    cases.append(dict(name=name, description=desc, statements=log.s, statement_ids=log.ids, **e))

A, B, C = "agent-a", "agent-b", "agent-c"

L = Log(); case("empty_log", "no statements", L, "ok", open_ids=[])
L = Log(); i0 = L.add(A, "intent"); case("single_intent_open", "one open intent", L, "ok", open_ids=[i0])
L = Log(); i0 = L.add(A, "intent"); L.add(A, "outcome", i0); case("intent_then_outcome", "closed pair", L, "ok", open_ids=[])
L = Log(); i0 = L.add(A, "intent"); i1 = L.add(A, "intent"); L.add(A, "outcome", i0)
case("two_intents_close_first", "second remains open", L, "ok", open_ids=[i1])
L = Log(); i0 = L.add(A, "intent"); i1 = L.add(A, "intent"); L.add(A, "outcome", i1)
case("two_intents_close_second", "out-of-order close; first remains open", L, "ok", open_ids=[i0])
L = Log(); a0 = L.add(A, "intent"); b0 = L.add(B, "intent"); L.add(A, "outcome", a0); L.add(B, "outcome", b0)
case("interleaved_two_agents_all_closed", "A,B,A,B interleaving, each chain independent", L, "ok", open_ids=[])
L = Log(); a0 = L.add(A, "intent"); b0 = L.add(B, "intent"); a1 = L.add(A, "intent"); b1 = L.add(B, "intent"); L.add(B, "outcome", b0); L.add(A, "outcome", a1)
case("interleaved_two_agents_open_in_log_order", "open ids reported in log order across agents", L, "ok", open_ids=[a0, b1])
L = Log(); a0 = L.add(A, "intent"); b0 = L.add(B, "intent"); c0 = L.add(C, "intent"); L.add(C, "outcome", c0); a1 = L.add(A, "intent"); L.add(B, "outcome", b0); L.add(A, "outcome", a0); c1 = L.add(C, "intent")
case("three_agents_interleaved", "three agents mixed", L, "ok", open_ids=[a1, c1])
L = Log(); i = [L.add(A, "intent") for _ in range(3)]; L.add(A, "outcome", i[1]); i3 = L.add(A, "intent"); L.add(A, "outcome", i[0]); L.add(A, "outcome", i3)
case("long_chain_mixed", "8 statements one agent", L, "ok", open_ids=[i[2]])

L = Log(); L.add(A, "intent", seq=1); case("seqgap_first_statement_seq1", "first statement must be seq 0", L, "SeqGap", 0)
L = Log(); L.add(A, "intent"); L.add(A, "intent"); L.add(A, "intent", seq=3); case("seqgap_skip", "seq jumps 1 -> 3", L, "SeqGap", 2)
L = Log(); L.add(A, "intent"); L.add(A, "intent"); L.add(A, "intent", seq=1); case("seqgap_duplicate_seq", "repeated seq 1 (non-increasing)", L, "SeqGap", 2)
L = Log(); L.add(A, "intent"); b = L.add(B, "intent"); L.add(B, "intent"); L.add(A, "intent", seq=2); case("seqgap_interleaved_other_agent_unaffected", "A skips seq while B is fine", L, "SeqGap", 3)
L = Log(); L.add(A, "intent", prev="1" * 64); case("badprev_first_not_zero", "first statement prev nonzero", L, "BadPrev", 0)
L = Log(); L.add(A, "intent"); L.add(A, "intent", prev=ZERO); case("badprev_second_uses_zero", "seq1 with zero prev", L, "BadPrev", 1)
L = Log(); L.add(A, "intent"); b0 = L.add(B, "intent"); L.add(A, "intent", prev=b0); case("badprev_global_chain_mistake", "prev points at other agent's statement (id of previous log entry)", L, "BadPrev", 2)
L = Log(); a0 = L.add(A, "intent"); L.add(A, "intent"); L.add(A, "intent", prev=a0); case("badprev_skips_one", "prev is two back", L, "BadPrev", 2)
L = Log(); L.add(A, "intent", prev=ZERO[:-1] + "1", seq=1); case("both_seq_and_prev_wrong_seqgap_wins", "check order: SeqGap before BadPrev", L, "SeqGap", 0)
L = Log(); L.add(A, "outcome", "f" * 64); case("unknown_intent_nonexistent_first_stmt", "outcome as first statement of agent", L, "UnknownIntent", 0)
L = Log(); L.add(A, "intent"); L.add(A, "outcome", "a" * 64); case("unknown_intent_bogus_id", "id matches nothing", L, "UnknownIntent", 1)
L = Log(); a0 = L.add(A, "intent"); a1 = L.add(A, "outcome", a0); L.add(A, "outcome", a1)
case("unknown_intent_references_outcome", "pair targets an outcome statement id, not an intent", L, "UnknownIntent", 2)
L = Log(); L.add(A, "intent"); L.add(A, "outcome", sid(dict(v=2, kind="intent", agent=dict(id=A), seq=2, prev=ZERO, ts=ts_str(0))))
case("unknown_intent_forward_reference", "references an id not (yet) in the log", L, "UnknownIntent", 1)
L = Log(); a0 = L.add(A, "intent"); L.add(A, "outcome", a0); L.add(A, "outcome", a0); case("duplicate_outcome_same_agent", "intent closed twice", L, "DuplicateOutcome", 2)
L = Log(); a0 = L.add(A, "intent"); b0 = L.add(B, "intent"); L.add(A, "outcome", a0); L.add(B, "intent"); L.add(A, "outcome", a0)
case("duplicate_outcome_interleaved", "second close after interleaved activity", L, "DuplicateOutcome", 4)
L = Log(); a0 = L.add(A, "intent"); L.add(B, "intent"); L.add(B, "outcome", a0); case("agent_mismatch_outcome_for_other_agents_intent", "B closes A's intent", L, "AgentMismatch", 2)
L = Log(); a0 = L.add(A, "intent"); L.add(A, "outcome", a0); L.add(B, "intent"); L.add(B, "outcome", a0)
case("agent_mismatch_other_agents_closed_intent", "B references A's already-closed intent: AgentMismatch (not Duplicate)", L, "AgentMismatch", 3)
L = Log(); a0 = L.add(A, "intent"); b0 = L.add(B, "intent"); L.add(A, "outcome", a0); L.add(B, "outcome", b0); L.add(C, "intent"); L.add(C, "outcome", b0)
case("error_late_in_multi_agent_log", "valid prefix of 4, error at index 5", L, "AgentMismatch", 5)

doc = dict(version=1, spec="statement chain v1 (see tools/vectors/gen_chain.py docstring)", zero_prev=ZERO,
           notes=["statement id = lowercase hex SHA-256 of RFC 8785 canonical bytes of the statement object",
                  "statements are schema-valid (v=2, ts RFC 3339 UTC string); all other fields are strings or non-negative integers",
                  "validation order: SeqGap, BadPrev, then outcome checks UnknownIntent, AgentMismatch, DuplicateOutcome; first error in log order is reported",
                  "open_intent_ids are in log order", "statement_ids[i] is the id of statements[i]"],
           cases=cases)
with open(os.path.join(OUT, "chain_v1.json"), "w") as f:
    f.write("{\n" + ",\n".join(json.dumps(k) + ": " + (json.dumps(vv) if k != "cases" else
            "[\n" + ",\n".join("  " + json.dumps(c, separators=(",", ":")) for c in vv) + "\n]") for k, vv in doc.items()) + "\n}\n")
# reload + recheck from file, ids recomputed
d = json.load(open(os.path.join(OUT, "chain_v1.json")))
for c in d["cases"]:
    assert [sid(s) for s in c["statements"]] == c["statement_ids"], c["name"]
    r = check(c["statements"])
    exp = ("ok", c["open_intent_ids"]) if c["expected"] == "ok" else (c["error"], c["index"])
    assert r == exp, c["name"]
print("chain_v1.json cases:", len(d["cases"]), "ok:", sum(c["expected"] == "ok" for c in d["cases"]))
print("self-checks passed")
