use std::net::TcpListener;
use std::io::{Read, Write}; // this is a trait similar to an interface
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::convert::TryInto;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String,
}

impl Server {

    pub fn new(address: String) -> Self {
        Self {
            address
        }
    } 

    pub fn run(self, mut handler: impl Handler) {
        println!("Server running on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                
                Ok((mut stream, _)) => {

                    // if this was a production server we would have to be smarter
                    // than this, but at the moment this will be more than enough
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {

                            println!("Received a requst: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection {}", e)
                    };
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }

        }

    }

}
