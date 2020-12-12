use crate::card;

pub trait Opponent {
    fn determine_first_dealer(&self) -> bool;

    fn deal(&self, dealer: bool) -> (Vec<card::Card>, card::Card);

    fn discard(
        &self,
        dealer: bool,
        hand: &mut Vec<card::Card>,
        ind1: usize,
        ind2: usize
    ) -> Option<Vec<card::Card>>;
}

