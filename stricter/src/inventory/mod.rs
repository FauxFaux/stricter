use std::collections::HashMap;

use failure::Error;
use serde_json::Value;

mod load_python;

#[derive(Debug, Clone)]
struct HostTemplate {
    connectors: Vec<HashMap<String, String>>,
    tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct InventoryTemplate {
    hosts: Vec<HostTemplate>,
}

pub fn load(input: &str) -> Result<Value, Error> {
    load_python::run_rust_python(input)
}
