/**
 * BogusOpponent serves as a bare-bones "stupid" automatic opponent, meaning it
 * satisfies the `Opponent` trait. It's intended to be used to aid development.
 */

use crate::opponent;

pub struct BogusOpponent {
}

// TODO: Find a way to implement this as a generic over the user interface
// (otherwise, find a better way to solve this problem)
impl opponent::Opponent for BogusOpponent {
    fn determine_first_dealer(&self) -> u8 {
        return 1;
    }
}

