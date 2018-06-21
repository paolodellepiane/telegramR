#![windows_subsystem = "windows"]
extern crate serde_json;
extern crate web_view;
extern crate ws;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_derive;

mod actions;
mod config;
mod protocol;

use config::Config;
use protocol::Protocol;

fn main() {
    let config = Config::read("config.json").or(Config::read("../../config.json"))
                                            .unwrap_or_default();
    Protocol::new(config);
}
