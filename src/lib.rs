pub mod playing;
pub mod serialization;

pub const DEFAULT_PORT: u32 = 25545;
pub const STATE_TAG: u8 = b'S';
pub const INFO_TAG: u8 = b'I';
pub const PROMPT_TAG: u8 = b'P';
pub const RESPONSE_TAG: u8 = b'R';

pub const CHECK_TAGS_ARE_DIFFERENT: () = {
    let tags = [STATE_TAG, INFO_TAG, PROMPT_TAG, RESPONSE_TAG];
    let mut t1 = 0;
    while t1 < tags.len() {
        let mut t2 = 0;
        while t2 < tags.len() {
            if t1 == t2 {
                t2 += 1;
                continue;
            }
            assert!(tags[t1] != tags[t2]);
            t2 += 1;
        }
        t1 += 1;
    }
};

#[derive(Debug, Clone, Copy)]
pub struct Carta {
    pub coll: Coll,
    pub valor: Numero,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Coll {
    Monedes,
    Copes,
    Espases,
    Garrots,
}

#[derive(Clone, Copy, Debug)]
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
