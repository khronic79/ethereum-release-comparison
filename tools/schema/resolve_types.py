#!/usr/bin/env python3
"""Resolve Reth RPC method signatures to concrete type identifiers per version."""
import os, re, json, sys

# associated-type substitutions for the Ethereum network
ASSOC = {
    "B": "alloy_rpc_types_eth::Block",
    "T": "alloy_rpc_types_eth::Transaction",
    "R": "alloy_rpc_types_eth::TransactionReceipt",
    "H": "alloy_rpc_types_eth::Header",
    "L": "alloy_rpc_types_eth::Log",
    "TxReq": "alloy_rpc_types_eth::TransactionRequest",
}
VERSION_SUFFIX = {}
IDENT = re.compile(r'\b([A-Z][A-Za-z0-9_]*)\b')

# Types handled specially by the schema generator (no definition needed).
SPECIAL = {
    # alloy-primitives (defined via macros; serialize as hex/quantity)
    "Address": "hex:20", "B256": "hex:32", "H256": "hex:32", "B512": "hex",
    "B64": "hex", "Bloom": "hex", "Bytes": "hex", "Bytecode": "hex",
    "FixedBytes": "hex", "LogData": "logdata",
    "U256": "quantity", "U128": "quantity", "U64": "quantity", "U32": "quantity",
    "U16": "quantity", "U8": "quantity", "U": "quantity", "I256": "quantity",
    # JSON / serde
    "Value": "any", "String": "string", "bool": "boolean",
    # reth / generic params resolved elsewhere
    "RawTx": "todo:generic", "TypedData": "any",
    "AccessListResult": "object",  # alloy-eip2930: {accessList,gasUsed,error?}
}
CONTAINERS = {"Option", "Vec", "Box", "HashMap", "BTreeMap", "Result", "RpcResult", "String",
              "Arc", "Cow", "IndexMap"}


def strip_result(t):
    m = re.search(r'(?:RpcResult|Result)\s*<(.+)>\s*$', t, re.S)
    return m.group(1).strip() if m else t


def tokens(t):
    return set(IDENT.findall(t))


def main():
    ver = sys.argv[1]
    sig_path = sys.argv[2]
    reg_path = sys.argv[3]
    out_path = sys.argv[4]

    sigs = json.load(open(sig_path))
    reg = json.load(open(reg_path))

    def resolve(name):
        entries = reg.get(name, [])
        # prefer the standard Ethereum RPC type crates over duplicates
        for pref in ("rpc-types-eth", "eips", "consensus", "rpc-types-txpool", "rpc-types-mev"):
            for e in entries:
                if e["crate"] == pref:
                    return e
        return entries[0] if entries else None

    inventory = {}
    unresolved = {}
    for method, d in sorted(sigs.items()):
        params = []
        for p in d.get("params", []):
            t = p["type"]
            for k, v in ASSOC.items():
                t = re.sub(rf'\b{k}\b', v, t)
            params.append({"name": p["name"], "type": t})
        res = d.get("result")
        res_c = strip_result(res) if res else None
        if res_c:
            for k, v in ASSOC.items():
                res_c = re.sub(rf'\b{k}\b', v, res_c)
        names = set()
        for p in params:
            names |= tokens(p["type"])
        if res_c:
            names |= tokens(res_c)
        resolved = {}
        for n in sorted(names):
            if n in CONTAINERS:
                continue
            e = resolve(n)
            if e:
                resolved[n] = f"{e['crate']}::{e['kind']}"
            elif n in SPECIAL:
                resolved[n] = f"#special:{SPECIAL[n]}"
            else:
                unresolved.setdefault(n, []).append(method)
        inventory[method] = {"params": params, "result": res_c, "types": resolved}

    json.dump({"methods": inventory, "unresolved": unresolved},
              open(out_path, "w"), indent=2, ensure_ascii=False)
    print(f"{ver}: {len(inventory)} methods")
    print(f"unresolved type identifiers: {len(unresolved)}")
    for n in sorted(unresolved):
        print(f"  ? {n}  (used by {len(unresolved[n])} methods)")


if __name__ == "__main__":
    main()
