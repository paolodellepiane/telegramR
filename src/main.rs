#![windows_subsystem = "windows"]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod actions;
mod config;
mod protocol;

#[cfg(feature = "use-ws")]
mod ws_protocol;
#[cfg(feature = "use-wv")]
mod wv_protocol;

use crate::config::Config;
use crate::protocol::Engine;

// TODO: pass actions as paramteres; Protocol -> Engine
fn main() {
    Engine::new(Config::default());
}
