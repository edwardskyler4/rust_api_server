use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not bind to address");

    println!("Server listening on http://127.0.0.1:7878");

    loop {
        let (mut stream, address) = listener.accept().expect("Could not accept connection");

        println!("Connection received from {address}");

        let mut buffer = [0; 1024];

        let bytes_read = stream.read(&mut buffer).expect("Could not read request");

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        println!("Request received: {request}");

        let body = "Hello from my Rust server!";

        let response = format!(
            "HTTP/1.1 200 OK\r\n\
Content-Type: text/plain\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
\r\n\
{}",
            body.len(),
            body
        );

        stream
            .write_all(response.as_bytes())
            .expect("Could not write response");
    }
}
