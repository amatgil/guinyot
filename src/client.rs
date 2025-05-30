use guinyot::*;
use std::{io::Read, net::TcpStream};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let msg = b"Haiiiii";
    //stream.write(msg)?;
    let mut buffer = [0; 256];
    while stream.read(&mut buffer).is_ok() {
        println!("Thing: '{}'", String::from_utf8_lossy(&buffer[..]));
        buffer = [0; 256];
    }
    Ok(())
} // the stream closed here
