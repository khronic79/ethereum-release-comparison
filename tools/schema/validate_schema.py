import os, glob, yaml, json, sys
from jsonschema import Draft202012Validator

SPEC = "/home/khron/projects/ethereum-release-comparison/schemas/execution-apis/src"
DOC = sys.argv[1] if len(sys.argv) > 1 else "schemas/reth-v2.7.0.schema.json"
INV = sys.argv[2] if len(sys.argv) > 2 else "tools/schema/out/inventory-v2.7.0.json"

doc = json.load(open(DOC))
inv = json.load(open(INV))["methods"]

# schema validity
try:
    Draft202012Validator.check_schema(doc)
    print(f"{DOC}: VALID draft 2020-12 | methods={len(doc['methods'])} $defs={len(doc.get('$defs',{}))}")
except Exception as e:
    print(f"{DOC}: INVALID -> {str(e)[:200]}")

# examples from execution-apis
ex = {}
for p in glob.glob(os.path.join(SPEC, "**", "*.yaml"), recursive=True):
    try:
        d = yaml.safe_load(open(p, encoding="utf-8", errors="replace"))
    except Exception:
        continue
    if not isinstance(d, list):
        continue
    for m in d:
        if isinstance(m, dict) and isinstance(m.get("name"), str) and m["name"] in doc["methods"]:
            ex.setdefault(m["name"], []).append(m.get("examples") or [])

v = Draft202012Validator(doc)
ok = fail = 0
fails = []
for name, examples in ex.items():
    order = [p["name"] for p in inv.get(name, {}).get("params", [])]
    ol = [o.lower() for o in order]
    for group in examples:
        for e in group:
            eparams = e.get("params") or []
            vals = [None] * len(order)
            name_hit = any((pp.get("name", "").lower() in ol) for pp in eparams)
            if name_hit:
                for pp in eparams:
                    k = pp.get("name", "").lower()
                    if k in ol:
                        vals[ol.index(k)] = pp.get("value")
            else:
                # fall back to positional mapping (spec examples often use descriptive names)
                for i, pp in enumerate(eparams):
                    if i < len(vals):
                        vals[i] = pp.get("value")
            while vals and vals[-1] is None:
                vals.pop()
            perrs = list(v.descend(vals, doc["methods"][name]["params"], schema_path="params"))
            rerrs = []
            if "value" in e.get("result", {}) and "result" in doc["methods"][name]:
                rerrs = list(v.descend(e["result"]["value"], doc["methods"][name]["result"], schema_path="result"))
            for er in perrs + rerrs:
                fail += 1
                if len(fails) < 12:
                    side = "params" if er.schema_path and "params" in list(er.schema_path)[:1] else "result"
                    fails.append((name, side, er.validator, "/".join(map(str, er.absolute_path))[:50], er.message[:90].replace("\n"," ")))
            if not perrs and not rerrs:
                ok += 1
print(f"examples: ok={ok} fail={fail}")
for f in fails:
    print(f"  {f[0]:34} [{f[1]}/{f[2]}] {f[3]}: {f[4]}")
