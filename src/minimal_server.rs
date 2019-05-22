use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt, fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    path::Path,
};

lazy_static! {
    static ref RE: Regex = Regex::new("GET /(.*)HTTP/1.1").unwrap();
}

trait HttpMessenger {
    fn not_found(&mut self);
    fn static_file<P: AsRef<Path>>(&mut self, path: P);
}

impl HttpMessenger for TcpStream {
    fn not_found(&mut self) { self.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap(); }
    fn static_file<P: AsRef<Path>>(&mut self, uri: P) {
        if let Ok(contents) = fs::read_to_string(uri) {
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
            self.write(response.as_bytes()).unwrap();
        } else {
            self.not_found();
        }
    }
}

pub struct MinimalServer {}
impl MinimalServer {
    pub fn listen<A: ToSocketAddrs + fmt::Debug, P: AsRef<Path>>(addr: A, base_uri: P) {
        println!("listening on {:?} - base_uri \"{}\"", addr, base_uri.as_ref().display());
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => Self::handle_connection(stream, base_uri.as_ref()),
                Err(e) => println!("connection failed: {}", e),
            }
        }
    }

    fn handle_connection<P: AsRef<Path>>(mut stream: TcpStream, base_uri: P) {
        let base_uri = base_uri.as_ref();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let req = String::from_utf8_lossy(&buffer);
        if let Some(mut uri) = RE.captures(&req).and_then(|x| x.get(1).map(|x| x.as_str().trim())) {
            if uri.is_empty() {
                uri = "index.html";
            }
            let uri = base_uri.join(uri);
            println!("GET {}", uri.to_string_lossy());
            stream.static_file(uri);
        } else {
            stream.not_found();
        }
        stream.flush().unwrap();
    }
}
