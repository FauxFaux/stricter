#[macro_use]
extern crate clap;

use std::ffi::OsStr;
use std::fs;

use clap::Arg;
use failure::format_err;
use failure::Error;
use failure::ResultExt;

use stricter::inventory;
use stricter::model;

fn main() -> Result<(), Error> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

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

    let model = model::load(
        &fs::read(model).with_context(|_| format_err!("opening model file {:?}", model))?,
    )?;
    let inventory = inventory::load()?;
    for host in inventory.hosts {
        let connector = inventory::pick_connector(&host)?;
        let mut transport = connector.connect()?;
        stricter::connector::init(transport.as_mut())?;
        transport.shutdown()?;
    }
    Ok(())
}
