mod utils;
mod server;

use local_ip_address::local_ip;
use server::{Server};

fn main() {
    let ip_raw = local_ip();
    let ip = match ip_raw {
        Ok(ip) => ip,
        Err(_) => panic!("Failed to load IP Address!"),
    };
    let port = Some(7878 as u16);

    let mut web_server = Server::new(ip, port);
    web_server.start();
}