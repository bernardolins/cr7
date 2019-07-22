#[macro_use]
extern crate clap;
extern crate cr7;

use cr7::cli;
use cr7::Error;

use clap::App;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        cli::run(matches)?;
    }

    if let Some(matches) = matches.subcommand_matches("start") {
        cli::start(matches)?;
    }

    if let Some(matches) = matches.subcommand_matches("query") {
        cli::query(matches)?;
    }

    Ok(())
}
