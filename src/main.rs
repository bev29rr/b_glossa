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
    let bytes_read = stream.read(&mut buffer).unwrap();

    let binding = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_details: Vec<&str> = binding
        .lines()
        .collect();

    let mut request_type: Vec<&str> = request_details[0]
        .split('/')
        .collect(); 

    let request_file: Vec<&str> = request_type[1]
        .split(' ')
        .map(|s| s.trim())
        .collect();

    let get = b"GET / HTTP/1.1\r\n";

    let mut response: String = String::new();
    let mut status_line: &str = "";
    let mut contents = String::new();

    if buffer.starts_with(b"GET") {
        if request_file[1] == "HTTP" {
            if request_file[0] == "" {
                status_line = "HTTP/1.1 200 OK";
                contents = read_file(String::from("index.html"));
            }
        }
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = read_file(String::from("index.html"));
    }
    response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    println!("{:?}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}