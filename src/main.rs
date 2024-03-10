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

enum ContentTypes {
    TextPlain,
}

impl ContentTypes {
    fn as_bytes(&self) -> &'static [u8] {
        match self {
            ContentTypes::TextPlain => b"Content-Type: text/plain\r\n",
        }
    }
}

fn to_content_length(n: usize) -> Vec<u8> {
    format!("Content-Length: {n}\r\n\r\n").into_bytes()
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
                let content_length = buf_reader.read_line(&mut buf_writer).expect("Read error");
                let path = buf_writer.split(" ").skip(1).take(1).collect::<String>();
                let msg = path.split("/");
                let first_part = msg.clone().skip(1).take(1).collect::<String>();
                let second_part = msg.clone().skip(2).collect::<Vec<&str>>().join("/");
                // println!("{}", first_part);
                // println!("{}", second_part);

                match first_part.as_str() {
                    "echo" => {
                        conn.write(Responses::Ok.as_bytes())
                            .expect("Response Status write error");
                        conn.write(ContentTypes::TextPlain.as_bytes())
                            .expect("Content-type write error");
                        conn.write(&to_content_length(content_length))
                            .expect("Content Length write error");
                        conn.write(second_part.as_bytes())
                            .expect("Body write error");
                    }
                    "" => {
                        conn.write(Responses::Ok.as_bytes())
                            .expect("Response Status write error");
                    }
                    _ => {
                        conn.write(Responses::NotFound.as_bytes())
                            .expect("Write error");
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
