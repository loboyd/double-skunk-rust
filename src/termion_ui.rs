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
use std::io::{Write};

// TODO: Use or lose these
const CARD_HEIGHT: u16 = card::HEIGHT;
const CARD_WIDTH:  u16 = card::WIDTH;

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

    // Note the weird mod `ncards` stuff going on in this function is to ensure
    // we don't underflow the `usize` indices
    fn get_discard(&self, dealer: bool, hand: &mut Vec<card::Card>, starter: &card::Card)
        -> Vec::<card::Card>
    {
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let ncards = hand.iter().count();

        write!(stdout, "{}", Border{}).unwrap();

        let crib_message = if dealer {
             "You get the crib".to_string()
        } else {
             "Your opponent gets the crib".to_string()
        };

        let message_col = (78 - crib_message.chars().count() as u16) / 2;

        write!(stdout, "{}{}",
            termion::cursor::Goto(message_col, 9),
            crib_message).unwrap();

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

            if let termion::event::Key::Char('\n') = keystroke {
                if !raise_only {
                    if first_selection {
                        ind1 = curr;
                        first_selection = false;
                        curr = if ind1 == 0 {1} else {0};
                    } else {
                        ind2 = curr;
                        break;
                    }
                }

                // ensure a card has actually been raised before accepting commit card selection
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
                ).unwrap();

                // increment current index
                if let termion::event::Key::Right = keystroke {
                    if first_selection {
                        curr = (curr+1) % ncards;
                    } else {
                        let jump = (ncards + ind1 - curr) % ncards == 1;
                        curr = (curr + if jump {2} else {1}) % ncards;
                    }
                }

                // decrement current index
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
                    hand[curr],
                    termion::cursor::Goto(15 + (9 * curr) as u16, 19)
                ).unwrap();
            }

            stdout.flush().unwrap();
        }

        vec![
            hand.remove(std::cmp::max(ind1, ind2)),
            hand.remove(std::cmp::min(ind1, ind2))
        ]
    }

    fn draw_table(&self, dealer: bool, hand: &Vec::<card::Card>, starter: &card::Card) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        // draw border
        write!(stdout, "{}", Border{}).unwrap();

        self.draw_opponent_hand(4);

        self.draw_self_hand(hand);

        // draw starter
        write!(stdout, "{}{}",
            termion::cursor::Goto(69, 2),
            starter).unwrap();

        // draw crib
        let crib_row = if dealer {18} else {2};
        write!(stdout, "{}{}{}{}{}{}{}{}",
            termion::cursor::Goto(52, crib_row),
            card::Card::Facedown,
            termion::cursor::Goto(51, crib_row),
            card::Card::Facedown,
            termion::cursor::Goto(50, crib_row),
            card::Card::Facedown,
            termion::cursor::Goto(49, crib_row),
            card::Card::Facedown).unwrap();

        stdout.flush().unwrap();
    }

    fn draw_opponent_hand(&self, n_cards: usize) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}{} {} {} {}",
            termion::cursor::Goto(9, 2),
            if n_cards > 0 {card::Card::Facedown} else {card::Card::Empty},
            if n_cards > 1 {card::Card::Facedown} else {card::Card::Empty},
            if n_cards > 2 {card::Card::Facedown} else {card::Card::Empty},
            if n_cards > 3 {card::Card::Facedown} else {card::Card::Empty}
        ).unwrap();

        stdout.flush().unwrap();
    }

    fn draw_self_hand(&self, hand: &Vec::<card::Card>) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let n_cards = hand.iter().count();
        write!(stdout, "{}{} {} {} {}",
            termion::cursor::Goto(9, 18),
            if n_cards > 0 {hand[0]} else {card::Card::Empty},
            if n_cards > 1 {hand[1]} else {card::Card::Empty},
            if n_cards > 2 {hand[2]} else {card::Card::Empty},
            if n_cards > 3 {hand[3]} else {card::Card::Empty}
        ).unwrap();

        stdout.flush().unwrap();
    }

    // TODO: Test this
    fn draw_played_cards(&self, dealer: bool, played_cards: &Vec::<card::Card>) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let mut my_card = !dealer;

        for (ind, card) in played_cards.iter().enumerate() {
            let row = if my_card {11} else {9};
            let col = 4 + 9 * ind;
            write!(stdout, "{}{}",
                termion::cursor::Goto(col as u16, row),
                card
            ).unwrap();
            my_card = !my_card;
        }

        stdout.flush().unwrap();
    }

    fn get_play_card(&self, hand: &mut Vec::<card::Card>) -> card::Card {
        let stdin  = std::io::stdin();
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        let ncards = hand.iter().count();

        let mut ind: usize = 0; // current card
        let mut raise_only = true;
        for c in stdin.keys() {
            let keystroke = c.unwrap();

            if let termion::event::Key::Char('\n') = keystroke {
                if !raise_only {
                    raise_only = false; // TODO: Do we need this line?

                    self.deselect_card(&hand[ind], 9 + (9 * ind) as u16, 18);

                    break;
                }

                // ensure a card has actually been raised before accepting commit card selection
                raise_only = true;
            }

            else if raise_only {
                write!(stdout, "{}{}{}        ",
                    termion::cursor::Goto(9 + (9 * ind) as u16, 17),
                    hand[ind],
                    termion::cursor::Goto(9 + (9 * ind) as u16, 23)
                ).unwrap();

                raise_only = false;
            }

            // update card selection
            else {
                self.deselect_card(&hand[ind], 9 + (9 * ind) as u16, 18);

                // increment current index
                if let termion::event::Key::Right = keystroke {
                    ind = (ind+1) % ncards;
                }

                // decrement indent index
                else if let termion::event::Key::Left = keystroke {
                    ind = (ncards + ind - 1) % ncards;
                }

                self.select_card(&hand[ind], 9 + (9 * ind) as u16, 18);
            }

            stdout.flush().unwrap();
        }

        hand.remove(ind)
    }

    fn select_card(&self, card: &card::Card, col: u16, row: u16) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}{}{}        ",
            termion::cursor::Goto(col, row-1),
            card,
            termion::cursor::Goto(col, row+5)
        ).unwrap();

        stdout.flush().unwrap();
    }

    fn deselect_card(&self, card: &card::Card, col: u16, row: u16) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(stdout, "{}        {}{}",
            termion::cursor::Goto(col, row-1),
            termion::cursor::Goto(col, row),
            card
        ).unwrap();

        stdout.flush().unwrap();
    }

    fn draw_scores(&self, self_score: u16, opp_score: u16) {
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();

        write!(
            stdout, "{} you {:3}{}them {:3}",
            termion::cursor::Goto(71, 22),
            self_score,
            termion::cursor::Goto(71, 23),
            opp_score
        ).unwrap();

        stdout.flush().unwrap();
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

