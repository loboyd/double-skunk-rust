use crate::card;

pub trait Opponent {
    fn determine_first_dealer(&self) -> bool;
    fn deal(&self, dealer: bool) -> (Vec<card::Card>, card::Card);
    fn discard(&self, dealer: bool, hand: Vec<card::Card>, ind1: u8, ind2: u8)
        -> (Vec<card::Card>, Option<Vec<card::Card>>);
}

