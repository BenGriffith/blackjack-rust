use std::fmt;

#[derive(Debug, Clone, PartialEq)]
// Custom error type for card validation (e.g., user input)
pub enum CardError {
    InvalidNumber { value: String },
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardError::InvalidNumber { value } => write!(f, "Invalid number: {}", value),
        }
    }
}

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
