use super::http::{Request, Response, Status};
use super::server::Handler;

pub struct WebHandler;

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(Status::Ok, Some("<h1>Hello</h1>".to_string()))
    }
}