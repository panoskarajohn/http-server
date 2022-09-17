use super::method::Method;
use std::convert::TryFrom;

pub struct Request {
    path: String, 
    query_string: Option<String>, 
    method: Method 
}


impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {

    }
}
// This is how conversions are meant to happen in Rust
// Implementing try_from trait(interface)
impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        
    }

}
