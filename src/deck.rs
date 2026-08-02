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
    use std::vec::Vec;
    use std::collections::HashMap;
    use std::collections::HashSet;

    #[test]
    fn test_deck_length() {
        let deck: Deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_suits() {
        let deck: Deck = Deck::new();
        let mut suits: HashSet<String> = HashSet::new();
        for card in deck.cards {
            suits.insert(card.unwrap().suit);
        }
        assert_eq!(suits.len(), 4);
        assert_eq!(suits.contains("Hearts"), true);
        assert_eq!(suits.contains("Diamonds"), true);
        assert_eq!(suits.contains("Clubs"), true);
        assert_eq!(suits.contains("Spades"), true);
    }

    #[test]
    fn test_deck_suits_face_cards() {
        let deck: Deck = Deck::new();
        let mut suit_face_cards: HashMap<String, Vec<String>> = HashMap::new();

        for _suit in &deck.cards {
            if let Ok(suit) = _suit {
                let mut face_cards: Vec<String> = Vec::new();

                for _card in &deck.cards {
                    if let Ok(face_card) = _card && face_card.suit == suit.suit {
                        match face_card.rank.as_str() {
                            "Jack" | "Queen" | "King" => face_cards.push(face_card.rank.clone()),
                            _ => continue,
                        }
                    }
                }
                suit_face_cards.insert(suit.suit.clone(), face_cards);
            }
        }

        assert_eq!(suit_face_cards.len(), 4);
        assert_eq!(suit_face_cards.get("Hearts").unwrap().len(), 3);
        assert_eq!(suit_face_cards.get("Diamonds").unwrap().len(), 3);
        assert_eq!(suit_face_cards.get("Clubs").unwrap().len(), 3);
        assert_eq!(suit_face_cards.get("Spades").unwrap().len(), 3);
    }

    #[test]
    fn test_deck_rank() {
        let deck: Deck = Deck::new();
        let mut suit_nonface_cards: HashMap<String, Vec<String>> = HashMap::new();

        for suit in &deck.cards {
            if let Ok(s) = suit {
                let mut nonface_cards: Vec<String> = vec![];

                for card in &deck.cards {
                    if let Ok(c) = card && c.suit == s.suit {
                        let is_nonface_card = !matches!(c.rank.as_str(), "Jack" | "Queen" | "King"); 
                            if is_nonface_card {
                                nonface_cards.push(c.rank.clone());
                        }
                    }
                }
                suit_nonface_cards.insert(s.suit.clone(), nonface_cards);
            }
        }
        println!("{:?}", suit_nonface_cards);
    }

    #[test]
    fn test_deck_card_values_sum() {
        let deck: Deck = Deck::new();
        let total_value: usize = deck
            .cards
            .iter()
            .map(|c| {
                if let Ok(card) = c {
                    card.value
                } else {
                    0
                }})
            .sum();
        assert_eq!(total_value, 380);
    }
}
