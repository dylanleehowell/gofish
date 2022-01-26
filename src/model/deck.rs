use super::{card::Card, rank::Rank, suit::Suit};
use rand::{seq::SliceRandom, thread_rng};
use strum::IntoEnumIterator;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deck_is_complete() {
        let some_cards = Deck::default().cards;
        assert_eq!(some_cards.len(), 52); // 4 suites * 13 ranks = 52 🧠.
        let rank_len = Rank::iter().len();
        assert_eq!(rank_len, 13);
        for suit in Suit::iter() {
            // 13 cards of each suit.
            let count = some_cards
                .iter()
                .filter(|card| card.suit == suit)
                .collect::<Vec<&Card>>()
                .len();
            assert_eq!(count, rank_len);
        }
        let suit_len = Suit::iter().len();
        assert_eq!(suit_len, 4);
        for rank in Rank::iter() {
            // 4 cards of each rank.
            let count = some_cards
                .iter()
                .filter(|card| card.rank == rank)
                .collect::<Vec<&Card>>()
                .len();
            assert_eq!(count, suit_len);
        }
    }
}
