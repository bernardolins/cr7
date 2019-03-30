#[macro_use]
extern crate clap;
extern crate cr7;

use clap::App;
use cr7::container;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    /*
     * create command
     */
    if let Some(matches) = matches.subcommand_matches("create") {
        let bundle_path = matches.value_of("bundle").unwrap_or(".");
        let container_id = matches.value_of("container-id").unwrap();

        match container::create(container_id, bundle_path) {
            Ok(container) => println!("{:?}", container),
            Err(err) => println!("{}", err),
        };
    }

    /*
     * state command
     */
    if let Some(matches) = matches.subcommand_matches("state") {
        let container_id = matches.value_of("container-id").unwrap();

        match container::state(container_id) {
            Ok(state_json) => println!("{}", state_json),
            Err(err) => println!("{}", err),
        };
    }
}
