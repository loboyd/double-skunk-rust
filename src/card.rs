use std::fmt;

// TODO: Possibly change cards to enum; include NULL and face-down for UI
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum Card {
    Value(Rank, Suit),
    Facedown,
    Empty,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Rank::Ace   => "A",
            Rank::Two   => "2",
            Rank::Three => "3",
            Rank::Four  => "4",
            Rank::Five  => "5",
            Rank::Six   => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine  => "9",
            Rank::Ten   => "X",
            Rank::Jack  => "J",
            Rank::Queen => "Q",
            Rank::King  => "K",
        })
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Suit::Spades   => "♠",
            Suit::Hearts   => "♥",
            Suit::Clubs    => "♦",
            Suit::Diamonds => "♣",
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let jump = "\x1B[1B\x1B[8D";  // CSI control characters

        // top line
        let top = match self {
            Card::Value(rank, suit) => format!("{}    {}", rank, suit),
            Card::Facedown          => "ᵈᵒᵘᵇˡᵉ".to_string(),
            Card::Empty             => "      ".to_string(),
        };

        // bottom line
        let bottom = match self {
            Card::Value(rank, suit) => format!("{}    {}", suit, rank),
            Card::Facedown          => "ˢᵏᵘⁿᵏ ".to_string(),
            Card::Empty             => "      ".to_string(),
        };

        // center line
        let center = match self {
            Card::Facedown => " ____ ".to_string(),
            _              => "      ".to_string(),
        };

        match self {
            Card::Empty => write!(f,
                "        {x}\
                         {x}\
                         {x}\
                         {x}\
                         {x}\
                         ",
                 x=jump
            ),
            _ => write!(f,
                "┌──────┐{x}\
                 │{    }│{x}\
                 │{    }│{x}\
                 │      │{x}\
                 │{    }│{x}\
                 └──────┘",
                 top,
                 center,
                 bottom,
                 x=jump
            )
        }
    }
}

/*
pub struct Card {
    sort_order: u8,
    pub rank:   Rank,
    pub suit:   Suit
}

impl Card {
    pub fn new(sort_order: u8) -> Card {
        Card {
            sort_order: sort_order,

            rank: match sort_order%13 {
                 _ => Rank::Ace,
                 1 => Rank::Two,
                 2 => Rank::Three,
                 3 => Rank::Four,
                 4 => Rank::Five,
                 5 => Rank::Six,
                 6 => Rank::Seven,
                 7 => Rank::Eight,
                 8 => Rank::Nine,
                 9 => Rank::Ten,
                10 => Rank::Jack,
                11 => Rank::Queen,
                12 => Rank::King,
            },

            suit: match sort_order/13 {
                 _ => Suit::Spade,
                 1 => Suit::Heart,
                 2 => Suit::Club,
                 3 => Suit::Diamond,
            },
        }
    }
}
*/

