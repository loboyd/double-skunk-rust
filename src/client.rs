/**
 * Implements the `Client` which  holds the user-interface and probably
 * eventually some other stuff.
 */

use crate::ui;
use crate::game;
use crate::bogus_opponent;

pub struct Client<U: ui::UserInterface + Copy> {
    user_interface: U,
}

impl<U: ui::UserInterface + Copy> Client<U> {

    pub fn new(user_interface: U) -> Client<U> {
        Client{
            user_interface: user_interface,
        }
    }

    /**
     * Implement the main client loop
     */
    pub fn run(&self){
        loop {
            // TODO: Learn how to handle errors here, e.g., resulting from a
            // user passing in an empty string
            match self.user_interface.main_menu() {
                1 => self.play_game(),
                2 => std::process::exit(0),
                _ => println!("great, now we're lost ... "),
            };
        }
    }

    /**
     * Implement the main game loop
     */
    fn play_game(&self) {
        // create a new opponent struct
        let opponent = bogus_opponent::BogusOpponent{};

        // create new Game struct
        let mut game = game::Game::new(
            opponent,
            self.user_interface
        );

        game.play();
    }
}
