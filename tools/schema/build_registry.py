#!/usr/bin/env python3
"""Build a registry of type definitions from a source tree.

Usage: build_registry.py <root> <out.json>

Scans `*.rs` under any `.../<crate>/src/` directory and records
`pub struct|enum|type <Name>` definitions with the owning crate name
(the path component immediately before `src`).
"""
import os, re, json, sys
from collections import Counter

DEF_RE = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+(struct|enum|type)\s+([A-Za-z_]\w*)', re.M)


def crate_of(path):
    parts = path.split(os.sep)
    if "src" in parts:
        i = parts.index("src")
        if i > 0:
            return parts[i - 1]
    return "?"


def main():
    root, out = sys.argv[1], sys.argv[2]
    reg = {}
    for dirpath, _, files in os.walk(root):
        if f"{os.sep}src" not in dirpath + os.sep and not dirpath.endswith(f"{os.sep}src"):
            if f"{os.sep}src{os.sep}" not in dirpath + os.sep:
                continue
        for f in files:
            if not f.endswith(".rs"):
                continue
            p = os.path.join(dirpath, f)
            crate = crate_of(p)
            try:
                text = open(p, encoding="utf-8", errors="replace").read()
            except OSError:
                continue
            for m in DEF_RE.finditer(text):
                reg.setdefault(m.group(2), []).append({
                    "crate": crate, "kind": m.group(1),
                    "file": os.path.relpath(p, root)})
    json.dump(reg, open(out, "w"), indent=2, ensure_ascii=False)
    print(f"{root}: {len(reg)} types -> {out}")
    for k, v in Counter(e[0]["crate"] for e in reg.values()).most_common(12):
        print(f"  {v:4d}  {k}")


if __name__ == "__main__":
    main()
