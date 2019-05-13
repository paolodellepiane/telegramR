#![windows_subsystem = "windows"]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

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
    // browser::Browser::init(Config::default());
}
