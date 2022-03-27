fn main() {
    let server = Server::new("127.0.0.1:3131");
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
}
