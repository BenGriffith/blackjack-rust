use crate::card::Card;

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
    pub fn new() -> Self {
        let cards: Vec<String> = Card::get_cards();
        let suits: Vec<String> = Card::get_suits();
        let deck_cards: Vec<Card> = Self::set_deck(&suits, &cards);
        Self { cards: deck_cards }
    }

    fn set_deck(suits: &[String], cards: &[String]) -> Vec<Card> {
        let deck_cards: Vec<Card> = suits
            .iter()
            .flat_map(|suit: &String| {
                cards.iter().map(move |card: &String| Card {
                    suit: suit.to_string(),
                    rank: card.to_string(),
                })
            })
            .collect();
        deck_cards
    }
}
