use crate::card::Card;
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

#[derive(Debug, Clone, PartialEq)]
enum GameResult {
    Blackjack,
    Bust,
    Move,
}

enum PlayerMove {
    Hit,
    Stand,
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
        let first_round = self.first_round(player, dealer)?;
        // let is_last_round = matches!(first_round, GameResult::Move);

        match first_round {
            GameResult::Blackjack => Message::player_blackjack(player),
            GameResult::Bust => Message::player_bust(),
            GameResult::Move => {
                let last_round = self.last_round(player)?;
                match last_round {
                    GameResult::Blackjack => Message::player_blackjack(player),
                    GameResult::Bust => Message::player_bust(),
                    GameResult::Move => {
                        self.compare_player_dealer_cards(player, dealer);
                    }
                }
            }
        }

        Ok(())
    }

    fn first_round(
        &mut self,
        player: &mut Player,
        dealer: &mut Dealer,
    ) -> Result<GameResult, GameError> {
        let player_str = player.to_string();
        let dealer_str = dealer.to_string();

        self.player_bet(player)?;
        player.hand.add_card(self.deal_card(&player_str)?);
        dealer.hand.add_card(self.deal_card(&dealer_str)?);
        player.hand.add_card(self.deal_card(&player_str)?);
        dealer.hand.add_card(self.deal_card(&dealer_str)?);

        self.print_round_summary(player, dealer, true);
        let result = self.process_person_hand(player);
        Ok(result)
    }

    fn last_round(&mut self, player: &mut Player) -> Result<GameResult, GameError> {
        let player_move = self.player_move()?;
        match player_move {
            PlayerMove::Hit => {
                player.hand.add_card(self.deal_card(&player.to_string())?);
                let game_result = self.process_person_hand(player);
                Ok(game_result)
            }
            PlayerMove::Stand => {
                let game_result = self.process_person_hand(player);
                Ok(game_result)
            }
        }
    }

    fn compare_player_dealer_cards(&self, player: &Player, dealer: &Dealer) {
        match player.hand.hand_value() {
            value if value > dealer.hand.hand_value() => Message::player_wins(player),
            value if value == dealer.hand.hand_value() => Message::tie(),
            _ => Message::dealer_wins(),
        }
    }

    fn player_bet(&self, player: &mut Player) -> Result<(), GameError> {
        let player_bet = Prompt::player_bet()?;
        player.place_bet(player_bet)?;
        Ok(())
    }

    fn deal_card(&mut self, person: &str) -> Result<Card, GameError> {
        Message::deal_card(person);
        let sleep = Duration::from_secs(DELAY);
        thread::sleep(sleep);
        let card = self.deck.remove_card()?;
        Ok(card)
    }

    fn print_round_summary(&self, player: &Player, dealer: &Dealer, first_round: bool) {
        Message::round_result(first_round);
        player.print_stats();
        player.hand.print_hand();
        dealer.print_stats();
    }

    fn process_person_hand(&self, player: &Player) -> GameResult {
        match player.hand.hand_value() {
            BLACKJACK => GameResult::Blackjack,
            value if value > BLACKJACK => GameResult::Bust,
            _ => GameResult::Move,
        }
    }

    fn player_move(&self) -> Result<PlayerMove, GameError> {
        let input = Prompt::player_move()?;
        let response = input.to_uppercase();
        match response.trim() {
            "H" | "HIT" => Ok(PlayerMove::Hit),
            _ => Ok(PlayerMove::Stand),
        }
    }
}
