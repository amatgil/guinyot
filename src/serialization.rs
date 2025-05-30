use std::mem;

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
}

impl ClientStatement {
    pub fn serialize(&self) -> Vec<u8> {
        self.played_card.serialize().to_vec()
    }
    pub fn deserialize(b: &[u8]) -> Option<Self> {
        if b.is_empty() {
            return None;
        }
        let tag = b[0];
        if tag != RESPONSE_TAG {
            return None;
        }

        Carta::deserialize(&b[1..]).map(|c| ClientStatement { played_card: c })
    }
}

impl Coll {
    fn serialize(&self) -> [u8; 1] {
        match self {
            Coll::Monedes => [b'm'],
            Coll::Copes => [b'c'],
            Coll::Espases => [b'e'],
            Coll::Garrots => [b'g'],
        }
    }
    fn deserialize(b: &[u8]) -> Option<Self> {
        let [b] = b else {
            return None;
        };
        match &[*b; 1] {
            b"m" => Some(Coll::Monedes),
            b"c" => Some(Coll::Copes),
            b"e" => Some(Coll::Espases),
            b"g" => Some(Coll::Garrots),
            _ => None,
        }
    }
}
impl Numero {
    fn serialize(&self) -> [u8; 1] {
        [*self as u8]
    }
    fn deserialize(b: &[u8]) -> Option<Self> {
        let [b] = b else {
            return None;
        };

        if *b >= 10 {
            None
        } else {
            unsafe { Some(mem::transmute::<u8, Numero>(*b)) }
        }
    }
}

impl Carta {
    fn serialize(&self) -> [u8; 2] {
        let [c] = self.coll.serialize();
        let [v] = self.valor.serialize();
        [c, v]
    }

    fn deserialize(b: &[u8]) -> Option<Self> {
        let [c_b, v_b] = b else {
            return None;
        };
        Coll::deserialize(&[*c_b])
            .and_then(|c| Numero::deserialize(&[*v_b]).map(|v| Carta { coll: c, valor: v }))
    }
}
