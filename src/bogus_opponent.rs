/**
 * BogusOpponent serves as a bare-bones "stupid" automatic opponent, meaning it
 * satisfies the `Opponent` trait. It's intended to be used to aid development.
 */

//use crate::ui;
use crate::opponent;
use crate::card;

pub struct BogusOpponent { }

// TODO: Find a way to implement this as a generic over the user interface
// (otherwise, find a better way to solve this problem)
impl opponent::Opponent for BogusOpponent {
    fn determine_first_dealer(&self) -> bool {
        false
    }

    fn deal(&self, dealer: bool) -> (Vec<card::Card>, card::Card) {
        (vec![
            card::Card::new(0),
            card::Card::new(1),
            card::Card::new(2),
            card::Card::new(3),
            card::Card::new(4),
            card::Card::new(5)],
        card::Card::new(6))
    }

    fn discard(&self, dealer: bool, hand: Vec<card::Card>, ind1: u8, ind2: u8)
        -> (Vec<card::Card>, Option<Vec<card::Card>>) {

        let mut new_hand = Vec::new();
        // TODO: Write this in a better way
        for (ind, card) in hand.iter().enumerate() {
            if ind != ind1 as usize && ind != ind2 as usize {
                new_hand.push(card);
            }
        }

        if dealer {
            (hand, Some(vec![
                card::Card::new(0),
                card::Card::new(1),
                card::Card::new(2),
                card::Card::new(3)]))
        } else {
            (hand, None)
        }
    }
}

