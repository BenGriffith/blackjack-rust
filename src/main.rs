use blackjack_rust::Card;
use blackjack_rust::Deck;
fn main() {
    println!("Welcome to Blackjack!");
    let deck: Deck = Deck::new();
    let deck_cards: Vec<Card> = deck.cards;
    for card in &deck_cards {
        println!("{:?}", card);
    }
}
