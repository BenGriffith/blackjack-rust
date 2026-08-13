use crate::deck::Deck;
use crate::error::GameError;
use crate::message::{DEALER, Message, PLAYER, Prompt};
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
        Message::deal_card(PLAYER);
        player.hand.add_card(self.deck.remove_card()?);
        Message::deal_card(DEALER);
        dealer.hand.add_card(self.deck.remove_card()?);
        Message::deal_card(PLAYER);
        player.hand.add_card(self.deck.remove_card()?);
        Message::deal_card(DEALER);
        dealer.hand.add_card(self.deck.remove_card()?);
        Message::round_result(player, dealer, true);
        Ok(())
    }
}
