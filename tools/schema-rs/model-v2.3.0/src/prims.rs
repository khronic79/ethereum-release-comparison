use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject, StringValidation};
use schemars::JsonSchema;

fn hex(pattern: &str, name: &str) -> Schema {
    SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        string: Some(Box::new(StringValidation { pattern: Some(pattern.into()), ..Default::default() })),
        metadata: Some(Box::new(schemars::schema::Metadata { title: Some(name.into()), ..Default::default() })),
        ..Default::default()
    }.into()
}

pub struct Hex20;
impl JsonSchema for Hex20 {
    fn schema_name() -> String { "Hex20".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema { hex("^0x[0-9a-fA-F]{40}$", "address") }
}
pub struct Hex32;
impl JsonSchema for Hex32 {
    fn schema_name() -> String { "Hex32".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema { hex("^0x[0-9a-fA-F]{64}$", "hash") }
}
pub struct Hex;
impl JsonSchema for Hex {
    fn schema_name() -> String { "Hex".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema { hex("^0x[0-9a-fA-F]*$", "hex") }
}
pub struct Quantity;
impl JsonSchema for Quantity {
    fn schema_name() -> String { "Quantity".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema { hex("^0x([1-9a-fA-F][0-9a-fA-F]*|0)$", "quantity") }
}
pub struct Any;
impl JsonSchema for Any {
    fn schema_name() -> String { "Any".into() }
    fn json_schema(_: &mut SchemaGenerator) -> Schema { SchemaObject::default().into() }
}
#[derive(JsonSchema)]
pub struct LogData {
    pub address: Hex20,
    pub topics: Vec<Hex32>,
    pub data: Hex,
}
