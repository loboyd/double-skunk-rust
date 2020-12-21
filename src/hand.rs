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
}

impl Hand for Vec::<card::Card> {
    // TODO
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

    // TODO
    fn score_flush(&self) -> u16 {
        0
    }

    // TODO
    fn score_nobs(&self, starter: card::Card) -> u16 {
        0
    }
}

