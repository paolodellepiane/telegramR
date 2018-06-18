extern crate serde_json;

use protocol::View;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use web_view::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    getFile,
    info { text: String },
}

#[allow(non_camel_case_types, non_snake_case)]
pub fn process(msg: &str, view: &mut View) -> Result<String, Box<Error>> {
    use self::Action::*;
    println!("req: {}", msg);
    match serde_json::from_str(msg).unwrap() {
        getFile => {
            match &mut view.webview {
                Some(webview) => {
                    let path = webview.dialog(Dialog::OpenFile, "open", "");
                    let mut buf = String::new();
                    File::open(&path)?.read_to_string(&mut buf)?;
                    Ok(serde_json::to_string(&(path, buf)).unwrap())
                }
                None => Ok(String::from("nothing to open")),
            }
        },
        info { text } => {
            view.webview.as_mut()
                .map(|v| v.dialog(Dialog::Alert(Alert::Info), "", &text[..]))
                .ok_or(Box::from("info"))
        }
    }
}
