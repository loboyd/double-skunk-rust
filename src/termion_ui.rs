/**
 * A simple STDIN-based user interface to implement `ui::UserInterface`. To be
 * used for development purposes, and ultimately to be replaced by a nice
 * implementation using `ncurses` or similar.
 */

use crate::ui;
use crate::card;

// TODO: make this better
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode};

use std::fmt;
use std::io::{Write}; // TODO: Is Write needed?
use std::process::exit; // TODO: Remove this

#[derive(Clone)]
pub struct UI { }

impl Drop for UI {
    fn drop(&mut self) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show).unwrap();
    }
}

impl ui::UserInterface for UI {
    fn main_menu(&self) -> ui::MainMenu {
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}{}Welcome to double-skunk!{}",
            Border{},
            termion::cursor::Goto(29, 6),
            termion::cursor::Hide).unwrap();

        write!(stdout, "{}<Enter> to play{}<Esc>   to exit{}",
            termion::cursor::Goto(34, 12),
            termion::cursor::Goto(34, 14),
            termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => return ui::MainMenu::Play,
                Key::Esc        => return ui::MainMenu::Exit,
                _               => continue,
            }
        }

        ui::MainMenu::Exit
    }

    // TODO: Inform the user of who has the grib
    // Note the weird mod `ncards` stuff going on in this function is to ensure
    // we don't underflow the `u8` indices
    fn discard(&self, dealer: bool, hand: &Vec<card::Card>, starter: card::Card) -> (usize, usize)
    {
        let ncards = hand.len();
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}", Border{}).unwrap();

        let crib_message = if dealer {
             "You get the crib".to_string()
        } else {
             "You're opponent gets the crib".to_string()
        };

        let message_col = (78 - crib_message.chars().count() as u16) / 2;

        write!(stdout, "{}{}",
            termion::cursor::Goto(message_col, 9),
            crib_message);

        // TODO: Make 15 a variable
        write!(stdout, "{}{} {} {} {} {} {}{}",
            termion::cursor::Goto(15, 14),
            hand[0],
            hand[1],
            hand[2],
            hand[3],
            hand[4],
            hand[5],
            termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();

        let mut ind1: usize = 0;
        let mut ind2: usize = 0;
        let mut curr: usize = 0; // current card
        let mut first_selection = true;
        let mut raise_only = true;
        for c in stdin.keys() {
            let keystroke = c.unwrap();

            // TODO: ensure a card has actually been raised before accepting
            // commit card selection
            if let termion::event::Key::Char('\n') = keystroke {
                if first_selection {
                    ind1 = curr;
                    first_selection = false;
                    curr = if ind1 == 0 {1} else {0};
                } else {
                    ind2 = curr;
                    break;
                }

                raise_only = true;
            }

            else if raise_only {
                write!(stdout, "{}{}{}        ",
                    termion::cursor::Goto(15 + (9 * curr) as u16, 13),
                    hand[curr],
                    termion::cursor::Goto(15 + (9 * curr) as u16, 19)
                ).unwrap();

                raise_only = false;
            }

            // update card selection
            else {
                // lower the current card
                write!(stdout, "{}        {}{}",
                    termion::cursor::Goto(15 + (9 * curr) as u16, 13),
                    termion::cursor::Goto(15 + (9 * curr) as u16, 14),
                    hand[curr]
                    //card::Card::Facedown
                ).unwrap();

                // update current index
                if let termion::event::Key::Right = keystroke {
                    if first_selection {
                        curr = (curr+1) % ncards;
                    } else {
                        let jump = (ncards + ind1 - curr) % ncards == 1;
                        curr = (curr + if jump {2} else {1}) % ncards;
                    }
                }

                else if let termion::event::Key::Left = keystroke {
                    if first_selection {
                        curr = (ncards + curr - 1) % ncards;
                    } else {
                        let jump = (ncards + curr - ind1) % ncards == 1;
                        curr = (ncards + curr - if jump {2} else {1}) % ncards;
                    }
                }

                // raise new selection
                write!(stdout, "{}{}{}        ",
                    termion::cursor::Goto(15 + (9 * curr) as u16, 13),
                    //card::Card::Facedown,
                    hand[curr],
                    termion::cursor::Goto(15 + (9 * curr) as u16, 19)
                ).unwrap();
            }

            stdout.flush().unwrap();
        }

        (ind1, ind2)
    }
}

struct Border {}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let origin  = "\x1B[1;1H"; // top left cell
        let newline = "\x1B[1E";

        let top = format!("╔══double-skunk{}╗", "═".repeat(64));
        let middle = format!("{}║{}║", newline, " ".repeat(78)).repeat(22);
        let bottom = format!("{}╚{}╝", newline, "═".repeat(78));
        write!(f, "{}{}{}{}{}",
            termion::clear::All,
            origin,
            top,
            middle,
            bottom
        )
    }
}

