use crate::ui;
use crate::opponent;

pub struct Game<T: opponent::Opponent, U: ui::UserInterface> {
    opponent:       T,
    user_interface: U,
    first_dealer:   bool,
}

impl<T: opponent::Opponent, U: ui::UserInterface> Game<T, U> {
    pub fn new(opponent: T, user_interface: U) -> Game<T, U> {
        Game {
            opponent: opponent,
            user_interface: user_interface,
            first_dealer: false,
        }
    }

    pub fn play(&mut self) {
        
        println!("You are playing the game...");
        self.first_dealer = self.opponent.determine_first_dealer();
        self.user_interface.first_dealer(self.first_dealer);

        let dealer = if self.first_dealer {true} else {false};

        loop {
            // TODO: Where should the `dealer` state exist?
            let (hand, starter) = self.opponent.deal(dealer);

            // discard (get crib)
            //let (ind1, ind2) = self.user_interface.discard(hand, starter);
            let (ind1, ind2) = (0, 1); // TODO: Remove this

            let (hand, crib) = self.opponent.discard(dealer, hand, ind1, ind2);
            match crib {
                Some(crib) => println!("got the crib"),
                None       => println!("gotn't the crib"),
            }

            // TODO: pegging phase
            //peg(hand, starter);

            // TODO: get opponents hand+crib

            // TODO: count hands

            // TODO: check game termination

            break;
        }
    }
}

