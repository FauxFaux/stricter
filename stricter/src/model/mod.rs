use std::collections::HashMap;

use failure::bail;
use failure::ensure;
use failure::Error;
use failure::ResultExt;

mod write;

mod err;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Action {
    name: Option<String>,
    #[serde(rename = "become")]
    becomes: Option<bool>,
    #[serde(flatten)]
    task: Task,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub enum Task {
    Write(write::WriteTask),
}

pub fn load(from: &[u8]) -> Result<Vec<Action>, Error> {
    let as_map: HashMap<String, Vec<Action>> =
        toml::from_slice(from).with_context(|e| match e.line_col() {
            Some((target_line, col)) => err::source_context(from, e, target_line, col),
            None => "unknown".to_string(),
        })?;
    if as_map.is_empty() {
        bail!("empty document?");
    }

    let keys = as_map.keys().collect::<Vec<&String>>();
    ensure!(
        keys == vec![&"action".to_string()],
        "internal error parsing document, keys was {:?}",
        keys
    );

    Ok(as_map.into_iter().next().expect("validated keys").1)
}
