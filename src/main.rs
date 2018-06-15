// #![windows_subsystem = "windows"]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate web_view;
extern crate ws;

mod actions;
mod protocol;

fn main() {
    protocol::Protocol::new("_");
}
