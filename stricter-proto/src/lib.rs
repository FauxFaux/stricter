#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    msg: String,
}
