mod actions;
mod config;
mod protocol;
mod u;

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

pub fn run() {
    View::init(Config::default());
}
