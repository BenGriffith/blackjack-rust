use crate::deck::Deck;
use crate::person::{Dealer, Player};

pub struct Game {
    deck: Deck,
    dealer: Dealer,
    player: Player,
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

    pub fn deck(&self) -> &Deck {
        &self.deck
    }

    pub fn dealer(&self) -> &Dealer {
        &self.dealer
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}
