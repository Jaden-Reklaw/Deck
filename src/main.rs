use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

// Create an inherit implementation
impl Deck {
    // Associate Function tied to the struct definition run when an new instance is created
    fn new() -> Self {
        // Create an array of suits of cards
        let suits = ["Hearts", "Spades", "Clubs", "Diamonds"];

        // Create an array of values of card
        let values = [
            "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
            "Queen", "King",
        ];

        // Create all cards to put in a deck
        let mut cards: Vec<String> = vec![];

        // Double for loops to create all possible cards in a deck
        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        // explicit return
        //return Deck { cards };
        // implicit return simply doesn't have the ;
        Deck { cards }
        
    }

    // Methods that will operate on a specific instance

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn check_deck(&self) {
        println!("Deck: {:#?}", &self);
    }
}

fn main() {

    let mut deck = Deck::new();

    deck.shuffle();

    deck.check_deck();
}
