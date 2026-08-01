use crate::CardError;
use crate::card::{Card, get_card_value};

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Result<Card, CardError>>,
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    pub fn new() -> Self {
        let cards: Vec<String> = Card::get_cards();
        let suits: Vec<String> = Card::get_suits();
        let deck_cards: Vec<Result<Card, CardError>> = Self::set_deck(&suits, &cards);
        Self { cards: deck_cards }
    }

    fn set_deck(suits: &[String], cards: &[String]) -> Vec<Result<Card, CardError>> {
        let deck_cards: Vec<Result<Card, CardError>> = suits
            .iter()
            .flat_map(|suit: &String| {
                cards
                    .iter()
                    .map(move |card: &String| -> Result<Card, CardError> {
                        let card_value: usize = get_card_value(card)?;
                        Ok(Card {
                            suit: suit.to_string(),
                            rank: card.to_string(),
                            value: card_value,
                        })
                    })
            })
            .collect();
        deck_cards
    }
}
