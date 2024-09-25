mod utils;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use utils::{read_file};
use std::fs;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";
    let response: String;
    let status_line: &str;

    if buffer.starts_wtih(get) {
        let contents = read_file(String::from("index.html"));
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = read_file(String::from("index.html"));
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    }
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}