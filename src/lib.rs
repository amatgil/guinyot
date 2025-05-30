pub const DEFAULT_PORT: u32 = 25545;

pub struct Carta {
    coll: Coll,
    valor: Numero,
}

pub enum Coll {}
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
    fn value(&self) -> u32 {
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
