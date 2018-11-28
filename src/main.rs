#![windows_subsystem = "windows"]
extern crate serde_json;
extern crate web_view;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "use-ws")]
extern crate ws;

mod actions;
mod config;
mod protocol;
mod splitter;

use config::Config;
use protocol::Protocol;

fn main() { Protocol::new(Config::default()); }
