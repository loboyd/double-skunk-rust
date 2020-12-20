use crate::card;

pub trait Opponent {
    fn determine_first_dealer(&self) -> bool;

    fn deal(&self, dealer: bool) -> (Vec<card::Card>, card::Card);

    fn discard(&self, dealer: bool, discared: Vec::<card::Card>) -> Option<Vec<card::Card>>;

    fn get_play(&self) -> card::Card;
}

