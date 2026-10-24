#!/usr/bin/env python3
"""Generate Merkle test vectors (RFC 9162 s2.1) -> spec/vectors/merkle_v1.json and
spec/vectors/ct_reference_v1.json.  Stdlib only.  Independent of any Rust code.

Definitions (RFC 9162 2.1.1-2.1.4):
  MTH({})     = SHA-256()
  MTH({d})    = SHA-256(0x00 || d)
  MTH(D[n])   = SHA-256(0x01 || MTH(D[0:k]) || MTH(D[k:n])),  k = largest power of 2 < n
Exits non-zero if any self-check fails.
"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.normpath(os.path.join(HERE, "..", "..", "spec", "vectors"))


def H(b): return hashlib.sha256(b).digest()
def lpow2(n):  # largest power of two strictly less than n (n >= 2)
    return 1 << ((n - 1).bit_length() - 1)


class Tree:
    def __init__(self, entries):
        self.entries = entries
        self.leaf = [H(b"\x00" + e) for e in entries]
        self.memo = {}

    def mth(self, lo, hi):
        key = (lo, hi)
        if key in self.memo: return self.memo[key]
        n = hi - lo
        if n == 0: r = H(b"")
        elif n == 1: r = self.leaf[lo]
        else:
            k = lpow2(n)
            r = H(b"\x01" + self.mth(lo, lo + k) + self.mth(lo + k, hi))
        self.memo[key] = r
        return r

    def root(self, n): return self.mth(0, n)

    # 2.1.3.1  PATH(m, D[n])  (recursive, absolute index ranges)
    def path(self, m, lo, hi):
        n = hi - lo
        if n == 1: return []
        k = lpow2(n)
        if m < k: return self.path(m, lo, lo + k) + [self.mth(lo + k, hi)]
        return self.path(m - k, lo + k, hi) + [self.mth(lo, lo + k)]

    def inclusion(self, m, n): return self.path(m, 0, n)

    # 2.1.4.1  PROOF(m, D[n]) = SUBPROOF(m, D[n], true)
    def subproof(self, m, lo, hi, b):
        n = hi - lo
        if m == n: return [] if b else [self.mth(lo, hi)]
        k = lpow2(n)
        if m <= k: return self.subproof(m, lo, lo + k, b) + [self.mth(lo + k, hi)]
        return self.subproof(m - k, lo + k, hi, False) + [self.mth(lo, lo + k)]

    def consistency(self, m, n): return self.subproof(m, 0, n, True)


def is_hash(x): return isinstance(x, bytes) and len(x) == 32


# 2.1.3.2 verification of inclusion proof
def verify_inclusion(idx, size, leaf_hash, proof, root):
    if not (is_hash(leaf_hash) and is_hash(root) and all(is_hash(p) for p in proof)): return False
    if idx >= size: return False
    fn, sn, r = idx, size - 1, leaf_hash
    for p in proof:
        if sn == 0: return False
        if (fn & 1) or fn == sn:
            r = H(b"\x01" + p + r)
            if not (fn & 1):
                while not (fn & 1) and fn != 0:
                    fn >>= 1; sn >>= 1
        else:
            r = H(b"\x01" + r + p)
        fn >>= 1; sn >>= 1
    return sn == 0 and r == root


# 2.1.4.2 verification of consistency proof
def verify_consistency(first, second, first_hash, second_hash, proof):
    if not (is_hash(first_hash) and is_hash(second_hash) and all(is_hash(p) for p in proof)): return False
    if first < 1 or first > second: return False
    if first == second: return len(proof) == 0 and first_hash == second_hash
    path = list(proof)
    if first & (first - 1) == 0: path = [first_hash] + path
    if not path: return False
    fn, sn = first - 1, second - 1
    while fn & 1:
        fn >>= 1; sn >>= 1
    fr = sr = path[0]
    for c in path[1:]:
        if sn == 0: return False
        if (fn & 1) or fn == sn:
            fr = H(b"\x01" + c + fr); sr = H(b"\x01" + c + sr)
            if not (fn & 1):
                while not (fn & 1) and fn != 0:
                    fn >>= 1; sn >>= 1
        else:
            sr = H(b"\x01" + sr + c)
        fn >>= 1; sn >>= 1
    return fr == first_hash and sr == second_hash and sn == 0


def iterative_root(leaves_hashes):
    """Second, independent algorithm (stack of perfect subtrees) used only as a cross-check."""
    stack = []  # (height, hash)
    for lh in leaves_hashes:
        h, x = 0, lh
        while stack and stack[-1][0] == h:
            _, l = stack.pop(); x = H(b"\x01" + l + x); h += 1
        stack.append((h, x))
    if not stack: return H(b"")
    r = stack[-1][1]
    for _, l in reversed(stack[:-1]): r = H(b"\x01" + l + r)
    return r


X = lambda b: b.hex()
FAILS = []
def check(cond, msg):
    if not cond: FAILS.append(msg)


def flip(h, which=0):
    b = bytearray(h); b[which % 32] ^= 0x01; return bytes(b)


# ------------------------------------------------------------------ generation
def inc_vec(T, m, n):
    return dict(id=f"inc-{n}-{m}", leaf_index=m, tree_size=n, leaf_hash=X(T.leaf[m]),
                root=X(T.root(n)), proof=[X(p) for p in T.inclusion(m, n)], expect="accept")


def inc_negatives(T, m, n):
    base = inc_vec(T, m, n)
    proof = T.inclusion(m, n); leaf = T.leaf[m]; root = T.root(n)
    out = []

    def add(cat, idx=m, size=n, lh=leaf, pr=proof, rt=root):
        # a negative must be rejected by our verifier, else it is not a valid negative
        if verify_inclusion(idx, size, lh, pr, rt): return False
        out.append(dict(id=f"{base['id']}-neg-{cat}", category=cat, base=base["id"], leaf_index=idx, tree_size=size,
                        leaf_hash=X(lh), root=X(rt), proof=[X(p) for p in pr], expect="reject"))
        return True

    if proof:
        k = (m + n) % len(proof)
        pr = list(proof); pr[k] = flip(pr[k], m); add("flip_proof_hash", pr=pr)
        add("remove_proof_element", pr=proof[:-1])
        add("remove_first_proof_element", pr=proof[1:])
        pr = list(proof); pr[k] = pr[k][:31]; add("truncated_proof_hash", pr=pr)
    add("extra_proof_element", pr=proof + [H(b"extra")])
    add("flip_leaf_hash", lh=flip(leaf))
    for c in ((m + 1) % n, (m - 1) % n, m ^ 1):
        if c != m and c < n and add("wrong_index", idx=c): break
    add("index_out_of_range", idx=n)
    cnt = 0
    for c in (n + 1, n - 1, 2 * n, n + 2, max(n - 2, 0)):
        if c != n and cnt < 2 and add("wrong_size", size=c): cnt += 1
    add("size_zero", size=0)
    other = T.root(n + 1) if n + 1 <= len(T.leaf) else H(b"other-root")
    add("swapped_root", rt=other)
    add("zero_root", rt=bytes(32))
    return out


def cons_vec(T, m, n):
    return dict(id=f"cons-{m}-{n}", first_size=m, second_size=n, first_root=X(T.root(m)),
                second_root=X(T.root(n)), proof=[X(p) for p in T.consistency(m, n)], expect="accept")


def cons_negatives(T, m, n):
    base = cons_vec(T, m, n)
    proof = T.consistency(m, n); r1 = T.root(m); r2 = T.root(n)
    out = []

    def add(cat, a=m, b=n, x=r1, y=r2, pr=proof):
        if verify_consistency(a, b, x, y, pr): return False
        out.append(dict(id=f"{base['id']}-neg-{cat}", category=cat, base=base["id"], first_size=a, second_size=b,
                        first_root=X(x), second_root=X(y), proof=[X(p) for p in pr], expect="reject"))
        return True

    if proof:
        k = (m + n) % len(proof)
        pr = list(proof); pr[k] = flip(pr[k], n); add("flip_proof_hash", pr=pr)
        add("remove_proof_element", pr=proof[:-1])
        add("remove_first_proof_element", pr=proof[1:])
        pr = list(proof); pr[k] = pr[k][:31]; add("truncated_proof_hash", pr=pr)
    add("extra_proof_element", pr=proof + [H(b"extra")])
    for c in (m - 1, m + 1, m * 2):
        if 1 <= c <= n and c != m and add("wrong_first_size", a=c): break
    for c in (n + 1, n - 1, n * 2):
        if c >= m and c != n and add("wrong_second_size", b=c): break
    if m != n: add("swapped_roots", x=r2, y=r1)
    add("flip_first_root", x=flip(r1))
    add("flip_second_root", y=flip(r2))
    if m < n: add("first_size_greater_than_second", a=n, b=m)
    return out


def main():
    N_MAX = 1025
    T = Tree([f"entry-{i}".encode() for i in range(N_MAX)])
    small_roots = list(range(0, 65))
    large = [127, 128, 129, 255, 256, 257, 511, 512, 513, 1023, 1024, 1025]
    rsizes = small_roots + large

    # cross-check roots with the iterative algorithm
    for n in rsizes: check(T.root(n) == iterative_root(T.leaf[:n]), f"root mismatch {n}")
    roots = [dict(size=n, root=X(T.root(n))) for n in rsizes]

    # --- inclusion
    inc_pos, inc_neg = [], []
    def inc_indices_large(n): return sorted({0, 1, n // 2, n - 2, n - 1})
    for n in range(1, 41):
        for m in range(n): inc_pos.append(inc_vec(T, m, n))
        for m in sorted({0, 1, n // 2, n - 2, n - 1} & set(range(n))): inc_neg += inc_negatives(T, m, n)
    for n in large:
        for m in inc_indices_large(n):
            inc_pos.append(inc_vec(T, m, n)); inc_neg += inc_negatives(T, m, n)

    # --- consistency
    cons_pos, cons_neg = [], []
    for n in range(1, 41):
        for m in range(1, n + 1):
            cons_pos.append(cons_vec(T, m, n))
            if n <= 16 or m in {1, n // 2, n - 1, lpow2(n) if n > 1 else 1}:
                cons_neg += cons_negatives(T, m, n)
    for n in large:
        for m in sorted({1, n // 2, n - 1, lpow2(n)}):
            cons_pos.append(cons_vec(T, m, n)); cons_neg += cons_negatives(T, m, n)

    doc = dict(
        version=1, spec="RFC 9162 section 2.1 (SHA-256)",
        hash="SHA-256; leaf = H(0x00||entry), node = H(0x01||left||right); empty root = H('')",
        entry_template="entry i = UTF-8 bytes of the string 'entry-{i}' (decimal i, no padding)",
        empty_root=X(H(b"")),
        notes=["inclusion verification takes the already-hashed leaf (leaf_hash)",
               "negatives marked expect=reject are guaranteed to fail the RFC 9162 verification algorithms; "
               "wrong_size negatives only include sizes where the algorithm rejects (some wrong sizes are inherently "
               "accepted by the RFC algorithm because the verifier cannot see the right-hand structure)",
               "truncated_proof_hash has an element of 31 bytes (malformed length)"],
        roots=roots,
        inclusion=inc_pos, inclusion_negative=inc_neg,
        consistency=cons_pos, consistency_negative=cons_neg)

    # ------------------------------------------------------------- CT reference tree
    ct_hex = ["", "00", "10", "2021", "3031", "40414243", "5051525354555657", "606162636465666768696a6b6c6d6e6f"]
    CT = Tree([bytes.fromhex(h) for h in ct_hex])
    recalled = {  # from memory of the Certificate Transparency reference tests (unverified)
        1: "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d",
        2: "fac54203e7cc696cf0dfcb42c92a1d9dbaf70ad9e621f4bd8d98662f00e3c125",
        3: "aeb6bcfe274b70a14fb067a5e5578264db0fa9b51af5e0ba159158f329e06e77",
        4: "d37ee418976dd95753c1c73862b9398fa2a2cf9b4ff0fdfe8b30cd95209614b7",
        5: "4e3bbb1f7b478dcfe71fb631631519a3bca12c9aefca1612bfce4c13a86264d4",
        6: "76e67dadbcdf1e10e1b74ddc608abd2f98dfb16fbce75277b5232a127f2087ef",
        7: "ddb89be403809e325750d3d263cd78929c2942b7942a34b77e122c9594a74c8c",
        8: "5dc9da79a70659a9ad559cb701ded9a2ab9d823aad2f4960cfe370eff4604328"}
    ct_roots = []
    for n in range(1, 9):
        assert CT.root(n) == iterative_root(CT.leaf[:n])
        ct_roots.append(dict(size=n, root=X(CT.root(n)), recalled_reference_root=recalled[n],
                             matches_recalled=X(CT.root(n)) == recalled[n]))
    ct = dict(version=1,
              note="Roots were computed locally by gen_merkle.py from the entries below. 'recalled_reference_root' values "
                   "are from the author's memory of the Certificate Transparency reference test data and are UNVERIFIED; "
                   "cross-check the computed roots against the authoritative CT reference values before relying on either.",
              entries_hex=ct_hex, empty_root=X(H(b"")),
              leaf_hashes=[X(h) for h in CT.leaf], roots=ct_roots)

    # CT inclusion/consistency self-checks (not emitted, covered by merkle_v1)
    for n in range(1, 9):
        for m in range(n):
            check(verify_inclusion(m, n, CT.leaf[m], CT.inclusion(m, n), CT.root(n)), f"ct inc {m},{n}")
        for m in range(1, n + 1):
            check(verify_consistency(m, n, CT.root(m), CT.root(n), CT.consistency(m, n)), f"ct cons {m},{n}")

    # ------------------------------------------------------------- write, reload, self-check from file
    os.makedirs(OUT, exist_ok=True)
    def dump(path, d):
        with open(path, "w") as f:
            f.write("{\n")
            keys = list(d)
            for i, k in enumerate(keys):
                v = d[k]
                if isinstance(v, list) and v and isinstance(v[0], dict):
                    f.write(json.dumps(k) + ": [\n" + ",\n".join("  " + json.dumps(x, separators=(",", ":")) for x in v) + "\n]")
                else:
                    f.write(json.dumps(k) + ": " + json.dumps(v))
                f.write(",\n" if i < len(keys) - 1 else "\n")
            f.write("}\n")
    p_m = os.path.join(OUT, "merkle_v1.json"); p_c = os.path.join(OUT, "ct_reference_v1.json")
    dump(p_m, doc); dump(p_c, ct)

    d = json.load(open(p_m)); B = bytes.fromhex
    for v in d["inclusion"]:
        check(verify_inclusion(v["leaf_index"], v["tree_size"], B(v["leaf_hash"]), [B(p) for p in v["proof"]], B(v["root"])), "pos " + v["id"])
        # recompute leaf hash and root independently from entry template
        check(v["leaf_hash"] == X(H(b"\x00" + f"entry-{v['leaf_index']}".encode())), "leafhash " + v["id"])
    for v in d["inclusion_negative"]:
        check(not verify_inclusion(v["leaf_index"], v["tree_size"], B(v["leaf_hash"]), [B(p) for p in v["proof"]], B(v["root"])), "neg " + v["id"])
        check(v["expect"] == "reject", "expect " + v["id"])
    for v in d["consistency"]:
        check(verify_consistency(v["first_size"], v["second_size"], B(v["first_root"]), B(v["second_root"]), [B(p) for p in v["proof"]]), "pos " + v["id"])
    for v in d["consistency_negative"]:
        check(not verify_consistency(v["first_size"], v["second_size"], B(v["first_root"]), B(v["second_root"]), [B(p) for p in v["proof"]]), "neg " + v["id"])
        check(v["expect"] == "reject", "expect " + v["id"])
    for r in d["roots"]:
        check(r["root"] == X(iterative_root([H(b"\x00" + f"entry-{i}".encode()) for i in range(r["size"])])), f"root {r['size']}")

    cats = {}
    for v in d["inclusion_negative"] + d["consistency_negative"]: cats[v["category"]] = cats.get(v["category"], 0) + 1
    print(f"merkle_v1.json: roots={len(d['roots'])} inclusion={len(d['inclusion'])} inclusion_neg={len(d['inclusion_negative'])} "
          f"consistency={len(d['consistency'])} consistency_neg={len(d['consistency_negative'])}")
    print("negative categories:", cats)
    print(f"ct_reference_v1.json: roots={len(ct_roots)} matches_recalled={[r['matches_recalled'] for r in ct_roots]}")
    if FAILS:
        print("SELF-CHECK FAILURES:", len(FAILS), FAILS[:10]); sys.exit(1)
    print("self-checks passed")


if __name__ == "__main__":
    main()
