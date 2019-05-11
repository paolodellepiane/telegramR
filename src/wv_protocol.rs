use crate::actions::*;
use crate::config::Config;
use crate::protocol::*;
use crate::splitter;
use std::error::Error;
use std::path::Path;
use web_view::*;

impl Protocol for Engine {
    fn init<T: Into<Config>>(_config: T) {
        let out_dir = splitter::unzip_to_tmp(Protocol::HTML, "tr").expect("failed to expand view");
        WebViewBuilder::new()
            .title("tr")
            .content(Content::Url(out_dir.join("index.html").to_str().unwrap()))
            .size(900, 700)
            .resizable(true)
            .user_data(())
            .invoke_handler(move |webview, arg| {
                Ok(Protocol::handle(arg, &mut View { view: webview }, |m, v| {
                    Protocol::eval(m, v).map_err(Box::from)
                }))
            })
            .build()
            .unwrap()
            .run()
            .unwrap();
    }

    fn handle<S>(msg: &str, view: &mut View<&mut WebView<()>>, send: S)
    where
        S: FnOnce(String, &mut View<&mut WebView<()>>) -> Result<(), Box<Error>>,
    {
        if let Err(err) = Self::process(msg, view).map(|res| send(res, view)) {
            println!("error: {:?}", err);
        }
    }

    fn eval(s: String, view: &mut View<&mut WebView<()>>) -> Result<(), &'static str> {
        view.view
            .as_mut()
            .map(|v| {
                v.eval(&format!("window.render({})", s)).unwrap();
            })
            .ok_or("eval error")
    }

    #[allow(non_camel_case_types, non_snake_case)]
    fn process(msg: &str, view: &mut View<&mut WebView<()>>) -> Result<String, Box<Error>> {
        use self::Action::*;
        println!("req: {}", msg);
        match serde_json::from_str(msg).unwrap() {
            getFile => Ok(serde_json::to_string(&("mock file path", LOREM)).unwrap()),
            info { text } => Box::from("info"),
        }
    }
}

const HTML: &'static [u8] = include_bytes!("d");
const LOREM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
