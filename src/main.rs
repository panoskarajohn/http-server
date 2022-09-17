fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..]; // this is dangerous because these indices mean give from the
                                      // 10 bytes untili the end
    let string_borrow: &str = &string;
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
    
    //let server = Server::new();
    //server.run();
}

struct Server {
    address: String,
}

impl Server {

   fn new(address: String)-> Self{
       Self {
            address
       }
   } 

   fn run(self) {
        

   }

}
