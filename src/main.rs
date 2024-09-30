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

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }

    fn check_deck(&self) {
        println!("Deck: {:#?}", &self);
    }

    fn check_hand(&self, hand: &Vec<String>) {
        println!("Hand: {:#?}", hand);
    }
}

fn main() {

    let mut deck = Deck::new();

    deck.shuffle();

    let mut hand = deck.deal(5);

    deck.check_deck();
    deck.check_hand(&hand);
}
