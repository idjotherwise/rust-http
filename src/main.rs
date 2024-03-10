// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

enum Responses {
    Ok,
    NotFound,
}

impl Responses {
    fn as_str(&self) -> &str {
        match self {
            Responses::Ok => "HTTP/1.1 200 OK\r\n",
            Responses::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n",
        }
    }
}

enum ContentTypes {
    TextPlain,
}

impl ContentTypes {
    fn as_str(&self) -> &str {
        match self {
            ContentTypes::TextPlain => "Content-Type: text/plain\r\n",
        }
    }
}

fn to_content_length(n: usize) -> String {
    format!("Content-Length: {n}\r\n\r\n")
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
                buf_reader.read_line(&mut buf_writer).expect("Read error");
                let path = buf_writer.split(" ").skip(1).take(1).collect::<String>();
                let msg = path.split("/");
                let first_part = msg.clone().skip(1).take(1).collect::<String>();
                let second_part = msg.clone().skip(2).collect::<Vec<&str>>().join("/");
                // println!("{}", first_part);
                // println!("{}", second_part);

                let response = match first_part.as_str() {
                    "echo" => {
                        format!(
                            "{}{}{}{}\r\n",
                            Responses::Ok.as_str(),
                            ContentTypes::TextPlain.as_str(),
                            to_content_length(second_part.len()),
                            second_part.as_str()
                        )
                    }
                    "" => {
                        println!("got here");
                        format!("{}", Responses::Ok.as_str())
                    }
                    _ => {
                        format!("{}", Responses::NotFound.as_str())
                    }
                };
                conn.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
