use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    // for prod, should handle error, but for this example, unwrap will panic if error occurs
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // [x; n] array of length n, filled with x
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_code, reason, body_file) = if buffer.starts_with(get) {
        (200, "OK", "index.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        (200, "OK", "index.html")
    } else {
        (404, "NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(body_file).unwrap();

    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        reason,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
