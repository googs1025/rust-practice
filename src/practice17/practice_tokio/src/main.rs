use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let server = TcpListener::bind("0.0.0.0:8888").unwrap();
    // let stream = server.accept().unwrap();
    // handler(stream.0);
    for s in server.incoming() {
        let stream = s.unwrap();
        handler(stream);
    }
}
fn handler(mut stream: TcpStream) {
    let rsp = String::from("http/1.1 200 Ok\r\n\r\n rust server");
    stream.write(rsp.as_bytes()).unwrap();
}