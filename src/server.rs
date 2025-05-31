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
    let first_ma = match game.torn {
        Torn::A => &game.a_hand,
        Torn::B => &game.b_hand,
    };
    let second_ma = match game.torn {
        Torn::A => &game.b_hand,
        Torn::B => &game.a_hand,
    };

    let first_answer = loop {
        let first_answer = dbg!(get_answer_of(first, second, game, first_ma.clone(), None)?);
        if !first_ma.contains(&first_answer) {
            break first_answer;
        } else {
            eprintln!("(F) Card '{first_answer}' is not contained in that player's deck");
        }
    };
    dbg!("Time for P2");

    let second_answer = loop {
        let second_answer = dbg!(get_answer_of(
            second,
            first,
            game,
            second_ma.clone(),
            Some(first_answer)
        )?);
        if !second_ma.contains(&second_answer) {
            break second_answer;
        } else {
            eprintln!("(S) Card '{second_answer}' is not contained in that player's deck");
        }
    };

    Ok((first_answer, second_answer))
}

fn get_answer_of(
    to_play: &mut TcpStream,
    not_playing: &mut TcpStream,
    game: &Game,
    your_cards: Vec<Carta>,
    previous_answer: Option<Carta>,
) -> Result<Carta, io::Error> {
    let contents = TransferGame {
        carta_atot: game.atot(),
        your_cards,
        table_card: previous_answer,
    };
    dbg!(&contents);

    send_to(not_playing, Tag::Info, "L'altre està triant".as_bytes())?;

    dbg!("Waiting for answer");
    send_to(to_play, Tag::State, &contents.serialize())?;
    send_to(to_play, Tag::Prompt, "Et toca:".as_bytes())?;
    let answer = loop {
        let msg = dbg!(receive_from(to_play)?);
        let d = ClientStatement::deserialize(&msg);
        match d {
            Ok(c) => break c,
            Err(e) => println!("Got response, but it was invalid ({e})"),
        }
        dbg!("invalid");
        send_to(to_play, Tag::Info, "Card invàlida, reintenta-ho".as_bytes())?;
        send_to(to_play, Tag::State, &contents.serialize())?;
        send_to(to_play, Tag::Prompt, "Et toca:".as_bytes())?;
    };
    Ok(answer.played_card)
}
