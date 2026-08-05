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

    pub fn calculate_hand_value(&mut self, value: isize) -> Result<isize, GameError> {
        if value <= 0 {
            return Err(GameError::InvalidScore);
        }
        self.hand_value += value;
        Ok(self.hand_value)
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
