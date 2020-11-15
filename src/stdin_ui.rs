use std::io::{stdin, stdout, Write};

use crate::ui;

pub struct StdinInterface {
    pub width:  u8,
    pub height: u8
}

impl ui::UserInterface for StdinInterface {
    fn main_menu(&self) -> u8 {
        println!("1. Play, 2. Quit");

        let s = StdinInterface::input();
        return s.parse::<u8>().unwrap();
    }
}

impl StdinInterface {
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

