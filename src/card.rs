const RANKS: &[&str] = &[
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "X",
    "J",
    "Q",
    "K",
];

const SUITS: &[&str] = &[
    "S",
    "H",
    "C",
    "D",
];

pub struct Card {
    sort_order: u8,
    pub rank:   &'static str,
    pub suit:   &'static str,
}

impl Card {
    pub fn new(sort_order: u8) -> Card {
        Card {
            sort_order: sort_order,
            rank: Card::get_rank(sort_order),
            suit: Card::get_suit(sort_order),
        }
    }

    fn get_rank(sort_order: u8) -> &'static str {
        RANKS[(sort_order%13) as usize]
    }

    fn get_suit(sort_order: u8) -> &'static str {
        SUITS[(sort_order/13) as usize]
    }
}

