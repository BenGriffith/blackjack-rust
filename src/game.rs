use crate::deck::Deck;
use crate::error::GameError;
use crate::message::{Message, Prompt};
use crate::person::{Dealer, Player};

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

    fn first_round(&mut self, player: &mut Player, dealer: &mut Dealer) -> Result<(), GameError> {
        let player_str = player.to_string();
        let dealer_str = dealer.to_string();

        Message::deal_card(&player_str);
        player.hand.add_card(self.deck.remove_card()?);

        Message::deal_card(&dealer_str);
        dealer.hand.add_card(self.deck.remove_card()?);

        Message::deal_card(&player_str);
        player.hand.add_card(self.deck.remove_card()?);

        Message::deal_card(&dealer_str);
        dealer.hand.add_card(self.deck.remove_card()?);

        Message::round_result(player, dealer, true);
        Message::process_blackjack(&player_str, player, dealer);
        Ok(())
    }
}
