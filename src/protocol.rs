use crate::config::Config;
use std::error::Error;
use std::path::Path;

pub trait View {}

pub struct Engine {}

impl Engine {
    pub fn new<C: Into<Config>>(_config: C) -> Self {
        Engine {}
    }
}

pub trait Protocol {
    fn init<C: Into<Config>>(_config: C);
    fn handle<S>(msg: &str, view: &mut View, send: S)
    where
        S: FnOnce(String, &mut View) -> Result<(), Box<Error>>;

    fn eval(s: String, view: &mut View) -> Result<(), &'static str>;

    fn process(msg: &str, view: &mut View) -> Result<String, Box<Error>>;
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        Config::read(path).unwrap_or_default()
    }
}
