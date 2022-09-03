use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
    // for prod, should handle error, but for this example, unwrap will panic if error occurs
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream) {
    // [x; n] array of length n, filled with x
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("index.html").unwrap();

    // Response format:
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // e.g.: HTTP/1.1 200 OK\r\n\r\n

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}