#!/usr/bin/env python3
"""Traceability gate (Q19): every implemented/partial requirement must cite existing tests."""
import re, sys, pathlib
root = pathlib.Path(__file__).resolve().parents[2]
rows = [l for l in (root / "spec/requirements.md").read_text().splitlines() if l.startswith("| FR-")]
errors, counts = [], {"implemented": 0, "partial": 0, "pending": 0}
for r in rows:
    cells = [c.strip() for c in r.strip("|").split("|")]
    rid, status, tests = cells[0], cells[3], cells[4] if len(cells) > 4 else ""
    counts[status] = counts.get(status, 0) + 1
    if status in ("implemented", "partial"):
        refs = [t.strip() for t in tests.split(",") if t.strip()]
        if not refs: errors.append(f"{rid}: {status} but no tests listed")
        for ref in refs:
            path, _, fn = ref.partition("#")
            f = root / path
            if not f.exists(): errors.append(f"{rid}: missing file {path}"); continue
            if not re.search(rf"fn\s+{re.escape(fn)}\s*\(", f.read_text()): errors.append(f"{rid}: {fn} not found in {path}")
print("requirements:", counts)
for e in errors: print("TRACE ERROR:", e)
sys.exit(1 if errors else 0)
