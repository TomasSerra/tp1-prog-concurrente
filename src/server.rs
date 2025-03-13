use std::io::{Read, Write};
use std::net::TcpListener;
use crate::router::handle_request;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not start server");

    println!("Server running on http://localhost:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();

                let request = std::str::from_utf8(&buffer).unwrap();
                let response = handle_request(request);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }
}