use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject, StringValidation};
use schemars::{schema_for, JsonSchema, SchemaGenerator as _SG};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// ---- primitive schema types ----
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hex20;
impl JsonSchema for Hex20 {
    fn schema_name() -> String { "Hex20".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject { instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation { pattern: Some("^0x[0-9a-fA-F]{40}$".into()), ..Default::default() })),
            ..Default::default() }.into()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quantity;
impl JsonSchema for Quantity {
    fn schema_name() -> String { "Quantity".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject { instance_type: Some(InstanceType::String.into()),
            string: Some(Box::new(StringValidation { pattern: Some("^0x([1-9a-fA-F][0-9a-fA-F]*|0)$".into()), ..Default::default() })),
            ..Default::default() }.into()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LogData {
    pub address: Hex20,
    pub topics: Vec<Hex20>,
    pub data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(flatten)]
    pub inner: LogData,
    pub block_hash: Option<String>,
    #[schemars(with = "Option<Quantity>")]
    pub block_number: Option<u64>,
    pub removed: bool,
}

fn main() {
    let s = schema_for!(Log);
    println!("{}", serde_json::to_string_pretty(&s).unwrap());
}
