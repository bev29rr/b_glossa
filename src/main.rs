mod utils;
mod server;
mod filesystem;
mod response;

use server::{Server};

fn main() {
    let mut web_server = Server::from_presets();
    web_server.start();
}