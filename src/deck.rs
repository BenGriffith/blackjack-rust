use crate::card::{Card, Rank, Suit, get_card_value};
use crate::error::CardError;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Result<Card, CardError>>,
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

impl Deck {
    pub fn new() -> Self {
        let cards: Vec<Rank> = Card::get_cards();
        let suits: Vec<Suit> = Card::get_suits();
        let deck_cards: Vec<Result<Card, CardError>> = Self::set_deck(&suits, &cards);
        Self { cards: deck_cards }
    }

    fn set_deck(suits: &[Suit], cards: &[Rank]) -> Vec<Result<Card, CardError>> {
        let deck_cards: Vec<Result<Card, CardError>> = suits
            .iter()
            .flat_map(|suit| {
                cards.iter().map(|card| -> Result<Card, CardError> {
                    let card_value: usize = get_card_value(card)?;
                    Ok(Card::new(suit.clone(), card.clone(), card_value))
                })
            })
            .collect();
        deck_cards
    }

    pub fn cards(&self) -> &Vec<Result<Card, CardError>> {
        &self.cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::vec::Vec;

    #[test]
    fn test_deck_length() {
        let deck: Deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_suits() {
        let deck: Deck = Deck::new();
        let mut suits: HashSet<String> = HashSet::new();
        for card in deck.cards.iter().flatten() {
            suits.insert(card.suit().to_string());
        }
        assert_eq!(suits.len(), 4);
        assert!(suits.contains("Hearts"));
        assert!(suits.contains("Diamonds"));
        assert!(suits.contains("Clubs"));
        assert!(suits.contains("Spades"));
    }

    #[test]
    fn test_deck_suits_face_cards() {
        let deck: Deck = Deck::new();
        let mut suit_face_cards: HashMap<String, Vec<String>> = HashMap::new();

        for suit in deck.cards.iter().flatten() {
            let mut face_cards: Vec<String> = Vec::new();
            for _card in &deck.cards {
                if let Ok(face_card) = _card
                    && face_card.suit() == suit.suit()
                {
                    match face_card.rank() {
                        Rank::Jack | Rank::Queen | Rank::King => {
                            face_cards.push(face_card.rank().to_string())
                        }
                        _ => continue,
                    }
                }
            }
            suit_face_cards.insert(suit.suit().to_string(), face_cards);
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

        for s in deck.cards.iter().flatten() {
            let mut nonface_cards: Vec<String> = vec![];

            for card in &deck.cards {
                if let Ok(c) = card
                    && c.suit() == s.suit()
                {
                    let is_nonface_card =
                        !matches!(c.rank(), Rank::Jack | Rank::Queen | Rank::King);
                    if is_nonface_card {
                        nonface_cards.push(c.rank().to_string());
                    }
                }
            }
            suit_nonface_cards.insert(s.suit().to_string(), nonface_cards);
        }
        let hearts_expected = vec!["2", "3", "4", "5", "6", "7", "8", "9", "10", "Ace"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        assert_eq!(suit_nonface_cards.len(), 4);
        assert_eq!(suit_nonface_cards.get("Hearts").unwrap().len(), 10);
        assert_eq!(suit_nonface_cards.get("Hearts").unwrap(), &hearts_expected);
    }

    #[test]
    fn test_deck_card_values_sum() {
        let deck: Deck = Deck::new();
        let total_value: usize = deck
            .cards
            .iter()
            .map(|c| if let Ok(card) = c { card.value() } else { &0 })
            .sum();
        assert_eq!(total_value, 380);
    }
}
