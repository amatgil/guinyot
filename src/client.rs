use guinyot::{
    serialization::{ClientStatement, TransferGame, TransferTag},
    *,
};
use std::error::Error;
use std::{io::stdin, net::TcpStream, process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{DEFAULT_PORT}"))?;

    let mut state = TransferGame {
        carta_atot: None,
        your_cards: vec![],
        table_card: None,
    };

    loop {
        println!("Waiting...");
        let m = receive_from(&mut stream)?;
        if m.len() == 0 {
            println!("Got empty, exiting");
            exit(0);
        }
        let tag = &m[0..1];
        let rest = &m[1..];
        match TransferTag::deserialize(tag) {
            Err(e) => println!("Error '{}', source: '{:?}", e, e.source()),
            Ok(TransferTag::Info) => println!("Info: {}", String::from_utf8_lossy(rest)),
            Ok(TransferTag::Response) => println!("Response: {}", String::from_utf8_lossy(rest)),
            Ok(TransferTag::Prompt) => {
                println!("Prompt: {}", String::from_utf8_lossy(rest));
                let input = 'input: loop {
                    let mut s = String::new();
                    stdin()
                        .read_line(&mut s)
                        .expect("Did not enter a correct string");
                    if let Ok(n) = s.trim().parse::<usize>() {
                        if n >= state.your_cards.len() {
                            println!(
                                "Index {n} és invalid (només tens {} cartes)",
                                state.your_cards.len()
                            );
                        }
                        println!("He llegit entrada: {n} ('{}')", state.your_cards[n]);
                        break 'input n;
                    }
                };
                send_to(
                    &mut stream,
                    TransferTag::Response,
                    &ClientStatement {
                        played_card: state.your_cards[input],
                    }
                    .serialize(),
                )?;
            }
            Ok(TransferTag::State) => {
                state = TransferGame::deserialize(rest)?;
                println!("STATE:\n\n{}\n\n", state);
            }
        }
    }
}
