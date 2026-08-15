use crate::constants::*;
use crate::deck::Deck;
use crate::error::GameError;
use crate::message::{Message, Prompt};
use crate::person::{Dealer, Player, Stats};
use std::thread;
use std::time::Duration;

pub struct Game {
    deck: Deck,
}

enum GameResult {
    Blackjack,
    Bust,
    Stand,
    // HouseBlackjack,
    // HouseBust,
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

    pub fn start() -> Result<bool, GameError> {
        let start_game = Prompt::start_game()?;
        Ok(start_game)
    }

    pub fn run(&mut self, player: &mut Player, dealer: &mut Dealer) -> Result<(), GameError> {
        let player_bet = Prompt::player_bet()?;
        let _bet_amount = player.place_bet(player_bet)?;
        self.first_round(player, dealer)?;
        Ok(())
    }

    fn deal_card(&self, person: &str) {
        Message::deal_card(person);
        let sleep = Duration::from_secs(DELAY);
        thread::sleep(sleep);
    }

    fn print_round_summary(&self, player: &Player, dealer: &Dealer, first_round: bool) {
        Message::round_result(first_round);
        player.print_stats();
        player.hand.print_hand();
        dealer.print_stats();
    }

    fn first_round(&mut self, player: &mut Player, dealer: &mut Dealer) -> Result<(), GameError> {
        let player_str = player.to_string();
        let dealer_str = dealer.to_string();

        self.deal_card(&player_str);
        player.hand.add_card(self.deck.remove_card()?);

        self.deal_card(&dealer_str);
        dealer.hand.add_card(self.deck.remove_card()?);

        self.deal_card(&player_str);
        player.hand.add_card(self.deck.remove_card()?);

        self.deal_card(&dealer_str);
        dealer.hand.add_card(self.deck.remove_card()?);

        self.print_round_summary(player, dealer, true);
        let result = self.process_blackjack(player);
        match result {
            GameResult::Blackjack => {
                self.print_round_summary(player, dealer, false);
                Message::player_blackjack(player);
            }
            GameResult::Bust => {
                self.print_round_summary(player, dealer, false);
                Message::player_bust();
            }
            GameResult::Stand => todo!("add function to handle second round"),
        }
        Ok(())
    }

    fn process_blackjack(&self, player: &Player) -> GameResult {
        match player.hand.hand_value() {
            BLACKJACK => GameResult::Blackjack,
            value if value > BLACKJACK => GameResult::Bust,
            _ => GameResult::Stand,
        }
    }
}
