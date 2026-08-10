use crate::deck::Deck;
use crate::error::GameError;
use crate::person::{Dealer, Player};
use std::io::{self, Write};
use std::process;

pub struct Game {
    deck: Deck,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self { deck: Deck::new() }
    }

    pub fn start(&self) -> Result<bool, GameError> {
        print!("Welcome to Blackjack! Would you like to play a game? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let response = input.to_uppercase();
        match response.trim() {
            "YES" | "Y" => Ok(true),
            _ => {
                println!("Goodbye!");
                process::exit(0);
            }
        }
    }

    pub fn run(&mut self, player: &mut Player, dealer: &mut Dealer) -> Result<(), GameError> {
        let player_bet = self.player_bet_prompt()?;
        let _bet_amount = player.place_bet(player_bet)?;

        if let Some(card) = self.deck.deal_card() {
            dealer.hand.add_card(card);
        }
        Ok(())
    }

    fn player_bet_prompt(&self) -> Result<isize, GameError> {
        print!("How much would you like to bet? ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let response = input.trim();
        match response.parse::<isize>() {
            Ok(result) => Ok(result),
            Err(_) => Err(GameError::InvalidNumber),
        }
    }
}
