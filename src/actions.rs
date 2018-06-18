extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use ::protocol::View;
use web_view::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    getFile { filePath: String },
    openDialog,
    info { text: String },
}

#[allow(non_camel_case_types, non_snake_case)]
pub fn process(msg: &str, view: &mut View) -> Result<String, Box<Error>> {
    use self::Action::*;
    println!("req: {}", msg);
    match serde_json::from_str(msg).unwrap() {
        getFile { filePath } => { 
            let mut f = File::open(filePath)?;
            let mut buf = String::new();
            f.read_to_string(&mut buf)?;
            Ok(serialize(buf))
        },
        openDialog => view.webview.as_mut().map(|v| v.dialog(Dialog::OpenFile, "open", "")).ok_or(Box::from("openDialog")),
        info { text: _ } => view.webview.as_mut().map(|v| v.dialog(Dialog::Alert(Alert::Info), "test", "test")).ok_or(Box::from("info")),
    }
}

fn serialize(s: String) -> String { println!("ser: {}", s); serde_json::to_string(&s).unwrap() }