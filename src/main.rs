use blackjack_rust::Card;
use blackjack_rust::CardError;
use blackjack_rust::Deck;

fn main() {
    println!("Welcome to Blackjack!");
    let deck: Deck = Deck::new();
    let deck_cards: Vec<Result<Card, CardError>> = deck.cards;
    for card in &deck_cards {
        println!("{:?}", card);
    }
}
