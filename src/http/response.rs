use super::Status;
use std::fmt::{Display, Formatter, Result};

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

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let body = match &self.body {
            Some(content) => content,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status,
            self.status.reason(),
            body,
        )
    }
}