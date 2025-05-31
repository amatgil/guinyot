use std::mem;
use thiserror::Error;

use crate::*;

/// Information sent to each player
/// Only the information that's vital to be sent
/// Server -> Client
#[derive(Debug, Clone)]
pub struct TransferGame {
    pub carta_atot: Option<Carta>,
    pub your_cards: Vec<Carta>,

    /// The card the other player played
    pub table_card: Option<Carta>,
}

/// Client -> Server
#[derive(Debug, Clone)]
pub struct ClientStatement {
    pub played_card: Carta,
}

#[derive(Debug, Error)]
pub enum InvalidTransferGame {
    #[error("not enough bytes")]
    NotEnoughBytes,
    #[error("invalid card")]
    InvalidCarta(#[from] InvalidCard),
    #[error("carta was empty at index {0}")]
    CartaWasNone(usize),
}
impl TransferGame {
    pub fn serialize(&self) -> Vec<u8> {
        let mut out = vec![];

        let atot_bytes: [u8; 2] = self.carta_atot.map(|c| c.serialize()).unwrap_or([0; 2]);
        let table_card_bytes: [u8; 2] = [0; 2];
        let user_cards: Vec<u8> = {
            // Each card is [u8; 2]
            let mut v = vec![2 * self.your_cards.len() as u8];
            for c in &self.your_cards {
                v.extend(c.serialize());
            }
            v
        };

        out.extend(&atot_bytes);
        out.extend(&table_card_bytes);
        out.extend(&user_cards);

        out
    }
    pub fn deserialize(b: &[u8]) -> Result<Self, InvalidTransferGame> {
        if b.len() < 5 {
            return Err(InvalidTransferGame::NotEnoughBytes);
        }
        let carta_atot = Carta::deserialize(&b[0..2])?;
        let table_card = Carta::deserialize(&b[2..4])?;

        let hand_cards_amount = b[4];
        if b.len() < 2 + 2 + 1 + hand_cards_amount as usize {
            return Err(InvalidTransferGame::NotEnoughBytes);
        }
        let mut your_cards = vec![];
        for i in 0..hand_cards_amount as usize / Carta::BYTES_LENGTH {
            let j = 2 + 2 + 1 + i * 2;
            match Carta::deserialize(&b[j..j + 2])? {
                Some(c) => your_cards.push(c),
                None => return Err(InvalidTransferGame::CartaWasNone(i)),
            }
        }

        Ok(Self {
            carta_atot,
            table_card,
            your_cards,
        })
    }
}

#[derive(Error, Debug)]
pub enum InvalidClientStatement {
    #[error("not enough bytes")]
    NotEnoughBytes,
    #[error("tag is not response")]
    NotResponseTag,
    #[error("invalid card")]
    InvalidCard(#[from] InvalidCard),
    #[error("missing card")]
    MissingCard,
    #[error("invalid tag")]
    InvalidTag(#[from] InvalidTransferTag),
}
impl ClientStatement {
    pub fn serialize(&self) -> Vec<u8> {
        self.played_card.serialize().to_vec()
    }
    pub fn deserialize(b: &[u8]) -> Result<Self, InvalidClientStatement> {
        if b.is_empty() {
            return Err(InvalidClientStatement::NotEnoughBytes);
        }
        if TransferTag::deserialize(&b[0..1])? != TransferTag::Response {
            return Err(InvalidClientStatement::NotResponseTag);
        }

        match Carta::deserialize(&b[1..])? {
            Some(c) => Ok(ClientStatement { played_card: c }),
            None => Err(InvalidClientStatement::MissingCard),
        }
    }
}

#[derive(Error, Debug)]
#[error("invalid coll")]
enum InvalidColl {
    #[error("invalid amount of bytes: {0}")]
    InvalidAmountOfBytes(usize),
    #[error("unrecognized coll: {0}")]
    Unrecognized(char),
}
impl Coll {
    const BYTES_LENGTH: usize = 1;
    fn serialize(&self) -> [u8; Self::BYTES_LENGTH] {
        match self {
            Coll::Monedes => [b'm'],
            Coll::Copes => [b'c'],
            Coll::Espases => [b'e'],
            Coll::Garrots => [b'g'],
        }
    }
    fn deserialize(b: &[u8]) -> Result<Self, InvalidColl> {
        let [b] = b else {
            return Err(InvalidColl::InvalidAmountOfBytes(b.len()));
        };
        match &[*b; 1] {
            b"m" => Ok(Coll::Monedes),
            b"c" => Ok(Coll::Copes),
            b"e" => Ok(Coll::Espases),
            b"g" => Ok(Coll::Garrots),
            x => Err(InvalidColl::Unrecognized(x[0] as char)),
        }
    }
}
#[derive(Error, Debug)]
#[error("invalid numero")]
enum InvalidNumero {
    #[error("invalid amount of bytes: {0}")]
    InvalidAmountOfBytes(usize),
    #[error("unrecognized numero: {0}")]
    Unrecognized(u8),
}

impl Numero {
    const BYTES_LENGTH: usize = 1;
    fn serialize(&self) -> [u8; Self::BYTES_LENGTH] {
        [*self as u8]
    }
    fn deserialize(b: &[u8]) -> Result<Self, InvalidNumero> {
        let [b] = b else {
            return Err(InvalidNumero::InvalidAmountOfBytes(b.len()));
        };

        if *b >= 10 {
            Err(InvalidNumero::Unrecognized(*b))
        } else {
            unsafe { Ok(mem::transmute::<u8, Numero>(*b)) }
        }
    }
}

#[derive(Error, Debug)]
pub enum InvalidCard {
    #[error("incorrect format")]
    IncorrectFormat,
    #[error("invalid coll")]
    InvalidColl(#[from] InvalidColl),
    #[error("invalid numero")]
    InvalidNumero(#[from] InvalidNumero),
}

impl Carta {
    const BYTES_LENGTH: usize = Numero::BYTES_LENGTH + Coll::BYTES_LENGTH;
    fn serialize(&self) -> [u8; Self::BYTES_LENGTH] {
        let [c] = self.coll.serialize();
        let [v] = self.valor.serialize();
        [c, v]
    }

    fn deserialize(b: &[u8]) -> Result<Option<Self>, InvalidCard> {
        let [c_b, v_b] = b else {
            return Err(InvalidCard::IncorrectFormat);
        };

        if *c_b == 0 && *v_b == 0 {
            return Ok(None);
        }
        Ok(Some(Carta {
            coll: Coll::deserialize(&[*c_b])?,
            valor: Numero::deserialize(&[*v_b])?,
        }))

        //match (Coll::deserialize(&[*c_b]), Numero::deserialize(&[*v_b])) {
        //    (Some(c), Some(v)) => Ok(Some(Carta { coll: c, valor: v })),
        //    (None, _) => ,
        //}
        //Coll::deserialize(&[*c_b]) .and_then(|c| Numero::deserialize(&[*v_b]).map(|v| Carta { coll: c, valor: v }))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferTag {
    State,
    Info,
    Prompt,
    Response,
}

impl From<TransferTag> for char {
    fn from(value: TransferTag) -> Self {
        u8::from(value) as char
    }
}
impl From<TransferTag> for u8 {
    fn from(value: TransferTag) -> Self {
        match value {
            TransferTag::State => b'S',
            TransferTag::Info => b'I',
            TransferTag::Prompt => b'P',
            TransferTag::Response => b'R',
        }
    }
}

#[derive(Error, Debug)]
pub enum InvalidTransferTag {
    #[error("invalid amount of bytes: '{0}'")]
    InvalidAmountOfBytes(usize),
    #[error("unrecognized: '{0}'")]
    Unrecognized(u8),
}
impl TransferTag {
    const BYTES_LENGTH: usize = 1;
    pub fn serialize(&self) -> [u8; Self::BYTES_LENGTH] {
        [u8::from(*self)]
    }
    pub fn deserialize(b: &[u8]) -> Result<Self, InvalidTransferTag> {
        if b.len() != 1 {
            return Err(InvalidTransferTag::InvalidAmountOfBytes(b.len()));
        }
        match b[0] {
            b'S' => Ok(Self::State),
            b'I' => Ok(Self::Info),
            b'P' => Ok(Self::Prompt),
            b'R' => Ok(Self::Response),
            _ => Err(InvalidTransferTag::Unrecognized(b[0])),
        }
    }
}
