use crate::ui;
use crate::opponent;
use crate::card;
use crate::hand::Hand;

pub struct Game<'a, T, U> {
    opponent:       T,
    user_interface: &'a U,
    first_dealer:   bool,
    self_score:     u16,
    opp_score:      u16,
}

impl<'a, T: opponent::Opponent, U: ui::UserInterface> Game<'a, T, U> {
    pub fn new(opponent: T, user_interface: &'a U) -> Game<'a, T, U> {
        Game {
            opponent: opponent,
            user_interface: user_interface,
            first_dealer: false,
            self_score: 0,
            opp_score: 0,
        }
    }

    pub fn play(&mut self) {
        
        self.first_dealer = self.opponent.determine_first_dealer();
        //self.user_interface.display_first_dealer(self.first_dealer);

        let dealer = if self.first_dealer {true} else {false};

        loop {
            let (mut hand, starter) = self.opponent.get_deal(dealer);

            // TODO: Count nibs for dealer

            // discard (get crib)
            let discarded = self.user_interface.get_discard(
                dealer,
                &mut hand,
                &starter
            );

            /*
            let crib = self.opponent.discard(dealer, discarded);
            match crib {
                Some(crib) => print_crib(crib),
                None       => println!("gotn't the crib"),
            }
            */

            // TODO: pegging phase
            self.peg(dealer, hand.clone(), &starter);

            // TODO: get opponents hand+crib
            let opp_hand = self.opponent.get_hand();

            // TODO: count hands
            self.self_score += hand.score(starter);
            self.opp_score  += opp_hand.score(starter);

            // TODO: check game termination

            break;
        }
    }

    fn peg(&mut self, dealer: bool, mut hand: Vec<card::Card>, starter: &card::Card) {
        let mut played: Vec<card::Card> = Vec::new();

        self.user_interface.draw_table(dealer, &hand, &starter);

        let mut to_play = !dealer;
        while played.iter().count() < 8 {
            if to_play {
                // TODO: Handle GO
                // select card
                let play = self.user_interface.get_play_card(&mut hand);
                played.push(play);

                // update table UI
                self.user_interface.draw_played_cards(dealer, &played);
                self.user_interface.draw_self_hand(&hand);

                // send card info to opponent
                self.opponent.send_play(play);

                // update score
                self.self_score += played.score_play(play);
            } else {
                // get card data from opponent
                let play = self.opponent.get_play();
                played.push(play);

                // update table UI
                let n_opp_cards = 8-played.iter().count() - hand.iter().count();
                self.user_interface.draw_opponent_hand(n_opp_cards);
                self.user_interface.draw_played_cards(dealer, &played);

                // update score
                self.opp_score += played.score_play(play);
            }

            self.user_interface.draw_scores(self.self_score, self.opp_score);

            to_play = !to_play;
        }
    }
}

