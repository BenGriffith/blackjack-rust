use blackjack_rust::Game;
use blackjack_rust::GameError;
use blackjack_rust::{Dealer, Player};

fn main() -> Result<(), GameError> {
    let mut game = Game::new();
    if game.start()? {
        let mut player = Player::new();
        let mut dealer = Dealer::new();
        game.run(&mut player, &mut dealer)?;
        // match result {
        //     Ok(result) => println!("{}", result),
        //     Err(error) => eprintln!("{}", error),
        // }
    }

    Ok(())
}
