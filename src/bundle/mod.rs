mod error;

use std::path::PathBuf;
use crate::error::Error;
use self::error::BundleError;

const CONFIG_FILE_NAME: &str = "config.json";

pub fn config_file_path(path: &str) -> Result<PathBuf, Error> {
    let bundle_path = load_bundle_path(path)?;
    let config_path = load_config_file_path(bundle_path)?;
    Ok(config_path)
}

fn load_bundle_path(path: &str) -> Result<PathBuf, BundleError> {
    match PathBuf::from(path).canonicalize() {
        Ok(path) => Ok(path),
        Err(err) => Err(BundleError{
            path: String::from(path),
            message: err.to_string()
        })
    }
}

fn load_config_file_path(bundle_path: PathBuf) -> Result<PathBuf, BundleError> {
    match bundle_path.join(CONFIG_FILE_NAME).canonicalize() {
        Ok(path) => Ok(path),
        Err(err) => {
            Err(BundleError{
                path: String::from(CONFIG_FILE_NAME),
                message: err.to_string()
            })
        }
    }
}
