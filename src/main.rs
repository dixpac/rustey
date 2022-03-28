fn main() {
    let server = Server::new("127.0.0.1:3131".to_string());
    server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self {
            address
        }
    }

    fn run(self) {
        println!("Listening on {}", self.address);
    }
}

struct Request {
    path: String,
    query_string: String,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
}
