extern crate serde_json;
extern crate snap;

use actions;
use std::io::prelude::*;
use web_view::*;

pub struct Protocol;

// impl<'a> View<'a> {
//     fn new() -> View<'a> { View { webview: None::<&mut WebView<'a, ()>>, } }
//     fn with_webview(&'a mut self, webview: &'a mut WebView<'a, ()>) -> &'a mut View<'a> {
//         self.webview = Some(webview);
//         self
//     }
// }

pub struct View<'a, 'b: 'a> {
    pub webview: Option<&'a mut WebView<'b, ()>>,
}

impl<'a, 'b> View<'a, 'b> {
    pub fn new() -> View { View { webview: None::<&mut WebView<'_, ()>> } }
    pub fn with_webview(webview: &'a mut WebView<'b, ()>) -> View { View { webview: Some(webview) } }
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
            move |msg: Message| msg.as_text().map(|s| Protocol::handle(s, &mut View::new(), |m, _| out.send(m).map_err(|_| "")))
        }).expect("Failed to create WebSocket")
    }

    fn h<'a, 'b>(webview: &'a mut WebView<'b, ()>, arg: &str, _: &mut ()) {
        let mut view = View { webview: Some(webview) };
        Protocol::handle(arg, &mut view, |m, v| Protocol::eval(m, v))
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
            Protocol::h,
            ());
    }

    #[allow(non_snake_case)]
    fn handle<S>(msg: &str, view: &mut View, send: S)
        where S: FnOnce(String, &mut View) -> Result<(), &'static str> {
        send(actions::process(msg, view).unwrap(), view).expect("error sending answer")
    }

    fn eval(s: String, view: &mut View) -> Result<(), &'static str> {
                view.webview.as_mut().map(|v| v.eval(&format!("window.render({})", s)));
        Ok(())
    }

    const HTML: &'static [u8] = include_bytes!("d.sz");
}
