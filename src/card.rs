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
    suit: String,
    rank: String,
    value: usize,
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
    pub fn new(suit: String, rank: String, value: usize) -> Self {
        Self { suit, rank, value }
    }

    pub fn suit(&self) -> &String {
        &self.suit
    }

    pub fn rank(&self) -> &String {
        &self.rank
    }

    pub fn value(&self) -> &usize {
        &self.value
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_card() -> Card {
        Card {
            suit: String::from("Hearts"),
            rank: String::from("5"),
            value: 5,
        }
    }

    #[test]
    fn test_card() {
        let card = get_card();
        assert_eq!(card.suit, String::from("Hearts"));
        assert_eq!(card.rank, "5".to_string());
        assert_eq!(card.value, 5);
    }

    #[test]
    fn test_get_suits() {
        let suits: Vec<String> = Card::get_suits();
        let expected: Vec<String> = vec![
            "Clubs".to_string(),
            "Diamonds".to_string(),
            "Hearts".to_string(),
            "Spades".to_string(),
        ];
        assert_eq!(suits, expected);
    }

    #[test]
    fn test_get_cards() {
        let cards: Vec<String> = Card::get_cards();
        let expected: Vec<String> = vec![
            "Jack".to_string(),
            "Queen".to_string(),
            "King".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "Ace".to_string(),
        ];
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_get_card_value() -> Result<(), CardError> {
        let jack_value: usize = get_card_value("Jack")?;
        let queen_value: usize = get_card_value("Queen")?;
        let king_value: usize = get_card_value("King")?;
        let ace_value: usize = get_card_value("Ace")?;
        let card_value: usize = get_card_value("9")?;

        assert_eq!(jack_value, 10);
        assert_eq!(queen_value, 10);
        assert_eq!(king_value, 10);
        assert_eq!(ace_value, 11);
        assert_eq!(card_value, 9);
        Ok(())
    }
}
