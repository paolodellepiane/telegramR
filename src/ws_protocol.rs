use crate::actions::*;
use crate::config::Config;
use crate::protocol::*;
use std::error::Error;

struct WsView {}
impl View for WsView {}

impl Protocol<WsView> for Engine {
    fn init<T: Into<Config>>(_config: T) {
        use ws::{listen, Message};
        listen("127.0.0.1:36767", |out| {
            move |msg: Message| {
                msg.as_text()
                    .map(|s| Engine::handle(s, &mut WsView {}, |m, _| out.send(m).map_err(Box::from)))
            }
        })
        .expect("Failed to create WebSocket");
    }

    fn handle<S>(msg: &str, view: &mut WsView, send: S)
    where
        S: FnOnce(String, &mut WsView) -> Result<(), Box<Error>>,
    {
        if let Err(err) = Engine::process(msg, view).map(|res| send(res, view)) {
            println!("error: {:?}", err);
        }
    }

    fn eval(_: String, _: &mut WsView) -> Result<(), &'static str> {
        Err("eval error")
    }

    #[allow(non_camel_case_types, non_snake_case)]
    fn process(msg: &str, _: &mut WsView) -> Result<String, Box<Error>> {
        use self::Action::*;
        println!("req: {}", msg);
        match serde_json::from_str(msg).unwrap() {
            getFile => Ok(serde_json::to_string(&("mock file path", LOREM)).unwrap()),
            info { text } => Err(text.into()),
        }
    }
}

const LOREM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
