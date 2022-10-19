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

#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Copy, Clone, Eq, PartialEq, Hash,
)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    SPADES,
    CLUBS,
}
