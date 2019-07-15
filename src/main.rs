#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;

use cr7::filesystem;
use cr7::Error;
use cr7::Config;
use cr7::bundle;
use cr7::libcontainer::Container;
use cr7::libcontainer::Environment;

use std::convert::TryFrom;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        let cwd = filesystem::cwd();
        let container_id = matches.value_of("container-id").unwrap();
        let bundle_path = matches.value_of("bundle").unwrap_or(&cwd);

        let config_file = bundle::read_config(&bundle_path)?;
        let config = Config::load(config_file)?;

        let environment = Environment::try_from(config).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(3);
        });


        let mut container = Container::new(container_id, environment);
        container.run().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(4);
        });
    }

    Ok(())
}
