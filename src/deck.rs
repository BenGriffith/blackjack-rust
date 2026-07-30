use crate::card::{Card, FaceCard, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let suits: Vec<Suit> = vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let facecards: Vec<FaceCard> = vec![FaceCard::Jack, FaceCard::Queen, FaceCard::King];
        let deck_suits: Vec<String> = suits.iter().map(|x: &Suit| x.to_string()).collect();
        let deck_facecards: Vec<String> =
            facecards.iter().map(|x: &FaceCard| x.to_string()).collect();

        let deck_cards: Vec<Card> = deck_suits
            .iter()
            .flat_map(|suit: &String| {
                deck_facecards.iter().map(move |facecard: &String| Card {
                    suit: suit.to_string(),
                    rank: facecard.to_string(),
                })
            })
            .collect();
        Self { cards: deck_cards }
    }
}
