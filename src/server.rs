use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr};
use std::io::prelude::*;
use std::fs;
use std::error::Error;

use crate::utils::{read_file};

pub enum State {
    Off, 
    Idle,
    Processing
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
        println!("Booting up at: {:?}", addr);
        let listener = 
            TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(self, stream);
        }
    }

    fn handle_connection(&mut self, mut stream:TcpStream) {
        self.state = State::Processing;
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer).unwrap();
    
        let binding = String::from_utf8_lossy(&buffer[..bytes_read]);
        let request_details: Vec<&str> = binding
            .lines()
            .collect();
    
        let request_type: Vec<&str> = request_details[0]
            .split('/')
            .collect(); 
    
        let request_file: Vec<&str> = request_type[1]
            .split(' ')
            .map(|s| s.trim())
            .collect();

        println!("{:?}", request_type);
        //display_connection(stream.local_addr())
    
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
        //println!("{:?}", response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        self.state = State::Idle;
    }

    fn display_connection(conn_ip: Result<IpAddr, Box<dyn Error>>, addr: Vec<&str>) {
        match conn_ip {
            Ok(ip) => println!("{:?} - ", ip),
            Err(_) => println!("Unknown - "),
        }
    }
}

pub struct ConnectionData<'a> {
    pub r#type: &'a str,
    pub file: &'a str,
    pub method: &'a str,
}