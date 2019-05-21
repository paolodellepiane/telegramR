mod actions;
mod config;
mod protocol;
mod u;

#[cfg(feature = "browser")]
#[path = ""]
mod browser_modules {
    pub mod browser;
    pub mod minimal_server;
}
#[cfg(feature = "browser")]
pub use browser_modules::*;

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
