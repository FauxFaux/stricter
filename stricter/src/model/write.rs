#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WriteTask {
    source: String,
    dest: String,
}
