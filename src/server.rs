use guinyot::playing::Game;
use guinyot::serialization::{ClientStatement, TransferGame};
use guinyot::*;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let mut streams = listener.incoming();

    let mut stream_a = streams.next().expect("A failed")?;
    stream_a.write("Ets l'A (waiting for B)".as_bytes())?;

    let mut stream_b = streams.next().expect("B failed")?;
    stream_b.write("Ets el B".as_bytes())?;

    stream_a.write("Ets l'A (both connected)".as_bytes())?;
    stream_b.write("Ets el B (both connected)".as_bytes())?;

    let mut game = Game::new();

    while !game.is_over() {
        let (first_answer, second_answer) = match game.torn {
            Torn::A => handle_turn(&mut game, &mut stream_a, &mut stream_b)?,
            Torn::B => handle_turn(&mut game, &mut stream_b, &mut stream_a)?,
        };

        game.play_cards(first_answer, second_answer);

        game.torn = match game.torn {
            Torn::A => Torn::B,
            Torn::B => Torn::A,
        };
    }

    Ok(())
}

fn handle_turn(
    game: &mut Game,
    first: &mut TcpStream,
    second: &mut TcpStream,
) -> Result<(Carta, Carta), io::Error> {
    let mut buf = vec![];

    let first_answer = loop {
        let transfer_first = TransferGame {
            carta_atot: game.atot(),
            your_cards: game.a_hand.clone(),
            table_card: None,
        };
        first.write(&[STATE_TAG])?;
        first.write(&transfer_first.serialize())?;

        second.write(&[INFO_TAG])?;
        second.write("L'altre jugador està triant ".as_bytes())?;

        let first_answer = loop {
            first.read(&mut buf)?;
            if let Some(c) = ClientStatement::deserialize(&buf) {
                break c;
            }
        };
        let hand = match game.torn {
            Torn::A => &game.a_hand,
            Torn::B => &game.b_hand,
        };
        if hand.contains(&first_answer.played_card) {
            break first_answer.played_card;
        }
    };

    let second_answer = loop {
        let second_answer = get_answer_of(second, first, game, Some(first_answer))?;
        match game.torn {
            Torn::A => {
                if game.a_hand.contains(&second_answer) {
                    break second_answer;
                }
            }
            Torn::B => {
                if game.b_hand.contains(&second_answer) {
                    break second_answer;
                }
            }
        }
    };

    Ok((first_answer, second_answer))
}

fn get_answer_of(
    to_play: &mut TcpStream,
    not_playing: &mut TcpStream,
    game: &Game,
    previous_answer: Option<Carta>,
) -> Result<Carta, io::Error> {
    let mut buf = vec![];

    let contents = TransferGame {
        carta_atot: game.atot(),
        your_cards: game.b_hand.clone(),
        table_card: previous_answer,
    };

    to_play.write(&[STATE_TAG])?;
    to_play.write(&contents.serialize())?;

    not_playing.write(&[INFO_TAG])?;
    not_playing.write("L'altre jugador està triant ".as_bytes())?;

    let second_answer = loop {
        not_playing.read(&mut buf)?;
        if let Some(c) = ClientStatement::deserialize(&buf) {
            break c;
        }
    };
    Ok(second_answer.played_card)
}
