#[derive(
    Debug, strum_macros::Display, strum_macros::EnumIter, Copy, Clone, Eq, PartialEq, Hash,
)]
pub enum Suit {
    HEARTS,
    DIAMONDS,
    SPADES,
    CLUBS,
}
