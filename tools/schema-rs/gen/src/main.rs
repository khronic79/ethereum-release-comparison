use anyhow::Result;
use std::collections::{BTreeSet, HashMap, HashSet};
use syn::{
    Attribute, Fields, GenericArgument, Item, ItemEnum, ItemStruct, ItemType, Meta, PathArguments,
    Type, Visibility,
};
use walkdir::WalkDir;

// ------------------------------------------------------------------ serde model
#[derive(Default, Clone)]
struct ContainerSerde {
    rename_all: Option<String>,
    tag: Option<String>,
    content: Option<String>,
    untagged: bool,
    transparent: bool,
}

#[derive(Default, Clone)]
struct FieldSerde {
    rename: Option<String>,
    skip: bool,
    default: bool,
    flatten: bool,
    with: Option<String>,
}

fn nested_metas(a: &Attribute) -> Vec<Meta> {
    let mut out = Vec::new();
    let path = a.path();
    if path.is_ident("serde") {
        if let Ok(ms) = a.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        ) {
            out.extend(ms);
        }
    } else if path.is_ident("cfg_attr") {
        if let Ok(ms) = a.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
        ) {
            for m in ms {
                if let Meta::List(l) = &m {
                    if l.path.is_ident("serde") {
                        use syn::parse::Parser as _;
                        if let Ok(inner) = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
                            .parse2(l.tokens.clone())
                        {
                            out.extend(inner);
                        }
                    }
                }
            }
        }
    }
    out
}

fn lit_str(m: &Meta) -> Option<String> {
    if let Meta::NameValue(nv) = m {
        if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value {
            return Some(s.value());
        }
    }
    None
}

fn parse_container(attrs: &[Attribute]) -> ContainerSerde {
    let mut c = ContainerSerde::default();
    for a in attrs {
        for m in nested_metas(a) {
            let path = m.path();
            if path.is_ident("rename_all") {
                c.rename_all = lit_str(&m);
            } else if path.is_ident("tag") {
                c.tag = lit_str(&m);
            } else if path.is_ident("content") {
                c.content = lit_str(&m);
            } else if path.is_ident("untagged") {
                c.untagged = true;
            } else if path.is_ident("transparent") {
                c.transparent = true;
            }
        }
    }
    c
}

fn parse_field(attrs: &[Attribute]) -> FieldSerde {
    let mut f = FieldSerde::default();
    for a in attrs {
        for m in nested_metas(a) {
            let path = m.path();
            if path.is_ident("rename") {
                f.rename = lit_str(&m);
            } else if path.is_ident("skip") {
                f.skip = true;
            } else if path.is_ident("skip_serializing") {
                f.skip = true;
            } else if path.is_ident("default") {
                f.default = true;
            } else if path.is_ident("flatten") {
                f.flatten = true;
            } else if path.is_ident("with") {
                f.with = lit_str(&m);
            }
        }
    }
    f
}

const WITH_MAP: &[(&str, &str)] = &[
    ("alloy_serde::quantity", "crate::prims::Quantity"),
    ("alloy_serde::quantity::opt", "Option<crate::prims::Quantity>"),
    ("alloy_serde::quantity::vec", "Vec<crate::prims::Quantity>"),
    ("alloy_serde::quantity::u128_vec", "Vec<crate::prims::Quantity>"),
    ("alloy_serde::quantity::u128_vec_vec_opt", "Option<Vec<Vec<crate::prims::Quantity>>>"),
];

// Types serialized through custom helpers (serde `into`/`try_from`) that cannot be
// derived from field attributes. Hand-written permissive JSON Schemas.
const OVERRIDES: &[(&str, &str)] = &[
    ("Recovered",
     r##"{"type":"object","additionalProperties":true}"##),
    ("Transaction",
     r##"{"type":"object","additionalProperties":true,"properties":{"hash":{"$ref":"#/definitions/Hex32"},"type":{"$ref":"#/definitions/Quantity"},"from":{"$ref":"#/definitions/Hex20"},"to":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"nonce":{"$ref":"#/definitions/Quantity"},"gas":{"$ref":"#/definitions/Quantity"},"gasPrice":{"$ref":"#/definitions/Quantity"},"maxFeePerGas":{"$ref":"#/definitions/Quantity"},"maxPriorityFeePerGas":{"$ref":"#/definitions/Quantity"},"maxFeePerBlobGas":{"$ref":"#/definitions/Quantity"},"value":{"$ref":"#/definitions/Quantity"},"input":{"$ref":"#/definitions/Hex"},"accessList":{"type":"array"},"blobVersionedHashes":{"type":"array"},"authorizationList":{"type":"array"},"v":{"$ref":"#/definitions/Quantity"},"r":{"$ref":"#/definitions/Quantity"},"s":{"$ref":"#/definitions/Quantity"},"yParity":{"$ref":"#/definitions/Quantity"},"chainId":{"$ref":"#/definitions/Quantity"},"blockHash":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"blockNumber":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"transactionIndex":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blockTimestamp":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]}}}"##),
    ("TransactionReceipt",
     r##"{"type":"object","additionalProperties":true,"properties":{"transactionHash":{"$ref":"#/definitions/Hex32"},"transactionIndex":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blockHash":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"blockNumber":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"from":{"$ref":"#/definitions/Hex20"},"to":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"cumulativeGasUsed":{"$ref":"#/definitions/Quantity"},"gasUsed":{"$ref":"#/definitions/Quantity"},"contractAddress":{"anyOf":[{"$ref":"#/definitions/Hex20"},{"type":"null"}]},"logs":{"type":"array","items":{"$ref":"#/definitions/Log"}},"logsBloom":{"$ref":"#/definitions/Hex"},"status":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"root":{"anyOf":[{"$ref":"#/definitions/Hex32"},{"type":"null"}]},"type":{"$ref":"#/definitions/Quantity"},"effectiveGasPrice":{"$ref":"#/definitions/Quantity"},"blobGasUsed":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]},"blobGasPrice":{"anyOf":[{"$ref":"#/definitions/Quantity"},{"type":"null"}]}}}"##),
];

fn override_json(name: &str) -> Option<&'static str> {
    OVERRIDES.iter().find(|(n, _)| *n == name).map(|(_, j)| *j)
}

fn override_impl(name: &str, json: &str) -> String {
    format!(
        "pub struct {name};\nimpl JsonSchema for {name} {{\n    fn schema_name() -> String {{ {name:?}.to_string() }}\n    fn json_schema(_: &mut SchemaGenerator) -> Schema {{\n        serde_json::from_str(r##\"{json}\"##).expect(\"override schema\")\n    }}\n}}"
    )
}

fn with_mapped(w: &Option<String>) -> Option<String> {
    w.as_ref().and_then(|s| WITH_MAP.iter().find(|(k, _)| *k == s).map(|(_, v)| v.to_string()))
}

// ------------------------------------------------------------------ registry
#[derive(Clone)]
enum Def {
    Struct(ItemStruct),
    Enum(ItemEnum),
    Alias(ItemType),
}

impl Def {
    fn kind(&self) -> &'static str {
        match self {
            Def::Struct(_) => "struct",
            Def::Enum(_) => "enum",
            Def::Alias(_) => "type",
        }
    }
}

// ------------------------------------------------------------------ type rewriting
fn ident_args(args: &PathArguments) -> Vec<GenericArgument> {
    match args {
        PathArguments::AngleBracketed(a) => a.args.iter().cloned().collect(),
        _ => Vec::new(),
    }
}

fn last_seg(t: &Type) -> Option<String> {
    if let Type::Path(tp) = t {
        tp.path.segments.last().map(|s| s.ident.to_string())
    } else {
        None
    }
}

fn rewrite_type(
    t: &Type,
    closure: &HashSet<String>,
    gen_names: &HashSet<String>,
    gen_defaults: &HashMap<String, Type>,
) -> String {
    match t {
        Type::Reference(r) => rewrite_type(&r.elem, closure, gen_names, gen_defaults),
        Type::Paren(p) => rewrite_type(&p.elem, closure, gen_names, gen_defaults),
        Type::Tuple(tp) => {
            let inner: Vec<String> = tp.elems.iter().map(|e| rewrite_type(e, closure, gen_names, gen_defaults)).collect();
            format!("({})", inner.join(", "))
        }
        Type::Array(a) => format!("Vec<{}>", rewrite_type(&a.elem, closure, gen_names, gen_defaults)),
        Type::Path(tp) => {
            let seg = match tp.path.segments.last() {
                Some(s) => s,
                None => return "crate::prims::Any".into(),
            };
            let name = seg.ident.to_string();
            let args = ident_args(&seg.arguments);
            if args.is_empty() {
                if let Some(m) = primitive(&name) { return m.into(); }
                if let Some(k) = concrete_kind(&name) { return k.into(); }
                if is_builtin(&name) { return name; }
                if gen_names.contains(&name) {
                    return gen_defaults.get(&name)
                        .map(|d| rewrite_type(d, closure, gen_names, gen_defaults))
                        .unwrap_or_else(|| "crate::prims::Any".into());
                }
                if closure.contains(&name) { return name; }
                return "crate::prims::Any".into();
            }
            let argv: Vec<String> = args.iter().map(|a| match a {
                GenericArgument::Type(ty) => rewrite_type(ty, closure, gen_names, gen_defaults),
                _ => "crate::prims::Any".into(),
            }).collect();
            let kind = concrete_kind(&name);
            if let Some(k) = kind { return k.into(); }
            if is_container(&name) { return format!("{}<{}>", name, argv.join(", ")); }
            if closure.contains(&name) { return name; }
            "crate::prims::Any".into()
        }
        _ => "crate::prims::Any".into(),
    }
}

fn primitive(name: &str) -> Option<&'static str> {
    Some(match name {
        "Address" => "crate::prims::Hex20",
        "B256" | "H256" | "BlockHash" | "TxHash" => "crate::prims::Hex32",
        "B512" | "B64" | "Bloom" | "Bytes" | "Bytecode" | "Bytes48" | "Blob" | "FixedBytes" => "crate::prims::Hex",
        "LogData" => "crate::prims::LogData",
        _ => return None,
    })
}

fn concrete_kind(name: &str) -> Option<&'static str> {
    Some(match name {
        "U256" | "U128" | "U64" | "U32" | "U16" | "U8" | "U" | "I256" | "BlockNumber" => "crate::prims::Quantity",
        _ => return None,
    })
}

fn is_container(name: &str) -> bool {
    matches!(name, "Option" | "Vec" | "Box" | "HashMap" | "BTreeMap" | "HashSet" | "BTreeSet" | "Arc" | "Cow")
}

fn references(ty: &str, owner: &str) -> bool {
    ty.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|w| w == owner)
}

fn is_builtin(name: &str) -> bool {
    matches!(name, "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32"
        | "i64" | "i128" | "isize" | "f32" | "f64" | "bool" | "String" | "char")
}

// ------------------------------------------------------------------ traversal
fn collect_type_names(t: &Type, out: &mut BTreeSet<String>) {
    if let Type::Path(tp) = t {
        if let Some(seg) = tp.path.segments.last() {
            out.insert(seg.ident.to_string());
            for a in ident_args(&seg.arguments) {
                if let GenericArgument::Type(ty) = a { collect_type_names(&ty, out); }
            }
        }
    } else if let Type::Reference(r) = t {
        collect_type_names(&r.elem, out);
    } else if let Type::Tuple(tp) = t {
        for e in &tp.elems { collect_type_names(e, out); }
    } else if let Type::Array(a) = t {
        collect_type_names(&a.elem, out);
    }
}

fn item_refs(def: &Def) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    match def {
        Def::Struct(s) => {
            for f in &s.fields {
                collect_type_names(&f.ty, &mut out);
            }
        }
        Def::Enum(e) => {
            for v in &e.variants {
                for f in &v.fields {
                    collect_type_names(&f.ty, &mut out);
                }
            }
        }
        Def::Alias(a) => collect_type_names(&a.ty, &mut out),
    }
    out
}

// ------------------------------------------------------------------ emit
fn gen_params(s: &ItemStruct, gen_defaults: &HashMap<String, Type>) -> (String, String, HashSet<String>) {
    let mut names = HashSet::new();
    let mut parts = Vec::new();
    for p in &s.generics.params {
        if let syn::GenericParam::Type(tp) = p {
            let n = tp.ident.to_string();
            names.insert(n.clone());
            if let Some(d) = &tp.default {
                let closure: HashSet<String> = HashSet::new();
                parts.push(format!("{} = {}", n, rewrite_type(d, &closure, &names, gen_defaults)));
            } else {
                parts.push(n);
            }
        }
    }
    let gen = if parts.is_empty() { String::new() } else { format!("<{}>", parts.join(", ")) };
    let where_s = if names.is_empty() { String::new() }
        else { format!("\nwhere {}", names.iter().map(|p| format!("{p}: JsonSchema")).collect::<Vec<_>>().join(", ")) };
    (gen, where_s, names)
}

fn emit_struct(s: &ItemStruct, closure: &HashSet<String>) -> String {
    let gen_defaults: HashMap<String, Type> = s.generics.params.iter().filter_map(|p| {
        if let syn::GenericParam::Type(tp) = p {
            tp.default.as_ref().map(|d| (tp.ident.to_string(), d.clone()))
        } else { None }
    }).collect();
    let (_gen, _where_s, names) = gen_params(s, &gen_defaults);
    let cont = parse_container(&s.attrs);
    let mut lines = Vec::new();
    lines.push("#[derive(JsonSchema)]".to_string());
    if let Some(r) = &cont.rename_all { lines.push(format!("#[schemars(rename_all = \"{r}\")]")); }
    if cont.untagged { lines.push("#[schemars(untagged)]".to_string()); }
    lines.push(format!("pub struct {} {{", s.ident));
    if let Fields::Named(named) = &s.fields {
        for f in &named.named {
            let fs = parse_field(&f.attrs);
            if fs.skip { continue; }
            let fname = f.ident.as_ref().unwrap();
            let wv = with_mapped(&fs.with);
            let mut ty = wv.clone().unwrap_or_else(|| rewrite_type(&f.ty, closure, &names, &gen_defaults));
            if references(&ty, &s.ident.to_string()) { ty = "crate::prims::Any".to_string(); }
            if let Some(r) = &fs.rename { lines.push(format!("    #[schemars(rename = \"{r}\")]")); }
            if fs.flatten { lines.push("    #[schemars(flatten)]".to_string()); }
            if let Some(w) = &wv { lines.push(format!("    #[schemars(with = \"{w}\")]")); }
            lines.push(format!("    pub {}: {},", fname, ty));
        }
    }
    lines.push("}".to_string());
    lines.join("\n")
}

fn emit_enum(e: &ItemEnum, closure: &HashSet<String>) -> String {
    let gen_defaults: HashMap<String, Type> = e.generics.params.iter().filter_map(|p| {
        if let syn::GenericParam::Type(tp) = p {
            tp.default.as_ref().map(|d| (tp.ident.to_string(), d.clone()))
        } else { None }
    }).collect();
    let (_gen, _where_s, names) = gen_params_enum(e, &gen_defaults);
    let cont = parse_container(&e.attrs);
    let mut lines = Vec::new();
    lines.push("#[derive(JsonSchema)]".to_string());
    if cont.untagged { lines.push("#[schemars(untagged)]".to_string()); }
    if let Some(t) = &cont.tag { lines.push(format!("#[schemars(tag = \"{t}\")]")); }
    if let Some(c) = &cont.content { lines.push(format!("#[schemars(content = \"{c}\")]")); }
    lines.push(format!("pub enum {} {{", e.ident));
    for v in &e.variants {
        let fs = parse_field(&v.attrs);
        if let Some(r) = &fs.rename { lines.push(format!("    #[schemars(rename = \"{r}\")]")); }
        match &v.fields {
            Fields::Unit => lines.push(format!("    {},", v.ident)),
            Fields::Unnamed(u) => {
                let own = e.ident.to_string();
                let tys: Vec<String> = u.unnamed.iter().map(|f| {
                    let t = rewrite_type(&f.ty, closure, &names, &gen_defaults);
                    if references(&t, &own) { "crate::prims::Any".to_string() } else { t }
                }).collect();
                lines.push(format!("    {}({}),", v.ident, tys.join(", ")));
            }
            Fields::Named(n) => {
                let mut fl = Vec::new();
                for f in &n.named {
                    let ffn = f.ident.as_ref().unwrap();
                    let mut ty = rewrite_type(&f.ty, closure, &names, &gen_defaults);
                    if references(&ty, &e.ident.to_string()) { ty = "crate::prims::Any".to_string(); }
                    fl.push(format!("{}: {}", ffn, ty));
                }
                lines.push(format!("    {} {{ {} }},", v.ident, fl.join(", ")));
            }
        }
    }
    lines.push("}".to_string());
    lines.join("\n")
}

fn gen_params_enum(e: &ItemEnum, gen_defaults: &HashMap<String, Type>) -> (String, String, HashSet<String>) {
    let mut names = HashSet::new();
    let mut parts = Vec::new();
    for p in &e.generics.params {
        if let syn::GenericParam::Type(tp) = p {
            let n = tp.ident.to_string();
            names.insert(n.clone());
            if let Some(d) = &tp.default {
                let closure: HashSet<String> = HashSet::new();
                parts.push(format!("{} = {}", n, rewrite_type(d, &closure, &names, gen_defaults)));
            } else {
                parts.push(n);
            }
        }
    }
    let gen = if parts.is_empty() { String::new() } else { format!("<{}>", parts.join(", ")) };
    let where_s = if names.is_empty() { String::new() }
        else { format!("\nwhere {}", names.iter().map(|p| format!("{p}: JsonSchema")).collect::<Vec<_>>().join(", ")) };
    (gen, where_s, names)
}

// ------------------------------------------------------------------ main
fn emit_methods(inventory_json: &str, closure: &HashSet<String>) -> Result<String> {
    let inv: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(inventory_json)?)?;
    let methods = inv.get("methods").and_then(|m| m.as_object()).context_expected()?;
    let empty_names: HashSet<String> = HashSet::new();
    let empty_defs: HashMap<String, Type> = HashMap::new();
    let to_expr = |s: &str| -> String {
        match syn::parse_str::<Type>(s) {
            Ok(t) => rewrite_type(&t, closure, &empty_names, &empty_defs),
            Err(_) => "crate::prims::Any".to_string(),
        }
    };
    let mut code = String::new();
    code.push_str("use schemars::gen::SchemaGenerator;\nuse schemars::JsonSchema;\nuse serde_json::json;\nuse serde_json::Value;\nuse std::collections::HashMap;\n#[allow(unused_imports)]\nuse crate::model::*;\n\n");
    code.push_str("pub fn build() -> Value {\n    let mut gen = SchemaGenerator::default();\n    let mut methods = serde_json::Map::new();\n");
    for (name, m) in methods {
        let params = m.get("params").and_then(|p| p.as_array()).cloned().unwrap_or_default();
        let result = m.get("result").and_then(|r| r.as_str()).map(|s| s.to_string());
        code.push_str("    {\n");
        code.push_str("        let mut prefix: Vec<Value> = Vec::new();\n");
        code.push_str("        let mut props = serde_json::Map::new();\n");
        code.push_str("        let mut required: Vec<Value> = Vec::new();\n");
        let mut n_req = 0;
        for (i, p) in params.iter().enumerate() {
            let pname = p.get("name").and_then(|x| x.as_str()).unwrap_or("");
            let ptype = p.get("type").and_then(|x| x.as_str()).unwrap_or("crate::prims::Any");
            let expr = to_expr(ptype);
            let req = i < 1 || !ptype.starts_with("Option<");
            if req { n_req = i + 1; }
            code.push_str(&format!("        prefix.push(serde_json::to_value(gen.subschema_for::<{expr}>()).unwrap());\n"));
            code.push_str(&format!("        props.insert({:?}.to_string(), prefix[{}].clone());\n", pname, i));
            if req {
                code.push_str(&format!("        required.push(json!({:?}));\n", pname));
            }
        }
        code.push_str(&format!("        let arr = json!({{\"type\":\"array\",\"prefixItems\":prefix,\"minItems\":{},\"maxItems\":{},\"items\":false}});\n", n_req, params.len()));
        code.push_str("        let obj = json!({\"type\":\"object\",\"properties\":props,\"required\":required,\"additionalProperties\":false});\n");
        code.push_str("        let params_schema = json!({\"anyOf\":[arr,obj]});\n");
        match result {
            Some(r) => {
                let expr = to_expr(&r);
                code.push_str(&format!("        let result_schema = serde_json::to_value(gen.subschema_for::<{expr}>()).unwrap();\n"));
                code.push_str(&format!("        methods.insert({:?}.to_string(), json!({{\"params\":params_schema,\"result\":result_schema}}));\n", name));
            }
            None => {
                code.push_str(&format!("        methods.insert({:?}.to_string(), json!({{\"params\":params_schema}}));\n", name));
            }
        }
        code.push_str("    }\n");
    }
    code.push_str("    let defs = serde_json::to_value(gen.definitions()).unwrap();\n");
    code.push_str("    let mut doc = json!({\"$schema\":\"https://json-schema.org/draft/2020-12/schema\",\"methods\":methods,\"$defs\":defs});\n");
    code.push_str("    let s = doc.to_string().replace(\"#/definitions/\", \"#/$defs/\");\n    serde_json::from_str(&s).unwrap()\n}\n");
    Ok(code)
}

trait ContextExpected<T> { fn context_expected(self) -> Result<T>; }
impl<T> ContextExpected<T> for Option<T> {
    fn context_expected(self) -> Result<T> { self.ok_or_else(|| anyhow::anyhow!("missing 'methods'")) }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let work = &args[1];
    let closure_json = &args[2];
    let inventory_json = &args[3];
    let outdir = &args[4];

    // closure: name -> {kind,file,crate}
    let closure: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(closure_json)?)?;
    const ALLOWED: &[&str] = &[
        "rpc-types-eth", "rpc-types-txpool", "rpc-types-mev", "rpc-types-any",
        "eips", "consensus", "consensus-any", "network-primitives", "serde",
        "rpc-eth-types", "rpc-api", "rpc-eth-api", "rpc-convert",
    ];
    let mut closure_names: HashSet<String> = HashSet::new();
    for (name, v) in closure.as_object().unwrap() {
        let crate_name = v.get("crate").and_then(|x| x.as_str()).unwrap_or("");
        if ALLOWED.contains(&crate_name) {
            closure_names.insert(name.clone());
        }
    }

    // build registry from work root
    let mut registry: HashMap<String, Vec<Def>> = HashMap::new();
    let mut def_files: HashMap<String, String> = HashMap::new();
    // collect file paths from closure to parse
    let mut wanted_files: BTreeSet<String> = BTreeSet::new();
    for (_, v) in closure.as_object().unwrap() {
        if let Some(f) = v.get("file").and_then(|x| x.as_str()) {
            wanted_files.insert(format!("{}/{}", work, f));
        }
    }
    for path in &wanted_files {
        let text = match std::fs::read_to_string(path) { Ok(t) => t, Err(_) => continue };
        let file = match syn::parse_file(&text) { Ok(f) => f, Err(_) => continue };
        for item in &file.items {
            let (name, def) = match item {
                Item::Struct(s) if matches!(s.vis, Visibility::Public(_)) => (s.ident.to_string(), Def::Struct(s.clone())),
                Item::Enum(e) if matches!(e.vis, Visibility::Public(_)) => (e.ident.to_string(), Def::Enum(e.clone())),
                Item::Type(t) if matches!(t.vis, Visibility::Public(_)) => (t.ident.to_string(), Def::Alias(t.clone())),
                _ => continue,
            };
            registry.entry(name.clone()).or_default().push(def);
            def_files.entry(name).or_insert_with(|| path.clone());
        }
    }

    let mut out = String::from("use schemars::JsonSchema;\nuse schemars::gen::SchemaGenerator;\nuse schemars::schema::Schema;\n#[allow(unused_imports)]\nuse std::collections::{BTreeMap, HashMap, HashSet};\n#[allow(unused_imports)]\nuse crate::prims::*;\n\n");
    let mut emitted = 0;
    for name in &closure_names {
        if let Some(j) = override_json(name) {
            out.push_str(&override_impl(name, j));
            out.push_str("\n\n");
            emitted += 1;
            continue;
        }
        let def = registry.get(name).and_then(|v| v.first());
        match def {
            Some(Def::Alias(t)) => {
                let empty = HashSet::new();
                let gd = HashMap::new();
                out.push_str(&format!("pub type {} = {};\n\n", name, rewrite_type(&t.ty, &closure_names, &empty, &gd)));
                emitted += 1;
            }
            Some(Def::Struct(s)) => { out.push_str(&emit_struct(s, &closure_names)); out.push_str("\n\n"); emitted += 1; }
            Some(Def::Enum(e)) => { out.push_str(&emit_enum(e, &closure_names)); out.push_str("\n\n"); emitted += 1; }
            None => { out.push_str(&format!("pub type {} = crate::prims::Any;\n\n", name)); }
        }
    }

    std::fs::create_dir_all(format!("{}/src", outdir))?;
    std::fs::write(format!("{}/src/model.rs", outdir), out)?;
    let methods_rs = emit_methods(inventory_json, &closure_names)?;
    std::fs::write(format!("{}/src/methods.rs", outdir), methods_rs)?;
    println!("emitted {emitted} items -> {}/src/model.rs + methods.rs", outdir);
    Ok(())
}
