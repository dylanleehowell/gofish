use std::fmt::{self, Display, Formatter};

#[derive(
    Debug,
    Hash,
    Eq,
    PartialEq,
    strum_macros::EnumIter,
    strum_macros::Display,
    Copy,
    Clone,
    Ord,
    PartialOrd,
)]
pub enum PlayerName {
    YOU,
    FOE,
}

#[derive(Debug, Hash, Eq, PartialEq, strum_macros::EnumIter, Copy, Clone, Ord, PartialOrd)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let symbol = match self {
            Rank::ACE => "A",
            Rank::TWO => "2",
            Rank::THREE => "3",
            Rank::FOUR => "4",
            Rank::FIVE => "5",
            Rank::SIX => "6",
            Rank::SEVEN => "7",
            Rank::EIGHT => "8",
            Rank::NINE => "9",
            Rank::TEN => "10",
            Rank::JACK => "J",
            Rank::QUEEN => "Q",
            Rank::KING => "K",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Debug, strum_macros::EnumIter, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    SPADES,
    CLUBS,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::CLUBS => "♣",
            Suit::DIAMONDS => "♦",
            Suit::HEARTS => "♥",
            Suit::SPADES => "♠️",
        };
        write!(f, "{symbol}")
    }
}
