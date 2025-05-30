use guinyot::playing::Game;
use guinyot::serialization::{ClientStatement, TransferGame};
use guinyot::*;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let mut streams = listener.incoming();

    let mut stream_a = streams.next().expect("A failed")?;
    stream_a.write_all("Ets l'A (waiting for B)".as_bytes())?;

    let mut stream_b = streams.next().expect("B failed")?;
    stream_b.write_all("Ets el B".as_bytes())?;

    stream_a.write_all("Ets l'A (both connected)".as_bytes())?;
    stream_b.write_all("Ets el B (both connected)".as_bytes())?;

    let game = Game::new();

    while !game.is_over() {
        match game.torn {
            Torn::A => handle_turn(&game, &mut stream_a, &mut stream_b)?,
            Torn::B => handle_turn(&game, &mut stream_b, &mut stream_a)?,
        }
    }

    Ok(())
}

fn handle_turn(
    game: &Game,
    first: &mut TcpStream,
    second: &mut TcpStream,
) -> Result<(), io::Error> {
    let mut buf = vec![];

    let transfer_first = TransferGame {
        carta_atot: game.atot(),
        your_cards: game.a_cards.clone(),
        table_card: None,
    };

    first.write_all(&[STATE_TAG])?;
    first.write_all(&transfer_first.serialize())?;

    second.write_all(&[INFO_TAG])?;
    second.write_all("L'altre jugador està triant ".as_bytes())?;

    let first_answer = loop {
        first.read_exact(&mut buf)?;
        if let Some(c) = ClientStatement::deserialize(&buf) {
            break c;
        }
    };

    Ok(())
}
