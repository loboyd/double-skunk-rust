use crate::opponent;
use crate::remote_opponent_channel;
use crate::card;

struct RemoteOpponent<T> {
    channel: T,
}

impl<T: remote_opponent_channel::Channel> Opponent for RemoteOpponent<T> {
    // TODO
    fn determine_first_dealer(&self) -> bool {
    }

    // TODO
    fn get_deal(&mut self, dealer: bool) -> (Vec<card::Card>, card::Card) {
    }

    // TODO
    fn get_play(&self) -> card::Card {
    }

    // TODO
    fn send_play(&self, card: card::Card) {
    }

    // TODO
    fn get_hand(&self) -> Vec::<card::Card> {
    }
}

