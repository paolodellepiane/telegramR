extern crate serde_json;
extern crate snap;

use actions;
use std::io::prelude::*;
use web_view::*;

pub struct Protocol {}

impl Protocol {
    pub fn new(protocol: &str) -> Protocol {
        match protocol {
            "ws" => Protocol::init_ws(),
            _ => Protocol::init_interop(),
        };
        Protocol {}
    }

    fn init_ws() {
        use ws::{listen, Message};
        listen("127.0.0.1:36767", |out| {
            move |msg: Message| msg.as_text().map(|s| Protocol::handle(s, |m| out.send(m).map_err(|_| "")))
        }).expect("Failed to create WebSocket")
    }

    fn init_interop() {
        let mut f = snap::Reader::new(Protocol::HTML);
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("can't inizialize view");
        run("telegramR",
            Content::Html(buffer),
            Some((900, 800)),
            true,
            true,
            |_| {},
            move |webview, arg, _| Protocol::handle(arg, |m| Protocol::eval(m, webview)),
            ());
    }

    #[allow(non_snake_case)]
    fn handle<S>(msg: &str, send: S)
        where S: FnOnce(String) -> Result<(), &'static str> {
        send(actions::process(msg).unwrap()).expect("error sending answer")
    }

    fn eval<'a, T>(s: String, webview: &mut WebView<'a, T>) -> Result<(), &'static str> {
        match s.as_ref() {
            "open" => {
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

    const HTML: &'static [u8] = include_bytes!("d.sz");
}
