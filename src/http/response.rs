use super::Status;

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

}