use crate::deck::Deck;
use crate::person::{Dealer, Player};

pub struct Game {
    pub deck: Deck,
    pub dealer: Dealer,
    pub player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            dealer: Dealer::new(),
            player: Player::new(),
        }
    }
}
