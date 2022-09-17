use std::net::TcpListener;
use std::io::Read; // this is a trait similar to an interface
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct Server {
    address: String,
}

impl Server {

    pub fn new(address: String) -> Self {
        Self {
            address
        }
    } 

    pub fn run(self) {
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

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {

                                },
                                Err(e) => println!("Failed to parse a request: {}", e),

                            }
                            

                        },
                        Err(e) => println!("Failed to read from connection {}", e)
                    } 
                },
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }

        }

    }

}
