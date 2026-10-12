mod model;
mod prims;

use schemars::schema_for;
use schemars::JsonSchema;

fn dump<T: JsonSchema>(name: &str) {
    let s = schema_for!(T);
    println!("===== {} =====\n{}\n", name, serde_json::to_string(&s).unwrap());
}

fn main() {
    dump::<model::Log>("Log");
}
