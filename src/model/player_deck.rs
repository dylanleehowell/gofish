use super::{card::Card, player_name::PlayerName, rank::Rank};
use std::collections::HashMap;
use string_builder;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PlayerDeck {
    pub cards: Vec<Card>,
    pub name: PlayerName,
}

impl PlayerDeck {
    pub fn from_name(name: PlayerName) -> PlayerDeck {
        PlayerDeck {
            cards: Vec::new(),
            name: name,
        }
    }
    pub fn get_unbooked_rank_counts(&self) -> Vec<(Rank, usize)> {
        let mut rank_map = HashMap::new();
        for card in &self.cards {
            let mut count = 1;
            if rank_map.contains_key(&card.rank) {
                count += rank_map.get(&card.rank).unwrap();
            }
            if count == 4 {
                rank_map.remove(&card.rank);
            } else {
                rank_map.insert(card.rank, count);
            }
        }
        let mut rank_map_vec: Vec<(Rank, usize)> = rank_map.into_iter().collect();
        rank_map_vec.sort_by_key(|elem| elem.0);
        return rank_map_vec;
    }
    pub fn get_books(&self) -> Vec<(&Rank, &PlayerName)> {
        let mut books = Vec::new();
        let mut rank_map = HashMap::new();
        for card in &self.cards {
            let mut count = 1;
            if rank_map.contains_key(&card.rank) {
                count += rank_map.get(&card.rank).unwrap();
            }
            if count == 4 {
                books.push((&card.rank, &self.name));
            }
            rank_map.insert(card.rank, count);
        }
        return books;
    }
}

impl std::fmt::Display for PlayerDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counts = HashMap::new();
        for card in &self.cards {
            if !counts.contains_key(&card.rank) {
                counts.insert(&card.rank, 1);
            } else {
                let rank_count = counts[&card.rank] + 1;
                counts.insert(&card.rank, rank_count);
            }
        }
        let mut sb = string_builder::Builder::default();
        for (key, value) in counts.into_iter() {
            sb.append(format!("\n  {}\t{}", key, value));
        }
        sb.append("\n");
        write!(f, "{}", sb.string().unwrap())
    }
}
