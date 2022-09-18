use std::str;
use std::str::Utf8Error;
use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str, 
    query_string: Option<QueryString<'buf>>, 
    method: Method 
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// This is how conversions are meant to happen in Rust
// Implementing try_from trait(interface)
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidMethod)?;
        println!("HTTP Method is {}", method);
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        println!("Path is {}", path);
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidProtocol)?;
        println!("Protocol is {}", protocol);

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        
        let q = path.find('?');

        if let Some(i) = path.find('?') {

            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path, 
            query_string, 
            method
        })

    }

}

fn get_next_word(request: &str) -> Option<(&str ,&str)> {

    for (i,c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // Here we can add 1 (1 byte), noting
            // that we need to be careful
            return Some((&request[..i], &request[i + 1..]))
        }
    }

    None
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

impl From<MethodError> for ParseError {

    fn from(_: MethodError) -> Self {
       Self::InvalidMethod
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
