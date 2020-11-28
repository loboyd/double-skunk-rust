/**
 * Implements the `Client` which  holds the opponent and the user-interface
 */

use crate::ui;
use crate::opponent;

pub struct Client<T: opponent::Opponent, U: ui::UserInterface> {
    pub opponent: T,
    pub user_interface: U,
    
    first_dealer: bool,
}

impl<T: opponent::Opponent, U: ui::UserInterface> Client<T, U> {

    pub fn new(opponent: T, user_interface: U) -> Client<T, U> {
        Client{
            opponent: opponent,
            user_interface: user_interface,
            first_dealer: false,
        }
    }

    /**
     * Implement the main client loop
     */
    pub fn run(&mut self) {
        loop {
            // TODO: Learn how to handle errors here, e.g., resulting from a
            // user passing in an empty string
            match self.user_interface.main_menu() {
                //1 => println!("somebody's looking for a good time"),
                1 => self.play_game(),
                2 => std::process::exit(0),
                _ => println!("great, now we're lost ... "),
            }
        }
    }

    /**
     * Implement the main game loop
     */
    fn play_game(&mut self) {
        self.first_dealer = self.opponent.determine_first_dealer();
        self.user_interface.first_dealer(self.first_dealer);

        loop {
            // TODO: Where should the `dealer` state exist?
            //let (_hand, _starter) = opp.deal();

            // TODO: discard (get crib)

            // TODO: pegging phase

            // TODO: get opponents hand+crib

            // TODO: count hands

            // TODO: check game termination

            break;
        }
    }
}
