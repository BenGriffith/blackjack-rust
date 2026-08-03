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
