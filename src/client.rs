use multi_guinyot::*;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let msg = b"Hello, my name is client!";
    stream.write(msg)?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream closed here
