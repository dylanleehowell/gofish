use super::{rank::Rank, suit::Suit};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}
