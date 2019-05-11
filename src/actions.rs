extern crate serde_json;

use crate::protocol::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    getFile,
    info { text: String },
}

fn process(msg: &str, view: &mut View<()>) -> Result<String, Box<Error>> {
    Ok(String::from(""))
}
