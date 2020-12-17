//use crate::card;
use crate::ui;
use crate::opponent;

pub struct Game<'a, T, U> {
    opponent:       T,
    user_interface: &'a U,
    first_dealer:   bool,
}

impl<'a, T: opponent::Opponent, U: ui::UserInterface> Game<'a, T, U> {
    pub fn new(opponent: T, user_interface: &'a U) -> Game<'a, T, U> {
        Game {
            opponent: opponent,
            user_interface: user_interface,
            first_dealer: false,
        }
    }

    pub fn play(&mut self) {
        
        self.first_dealer = self.opponent.determine_first_dealer();
        //self.user_interface.display_first_dealer(self.first_dealer);

        let dealer = if self.first_dealer {true} else {false};

        loop {
            // TODO: Where should the `dealer` state exist?
            let (mut hand, starter) = self.opponent.deal(dealer);

            // discard (get crib)
            let (ind1, ind2) = self.user_interface.discard(
                dealer,
                &hand,
                starter
            );

            let crib = self.opponent.discard(dealer, &mut hand, ind1, ind2);
            /*
            match crib {
                Some(crib) => print_crib(crib),
                None       => println!("gotn't the crib"),
            }
            */

            // TODO: pegging phase
            //peg(hand, starter);

            // TODO: get opponents hand+crib

            // TODO: count hands

            // TODO: check game termination

            break;
        }
    }
}

