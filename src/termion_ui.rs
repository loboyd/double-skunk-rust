/**
 * A simple STDIN-based user interface to implement `ui::UserInterface`. To be
 * used for development purposes, and ultimately to be replaced by a nice
 * implementation using `ncurses` or similar.
 */

use crate::ui;
use crate::tinput;
use tinput::TermRead;

// TODO: make this better
use termion::event::Key;
//use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::io::{Write, stdin, Stdin, stdout, Stdout}; // TODO: Is Write needed?

pub struct UI {
    stdin:  std::io::Stdin,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl UI {
    pub fn new() -> UI {
        UI {
            stdin:  stdin(),
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    fn clean_up(&mut self) {
        drop(&self.stdout);
    }
}

impl ui::UserInterface for UI {
    fn main_menu(&mut self) -> ui::MainMenu {
        write!(self.stdout, "{}{}Welcome to double-skunk!{}",
            termion::clear::All,
            termion::cursor::Goto(30, 6),
            termion::cursor::Hide).unwrap();

        write!(self.stdout, "{}<Enter> to play{}<Esc>   to exit{}",
            termion::cursor::Goto(35, 12),
            termion::cursor::Goto(35, 13),
            termion::cursor::Hide).unwrap();
        self.stdout.flush().unwrap();

        for c in self.stdin.keys() {
            match c.unwrap() {
                //Key::Char('q') => break,
                Key::Char('\n') => return ui::MainMenu::Play,
                Key::Esc        => return ui::MainMenu::Exit,
                _               => println!("unrecognized"), // TODO: Will this cause problems?
            }
        }

        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        return ui::MainMenu::Exit;
    }

    fn display_first_dealer(&mut self, first_dealer: bool) {
    }

    /*
    fn first_dealer(&self, first_dealer: bool) {
        match first_dealer {
            true => println!("You're the first dealer"),
            false => println!("You're opponent is the first dealer"),
        }
    }
    */
}

