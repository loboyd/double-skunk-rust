/**
 * A simple STDIN-based user interface to implement `ui::UserInterface`. To be
 * used for development purposes, and ultimately to be replaced by a nice
 * implementation using `ncurses` or similar.
 */

use crate::ui;

// TODO: make this better
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use std::io::{Write, stdin, Stdin, stdout, Stdout}; // TODO: Is Write needed?

#[derive(Clone)]
pub struct UI {
    //stdin:  std::io::Stdin,
    //stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Drop for UI {
    fn drop(&mut self) {
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show).unwrap();
    }
}

impl ui::UserInterface for UI {
    fn main_menu(self) -> ui::MainMenu {
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}{}Welcome to double-skunk!{}",
            termion::clear::All,
            termion::cursor::Goto(30, 6),
            termion::cursor::Hide).unwrap();

        write!(stdout, "{}<Enter> to play{}<Esc>   to exit{}",
            termion::cursor::Goto(35, 12),
            termion::cursor::Goto(35, 13),
            termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                //Key::Char('q') => break,
                Key::Char('\n') => return ui::MainMenu::Play,
                Key::Esc        => return ui::MainMenu::Exit,
                _               => println!("unrecognized"), // TODO: Will this cause problems?
            }
        }

        //write!(stdout, "{}\n", termion::cursor::Show).unwrap();
        return ui::MainMenu::Exit;
    }
}

