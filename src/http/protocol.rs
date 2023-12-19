use std::str::FromStr;

use super::RequestError;

#[derive(Debug)]
pub enum Protocol {
    HttpVersionOne,
    HttpVersionOnePointOne,
    HttpVersionTwo,
    HttpVersionThree,
}

impl FromStr for Protocol {
    type Err = RequestError;

    fn from_str(protocol: &str) -> Result<Self, Self::Err> {
        use Protocol::*;

        match protocol {
            "HTTP/1.0" => Ok(HttpVersionOne),
            "HTTP/1.1" => Ok(HttpVersionOnePointOne),
            "HTTP/2.0" => Ok(HttpVersionTwo),
            "HTTP/3.0" => Ok(HttpVersionThree),
            _ => Err(RequestError::InvalidProtocol)
        }
    }
}