#!/usr/bin/env python3
"""Extract JSON-RPC method signatures (namespace, params, result) from reth RPC traits.

Parses `#[method(name = "...")]` attributes and the following `fn` signature,
associating each method with the `namespace = "..."` of its trait.
"""
import os, re, json, sys

NAMESPACES = ("eth", "net", "web3", "txpool")
TARGET_DIRS = ("crates/rpc/rpc-api/src", "crates/rpc/rpc-eth-api/src")

NS_RE = re.compile(r'namespace\s*=\s*"([^"]+)"')
METHOD_RE = re.compile(r'#\[(?:method|subscription)\(\s*name\s*=\s*"([^"]+)"')
FN_RE = re.compile(r'\bfn\s+(\w+)')


def skip_balanced(text, i, open_c, close_c):
    """Return index just after the balanced open_c...close_c block starting at i."""
    assert text[i] == open_c
    depth = 0
    while i < len(text):
        if text[i] == open_c:
            depth += 1
        elif text[i] == close_c:
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return i


def scan_signature(text, start):
    """From index `start` (at 'fn'), return the substring up to the terminating ';'."""
    i = start
    depth = 0
    while i < len(text):
        c = text[i]
        if c in "([{<":
            depth += 1
        elif c in ")]}>":
            depth -= 1
        elif c == ";" and depth <= 0:
            return text[start:i].strip()
        elif c == "{" and depth <= 0:
            # trait default body; stop
            return text[start:i].strip()
        i += 1
    return text[start:].strip()


def split_params(param_str):
    """Split a top-level comma-separated parameter list, respecting nesting."""
    out, depth, cur = [], 0, ""
    for c in param_str:
        if c in "([{<":
            depth += 1
        elif c in ")]}>":
            depth -= 1
        if c == "," and depth == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += c
    if cur.strip():
        out.append(cur.strip())
    return out


def parse_signature(sig):
    # sig like: "async fn balance(&self, address: Address, block: Option<BlockId>) -> RpcResult<U256>"
    m = FN_RE.search(sig)
    if not m:
        return None
    i = m.end()
    while i < len(sig) and sig[i].isspace():
        i += 1
    if i < len(sig) and sig[i] == "<":
        i = skip_balanced(sig, i, "<", ">")
        while i < len(sig) and sig[i].isspace():
            i += 1
    if i >= len(sig) or sig[i] != "(":
        return None
    j = skip_balanced(sig, i, "(", ")")
    params_raw = sig[i + 1:j - 1]
    rest = sig[j:]
    # return type
    rm = re.search(r'->\s*(.+)$', rest, re.S)
    ret = rm.group(1).strip() if rm else "()"
    params = []
    for p in split_params(params_raw):
        p = re.sub(r'^mut\s+', '', p.strip())
        if p.startswith("&self") or p in ("self", "&mut self"):
            continue
        if ":" in p:
            name, typ = p.split(":", 1)
            params.append({"name": name.strip(), "type": typ.strip()})
        else:
            params.append({"name": None, "type": p.strip()})
    return {"params": params, "result": ret}


def collect(root):
    found = {}
    for d in TARGET_DIRS:
        base = os.path.join(root, d)
        for dirpath, _, files in os.walk(base):
            for f in sorted(files):
                if not f.endswith(".rs"):
                    continue
                p = os.path.join(dirpath, f)
                text = open(p, encoding="utf-8", errors="replace").read()
                ns = None
                for m in NS_RE.finditer(text):
                    # record positions of namespace decls
                    pass
                # iterate method attrs in order, track nearest preceding namespace
                events = []
                for m in NS_RE.finditer(text):
                    events.append((m.start(), "ns", m.group(1)))
                for m in METHOD_RE.finditer(text):
                    events.append((m.start(), "method", m.group(1)))
                events.sort()
                last_ns = None
                for pos, kind, val in events:
                    if kind == "ns":
                        last_ns = val
                    elif kind == "method" and last_ns in NAMESPACES:
                        fstart = text.find("fn ", pos)
                        if fstart == -1:
                            continue
                        sig = scan_signature(text, fstart)
                        parsed = parse_signature(sig)
                        full = f"{last_ns}_{val}"
                        found.setdefault(full, {"file": os.path.relpath(p, root),
                                                "trait_ns": last_ns,
                                                "raw": sig, **(parsed or {})})
    return found


def main():
    root = sys.argv[1]
    out = sys.argv[2]
    data = collect(root)
    json.dump(data, open(out, "w"), indent=2, ensure_ascii=False)
    print(f"{root}: {len(data)} methods -> {out}")
    for k in sorted(data):
        d = data[k]
        ps = ", ".join(f"{p['name']}: {p['type']}" for p in d.get("params", []))
        print(f"  {k}({ps}) -> {d.get('result')}")


if __name__ == "__main__":
    main()
