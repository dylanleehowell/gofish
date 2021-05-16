use super::{card::Card, rank::Rank, suit::Suit};
use strum::IntoEnumIterator;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Default for Deck {
    fn default() -> Deck {
        let mut deck = Deck { cards: Vec::new() };
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                deck.cards.push(Card { suit, rank });
            }
        }
        deck.cards.shuffle(&mut thread_rng());
        return deck;
    }
}
