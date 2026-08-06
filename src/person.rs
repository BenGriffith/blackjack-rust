use crate::card::Card;
use crate::error::GameError;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct HandState {
    hand: Vec<Card>,
    hand_value: isize,
}

impl HandState {
    pub fn new() -> Self {
        Self {
            hand: Vec::new(),
            hand_value: 0,
        }
    }

    // pub fn calculate_hand_value(&mut self, value: isize) -> Result<isize, GameError> {
    //     if value <= 0 {
    //         return Err(GameError::InvalidScore);
    //     }
    //     self.hand_value += value;
    //     Ok(self.hand_value)
    // }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card)
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn get_hand_value(&self) -> &isize {
        &self.hand_value
    }
}

pub struct Player {
    bet: isize,
    pub hand: HandState,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            bet: 0,
            hand: HandState::new(),
        }
    }

    pub fn place_bet(&mut self, value: isize) -> Result<isize, GameError> {
        if value <= 0 {
            return Err(GameError::InvalidBet);
        }
        self.bet = value;
        Ok(self.bet)
    }
}

pub struct Dealer {
    pub hand: HandState,
}

impl Default for Dealer {
    fn default() -> Self {
        Self::new()
    }
}

impl Dealer {
    pub fn new() -> Self {
        Self {
            hand: HandState::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // test handstate creation
    // test handstate invalid
    // test handstate calculate hand value
    // test handstate get hand
    // test handstate get hand value
    // test player creation
    // test player place bet
    // test player invalid bet
    // test dealer creation
    #[test]
    fn test_player_hand() {
        let mut player = Player::new();
        let first_card = Card::new("Clubs".to_string(), "Queen".to_string(), 10);
        let second_card = Card::new("Spades".to_string(), "Ace".to_string(), 11);
        player.hand.add_card(first_card);
        player.hand.add_card(second_card);
        assert!(!player.hand.get_hand().is_empty());
        assert_eq!(player.hand.get_hand().len(), 2);
    }

    #[test]
    fn test_dealer_hand() {
        let mut dealer = Dealer::new();
        let first_card = Card::new("Hearts".to_string(), "Jack".to_string(), 10);
        let second_card = Card::new("Diamonds".to_string(), "King".to_string(), 10);
        dealer.hand.add_card(first_card);
        dealer.hand.add_card(second_card);
        assert_eq!(dealer.hand.get_hand().len(), 2);
    }
}
