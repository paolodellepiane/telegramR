// #![windows_subsystem = "windows"]
extern crate web_view;
extern crate ws;
extern crate serde_json;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate serde_derive;

mod actions;
mod config;
mod protocol;

fn main() {
    let config = config::Config::read("config.json")
        .or(config::Config::read("../../config.json"))
        .unwrap_or_default();
    protocol::Protocol::new(config);
}
