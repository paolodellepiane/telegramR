#![windows_subsystem = "windows"]
mod actions;
mod config;
mod protocol;

#[cfg(feature = "browser")]
mod browser;

#[cfg(feature = "embedded")]
#[path = ""]
mod embedded_modules {
    pub mod embedded;
    pub mod splitter;
}
#[cfg(feature = "embedded")]
pub use embedded_modules::*;

use crate::config::Config;
use crate::protocol::*;

fn main() {
    View::init(Config::default());
}
