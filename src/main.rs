use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card)
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    fn deal(&mut self, num: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num)
    }
}
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // probably need to add error handling!!
    let cards = deck.deal(3);
    
    println!("Here your hand: {:#?}", cards);
    println!("Here your deck: {:#?}", deck);
}
