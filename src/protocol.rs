use crate::actions;
use crate::config::Config;
use crate::splitter;
use std::error::Error;
use std::path::Path;

pub struct View<T> {
    pub view: Option<T>,
}

pub struct Engine {}

impl Engine {
    pub fn new<C: Into<Config>>(_config: C) -> Self {
        Engine {}
    }
}

pub trait Protocol<T> {
    fn init<C: Into<Config>>(_config: C);
    fn handle<S>(msg: &str, view: &mut View<T>, send: S)
    where
        S: FnOnce(String, &mut View<T>) -> Result<(), Box<Error>>;

    fn eval(s: String, view: &mut View<T>) -> Result<(), &'static str>;

    fn process(msg: &str, view: &mut View<T>) -> Result<String, Box<Error>>;
}

impl<P: AsRef<Path>> From<P> for Config {
    fn from(path: P) -> Self {
        Config::read(path).unwrap_or_default()
    }
}
