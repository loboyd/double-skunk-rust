/**
 * A simple STDIN-based user interface to implement `ui::UserInterface`. To be
 * used for development purposes, and ultimately to be replaced by a nice
 * implementation using `ncurses` or similar.
 */

use std::io::{stdin, stdout, Write};

use crate::ui;

pub struct StdinInterface {
    pub width:  u8,
    pub height: u8
}

impl ui::UserInterface for StdinInterface {
    //fn main_menu(self) -> (ui::MainMenu, Self) {
    fn main_menu(&mut self) -> ui::MainMenu {
        println!("1. Play, 2. Quit");

        let s = input();
        match s.parse::<u8>().unwrap() {
            1 => ui::MainMenu::Play,
            _ => ui::MainMenu::Exit,
        }
        //return (ui::MainMenu::Play, self);
    }

    fn display_first_dealer(&self, first_dealer: bool) {
        match first_dealer {
            true => println!("You're the first dealer"),
            false => println!("You're opponent is the first dealer"),
        }
    }
}

fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

/*
fn clear_screen() {
    for _ in 0..100 {
        println!("\n")
    }
}

fn print_title_bar() {
    println!("===double-skunk==========================")
}

*/

