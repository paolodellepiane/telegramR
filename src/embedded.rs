use crate::actions::*;
use crate::config::Config;
use crate::protocol::*;
use crate::splitter;
use std::error::Error;
use std::path::Path;
use web_view::*;

struct Embedded<'a> {
    view: &'a mut WebView<'a, ()>,
}

impl<'a> Bag for WebView<'a, ()> {}

impl<'a> Protocol<WebView<'a, ()>> for Embedded<'a> {
    fn init<C: Into<Config>>(_config: C) {
        let out_dir = splitter::unzip_to_tmp(HTML, "tr").expect("failed to expand view");
        WebViewBuilder::new()
            .title("tr")
            .content(Content::Url(out_dir.join("index.html").to_str().unwrap()))
            .size(900, 700)
            .resizable(true)
            .user_data(())
            .invoke_handler(move |&mut view: &mut WebView<_>, arg| {
                Ok(Embedded::handle(arg, &mut view, |m, v| Embedded::eval(m, v).map_err(Box::from)))
            })
            .build()
            .unwrap()
            .run()
            .unwrap();
    }

    fn eval(s: String, view: &mut WebView<'a, ()>) -> Result<(), &'static str> {
        view.eval(&format!("window.render({})", s)).map_err(|_| "eval error")
    }

    fn handle<S>(msg: &str, view: &mut WebView<'a, ()>, send: S)
    where
        S: FnOnce(String, &mut WebView<'a, ()>) -> Result<(), Box<Error>>,
    {
        if let Err(err) = Embedded::process(msg, view).map(|res| send(res, view)) {
            println!("error: {:?}", err);
        }
    }

    #[allow(non_camel_case_types, non_snake_case)]
    fn process(msg: &str, view: &mut WebView<'a, ()>) -> Result<String, Box<Error>> {
        use self::Action::*;
        println!("req: {}", msg);
        match serde_json::from_str(msg).unwrap() {
            getFile => Ok(serde_json::to_string(&("mock file path", LOREM)).unwrap()),
            info { text } => Err("info".into()),
        }
    }
}

const HTML: &'static [u8] = include_bytes!("d");
const LOREM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
