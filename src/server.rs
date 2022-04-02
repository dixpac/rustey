use std::io::{Read, Write};
use std::convert::TryFrom;
use crate::http::{Request, Response, Status};
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut socket, _address)) => {
                    let mut buffer = [0; 1024];
                    match socket.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(Status::NotFound, None);
                                    response.send(&mut socket);
                                },
                                Err(e) => println!("Failed to parse a request: {}", e),
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