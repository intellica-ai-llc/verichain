#!/usr/bin/env python3
"""Generate RFC 8785 (JCS) vectors -> spec/vectors/jcs_v1.json.  Stdlib only.

Expected values are written by hand from RFC 8785 (not produced by json.dumps).
Self-check: an independent mini-canonicalizer (own string escaper, own UTF-16 key sort,
own ECMAScript Number::toString built from repr() shortest digits) must reproduce every
expected value.  json.loads is used ONLY to parse the input text (parsing is not in doubt).
"""
import json, os, sys, decimal
OUT = os.path.normpath(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "spec", "vectors"))

# (name, input_json_text, expected_canonical_text, confidence)  -- python escapes used for non-ASCII
V = []
def v(name, inp, exp, conf="high"): V.append((name, inp, exp, conf))

# --- structure
v("empty_object", "{}", "{}")
v("empty_array", "[]", "[]")
v("nested_empty_containers", '{"b":[],"a":{},"c":[{},[]]}', '{"a":{},"b":[],"c":[{},[]]}')
v("whitespace_stripped", ' \n{ "b" :\t1 , "a" : [ 1 , 2 , { } ] }\r\n', '{"a":[1,2,{}],"b":1}')
v("top_level_scalar_number", "  42 ", "42")
v("top_level_string", ' "x" ', '"x"')
v("literals", "[ null , true , false ]", "[null,true,false]")
v("array_order_preserved_objects_sorted", '[{"z":1,"y":2},{"b":[3,2,1],"a":null}]', '[{"y":2,"z":1},{"a":null,"b":[3,2,1]}]')
v("nested_object_sorting_recursive", '{"o":{"b":{"d":1,"c":2},"a":3},"n":0}', '{"n":0,"o":{"a":3,"b":{"c":2,"d":1}}}')

# --- key sorting
v("sort_ascii_case_and_prefix", '{"b":1,"a":2,"B":3,"aa":4,"A":5}', '{"A":5,"B":3,"a":2,"aa":4,"b":1}')
v("sort_digits_before_letters_lexicographic", '{"10":1,"9":2,"1":3,"a":4}', '{"1":3,"10":1,"9":2,"a":4}')
v("sort_astral_vs_fullwidth_tilde",  # U+1F600 = D83D DE00 < U+FF5E in UTF-16 order (code-point order would differ)
  '{"\\uff5e":2,"\\ud83d\\ude00":1}', '{"\U0001F600":1,"～":2}')
v("sort_astral_vs_private_use_E000",  # E000 > D83D
  '{"\\ue000":2,"\\ud83d\\ude00":1}', '{"\U0001F600":1,"":2}')
v("sort_astral_vs_ffff", '{"\\uffff":2,"\\ud83d\\ude00":1}', '{"\U0001F600":1,"￿":2}')
v("sort_mixed_bmp_and_astral",  # a(0061) < U+D7FF < U+10000(D800 DC00) < U+1F600(D83D) < U+E000 < U+FF5E < U+FFFF
  '{"\\uffff":7,"\\uff5e":6,"\\ue000":5,"\\ud83d\\ude00":4,"\\ud800\\udc00":3,"\\ud7ff":2,"a":1}',
  '{"a":1,"퟿":2,"\U00010000":3,"\U0001F600":4,"":5,"～":6,"￿":7}')
v("sort_rfc8785_section_3_2_3_example",
  '{"\\u20ac":"Euro Sign","\\r":"Carriage Return","\\ufb33":"Hebrew Letter Dalet With Dagesh","1":"One","\\ud83d\\ude00":"Emoji: Grinning Face","\\u0080":"Control","\\u00f6":"Latin Small Letter O With Diaeresis"}',
  '{"\\r":"Carriage Return","1":"One","\u0080":"Control","ö":"Latin Small Letter O With Diaeresis","€":"Euro Sign","\U0001F600":"Emoji: Grinning Face","דּ":"Hebrew Letter Dalet With Dagesh"}')
v("sort_keys_needing_escapes", '{"a":1,"\\n":2,"\\"":3,"\\u0000":4}', '{"\\u0000":4,"\\n":2,"\\"":3,"a":1}')
v("sort_nonbmp_pair_ordering", '{"\\ud83d\\ude01":2,"\\ud83d\\ude00":1,"\\ud83c\\udf55":0}', '{"\U0001F355":0,"\U0001F600":1,"\U0001F601":2}')

# --- numbers
v("num_1e30", "1e30", "1e+30")
v("num_4_50", "4.50", "4.5")
v("num_1e_minus_7", "1e-7", "1e-7")
v("num_1e21", "1e21", "1e+21")
v("num_1e20_plain_digits", "1e20", "100000000000000000000")
v("num_100000000000000000000_literal", "100000000000000000000", "100000000000000000000")
v("num_1e22", "1E+22", "1e+22")
v("num_negative_zero", "-0", "0")
v("num_negative_zero_float", "-0.0", "0")
v("num_zero_variants", "[0,0.0,0e0,-0e5]", "[0,0,0,0]")
v("num_integral_floats", "[1.0,10e-1,100E-2,1e0,2.50e1]", "[1,1,1,1,25]")
v("num_2pow53", "9007199254740992", "9007199254740992")
v("num_2pow53_minus1_neg", "-9007199254740991", "-9007199254740991")
v("num_2pow53_plus1_rounds", "9007199254740993", "9007199254740992", "medium")  # IEEE double parse; implementations with exact bigints may differ
v("num_1e_minus_6_boundary", "[0.000001,0.0000001,0.00000123]", "[0.000001,1e-7,0.00000123]")
v("num_fraction_exponent", "[1.5e-7,123e-2,-1.50]", "[1.5e-7,1.23,-1.5]")
v("num_big_20_21_digits", "[123456789012345680000,999999999999999900000,1.2345678901234568e20]",
  "[123456789012345680000,999999999999999900000,123456789012345680000]")
v("num_min_subnormal", "5e-324", "5e-324", "medium")
v("num_max_double", "1.7976931348623157e308", "1.7976931348623157e+308", "medium")
v("num_rfc_example_array", "[333333333.33333329,1E30,4.50,2e-3,0.000000000000000000000000001]", "[333333333.3333333,1e+30,4.5,0.002,1e-27]")

# --- strings
v("esc_u0007", '"\\u0007"', '"\\u0007"')
v("esc_u000b_lowercase_hex", '"\\u000B\\u001F"', '"\\u000b\\u001f"')
v("esc_nul", '"\\u0000"', '"\\u0000"')
v("esc_short_forms", '"\\b\\f\\n\\r\\t"', '"\\b\\f\\n\\r\\t"')
v("esc_short_forms_via_u", '"\\u0008\\u000c\\u000a\\u000d\\u0009"', '"\\b\\f\\n\\r\\t"')
v("esc_del_unescaped", '"\\u007f"', '"\x7f"')
v("esc_solidus_unescaped", '"\\/a/"', '"/a/"')
v("esc_quote_backslash", '"\\"\\\\"', '"\\"\\\\"')
v("esc_unicode_escapes_become_literal", '"\\u00e9\\u20ac\\ud83d\\ude00"', '"é€\U0001F600"')
v("esc_line_separators_literal", '"\\u2028\\u2029"', '"  "')
v("esc_uppercase_hex_input_u00A9", '"\\u00A9"', '"©"')
v("rfc8785_section_3_2_2_example",
  '{"numbers":[333333333.33333329,1E30,4.50,2e-3,0.000000000000000000000000001],"string":"\\u20ac$\\u000F\\u000aA\'\\u0042\\u0022\\u005c\\\\\\"\\/","literals":[null,true,false]}',
  '{"literals":[null,true,false],"numbers":[333333333.3333333,1e+30,4.5,0.002,1e-27],"string":"€$\\u000f\\nA\'B\\"\\\\\\\\\\"/"}')

# ----------------------------------------------------------------- independent mini-JCS
def es_number(d):
    if d == 0: return "0"
    s = "-" if d < 0 else ""; d = abs(d)
    t = decimal.Decimal(repr(d)).as_tuple()          # shortest round-trip digits
    digits = "".join(map(str, t.digits)).rstrip("0") or "0"
    n = len(t.digits) + t.exponent                    # decimal point position
    # recompute n against stripped digits: value = 0.digits * 10^n
    k = len(digits)
    if k <= n <= 21: return s + digits + "0" * (n - k)
    if 0 < n <= 21: return s + digits[:n] + "." + digits[n:]
    if -6 < n <= 0: return s + "0." + "0" * (-n) + digits
    e = n - 1; es = ("+" if e >= 0 else "-") + str(abs(e))
    return s + (digits if k == 1 else digits[0] + "." + digits[1:]) + "e" + es

def es_string(x):
    o = ['"']
    for ch in x:
        c = ord(ch)
        if ch == '"': o.append('\\"')
        elif ch == "\\": o.append("\\\\")
        elif ch == "\b": o.append("\\b")
        elif ch == "\f": o.append("\\f")
        elif ch == "\n": o.append("\\n")
        elif ch == "\r": o.append("\\r")
        elif ch == "\t": o.append("\\t")
        elif c < 0x20: o.append("\\u%04x" % c)
        else: o.append(ch)
    o.append('"'); return "".join(o)

def u16(s): return s.encode("utf-16-be")   # big-endian code units compare bytewise == unit-wise

def canon(x):
    if x is None: return "null"
    if x is True: return "true"
    if x is False: return "false"
    if isinstance(x, int): return es_number(float(x))
    if isinstance(x, float): return es_number(x)
    if isinstance(x, str): return es_string(x)
    if isinstance(x, list): return "[" + ",".join(canon(i) for i in x) + "]"
    return "{" + ",".join(es_string(k) + ":" + canon(x[k]) for k in sorted(x, key=u16)) + "}"

def main():
    names = set(); out = []; fails = []
    for name, inp, exp, conf in V:
        assert name not in names, name; names.add(name)
        got = canon(json.loads(inp))
        if got != exp: fails.append((name, got, exp))
        out.append(dict(name=name, input_json_text=inp, expected_canonical_utf8=exp,
                        expected_canonical_hex=exp.encode("utf-8").hex(), confidence=conf))
    doc = dict(version=1, spec="RFC 8785 JSON Canonicalization Scheme",
               notes=["input_json_text and expected_canonical_utf8 are JSON-decoded Python/JSON strings; expected_canonical_hex is the exact UTF-8 byte sequence (authoritative)",
                      "inputs contain only finite IEEE-754 double-representable numbers",
                      "lone surrogates / duplicate keys / NaN / Infinity are not covered (inputs must be rejected, behavior is I-JSON level and unspecified here)"],
               vectors=out)
    with open(os.path.join(OUT, "jcs_v1.json"), "w", encoding="utf-8") as f:
        f.write("{\n" + ",\n".join(json.dumps(k) + ": " + (json.dumps(vv, ensure_ascii=True) if k != "vectors" else
                "[\n" + ",\n".join("  " + json.dumps(x, ensure_ascii=True, separators=(",", ":")) for x in vv) + "\n]") for k, vv in doc.items()) + "\n}\n")
    # reload check: hex matches text
    d = json.load(open(os.path.join(OUT, "jcs_v1.json"), encoding="utf-8"))
    for x in d["vectors"]:
        if x["expected_canonical_utf8"].encode("utf-8").hex() != x["expected_canonical_hex"]: fails.append(("hex", x["name"]))
    print("jcs_v1.json vectors:", len(d["vectors"]))
    if fails:
        for f_ in fails: print("FAIL", f_)
        sys.exit(1)
    print("self-checks passed")

if __name__ == "__main__": main()
