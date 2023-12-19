use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::OK => "OK",
            StatusCode::BadRequest => "BAD REQUEST",
            StatusCode::NotFound => "NOT FOUND"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", *self as u16)
    }
}