use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum GameError {
    InvalidScore,
    InvalidBet,
    InvalidResponse,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidScore => write!(f, "value should be greater than 0"),
            GameError::InvalidBet => write!(f, "please provide a positive integer value"),
            GameError::InvalidResponse => write!(f, "please enter an acceptable value"),
        }
    }
}
