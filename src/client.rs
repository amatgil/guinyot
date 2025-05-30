use guinyot::*;
use std::{io::Read, net::TcpStream, process::exit};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;

    let mut buffer = vec![];
    while let Ok(n) = stream.read(&mut buffer) {
        dbg!(n);
        //if n == 0 {
        //    println!("Server closed, shutting down");
        //    exit(0);
        //}
        println!("Thing: '{}'", String::from_utf8_lossy(&buffer[..]));
        buffer = vec![];
    }
    Ok(())
} // the stream closed here
