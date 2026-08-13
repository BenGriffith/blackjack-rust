use crate::error::GameError;
use crate::person::{Dealer, Player, Stats};
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

pub const WELCOME: &str = "Welcome to Blackjack!";
pub const INVITE: &str = "Would you like to play a game? [y/n] ";
pub const BET: &str = "How much would you like to bet? ";
// pub const PLAY_AGAIN: &str = "Would you like to play again? [y/n] ";
// pub const ACTION: &str = "Choose action: [h]it or [s]tand ";
pub const GOODBYE: &str = "Goodbye!";
pub const DELAY: u64 = 1;
pub const PLAYER: &str = "Player";
pub const DEALER: &str = "Dealer";

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
}

impl Message {
    pub fn deal_card(person: &str) {
        println!("Dealing {} Card...", person);
        let sleep = Duration::from_secs(DELAY);
        thread::sleep(sleep);
    }

    pub fn round_result(player: &Player, dealer: &Dealer, first_round: bool) {
        if first_round {
            println!("Result after First Round");
        } else {
            println!("Final Result");
        }
        println!("---------------");
        player.print_stats();
        player.hand.print_hand();
        dealer.print_stats();
    }
}
