use std::str;
use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};

pub struct Request {
    path: String, 
    query_string: Option<String>, 
    method: Method 
}

// This is how conversions are meant to happen in Rust
// Implementing try_from trait(interface)
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
    }

}

pub enum ParseError {

    InvalidRequest,
    InvalidEncoding, 
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid Method", 
            Self::InvalidEncoding => "Invalid Encoding", 
            Self::InvalidProtocol => "Invalid Protocol", 
            Self::InvalidRequest => "Invalid Request", 
        }
    }
}

impl From<Utf8Error> for ParseError {

    fn from(_: Utf8Error) -> Self {
       Self::InvalidEncoding 
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
        
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", self.message())
        
    }
}

impl Error for ParseError {}
