extern crate serde_json;
extern crate snap;

use actions;
use std::io::prelude::*;
use web_view::*;

pub type MyWebView = WebView<'static, ()>;
pub struct Protocol;
pub struct View<'a> {
    webview: Option<&'a mut WebView<'static, ()>>,
}

impl View<'a> {
    new() {
        View { webview }
    }
}

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
            move |msg: Message| msg.as_text().map(|s| Protocol::handle(s, , |m, _| out.send(m).map_err(|_| "")))
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
            move |webview, arg, _| Protocol::handle(arg, &mut View { webview: Some(webview) }, |m, v| Protocol::eval(m, v)),
            ());
    }

    #[allow(non_snake_case)]
    fn handle<S>(msg: &str, webview: &mut View, send: S)
        where S: FnOnce(String, &mut View) -> Result<(), &'static str> {
        send(actions::process(msg, webview).unwrap(), webview).expect("error sending answer")
    }

    fn eval(s: String, view: &mut View) -> Result<(), &'static str> {
        match s.as_ref() {
            "open" => {
                view.webview.as_mut().map(|v| v.dialog(Dialog::OpenFile, "open", ""));
            }
            "info" => {
                view.webview.as_mut().map(|v| v.dialog(Dialog::Alert(Alert::Info), "test", "test"));
            }
            _ => {
                view.webview.as_mut().map(|v| v.eval(&format!("window.render({})", s));
            }
        }
        Ok(())
    }

    const HTML: &'static [u8] = include_bytes!("d.sz");
}
