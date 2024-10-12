use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use std::fs;

use crate::filesystem::{FileSystem};

pub enum State {
    Off, 
    Idle,
    Processing
}

#[derive(Debug)]
pub struct Response {
    pub status_line: String,
    pub contents: String,
    pub response_data: String
}

impl Response {
    pub fn new() -> Self {
        let empty_string = String::new();
        Self {
            status_line: empty_string.clone(),
            contents: empty_string.clone(),
            response_data: empty_string
        }
    }

    pub fn format_file(&mut self, string_path: String) {
        self.status_line = String::from("HTTP/1.1 200 OK");
        match FileSystem::get_template(string_path) {
            Some(contents) => {
                self.contents = contents;
                self.response_data = Self::format_response(self);
            },
            None => { 
                println!("Call 1");
                self.response_data = Self::format_error(self);
            }
        };
    }

    pub fn format_error(&mut self) -> String {
        self.status_line = String::from("HTTP/1.1 404 NOT FOUND");
        self.contents = String::from("NOT FOUND");
        Self::format_response(&self)
    }

    fn format_response(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            self.contents.len(),
            self.contents
        )
    }
}

pub struct Server {
    pub ip: IpAddr,
    pub port: u16,
    pub state: State,
}

impl Server {
    pub fn new(ip: IpAddr, port_raw: Option<u16>) -> Self {
        let port = match port_raw {
            Some(num) => num,
            None => 7878
        };
        Self {
            ip,
            port,
            state: State::Off
        }
    }

    pub fn start(&mut self) {
        let addr = SocketAddr::new(self.ip, self.port);
        self.state = State::Idle;
        println!("Booting up at: \x1b]8;;http://{:?}\x1b\\{:?}\x1b]8;;\x1b\\", addr, addr);
        let listener = 
            TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(self, stream);
        }
    }

    fn handle_connection(&mut self, mut stream:TcpStream) {
        self.state = State::Processing;

        let connection_info = Self::get_connection_info(&mut stream);
        Self::display_connection(&connection_info);

        let mut response = Response::new();

        match connection_info {
            Some(conn_info) => {
                if conn_info.r#type == "GET".to_string() {
                    if conn_info.method == "HTTP" {
                        if conn_info.file.as_str() == "" {
                            response.format_file(
                                String::from("index.html")
                            );
                        }
                        else {
                            response.format_file(
                                conn_info.file
                            );
                        }
                    }
                } else {
                    response.format_error();
                    println!("Call 2");
                }
            },
            None => {
                response.format_error();
                println!("Call 3");
            }
        }
        stream.write(response.response_data.as_bytes()).unwrap();
        stream.flush().unwrap();
        self.state = State::Idle;
    }

    fn get_connection_info(stream: &mut TcpStream) -> Option<ConnectionData> {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
    
        let binding = 
            String::from_utf8_lossy(&buffer[..bytes_read])
                .to_string();
        let mut request_details: Vec<String> = binding
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        if request_details.len() <= 0 {
            return None;
        }

        let mut request_type: Vec<String> = request_details[0]
            .split('/')
            .map(|s| s.to_string())
            .collect(); 
        
        if request_type.len() <= 0 {
            return None;
        }

        let request_file: Vec<String> = request_type[1]
            .split(' ')
            .map(|s| s.trim().to_string())
            .collect();
    
        let this_ip = match stream.local_addr() {
            Ok(ip) => Some(ip),
            Err(_) => None
        };

        let connection_info = ConnectionData {
            r#type: request_type[0].trim().clone().to_string(),
            file: request_file[0].trim().clone().to_string(),
            method: request_file[1].trim().clone().to_string(),
            conn_ip: this_ip
        };

        Some(connection_info)
    }

    fn display_connection(connection_info: &Option<ConnectionData>) {
        match connection_info {
            Some(conn_info) => {
                println!("{:?}", conn_info.file);
                let addr = match &conn_info.file.as_str() {
                    &"" => String::from("/"),
                    _ => {conn_info.file.clone()}
                };
                match conn_info.conn_ip {
                    Some(ip) => println!("{:?} - {}", ip, addr),
                    None => println!("Unknown - {}", addr),
                }
            },
            None => println!("Unknown connection"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionData {
    pub r#type: String,
    pub file: String,
    pub method: String,
    pub conn_ip: Option<SocketAddr>
}