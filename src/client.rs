use guinyot::*;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let msg = b"Haiiiii";
    //stream.write(msg)?;
    let mut buffer = [0; 128];
    while stream.read(&mut buffer).is_ok() {
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
    Ok(())
} // the stream closed here
