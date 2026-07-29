pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

pub enum FaceCard {
    Jack,
    Queen,
    King,
}

pub struct Card {
    suit: String,
    rank: String,
}

impl Card {
    fn card_value(&self) -> usize {
        unimplemented!();
    }
}
