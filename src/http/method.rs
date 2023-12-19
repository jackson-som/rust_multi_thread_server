use std::str::FromStr;

use super::RequestError;

#[derive(Debug)]
pub enum RequestMethod {
    Post,
    Get,
    Update,
    Delete,
    Head,
    Connect,
    Patch,
    Trace,
    Options,
}

impl FromStr for RequestMethod {
    type Err = RequestError;

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        use RequestMethod::*;

        match method {
            "POST" => Ok(Post),
            "GET" => Ok(Get),
            "UPDATE" => Ok(Update),
            "DELETE" => Ok(Delete),
            "HEAD" => Ok(Head),
            "CONNECT" => Ok(Connect),
            "PATCH" => Ok(Patch),
            "TRACE" => Ok(Trace),
            "OPTIONS" => Ok(Options),
            _ => Err(RequestError::InvalidMethod),
        }
    }
}