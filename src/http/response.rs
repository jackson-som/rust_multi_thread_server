use std::io::{Result as IoResult, Write};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {
            status_code,
            body,
        }
    }

    pub fn ok(body: Option<String>) -> Self {
        Response {
            status_code: StatusCode::OK,
            body,
        }
    }

    pub fn not_found(body: Option<String>) -> Self {
        Response {
            status_code: StatusCode::NotFound,
            body,
        }
    }

    // HTTP/1.1 200 OK <body/html>
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(v) => v,
            None => ""
        };
        let response = format!(
            "HTTP/1.1 {} {} \r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body
        );

        stream.write_all(response.as_bytes())?;
        stream.flush()
    }
}