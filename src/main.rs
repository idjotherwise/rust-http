// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
};

enum Responses {
    Ok,
    NotFound,
}

impl Responses {
    fn as_bytes(&self) -> &'static [u8] {
        match self {
            Responses::Ok => b"HTTP/1.1 200 OK\r\n\r\n",
            Responses::NotFound => b"HTTP/1.1 404 Not Found\r\n\r\n",
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut conn) => {
                let mut buf_reader = BufReader::new(&mut conn);
                let mut buf_writer: String = String::new();
                println!("Got request");
                buf_reader.read_line(&mut buf_writer).expect("Read error");
                let path = buf_writer.split(" ").skip(1).take(1).collect::<String>();
                println!("{}", path);

                match path.as_str() {
                    "/" => conn.write(Responses::Ok.as_bytes()).expect("Write error"),
                    _ => conn
                        .write(Responses::NotFound.as_bytes())
                        .expect("Write error"),
                };
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
