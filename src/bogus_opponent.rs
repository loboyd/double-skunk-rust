/**
 * BogusOpponent serves as a bare-bones "stupid" automatic opponent, meaning it
 * satisfies the `Opponent` trait. It's intended to be used to aid development.
 */

//use crate::ui;
use crate::opponent;
use crate::card::{Card, Rank, Suit};

pub struct BogusOpponent { }

// TODO: Find a way to implement this as a generic over the user interface
// (otherwise, find a better way to solve this problem)
impl opponent::Opponent for BogusOpponent {
    fn determine_first_dealer(&self) -> bool {
        true
    }

    fn deal(&self, dealer: bool) -> (Vec<Card>, Card) {
        (vec![
            Card::Value(Rank::Ace, Suit::Spades),
            Card::Value(Rank::Two, Suit::Spades),
            Card::Value(Rank::Three, Suit::Spades),
            Card::Value(Rank::Four, Suit::Spades),
            Card::Value(Rank::Five, Suit::Spades),
            Card::Value(Rank::Six, Suit::Spades)],
        Card::Value(Rank::Seven, Suit::Spades))
    }

    fn discard( &self, dealer: bool, discarded: Vec::<Card>) -> Option<Vec<Card>>
    {
        if dealer {
            Some(vec![
                Card::Value(Rank::Three, Suit::Spades),
                Card::Value(Rank::Four, Suit::Spades),
                Card::Value(Rank::Five, Suit::Spades),
                Card::Value(Rank::Six, Suit::Spades)]
            )
        } else {
            None
        }
    }

    fn get_play(&self) -> Card {
        Card::Value(Rank::Ace, Suit::Hearts)
    }
}

