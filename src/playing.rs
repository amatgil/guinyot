use crate::*;

use self::serialization::TransferGame;

#[derive(Debug, Clone)]
pub struct Game {
    /// Les cartes cara cap avall
    pub uncovered: Vec<Carta>,

    /// A qui li toca
    pub torn: Torn,

    /// Cartes de A
    pub a_cards: Vec<Carta>,

    /// Cartes de B
    pub b_cards: Vec<Carta>,
}

impl Game {
    pub fn new() -> Self {
        let mut uncovered = vec![];
        let mut a_cards = vec![];
        let mut b_cards = vec![];

        for coll in [Coll::Copes, Coll::Espases, Coll::Garrots, Coll::Monedes] {
            for n in Numero::ALL {
                uncovered.push(Carta {
                    coll,
                    valor: Numero::try_from(dbg!(n)).unwrap(),
                })
            }
        }

        assert_eq!(uncovered.len(), 40);
        uncovered = shuffle(uncovered);

        for _ in 0..3 {
            a_cards.push(uncovered.pop().unwrap());
        }

        for _ in 0..3 {
            b_cards.push(uncovered.pop().unwrap());
        }

        assert_eq!(uncovered.len(), 40 - 2 * 3);
        Game {
            uncovered,
            torn: Torn::A,
            a_cards,
            b_cards,
        }
    }

    /// La carta que marca l'atot (el coll més poderòs)
    pub fn atot(&self) -> Option<Carta> {
        self.uncovered.first().copied()
    }

    pub fn is_over(&self) -> bool {
        todo!()
    }
}
