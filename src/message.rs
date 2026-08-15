use crate::constants::*;
use crate::error::GameError;
use crate::person::{Dealer, Player, Stats};
use std::io::{self, Write};
use std::process;
use std::thread;
use std::time::Duration;

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

    pub fn process_blackjack(person: &str, player: &Player, dealer: &Dealer) {
        match (person, player.hand.hand_value()) {
            ("Player", BLACKJACK) => {
                Message::round_result(player, dealer, false);
                println!(
                    "Congratulations! You scored Blackjack and win {}!",
                    player.bet() * PRIZE
                );
            }
            ("Player", value) if value > BLACKJACK => {
                Message::round_result(player, dealer, false);
                println!("BUST! House wins!");
            }
            ("Player", _) => Message::round_result(player, dealer, false),
            (_, _) => todo!(),
        }
    }
}
