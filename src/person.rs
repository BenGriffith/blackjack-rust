use crate::card::Card;
use crate::error::GameError;
use std::fmt;
use std::vec::Vec;

pub trait Stats {
    fn print_stats(&self);
}

#[derive(Debug, Clone, PartialEq)]
pub struct HandState {
    hand: Vec<Card>,
    hand_value: usize,
}

impl HandState {
    fn new() -> Self {
        Self {
            hand: Vec::new(),
            hand_value: 0,
        }
    }

    pub fn calculate_hand_value(&mut self) {
        self.hand_value = self.hand.iter().map(|card| card.value()).sum()
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
        self.calculate_hand_value();
    }

    pub fn hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn hand_value(&self) -> &usize {
        &self.hand_value
    }

    pub fn print_hand(&self) -> String {
        let hand: Vec<String> = self
            .hand
            .iter()
            .map(|card| format!("({}, {})", card.suit(), card.rank()))
            .collect();
        hand.join(", ")
    }
}

#[derive(Debug)]
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

    pub fn place_bet(&mut self, value: isize) -> Result<(), GameError> {
        if value <= 0 {
            return Err(GameError::InvalidBet);
        }
        self.bet = value;
        Ok(())
    }

    pub fn bet(&self) -> &isize {
        &self.bet
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player")
    }
}

impl Stats for Player {
    fn print_stats(&self) {
        println!("{} Bet: {}", self, self.bet);
        println!("{} Hand Value: {}", self, self.hand.hand_value);
        println!("{} Hand: {}", self, self.hand.print_hand());
    }
}

#[derive(Debug)]
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

impl fmt::Display for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dealer")
    }
}

impl Stats for Dealer {
    fn print_stats(&self) {
        println!("{} Hand Value: {}", self, self.hand.hand_value);
        println!("{} Hand: {}", self, self.hand.print_hand());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Rank, Suit};

    #[test]
    fn test_hand() {
        let hand = HandState::new();
        assert!(hand.hand.is_empty());
        assert_eq!(hand.hand_value, 0);
    }

    #[test]
    fn test_hand_add_card() {
        let mut hand = HandState::new();
        let card = Card::new(Suit::Hearts, Rank::Two);
        hand.add_card(card);
        assert_eq!(hand.hand, vec![Card::new(Suit::Hearts, Rank::Two)]);
        assert_eq!(hand.hand.iter().len(), 1);
    }

    #[test]
    fn test_get_hand() {
        let mut hand = HandState::new();
        let first_card = Card::new(Suit::Clubs, Rank::Three);
        let second_card = Card::new(Suit::Diamonds, Rank::Six);
        let expected = vec![first_card.clone(), second_card.clone()];
        hand.add_card(first_card);
        hand.add_card(second_card);
        assert_eq!(hand.hand(), &expected);
    }

    #[test]
    fn test_player_hand() {
        let mut player = Player::new();
        let first_card = Card::new(Suit::Clubs, Rank::Queen);
        let second_card = Card::new(Suit::Spades, Rank::Ace);
        player.hand.add_card(first_card);
        player.hand.add_card(second_card);
        assert!(!player.hand.hand().is_empty());
        assert_eq!(player.hand.hand().len(), 2);
    }

    #[test]
    fn test_player_valid_bet() -> Result<(), GameError> {
        let mut player = Player::new();
        let first_card = Card::new(Suit::Diamonds, Rank::Jack);
        player.hand.add_card(first_card);
        player.place_bet(5)?;
        assert_eq!(player.bet, 5);
        Ok(())
    }

    #[test]
    fn test_player_invalid_bet() -> Result<(), GameError> {
        let mut player = Player::new();
        let first_card = Card::new(Suit::Spades, Rank::Ten);
        player.hand.add_card(first_card);
        let bet = player.place_bet(-10);
        assert!(matches!(bet, Err(GameError::InvalidBet)));
        Ok(())
    }

    #[test]
    fn test_dealer_hand() {
        let mut dealer = Dealer::new();
        let first_card = Card::new(Suit::Hearts, Rank::Jack);
        let second_card = Card::new(Suit::Diamonds, Rank::King);
        dealer.hand.add_card(first_card);
        dealer.hand.add_card(second_card);
        assert_eq!(dealer.hand.hand().len(), 2);
    }
}
