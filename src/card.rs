use std::fmt;

pub const HEIGHT: u16 = 6;
pub const WIDTH:  u16 = 9;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
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

impl Rank {
    pub fn next(self) -> Option<Self> {
        match self {
            Rank::Ace   => Some(Rank::Two),
            Rank::Two   => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four  => Some(Rank::Five),
            Rank::Five  => Some(Rank::Six),
            Rank::Six   => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine  => Some(Rank::Ten),
            Rank::Ten   => Some(Rank::Jack),
            Rank::Jack  => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King  => None,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Card {
    Value(Rank, Suit),
    Facedown,
    Empty,
}

impl Card {
    pub fn get_rank(self) -> Option<Rank>{
        match self {
            Card::Value(rank, _) => Some(rank),
            _                    => None,
        }
    }

    pub fn get_suit(self) -> Option<Suit> {
        match self {
            Card::Value(_, suit) => Some(suit),
            _                    => None,
        }
    }

    pub fn val(self) -> u16 {
        match self {
            Card::Facedown => 0,
            Card::Empty => 0,
            Card::Value(Rank::Ace,   _) => 1,
            Card::Value(Rank::Two,   _) => 2,
            Card::Value(Rank::Three, _) => 3,
            Card::Value(Rank::Four,  _) => 4,
            Card::Value(Rank::Five,  _) => 5,
            Card::Value(Rank::Six,   _) => 6,
            Card::Value(Rank::Seven, _) => 7,
            Card::Value(Rank::Eight, _) => 8,
            Card::Value(Rank::Nine,  _) => 9,
            Card::Value(Rank::Ten,   _) => 10,
            Card::Value(Rank::Jack,  _) => 10,
            Card::Value(Rank::Queen, _) => 10,
            Card::Value(Rank::King,  _) => 10,
        }
    }
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
            Suit::Clubs    => "♣",
            Suit::Diamonds => "♦",
        })
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let jump = "\x1B[1B\x1B[8D";  // CSI control characters
        let next_cursor = "\x1B[5A";

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
                "{w}{x}{w}{x}{w}{x}{w}{x}{w}{x}{w}{y}",
                 x=jump,
                 y=next_cursor,
                 w="        "
            ),
            _ => write!(f,
                "┌──────┐{x}\
                 │{    }│{x}\
                 │{    }│{x}\
                 │      │{x}\
                 │{    }│{x}\
                 └──────┘{y}",
                 top,
                 center,
                 bottom,
                 x=jump,
                 y=next_cursor
            )
        }
    }
}

