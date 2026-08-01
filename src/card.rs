use crate::error::CardError;
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

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: String,
    pub rank: String,
    pub value: usize,
}

pub fn get_card_value(card: &str) -> Result<usize, CardError> {
    match card {
        "Jack" | "Queen" | "King" => Ok(10),
        "Ace" => Ok(11),
        _ => match card.parse::<usize>() {
            Ok(n) => Ok(n),
            Err(_) => Err(CardError::InvalidNumber {
                value: card.to_string(),
            }),
        },
    }
}

impl Card {
    pub fn get_suits() -> Vec<String> {
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let deck_suits: Vec<String> = suits.iter().map(|s: &Suit| s.to_string()).collect();
        deck_suits
    }

    pub fn get_cards() -> Vec<String> {
        let face_cards: Vec<FaceCard> = vec![FaceCard::Jack, FaceCard::Queen, FaceCard::King];
        let mut playing_cards: Vec<String> = (2..=10).map(|c: usize| c.to_string()).collect();
        let ace: String = String::from("Ace");
        playing_cards.push(ace);

        let cards: Vec<String> = face_cards
            .iter()
            .map(|card: &FaceCard| card.to_string())
            .chain(playing_cards)
            .collect();

        cards
    }
}
