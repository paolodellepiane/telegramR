#![windows_subsystem = "windows"]
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
use crate::protocol::*;

fn main() {
    View::init(Config::default());
}
