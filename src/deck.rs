use crate::card::{Card, FaceCard, Suit};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    pub fn new(&self) -> Self {
        let cards = self.get_cards();
        let deck_suits = self.get_suits();
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

    fn get_suits(&self) -> Vec<String> {
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let deck_suits: Vec<String> = suits.iter().map(|s: &Suit| s.to_string()).collect();
        deck_suits
    }

    fn get_cards(&self) -> Vec<String> {
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

    fn get_card_value(&self) -> usize {
        unimplemented!();
    }

    fn set_deck(&self) -> Vec<Card> {
        unimplemented!();
    }
}
