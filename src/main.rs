// Uncomment this block to pass the first stage
use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
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
    OctetStream,
}

impl ContentTypes {
    fn as_str(&self) -> &str {
        match self {
            ContentTypes::TextPlain => "Content-Type: text/plain\r\n",
            ContentTypes::OctetStream => "Content-Type: application/octet-stream\r\n",
        }
    }
}

fn to_content_length(n: usize) -> String {
    format!("Content-Length: {n}\r\n\r\n")
}

fn take_line(reader: &mut BufReader<&mut TcpStream>) -> String {
    let mut buf_writer = String::new();
    reader
        .read_line(&mut buf_writer)
        .expect("Read next line error");
    buf_writer
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // let dir = args[0];

    for stream in listener.incoming() {
        std::thread::spawn(|| match stream {
            Ok(mut conn) => {
                let cli = Cli::parse();
                let mut buf_reader = BufReader::new(&mut conn);
                let mut buf_writer: String = String::new();
                buf_reader.read_line(&mut buf_writer).expect("Read error");
                let path = buf_writer.split(" ").skip(1).take(1).collect::<String>();
                let msg = path.split("/");
                let first_part = msg.clone().skip(1).take(1).collect::<String>();
                let second_part = msg.clone().skip(2).collect::<Vec<&str>>().join("/");
                take_line(&mut buf_reader);
                let mut user_agent = String::new();
                let next_line = take_line(&mut buf_reader);

                let mut header = next_line.split(':');
                match header.next().unwrap() {
                    "User-Agent" => user_agent = header.next().unwrap().trim().to_owned(),
                    _ => {}
                }

                let response = match first_part.as_str() {
                    "echo" => {
                        format!(
                            "{}{}{}{}\r\n\r\n",
                            Responses::Ok.as_str(),
                            ContentTypes::TextPlain.as_str(),
                            to_content_length(second_part.len()),
                            second_part.as_str()
                        )
                    }
                    "user-agent" => {
                        format!(
                            "{}{}{}{}\r\n\r\n",
                            Responses::Ok.as_str(),
                            ContentTypes::TextPlain.as_str(),
                            to_content_length(user_agent.len()),
                            user_agent
                        )
                    }
                    "files" => {
                        if let Some(contents) = fs::read_to_string(format!(
                            "{}/{}",
                            cli.directory.unwrap(),
                            second_part
                        ))
                        .ok()
                        {
                            format!(
                                "{}{}{}{}\r\n\r\n",
                                Responses::Ok.as_str(),
                                ContentTypes::OctetStream.as_str(),
                                to_content_length(contents.len()),
                                contents
                            )
                        } else {
                            format!("{}\r\n", Responses::NotFound.as_str(),)
                        }
                    }
                    "" => {
                        format!("{}\r\n", Responses::Ok.as_str())
                    }
                    _ => {
                        format!("{}\r\n", Responses::NotFound.as_str())
                    }
                };
                conn.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
    }
}
