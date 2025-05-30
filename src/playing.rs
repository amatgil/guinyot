use crate::*;

#[derive(Debug, Clone)]
pub struct Game {
    /// Les cartes cara cap avall
    pub uncovered: Vec<Carta>,

    /// A qui li toca
    pub torn: Torn,

    pub a_hand: Vec<Carta>,
    pub b_hand: Vec<Carta>,

    pub a_won_cards: Vec<Carta>,
    pub b_won_cards: Vec<Carta>,

    pub atot: Coll,
}

#[derive(Debug, Clone, Copy)]
pub enum GameError {
    PlayerDoesNotHaveCard,
}

impl Game {
    pub fn new() -> Self {
        let mut uncovered = vec![];
        let mut a_cards = vec![];
        let mut b_cards = vec![];

        for coll in [Coll::Copes, Coll::Espases, Coll::Garrots, Coll::Monedes] {
            for valor in Numero::ALL {
                uncovered.push(Carta { coll, valor })
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
            atot: uncovered.first().unwrap().coll,
            uncovered,
            torn: Torn::A,
            a_hand: a_cards,
            b_hand: b_cards,
            a_won_cards: vec![],
            b_won_cards: vec![],
        }
    }

    /// La carta que marca l'atot (el coll més poderòs)
    pub fn atot(&self) -> Option<Carta> {
        self.uncovered.first().copied()
    }

    pub fn is_over(&self) -> bool {
        dbg!(false) // TODO: implement
    }

    pub fn play_cards(&mut self, first: Carta, second: Carta) {
        let first_won = first_wins_basa(first, second, self.atot);
        match (self.torn, first_won) {
            (Torn::A, true) | (Torn::B, false) => {
                self.a_hand.push(first);
                self.a_hand.push(second);
            }
            (Torn::A, false) | (Torn::B, true) => todo!(),
        }
    }
}

fn first_wins_basa(first: Carta, second: Carta, atot: Coll) -> bool {
    dbg!(true) // TODO: implement
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
