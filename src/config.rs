use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{default::Default, error::Error, fs::File, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config;

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
        Ok(serde_json::from_reader(File::open(path)?)?)
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {}
    }
}
