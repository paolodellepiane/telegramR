#![windows_subsystem = "windows"]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod actions;
mod config;
mod protocol;

#[cfg(feature = "use-browser")]
mod browser;

#[cfg(feature = "use-embedded")]
mod embedded;
#[cfg(feature = "use-embedded")]
mod splitter;

use crate::config::Config;
use crate::protocol::Protocol;

fn main() {
    protocol::View::init(Config::default());
}
