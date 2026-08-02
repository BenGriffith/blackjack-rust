use crate::card::{get_card_value, Card};
use crate::CardError;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_deck_length() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_suits() {
        let deck = Deck::new();
        let suits: HashSet<String> = deck
            .cards
            .iter()
            .filter_map(|card_result| match card_result {
                Ok(card) => Some(card.suit.clone()),
                Err(_) => None,
            })
            .collect();
        assert_eq!(suits.len(), 4);
        assert_eq!(suits.contains("Hearts"), true);
        assert_eq!(suits.contains("Diamonds"), true);
        assert_eq!(suits.contains("Clubs"), true);
        assert_eq!(suits.contains("Spades"), true);
    }

    #[test]
    fn test_deck_suits_face_cards() {
        let deck = Deck::new();
        let face_cards: HashMap<String, Vec<String>> = deck
            .cards
            .iter()
            .filter_map(|card_result| match card_result {
                Ok(card) => {
                    if card.rank == "Jack" || card.rank == "Queen" || card.rank == "King" {
                        Some((card.suit.clone(), card.rank.clone()))
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .fold(HashMap::new(), |mut acc, (suit, rank)| {
                acc.entry(suit).or_insert_with(Vec::new).push(rank);
                acc
            });

        assert_eq!(face_cards.len(), 4);
        assert_eq!(face_cards.get("Hearts").unwrap().len(), 3);
        assert_eq!(face_cards.get("Diamonds").unwrap().len(), 3);
        assert_eq!(face_cards.get("Clubs").unwrap().len(), 3);
        assert_eq!(face_cards.get("Spades").unwrap().len(), 3);
    }

    #[test]
    fn test_deck_rank() {
        let deck = Deck::new();
        let ranks: HashSet<String> = deck
            .cards
            .iter()
            .filter_map(|card_result| match card_result {
                Ok(card) => Some(card.rank.clone()),
                Err(_) => None,
            })
            .collect();
        assert_eq!(ranks.len(), 13);
    }

    #[test]
    fn test_deck_card_values_sum() {
        let deck = Deck::new();
        let total_value: usize = deck
            .cards
            .iter()
            .filter_map(|card_result| match card_result {
                Ok(card) => Some(card.value),
                Err(_) => None,
            })
            .sum();
        assert_eq!(total_value, 380);
    }
}
