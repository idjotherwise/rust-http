// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    // let mut buf = vec![];
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Got request");
                // let _ = stream.read_to_end(&mut buf);
                stream
                    .write(b"HTTP/1.1 200 OK\r\n\r\n")
                    .expect("Write error");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
