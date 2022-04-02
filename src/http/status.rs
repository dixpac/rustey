use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone, Debug)]
pub enum Status {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl Status {
    pub fn reason(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u16)
    }
}