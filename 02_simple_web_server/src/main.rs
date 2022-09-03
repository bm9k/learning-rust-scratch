use std::net::TcpListener;

fn main() {
    // for prod, should handle error, but for this example, unwrap will panic if error occurs
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
