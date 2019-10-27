use std::collections::HashMap;

use failure::Error;

#[derive(Debug, Clone)]
pub struct HostTemplate {
    connectors: Vec<HashMap<String, String>>,
    tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct InventoryTemplate {
    hosts: Vec<HostTemplate>,
}

pub fn load() -> Result<InventoryTemplate, Error> {
    Ok(InventoryTemplate {
        hosts: vec![HostTemplate {
            connectors: vec![],
            tags: maplit::hashmap! {},
        }],
    })
}
