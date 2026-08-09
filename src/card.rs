use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
    value: usize,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        let value: usize = Self::get_card_value(&rank);
        Self { suit, rank, value }
    }

    pub fn suit(&self) -> &Suit {
        &self.suit
    }

    pub fn rank(&self) -> &Rank {
        &self.rank
    }

    pub fn value(&self) -> &usize {
        &self.value
    }

    pub fn get_suits() -> Vec<Suit> {
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        suits
    }

    pub fn get_cards() -> Vec<Rank> {
        let rank: Vec<Rank> = vec![
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];
        rank
    }

    fn get_card_value(rank: &Rank) -> usize {
        match rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_card() -> Card {
        Card {
            suit: Suit::Hearts,
            rank: Rank::Five,
            value: 5,
        }
    }

    #[test]
    fn test_card() {
        let card: Card = get_card();
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.rank, Rank::Five);
        assert_eq!(card.value, 5);
    }

    #[test]
    fn test_get_suits() {
        let suits: Vec<String> = Card::get_suits().iter().map(|s| s.to_string()).collect();
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
        let cards: Vec<String> = Card::get_cards().iter().map(|c| c.to_string()).collect();
        let expected: Vec<String> = vec![
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "Jack".to_string(),
            "Queen".to_string(),
            "King".to_string(),
            "Ace".to_string(),
        ];
        assert_eq!(cards, expected);
    }

    #[test]
    fn test_get_card_value() {
        let jack_value: usize = Card::get_card_value(&Rank::Jack);
        let queen_value: usize = Card::get_card_value(&Rank::Queen);
        let king_value: usize = Card::get_card_value(&Rank::King);
        let ace_value: usize = Card::get_card_value(&Rank::Ace);
        let card_value: usize = Card::get_card_value(&Rank::Nine);

        assert_eq!(jack_value, 10);
        assert_eq!(queen_value, 10);
        assert_eq!(king_value, 10);
        assert_eq!(ace_value, 11);
        assert_eq!(card_value, 9);
    }
}
