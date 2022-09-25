# learn-rust
Just a small repo to start learning some rust

# Build an HTTP server 

* HTTP/1.1
* L7 protocol
* Sent over TCP
* Message based

## Testing

```
cargo run
Server running on 127.0.0.1 <== Started successfully

We can use netstat tool to send requests to our server
echo "GET /search?name=abc&sort=1 HTTP/1.1\r" | netcat 127.0.0.1 8080

Output
Server running on 127.0.0.1
Received a requst: GET /search?name=abc&sort=1 HTTP/1.1

HTTP Method is GET
Path is /search?name=abc&sort=1
Protocol is HTTP/1.1
```

All credits go to this course -> https://www.udemy.com/course/rust-fundamentals/

