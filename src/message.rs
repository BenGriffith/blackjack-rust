use crate::constants::*;
use crate::error::GameError;
use crate::person::Player;
use std::io::{self, Write};
use std::process;

pub struct Prompt;
pub struct Message;

impl Prompt {
    pub fn start_game() -> Result<bool, GameError> {
        print!("{} {}", WELCOME, INVITE);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let response = input.to_uppercase();
        match response.trim() {
            "YES" | "Y" => Ok(true),
            _ => {
                println!("{}", GOODBYE);
                process::exit(0);
            }
        }
    }

    pub fn player_bet() -> Result<isize, GameError> {
        print!("{}", BET);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let response = input.trim();
        match response.parse::<isize>() {
            Ok(result) => Ok(result),
            Err(_) => Err(GameError::InvalidNumber),
        }
    }

    pub fn player_move() -> Result<String, GameError> {
        print!("{}", MOVE);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input)
    }
}

impl Message {
    pub fn deal_card(person: &str) {
        println!("Dealing {} Card...", person);
    }

    pub fn round_result(first_round: bool) {
        if first_round {
            println!("Result after First Round");
        } else {
            println!("Final Result");
        }
        println!("---------------");
    }

    pub fn player_blackjack(player: &Player) {
        println!(
            "Congratulation! You scored Blackjack and win {}",
            player.bet() * PRIZE
        )
    }

    pub fn player_bust() {
        println!("BUST! House wins!");
    }
}
