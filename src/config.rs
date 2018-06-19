use std::error::Error;
use std::fs::File;
use std::default::Default;
use std::path::Path;
use serde_json;
use ::protocol::ProtocolKind;

#[derive(Debug, Serialize, Deserialize, Getters)]
pub struct Config {
    #[get = "pub"]
    protocol: ProtocolKind,
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
        let c: Config = serde_json::from_reader(File::open(path)?)?;
        Ok(c)
    }
}

impl Default for Config {
    fn default() -> Config { Config { protocol: ProtocolKind::interop } }
}