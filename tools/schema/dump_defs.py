#!/usr/bin/env python3
"""Dump definitions of closure types for inspection / pattern cataloguing."""
import os, re, json, sys

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


def def_text(text, kind, name):
    for m in DEF_HEAD.finditer(text):
        if m.group(2) == name:
            start = text.rfind("\n", 0, m.start()) + 1
            # include preceding attributes/derive
            pre = start
            lines = text[:start].rstrip("\n").split("\n")
            while lines and (lines[-1].lstrip().startswith("#[") or lines[-1].lstrip().startswith("///")
                             or lines[-1].lstrip().startswith("//") or lines[-1].strip() == ""):
                pre = start - len(lines[-1]) - 1
                lines.pop()
            if kind == "type" or ";" in text[m.end():].split("{")[0]:
                return text[pre:].split(";")[0] + ";"
            bp = text.find("{", m.end())
            return text[pre: bp] + "{" + balanced_body(text, bp) + "}"
    return None


def main():
    work, reg_path, closure_path, out_path = sys.argv[1:5]
    reg = json.load(open(reg_path))
    closure = json.load(open(closure_path))
    with open(out_path, "w") as out:
        for name in sorted(closure):
            e = closure[name]
            path = os.path.join(work, e["file"])
            try:
                text = open(path, encoding="utf-8", errors="replace").read()
            except OSError:
                continue
            d = def_text(text, e["kind"], name)
            if d:
                out.write(f"\n// ===== {name}  ({e['crate']}) =====\n{d}\n")
    print(f"dumped -> {out_path}")


if __name__ == "__main__":
    main()
