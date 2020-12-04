/**
 * Implements the `Client` which holds the user-interface and probably
 * eventually some other stuff.
 */

use crate::ui;
use crate::game;
use crate::bogus_opponent;

pub struct Client<U: ui::UserInterface> {
    user_interface: U,
}

impl<U: ui::UserInterface> Client<U> {

    pub fn new(user_interface: U) -> Client<U> {
        Client{
            user_interface: user_interface,
        }
    }

    /**
     * Implement the main client loop
     */
    pub fn run(&mut self) {
        loop {
            // TODO: Learn how to handle errors here, e.g., resulting from a
            // user passing in an empty string

            let menu_result = self.user_interface.main_menu();

            match menu_result {
                ui::MainMenu::Play => self.play_game(),
                ui::MainMenu::Exit => break,
            }
        }
    }

    /**
     * Implement the main game loop
     */
    fn play_game(&mut self) {
        println!("Somebody wants to play");
        /* TODO: Uncomment this
        // create a new opponent struct
        let opponent = bogus_opponent::BogusOpponent{};

        // create new Game struct
        let mut game = game::Game::new(
            opponent,
            &mut self.user_interface,
        );

        game.play();
        */
    }
}
