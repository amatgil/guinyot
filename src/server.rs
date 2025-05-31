use guinyot::playing::Game;
use guinyot::serialization::{ClientStatement, TransferGame};
use guinyot::*;
use std::io;
use std::net::{TcpListener, TcpStream};

use guinyot::serialization::TransferTag as Tag;

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind(format!("127.0.0.1:{DEFAULT_PORT}"))?;
    let mut streams = listener.incoming();

    let mut stream_a = streams.next().expect("A failed")?;
    send_to(&mut stream_a, Tag::Info, "Ets l'A (waiting: B)".as_bytes())?;
    dbg!('A');

    let mut stream_b = streams.next().expect("B failed")?;
    send_to(&mut stream_b, Tag::Info, "Ets el B".as_bytes())?;
    dbg!('B');

    send_to(&mut stream_a, Tag::Info, "Ets l'A (both up)".as_bytes())?;
    send_to(&mut stream_b, Tag::Info, "Ets el B (both up)".as_bytes())?;

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
    send_to(second, Tag::Info, "L'altre jugador està triant ".as_bytes())?;
    let first_answer = loop {
        let first_answer = get_answer_of(first, second, game, None)?;
        match game.torn {
            Torn::A => {
                if game.a_hand.contains(&first_answer) {
                    break first_answer;
                }
            }
            Torn::B => {
                if game.b_hand.contains(&first_answer) {
                    break first_answer;
                }
            }
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
    let contents = TransferGame {
        carta_atot: game.atot(),
        your_cards: game.b_hand.clone(),
        table_card: previous_answer,
    };
    dbg!(&contents);

    send_to(to_play, Tag::State, &contents.serialize())?;
    send_to(
        not_playing,
        Tag::Info,
        "L'altre jugador està triant ".as_bytes(),
    )?;

    let answer = loop {
        let msg = receive_from(not_playing)?;
        if let Ok(c) = ClientStatement::deserialize(&msg) {
            break c;
        }
    };
    Ok(answer.played_card)
}
