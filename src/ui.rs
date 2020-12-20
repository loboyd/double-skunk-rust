use crate::card;

pub trait UserInterface {
    fn main_menu(&self) -> MainMenu;

    //fn display_first_dealer(self, first_dealer: bool);

    fn get_discard(&self, dealer: bool, hand: &mut Vec<card::Card>, starter: &card::Card)
        -> Vec::<card::Card>;

    fn draw_table(&self, dealer: bool, hand: &Vec::<card::Card>, starter: &card::Card);

    fn draw_opponent_hand(&self, n_cards: usize);

    fn draw_self_hand(&self, hand: &Vec::<card::Card>);

    fn draw_played_cards(&self, dealer: bool, played_cards: &Vec::<card::Card>);

    fn get_play_card(&self, hand: &mut Vec::<card::Card>) -> card::Card;

    fn select_card(&self, card: &card::Card, col: u16, row: u16);

    fn deselect_card(&self, card: &card::Card, col: u16, row: u16);
}

pub enum MainMenu {
    Play,
    Exit,
}

