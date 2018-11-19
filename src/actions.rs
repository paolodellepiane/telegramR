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
        getFile => match &mut view.webview {
            Some(webview) => {
                let path = webview.dialog(Dialog::OpenFile, "open", "");
                let mut buf = String::new();
                File::open(&path)?.read_to_string(&mut buf)?;
                Ok(serde_json::to_string(&(path, buf)).unwrap())
            }
            None => Ok(serde_json::to_string(&("mocke file path", LOREM)).unwrap()),
        },
        info { text } => view.webview
                             .as_mut()
                             .map(|v| v.dialog(Dialog::Alert(Alert::Info), "", &text[..]))
                             .ok_or(Box::from("info")),
    }
}

const LOREM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
