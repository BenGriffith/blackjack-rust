use std::fmt;

pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

pub enum FaceCard {
    Jack,
    Queen,
    King,
}

impl fmt::Display for FaceCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FaceCard::Jack => write!(f, "Jack"),
            FaceCard::Queen => write!(f, "Queen"),
            FaceCard::King => write!(f, "King"),
        }
    }
}

pub struct Card {
    pub suit: String,
    pub rank: String,
}

impl Card {
    pub fn card_value(&self) -> usize {
        unimplemented!();
    }
}
