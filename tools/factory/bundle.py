#!/usr/bin/env python3
"""Acceptance bundle: evidence-backed acceptance for a change set (ARC42 v9 Appendix G, rule 4).
Usage: bundle.py NAME --reqs FR-.. [--verifier-report PATH] [--human-signoff NAME]
Writes factory/bundles/<NAME>.json. Later versions submit the bundle to a VeriChain log as an `artifact` statement."""
import argparse, hashlib, json, pathlib, re, subprocess, datetime
root = pathlib.Path(__file__).resolve().parents[2]
ap = argparse.ArgumentParser(); ap.add_argument("name"); ap.add_argument("--reqs", nargs="+", required=True)
ap.add_argument("--verifier-report"); ap.add_argument("--human-signoff"); a = ap.parse_args()
sh = lambda *c: subprocess.run(c, cwd=root, capture_output=True, text=True)
sha = lambda p: hashlib.sha256((root / p).read_bytes()).hexdigest()
test = sh("cargo", "test", "--workspace", "--no-fail-fast", "-q")
passed = sum(int(m) for m in re.findall(r"(\d+) passed", test.stdout)); failed = sum(int(m) for m in re.findall(r"(\d+) failed", test.stdout))
trace = sh("python3", "tools/factory/trace_check.py")
vectors = {p.name: sha(f"spec/vectors/{p.name}") for p in sorted((root / "spec/vectors").glob("*.json"))}
b = {"bundle_version": 1, "name": a.name, "created": datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
     "git_head": sh("git", "rev-parse", "HEAD").stdout.strip(), "git_dirty": bool(sh("git", "status", "--porcelain").stdout.strip()),
     "requirements": a.reqs, "tests": {"passed": passed, "failed": failed, "exit_code": test.returncode},
     "traceability": {"exit_code": trace.returncode, "summary": trace.stdout.strip().splitlines()[0] if trace.stdout else ""},
     "vector_sha256": vectors, "cargo_lock_sha256": sha("Cargo.lock"),
     "verifier_report": ({"path": a.verifier_report, "sha256": sha(a.verifier_report)} if a.verifier_report else None),
     "human_signoff": a.human_signoff, "accepted": test.returncode == 0 and trace.returncode == 0 and bool(a.verifier_report)}
out = root / "factory/bundles" / f"{a.name}.json"; out.write_text(json.dumps(b, indent=2) + "\n"); print(out, "accepted:", b["accepted"])
