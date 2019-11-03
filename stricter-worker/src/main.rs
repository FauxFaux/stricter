use failure::Error;

mod app;

const US: &str = env!("VERGEN_SHA");

fn usage() {
    eprintln!("usage: --accept VERSION");
    eprintln!("usage: --start");
}

fn entry() -> Result<i32, Error> {
    let mut args = std::env::args();
    let _us = args.next().expect("binary path exists");
    Ok(match args.next() {
        Some(arg) => match arg.as_str() {
            "--accept" => {
                let requested = args.next().expect("requested");
                if requested != US {
                    eprintln!("{} != {}", requested, US);
                    4
                } else {
                    0
                }
            }
            "--start" => {
                println!("U");
                app::app()?;
                0
            },
            _ => {
                usage();
                2
            }
        },
        None => {
            usage();
            2
        }
    })
}

fn main() -> Result<(), Error> {
    std::process::exit(entry()?)
}
