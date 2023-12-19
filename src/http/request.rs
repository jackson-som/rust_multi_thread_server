use std::str::from_utf8;

use super::{Protocol, RequestError, RequestMethod};
use super::Query;

pub struct Request<'a> {
    method: RequestMethod,
    path: &'a str,
    query: Option<Query<'a>>,
}

impl<'a> Request<'a> {
    pub fn method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
}

// Exp: GET /test?a=1&b=2 HTTP/1.1
impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = RequestError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let request = from_utf8(value)?;

        let (method, request) = read_request(request)?;
        let (mut path, request) = read_request(request)?;
        let (protocol, _) = read_request(request)?;

        let _protocol: Protocol = protocol.parse()?;
        let method: RequestMethod = method.parse()?;
        let mut query = None;

        if let Some(i) = path.find('?') {
            path = &path[..i];
            query = Some(Query::from(&path[i + 1..]));
        }

        Ok(Request {
            method,
            path,
            query,
        })
    }
}

fn read_request(buffer: &str) -> Result<(&str, &str), RequestError> {
    for (i, char) in buffer.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Ok((&buffer[..i], &buffer[i + 1..]));
        }
    }

    Err(RequestError::InvalidRequest)
}
