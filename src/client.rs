use guinyot::{
    serialization::{TransferGame, TransferTag},
    *,
};
use std::error::Error;
use std::{io::Read, net::TcpStream, process::exit};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;

    loop {
        let m = receive_from(&mut stream)?;
        if m.len() == 0 {
            println!("Got empty, exiting");
            exit(0);
        }
        match TransferTag::deserialize(&m[0..1]) {
            Err(e) => println!("Error '{}', source: '{:?}", e, e.source()),
            Ok(TransferTag::Info) => println!("Info: {}", String::from_utf8_lossy(&m)),
            Ok(TransferTag::Response) => println!("Response: {}", String::from_utf8_lossy(&m)),
            Ok(TransferTag::Prompt) => println!("Prompt: {}", String::from_utf8_lossy(&m)),
            Ok(TransferTag::State) => println!("STATE: {:?}", TransferGame::deserialize(&m[1..])),
        }
    }

    //while let Ok(n) = stream.read(&mut buffer) {
    //    if n == 0 {
    //        println!("Server closed, shutting down");
    //        exit(0);
    //    }
    //    println!("Received: '{}'", String::from_utf8_lossy(&buffer[..]));
    //    buffer = [0; BUFFER_LEN];
    //}
    //Ok(())
}
