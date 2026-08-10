use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GameError {
    InvalidScore,
    InvalidBet,
    InvalidResponse,
    InvalidNumber,
    Io(io::Error),
}

impl From<io::Error> for GameError {
    fn from(err: io::Error) -> Self {
        GameError::Io(err)
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidScore => write!(f, "value should be greater than 0"),
            GameError::InvalidBet => write!(f, "please provide a positive integer value"),
            GameError::InvalidResponse => write!(f, "please enter an acceptable value"),
            GameError::InvalidNumber => write!(f, "please enter a valid number"),
            GameError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for GameError {}
