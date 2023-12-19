use std::fmt::{Display, Formatter, Result};
use std::net::AddrParseError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum RequestError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidIpAddress,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use RequestError::*;

        match self {
            InvalidRequest => write!(f, "Invalid Request"),
            InvalidEncoding => write!(f, "Invalid Encoding"),
            InvalidProtocol => write!(f, "Invalid Protocol"),
            InvalidMethod => write!(f, "Invalid Request Method"),
            InvalidIpAddress => write!(f, "Invalid IP Address"),
        }
    }
}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<AddrParseError> for RequestError {
    fn from(_: AddrParseError) -> Self {
        Self::InvalidIpAddress
    }
}
