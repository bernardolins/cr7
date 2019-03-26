extern crate cr7;
extern crate rand;

mod common;

use cr7::config;
use cr7::error::Error;

use common::{TestBundle, ConfigTemplate};

#[test]
fn load_config_file_not_found() {
    let test_bundle = TestBundle::empty();
    let config_path = test_bundle.path().join("unexistent_file.json");

    let config = config::load(&config_path);
    assert!(config.is_err(), "expect {:?} to be err", config);
    assert_eq!(config.err().unwrap(), Error::NotFound);
}

#[test]
fn load_config_file_with_synxtax_error() {
    let test_bundle = TestBundle::new(ConfigTemplate::SyntaxError);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path);
    assert!(config.is_err(), "expect {:?} to be err", config);
    assert_eq!(config.err().unwrap(), Error::ConfigSyntax);
}

#[test]
fn load_config_file_invalid() {
    let test_bundle = TestBundle::new(ConfigTemplate::Invalid);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path);
    assert!(config.is_err(), "expect {:?} to be err", config);
    assert_eq!(config.err().unwrap(), Error::ParseConfig);
}

#[test]
fn load_config_file_successfully() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path);
    assert!(config.is_ok(), "expect {:?} to be ok", config);
}

#[test]
fn config_public_method_oci_version() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path).expect("expected config to be ok");
    assert_eq!(config.oci_version(), "1.0.1-dev");
}

#[test]
fn config_public_method_hostname_with_value() {
    let test_bundle = TestBundle::new(ConfigTemplate::Valid);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path).expect("expected config to be ok");
    assert_eq!(config.hostname(), &Some("container_hostname".to_string()));
}

#[test]
fn config_public_method_hostname_with_no_hostname() {
    let test_bundle = TestBundle::new(ConfigTemplate::NoHostname);
    let config_path = test_bundle.path().join("config.json");

    let config = config::load(&config_path).expect("expected config to be ok");
    assert_eq!(config.hostname(), &None);
}
