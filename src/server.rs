use multi_guinyot::*;
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    stream.write("haiiiiii".as_bytes()).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

pub fn create_listener(addr: String) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    Ok(())
}

fn main() {
    create_listener(format!("127.0.0.1:{DEFAULT_PORT}")).unwrap()
}
