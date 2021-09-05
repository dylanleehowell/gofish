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
