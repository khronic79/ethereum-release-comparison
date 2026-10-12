#!/usr/bin/env python3
"""Compute the transitive closure of types reachable from RPC methods.

Usage: closure.py <work-root> <registry.json> <inventory.json> <out.json>
"""
import os, re, json, sys

IDENT = re.compile(r'\b([A-Z][A-Za-z0-9_]*)\b')
DEF_HEAD = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+(struct|enum|type)\s+([A-Za-z_]\w*)', re.M)


def balanced_body(text, brace_pos):
    depth = 0
    i = brace_pos
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[brace_pos + 1:i]
        i += 1
    return text[brace_pos:]


def definition_body(text, kind, name):
    for m in DEF_HEAD.finditer(text):
        if m.group(2) == name:
            rest = text[m.end():]
            if kind == "type" or ";" in rest.split("{")[0]:
                return rest.split(";")[0]
            bp = text.find("{", m.end())
            if bp == -1:
                return ""
            return balanced_body(text, bp)
    return None


def main():
    work, reg_path, inv_path, out_path = sys.argv[1:5]
    reg = json.load(open(reg_path))
    inv = json.load(open(inv_path))

    roots = set()
    for m in inv["methods"].values():
        roots |= {t for t, v in m["types"].items() if not v.startswith("#special:")}

    seen, stack = set(), sorted(roots)
    closure = {}
    while stack:
        name = stack.pop()
        if name in seen:
            continue
        seen.add(name)
        entries = reg.get(name, [])
        if not entries:
            continue
        e = entries[0]
        closure[name] = {"crate": e["crate"], "kind": e["kind"], "file": e["file"]}
        path = os.path.join(work, e["file"])
        try:
            text = open(path, encoding="utf-8", errors="replace").read()
        except OSError:
            continue
        body = definition_body(text, e["kind"], name)
        if not body:
            continue
        for ident in IDENT.findall(body):
            if ident not in seen and ident in reg and ident not in roots:
                stack.append(ident)

    json.dump(closure, open(out_path, "w"), indent=2, ensure_ascii=False)
    from collections import Counter
    print(f"closure: {len(closure)} types -> {out_path}")
    for k, v in Counter(e["crate"] for e in closure.values()).most_common():
        print(f"  {v:4d}  {k}")


if __name__ == "__main__":
    main()
