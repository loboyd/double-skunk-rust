use crate::card;

pub trait Opponent {
    fn determine_first_dealer(&self) -> bool;

    fn deal(&mut self, dealer: bool) -> (Vec<card::Card>, card::Card);

    fn get_play(&self) -> card::Card;

    fn send_play(&self, card: card::Card);

    fn get_hand(&self) -> Vec::<card::Card>;
}

