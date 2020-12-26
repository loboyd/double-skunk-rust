/**
 * BogusOpponent serves as a bare-bones "stupid" automatic opponent, meaning it
 * satisfies the `Opponent` trait. It's intended to be used to aid development.
 */

//use crate::ui;
use crate::card;
use crate::opponent;
use crate::card::{Card, Rank, Suit};

use std::{thread, time};
use rand::{thread_rng, seq};

pub struct BogusOpponent {
    pub hand: Vec::<card::Card>,
}

impl opponent::Opponent for BogusOpponent {
    fn determine_first_dealer(&self) -> bool {
        true
    }

    fn deal(&mut self, dealer: bool) -> (Vec<Card>, Card) {
        let mut rng = thread_rng();
        let sampled_cards: Vec<u16> = seq::sample_iter(&mut rng, 0..51, 13).unwrap();

        self.hand = sampled_cards[..6].into_iter().map(|x| Card::from_int(*x)).collect();

        (
            sampled_cards[6..12].into_iter().map(|x| Card::from_int(*x)).collect(),
            Card::from_int(sampled_cards[12])
        )
    }

    fn get_play(&self) -> Card {
        thread::sleep(time::Duration::from_millis(500));
        Card::Value(Rank::Ace, Suit::Hearts)
    }

    fn send_play(&self, card: card::Card) {
    }

    fn get_hand(&self) -> Vec::<card::Card> {
        self.hand.clone()
    }
}

