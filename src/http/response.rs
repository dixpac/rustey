use super::Status;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Option<String>,
}

impl Response {
    pub fn new(status: Status, body: Option<String>) -> Self {
        Self {
            status,
            body,
        }
    }

    pub fn send(&self, socket: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(content) => content,
            None => "",
        };

        write!(
            socket,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.reason(),
            body,
        )
    }
}
