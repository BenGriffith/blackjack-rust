use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
