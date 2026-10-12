mod methods;
#[allow(dead_code)]
mod model;
#[allow(dead_code)]
mod prims;

fn main() {
    let doc = methods::build();
    println!("{}", serde_json::to_string_pretty(&doc).unwrap());
}
