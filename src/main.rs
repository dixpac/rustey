#![allow(dead_code)]

use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:3131".to_string());
    server.run();
}