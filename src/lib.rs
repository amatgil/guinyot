use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

use serialization::TransferTag;

pub mod playing;
pub mod serialization;

pub const DEFAULT_PORT: u32 = 25545;

#[test]
pub fn check_tags_are_different() {
    use TransferTag as T;
    let tags = [T::State, T::Info, T::Prompt, T::Response];
    let mut t1 = 0;
    while t1 < tags.len() {
        let mut t2 = 0;
        while t2 < tags.len() {
            if t1 == t2 {
                t2 += 1;
                continue;
            }
            assert!(u8::from(tags[t1]) != u8::from(tags[t2]));
            t2 += 1;
        }
        t1 += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Carta {
    pub coll: Coll,
    pub valor: Numero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Coll {
    Monedes,
    Copes,
    Espases,
    Garrots,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Numero {
    As,
    Dos,
    Tres,
    Quatre,
    Cinc,
    Sis,
    Set,
    Sota,
    Cavall,
    Rei,
}

impl Numero {
    pub const ALL: [Self; 10] = [
        Self::As,
        Self::Dos,
        Self::Tres,
        Self::Quatre,
        Self::Cinc,
        Self::Sis,
        Self::Set,
        Self::Sota,
        Self::Cavall,
        Self::Rei,
    ];

    pub fn value(&self) -> u32 {
        match self {
            Numero::As => 11,
            Numero::Tres => 10,
            Numero::Rei => 4,
            Numero::Cavall => 2,
            Numero::Sota => 3,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Torn {
    A,
    B,
}

fn shuffle<T>(mut x: Vec<T>) -> Vec<T> {
    use rand::prelude::*;
    let mut rng = rand::rng();
    for i in 0..x.len() {
        let j = rng.random_range(0..x.len());
        if i == j {
            continue;
        }
        x.swap(i, j);
    }

    x
}

// Communications area

const END_OF_COMM: u8 = 255;

/// Reads until newline
pub fn receive_from(s: &mut TcpStream) -> Result<Vec<u8>, io::Error> {
    let mut reader = BufReader::new(s);

    let mut received: Vec<u8> = vec![];
    let _n = reader.read_until(END_OF_COMM, &mut received)?;
    if let Some(END_OF_COMM) = received.pop() {
        Ok(received)
    } else {
        Ok(vec![])
    }
}

pub fn send_to(stream: &mut TcpStream, tag: TransferTag, b: &[u8]) -> Result<(), io::Error> {
    //dbg!(tag, b);
    stream.write_all(&[u8::from(tag)])?;
    stream.write_all(b)?;
    stream.write_all(&[END_OF_COMM])?;
    stream.flush()?;

    Ok(())
}
