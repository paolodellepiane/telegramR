extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;
use std::io::Result;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    getFile { filePath: String },
    openDialog,
}

#[allow(non_camel_case_types, non_snake_case)]
pub fn process(msg: &str) -> Result<String> {
    use self::Action::*;
    println!("req: {}", msg);
    match serde_json::from_str(msg).unwrap() {
        getFile { filePath } => { 
            let mut f = File::open(filePath)?;
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            return Ok(serialize(buf));
        },
        openDialog => Ok(String::from("info")),
    }
}

fn serialize(s: String) -> String { println!("ser: {}", s); serde_json::to_string(&s).unwrap() }