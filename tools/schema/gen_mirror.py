#!/usr/bin/env python3
"""Generate a Rust crate of `schemars` mirror types from vendored alloy/reth sources.

Usage: gen_mirror.py <work-root> <registry.json> <closure.json> <out-dir>
"""
import os, re, json, sys

PRIMITIVES = {
    "Address": "crate::prims::Hex20",
    "B256": "crate::prims::Hex32", "H256": "crate::prims::Hex32", "BlockHash": "crate::prims::Hex32",
    "TxHash": "crate::prims::Hex32", "B512": "crate::prims::Hex", "B64": "crate::prims::Hex",
    "Bloom": "crate::prims::Hex", "Bytes": "crate::prims::Hex", "Bytecode": "crate::prims::Hex",
    "Bytes48": "crate::prims::Hex", "Blob": "crate::prims::Hex", "FixedBytes": "crate::prims::Hex",
    "LogData": "crate::prims::LogData",
}
QUANTITY = {
    "U256", "U128", "U64", "U32", "U16", "U8", "U", "I256", "BlockNumber", "u64", "u128", "u32", "u16", "u8",
}
CONTAINERS = {"Option", "Vec", "Box", "HashMap", "BTreeMap", "HashSet", "BTreeSet", "Arc", "Cow"}
BUILTIN = {"u8","u16","u32","u64","u128","usize","i8","i16","i32","i64","i128","isize","f32","f64","bool","String","char"}
WITH_MAP = {
    "alloy_serde::quantity": "crate::prims::Quantity",
    "alloy_serde::quantity::opt": "Option<crate::prims::Quantity>",
    "alloy_serde::quantity::vec": "Vec<crate::prims::Quantity>",
    "alloy_serde::quantity::u128_vec": "Vec<crate::prims::Quantity>",
    "alloy_serde::quantity::u128_vec_vec_opt": "Option<Vec<Vec<crate::prims::Quantity>>>",
}
KEEP_CONTAINER_ATTRS = ("rename_all", "tag", "content", "untagged", "transparent")
KEEP_FIELD_ATTRS = ("rename", "skip", "default", "flatten")

DEF_HEAD = re.compile(r'^\s*pub(?:\s*\([^)]*\))?\s+(struct|enum|type)\s+([A-Za-z_]\w*)', re.M)


def balanced(text, i, o, c):
    d = 0
    while i < len(text):
        ch = text[i]
        if ch == '"':
            j = i + 1
            while j < len(text) and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            i = j + 1
            continue
        if ch == o: d += 1
        elif ch == c:
            d -= 1
            if d == 0: return i + 1
        i += 1
    return i


def split_leading_attrs(s):
    """Return (attrs, decl) stripping leading #[..] and comments."""
    attrs, i, n = [], 0, len(s)
    while i < n:
        while i < n and s[i] in " \t\r\n":
            i += 1
        if s.startswith("///", i) or s.startswith("//!", i):
            j = s.find("\n", i); i = j + 1 if j != -1 else n; continue
        if s.startswith("//", i):
            j = s.find("\n", i); i = j + 1 if j != -1 else n; continue
        if s.startswith("#[", i):
            j = balanced(s, i + 1, "[", "]")
            attrs.append(s[i + 2:j - 1]); i = j; continue
        break
    return attrs, s[i:].strip()


def split_top(s):
    out, d, cur, i = [], 0, "", 0
    while i < len(s):
        ch = s[i]
        if ch == '"':
            j = i + 1
            while j < len(s) and s[j] != '"':
                j += 2 if s[j] == "\\" else 1
            cur += s[i:j + 1]; i = j + 1; continue
        if s.startswith("//", i):
            j = s.find("\n", i); i = j + 1 if j != -1 else len(s); continue
        if ch in "(<[{": d += 1
        elif ch in ")>]}": d -= 1
        if ch == "," and d == 0:
            out.append(cur); cur = ""
        else:
            cur += ch
        i += 1
    if cur.strip(): out.append(cur)
    return out


def unwrap_serde(attr):
    a = attr.strip()
    if a.startswith("cfg_attr("):
        inner = a[len("cfg_attr("):]
        m = re.search(r'serde\s*\(', inner)
        if m:
            j = balanced(inner, m.end() - 1, "(", ")")
            return inner[m.start():j]  # includes 'serde'
        return None
    if a.startswith("serde("):
        return a
    return None


def parse_generics(rest):
    rest = rest.lstrip()
    names, defaults = [], {}
    if rest.startswith("<"):
        end = balanced(rest, 0, "<", ">")
        for p in split_top(rest[1:end-1]):
            p = p.strip()
            if not p: continue
            if "=" in p:
                nm, dv = p.split("=", 1); nm = nm.strip(); names.append(nm); defaults[nm] = dv.strip()
            else:
                names.append(p.split(":")[0].strip())
        rest = rest[end:]
    # skip where clause up to '{'
    return names, defaults, rest


def rewrite_type(t, closure, defaults, generic_names):
    t = t.strip()
    t = re.sub(r'^&\s*(\'[a-zA-Z_]\w*\s*)?(mut\s+)?', '', t)
    t = re.sub(r"'[a-zA-Z_]\w*", "", t)
    t = t.strip()
    if t in PRIMITIVES: return PRIMITIVES[t]
    if t in QUANTITY: return "crate::prims::Quantity"
    if t in BUILTIN: return t
    if t in generic_names: return defaults.get(t, "crate::prims::Any")
    m = re.match(r'^([A-Za-z_][\w:]*)\s*(<.*>)?$', t, re.S)
    if not m:
        if t.startswith("(") and t.endswith(")"):
            inner = split_top(t[1:-1])
            return "(" + ", ".join(rewrite_type(x, closure, defaults, generic_names) for x in inner) + ")"
        if t.startswith("[") and t.endswith("]"):
            inner = t[1:-1].split(";")[0]
            return f"Vec<{rewrite_type(inner, closure, defaults, generic_names)}>"
        return "crate::prims::Any"
    base, args = m.group(1), m.group(2)
    base_name = base.split("::")[-1]
    if args:
        argv = split_top(args[1:-1])
        argv_rw = ", ".join(rewrite_type(a, closure, defaults, generic_names) for a in argv)
        if base_name in CONTAINERS:
            return f"{base_name}<{argv_rw}>"
        if base_name in PRIMITIVES: return PRIMITIVES[base_name]
        if base_name in closure: return f"{base_name}<{argv_rw}>"
        return "crate::prims::Any"
    if base_name in PRIMITIVES: return PRIMITIVES[base_name]
    if base_name in closure: return base_name
    return "crate::prims::Any"


def clean_serde(s):
    s = re.sub(r'(serialize_with|deserialize_with|with)\s*=\s*"[^"]*"', '', s)
    s = re.sub(r'\(\s*,', '(', s)
    s = re.sub(r',\s*,', ',', s)
    s = re.sub(r',\s*\)', ')', s)
    return s


def gen_params(names_g, defaults, closure):
    parts = []
    for p in names_g:
        if p in defaults:
            parts.append(f"{p} = {rewrite_type(defaults[p], closure, {}, names_g)}")
        else:
            parts.append(p)
    return f"<{', '.join(parts)}>"


def emit_struct(name, names_g, defaults, body, closure):
    fields = []
    for item in split_top(body):
        if not item.strip(): continue
        attrs, decl = split_leading_attrs(item)
        if not decl or ":" not in decl: continue
        fname, ftype = decl.split(":", 1)
        fname = fname.replace("pub", "").strip()
        ftype = ftype.strip()
        s = [clean_serde(x) for a in attrs if (x := unwrap_serde(a))]
        s = [x for x in s if x != "serde()"]
        out = "".join(f"#[{x}]" for x in s if any(k in x for k in KEEP_FIELD_ATTRS))
        wv = None
        for x0 in [a for a in attrs if (a := unwrap_serde(a))]:
            mm = re.search(r'with\s*=\s*"([^"]+)"', x0)
            if mm and mm.group(1) in WITH_MAP:
                wv = WITH_MAP[mm.group(1)]
        if wv:
            out += f'#[schemars(with = "{wv}")]'
        ty = wv or rewrite_type(ftype, closure, defaults, names_g)
        fields.append(f"{out}\n    pub {fname}: {ty}")
    gen = gen_params(names_g, defaults, closure) if names_g else ""
    impl = f"\nwhere {', '.join(p + ': JsonSchema' for p in names_g)}" if names_g else ""
    return f"#[derive(JsonSchema)]\npub struct {name}{gen}{impl} {{\n    " + ",\n    ".join(fields) + ",\n}"


def emit_enum(name, names_g, defaults, body, closure):
    variants = []
    for item in split_top(body):
        if not item.strip(): continue
        attrs, decl = split_leading_attrs(item)
        if not decl: continue
        s = [x for a in attrs if (x := unwrap_serde(a))]
        vout = "".join(f"#[{x}]" for x in s if "rename" in x)
        m = re.match(r'^([A-Za-z_]\w*)\s*(.*)$', decl, re.S)
        if not m: continue
        vname, arg = m.group(1), m.group(2).strip()
        if not arg:
            variants.append(f"{vout}\n    {vname}")
        elif arg.startswith("(") and arg.endswith(")"):
            inner = [x for x in split_top(arg[1:-1]) if x.strip()]
            rw = ", ".join(rewrite_type(x, closure, defaults, names_g) for x in inner)
            variants.append(f"{vout}\n    {vname}({rw})")
        elif arg.startswith("{") and arg.endswith("}"):
            flds = []
            for f in split_top(arg[1:-1]):
                a2, d2 = split_leading_attrs(f)
                if ":" in d2:
                    fn, ft = d2.split(":", 1)
                    flds.append(f"pub {fn.replace('pub','').strip()}: {rewrite_type(ft, closure, defaults, names_g)}")
            variants.append(f"{vout}\n    {vname} {{ " + ", ".join(flds) + " }")
        else:
            variants.append(f"\n    {vname}")
    gen = f"<{', '.join(names_g)}>" if names_g else ""
    impl = f"\nwhere {', '.join(p + ': JsonSchema' for p in names_g)}" if names_g else ""
    return f"#[derive(JsonSchema)]\npub enum {name}{gen}{impl} {{\n    " + ",\n    ".join(variants) + ",\n}"


def main():
    work, reg_p, clo_p, outdir = sys.argv[1:5]
    reg = json.load(open(reg_p))
    closure = json.load(open(clo_p))
    names = set(closure)
    types_rs = []
    for name in sorted(closure):
        e = closure[name]
        path = os.path.join(work, e["file"])
        try:
            text = open(path, encoding="utf-8", errors="replace").read()
        except OSError:
            types_rs.append(f"pub type {name} = crate::prims::Any;"); continue
        hit = False
        for m in DEF_HEAD.finditer(text):
            if m.group(2) != name: continue
            pre = text[:m.start()]
            prev = pre.rfind("}")
            attrs, _ = split_leading_attrs(pre[prev + 1:])
            names_g, defaults, rest = parse_generics(text[m.end():])
            if e["kind"] == "type" or ";" in rest.split("{")[0]:
                alias = rest.split(";")[0].strip()
                types_rs.append(f"pub type {name} = {rewrite_type(alias, names, defaults, names_g)};")
                hit = True; break
            bp = text.find("{", m.end())
            body = text[bp + 1:balanced(text, bp, "{", "}") - 1]
            cserde = [x for a in attrs if (x := unwrap_serde(a))]
            cont = "".join(f"#[{x}]" for x in cserde if any(k in x for k in KEEP_CONTAINER_ATTRS))
            decl = emit_struct(name, names_g, defaults, body, names) if e["kind"] == "struct" \
                else emit_enum(name, names_g, defaults, body, names)
            types_rs.append(cont + "\n" + decl)
            hit = True
            break
        if not hit:
            types_rs.append(f"pub type {name} = crate::prims::Any;")

    os.makedirs(outdir + "/src", exist_ok=True)
    with open(outdir + "/src/model.rs", "w") as f:
        f.write("use schemars::JsonSchema;\n#[allow(unused_imports)]\nuse crate::prims::*;\n\n")
        f.write("\n\n".join(types_rs))
    print(f"generated {outdir}/src/model.rs with {len(types_rs)} items")


if __name__ == "__main__":
    main()
