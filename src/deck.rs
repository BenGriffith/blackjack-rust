use crate::card::{Card, FaceCard, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    pub fn new() -> Self {
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let face_cards: Vec<FaceCard> = vec![FaceCard::Jack, FaceCard::Queen, FaceCard::King];
        let base_cards: Vec<String> = (1..=10).map(|c: usize| c.to_string()).collect();

        let cards: Vec<String> = face_cards
            .iter()
            .map(|card: &FaceCard| card.to_string())
            .chain(base_cards)
            .collect();

        let deck_suits: Vec<String> = suits.iter().map(|s: &Suit| s.to_string()).collect();
        let deck_cards: Vec<Card> = deck_suits
            .iter()
            .flat_map(|suit: &String| {
                cards.iter().map(move |card: &String| Card {
                    suit: suit.to_string(),
                    rank: card.to_string(),
                })
            })
            .collect();
        Self { cards: deck_cards }
    }
}
