use super::{rank::Rank, suit::Suit};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
