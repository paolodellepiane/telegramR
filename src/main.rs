#![windows_subsystem = "windows"]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate snap;
extern crate web_view;
extern crate ws;

mod actions;

use std::io::prelude::*;
use web_view::*;
const HTML: &'static [u8] = include_bytes!("d.sz");

fn main() {
    if cfg!(feature = "protocol_ws") {
        use ws::{listen, Message};
        listen("127.0.0.1:36767", |out| {
            move |msg: Message| msg.as_text().map(|s| handle(s, |m| out.send(m).map_err(|_| "")))
        }).expect("Failed to create WebSocket")
    } else {
        let mut f = snap::Reader::new(HTML);
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("can't inizialize view");
        run("telegramR",
            Content::Html(buffer),
            Some((900, 800)),
            true,
            true,
            |_| {},
            move |webview, arg, _| handle(arg, |m| eval(webview, m)),
            ());
    }
}



#[allow(non_snake_case)]
fn handle<S>(msg: &str, send: S)
    where S: FnOnce(String) -> Result<(), &'static str> {
    send(actions::process(msg).unwrap()).expect("error sending answer")
}

fn eval<'a, T>(webview: &mut WebView<'a, T>, s: String) -> Result<(), &'static str> {
    match s.as_ref() {
        "openDialog" => {
            webview.dialog(Dialog::OpenFile, "open", "");
        }
        "info" => {
            webview.dialog(Dialog::Alert(Alert::Info), "test", "test");
        }
        _ => {
            webview.eval(&format!("window.render({})", s));
        }
    }
    Ok(())
}
