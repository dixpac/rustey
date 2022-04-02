use std::io::{Read, Write};
use std::convert::TryFrom;
use crate::http::{Request, Response, Status, ParseError};
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(Status::BadRequest, None)
    }


}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut socket, _address)) => {
                    let mut buffer = [0; 1024];
                    match socket.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            
                            if let Err(e) = response.send(&mut socket) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection {}", e),
                    }
                },
                Err(e) => println!("Failed to establish the coonection {}", e),
            }

        }
    }
}