use crate::card;

pub trait UserInterface {
    fn main_menu(&self) -> MainMenu;

    //fn display_first_dealer(self, first_dealer: bool);

    fn discard(&self, dealer: bool, hand: &Vec<card::Card>, starter: card::Card)
        -> (usize, usize);
}

pub enum MainMenu {
    Play,
    Exit,
}

