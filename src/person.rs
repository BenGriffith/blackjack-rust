use crate::card::Card;
use crate::error::GameError;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct HandState {
    pub hand: Vec<Card>,
    pub hand_value: isize,
}

impl HandState {
    pub fn new() -> Self {
        Self {
            hand: Vec::new(),
            hand_value: 0,
        }
    }

    pub fn add_card(&mut self, value: isize) -> Result<isize, GameError> {
        if value < 0 {
            return Err(GameError::InvalidScore);
        }
        self.hand_value += value;
        Ok(self.hand_value)
    }
}

pub struct Player {
    pub bet: isize,
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

    pub fn bet(&mut self, value: isize) -> Result<isize, GameError> {
        if value < 0 {
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
