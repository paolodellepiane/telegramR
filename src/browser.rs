use crate::actions::*;
use crate::config::Config;
use crate::protocol::*;
use crate::u::*;
use lazy_static::lazy_static;
use regex::Regex;
use rouille::Response;
use std::error::Error;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{fs, thread};

//lazy_static! {
//    static ref RE: Regex = Regex::new("GET /(.*)HTTP/1.1").unwrap();
//}

pub struct MinimalServer {}
impl MinimalServer {
    pub fn listen() {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => Self::handle_connection(stream),
                Err(_) => { /* connection failed */ }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let RE = Regex::new(r"GET (/.*)HTTP/1.1").unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let req = String::from_utf8_lossy(&buffer);
        if let Some(uri) = RE.captures(&req).and_then(|x| x.get(1).map(|x| x.as_str())) {
            println!("GET {}", Self::base_url().join(uri).to_str().unwrap());
            let contents = fs::read_to_string(Self::base_url().join(uri)).unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
            stream.write(response.as_bytes()).unwrap();
        } else {
            stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
        }
        stream.flush().unwrap();
    }

    pub fn base_url<'a>() -> &'a Path {
        Path::new("./ui/dist")
    }
}

impl Protocol for View {
    fn init<T: Into<Config>>(_config: T) {
        print_current_dir().expect("can't get current directory");

        thread::spawn(|| {
            MinimalServer::listen();
        });

        thread::spawn(move || {
            rouille::start_server("127.0.0.1:1234", move |request| {
                let response = rouille::match_assets(&request, "./ui/dist");
                if response.is_success() {
                    return response;
                }
                Response::html("404 error").with_status_code(404)
            });
        });
        println!("listening on 127.0.0.1:1234");

        use ws::{listen, Message};
        listen("127.0.0.1:36767", |out| {
            move |msg: Message| {
                msg.as_text()
                    .map(|s| View::handle(s, &mut (), |m, _| out.send(m).map_err(Box::from)))
            }
        })
        .expect("Failed to create WebSocket");
    }

    fn handle<S>(msg: &str, bag: &mut (), send: S)
    where
        S: FnOnce(String, &mut ()) -> Result<(), Box<Error>>,
    {
        if let Err(err) = View::process(msg, bag).map(|res| send(res, bag)) {
            println!("error: {:?}", err);
        }
    }

    fn eval(_: String, _: &mut ()) -> Result<(), &'static str> {
        Err("eval error")
    }

    #[allow(non_camel_case_types, non_snake_case)]
    fn process(msg: &str, _: &mut ()) -> Result<String, Box<Error>> {
        use self::Action::*;
        println!("req: {}", msg);
        match serde_json::from_str(msg).unwrap() {
            getFile => Ok(serde_json::to_string(&("mock file path", LOREM)).unwrap()),
            info { text } => Err(text.into()),
        }
    }
}

const LOREM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
