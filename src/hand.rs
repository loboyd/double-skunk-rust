use std::fmt;

use crate::card;

pub trait Hand {
    fn score(&self, starter: card::Card) -> u16;

    fn score_fifteens(&self) -> u16;

    fn count_sums(&self, value: u16, ind: usize) -> Option<u16>;

    fn score_runs(&self) -> u16;

    fn score_pairs(&self) -> u16;

    fn score_flush(&self) -> u16;

    fn score_nobs(&self, starter: card::Card) -> u16;

    // TODO: Name things better--this take a table, not a hand
    fn score_play(&self, play: card::Card) -> u16;
}

impl Hand for Vec::<card::Card> {
    fn score(&self, starter: card::Card) -> u16 {
        // count nobs first, then combine starter with hand
        let nobs_score = self.score_nobs(starter);

        let mut fullhand = self.clone();
        fullhand.push(starter);

        nobs_score + 
        fullhand.score_fifteens() +
        fullhand.score_runs() +
        fullhand.score_pairs() +
        fullhand.score_flush()
    }

    fn score_fifteens(&self) -> u16 {
        2 * self.count_sums(15, 0).unwrap()
    }

    fn count_sums(&self, value: u16, ind: usize) -> Option<u16> {
        if value == 0 {
            Some(0)
        }

        else if ind >= self.len() {
            None
        }

        else if ind == self.len() - 1 {
            if self[ind].val() == value {
                Some(1)
            } else {
                Some(0)
            }
        }

        else {
            let ind_card = self[ind].val();
            let count_with = self.count_sums(value-ind_card, ind+1).unwrap();
            let count_without = self.count_sums(value, ind+1).unwrap();
            Some(count_with + count_without)
        }
    }

    // TODO
    fn score_runs(&self) -> u16 {
        0
    }

    // TODO: Test this
    fn score_pairs(&self) -> u16 {
        let mut score = 0;
        for (i, card1) in self.iter().enumerate() {
            for (j, card2) in self.iter().enumerate() {
                if i != j && card1.get_rank().unwrap() == card2.get_rank().unwrap() {
                    score += 2;
                }
            }
        }

        score
    }

    // TODO Test this
    fn score_flush(&self) -> u16 {
        let n_cards = self.iter().count();

        if n_cards < 4 {
            return 0
        } else {
            let suit = self[0].get_suit();
            for card in self {
                if card.get_suit() != suit {
                    return 0
                }
            }
        }

        n_cards as u16
    }

    // TODO Test this
    fn score_nobs(&self, starter: card::Card) -> u16 {
        let turn_up_suit = starter.get_suit();

        for card in self {
            match card {
                card::Card::Value(
                    card::Rank::Jack,
                    turn_up_suit
                ) => return 1,
                _ => continue,
            }
        }
        0
    }

    // TODO
    fn score_play(&self, play: card::Card) -> u16 {
        1
    }
}

