use crate::model::card::Card;
use crate::model::enums::Rank;

pub fn go_fish(
    asking: &mut Vec<Card>,
    deck_cards: &mut Vec<Card>,
    target_rank: Option<&Rank>,
) -> bool {
    println!("Go fish!");
    if let Some(card) = deck_cards.pop() {
        asking.push(card);
        return match target_rank {
            Some(target) => target == &card.rank,
            None => false,
        };
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::enums::Suit;

    #[test]
    fn go_fish_not_caught() {
        let mut deck_cards = vec![Card {
            rank: Rank::TWO,
            suit: Suit::SPADES,
        }];
        let mut asking: Vec<Card> = Vec::new();
        let target = Some(&Rank::ACE);
        let result = go_fish(&mut asking, &mut deck_cards, target);
        assert_eq!(result, false);
    }

    #[test]
    fn go_fish_caught() {
        let mut deck_cards = vec![Card {
            rank: Rank::ACE,
            suit: Suit::SPADES,
        }];
        let mut asking: Vec<Card> = Vec::new();
        let target = Some(&Rank::ACE);
        let result = go_fish(&mut asking, &mut deck_cards, target);
        assert!(result);
    }

    #[test]
    fn go_fish_no_target() {
        let mut deck_cards = vec![Card {
            rank: Rank::TWO,
            suit: Suit::SPADES,
        }];
        let mut asking: Vec<Card> = Vec::new();
        let result = go_fish(&mut asking, &mut deck_cards, None);
        assert_eq!(result, false);
    }
}
