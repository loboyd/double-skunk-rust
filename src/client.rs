/**
 * Implements the `Client` which holds the user-interface and probably
 * eventually some other stuff.
 */

use crate::ui;
use crate::game;
use crate::bogus_opponent;

pub struct Client<U> {
    user_interface: U,
}

impl<U: ui::UserInterface + Clone> Client<U> {

    pub fn new(user_interface: U) -> Client<U> {
        Client{
            user_interface: user_interface,
        }
    }

    /**
     * Implement the main client loop
     */
    pub fn run(&self) {
        loop {
            let menu_result = self.user_interface.clone().main_menu();

            match menu_result {
                ui::MainMenu::Play => self.play_game(),
                ui::MainMenu::Exit => break,
            }
        }
    }

    /**
     * Implement the main game loop
     */
    fn play_game(&self) {
        // TODO: Eventually ask what type of opponent to use

        let opponent = bogus_opponent::BogusOpponent{hand: Vec::new(),};

        let mut game = game::Game::new(opponent, &self.user_interface);

        game.play();
    }
}

