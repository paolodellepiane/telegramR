use crate::config::Config;
use std::error::Error;
use std::path::Path;

pub trait Bag {}
impl Bag for () {}

pub trait Protocol<T: Bag = ()> {
    fn init<C: Into<Config>>(_config: C);
    fn handle<S>(msg: &str, bag: &mut T, send: S)
    where
        S: FnOnce(String, &mut T) -> Result<(), Box<Error>>;
    fn eval(s: String, bag: &mut T) -> Result<(), &'static str>;
    fn process(msg: &str, bag: &mut T) -> Result<String, Box<Error>>;
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        Config::read(path).unwrap_or_default()
    }
}
