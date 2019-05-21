use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::path::Path;

lazy_static! {
    static ref RE: Regex = Regex::new("GET /(.*)HTTP/1.1").unwrap();
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
            let uri = uri.to_str().unwrap();
            println!("GET {}", uri);
            if let Ok(contents) = fs::read_to_string(uri) {
                let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
                stream.write(response.as_bytes()).unwrap();
            } else {
                stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
            }
        } else {
            stream.write("HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes()).unwrap();
        }
        stream.flush().unwrap();
    }
}
