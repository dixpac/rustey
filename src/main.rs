#![allow(dead_code)]

use server::Server;
use web_handler::WebHandler;

mod http;
mod server;
mod web_handler;

fn main() {
    let server = Server::new("127.0.0.1:3131".to_string());
    server.run(WebHandler);
}