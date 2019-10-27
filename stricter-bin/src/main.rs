#[macro_use]
extern crate clap;

use std::ffi::OsStr;
use std::fs;

use clap::Arg;
use failure::Error;

use stricter::inventory;
use stricter::model;

fn main() -> Result<(), Error> {
    let matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("model")
                .short("m")
                .long("model")
                .takes_value(true),
        )
        .get_matches();

    let model = matches
        .value_of_os("model")
        .unwrap_or(OsStr::new("model.toml"));

    let model = model::load(&fs::read(model)?)?;
    let inventory = inventory::load()?;
    for host in inventory.hosts {
        let connector = inventory::pick_connector(&host)?;
        let transport = connector.connect()?;
        transport.shutdown()?;
    }
    Ok(())
}
