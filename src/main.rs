mod model;
use model::{card::Card, deck::Deck, player_deck::PlayerDeck, player_name::PlayerName, rank::Rank};
use rand::{seq::SliceRandom, thread_rng};
use std::io::{self, prelude::*, Write};

fn main() {
    let mut main_deck = Deck::default().cards;
    let mut user_deck = PlayerDeck::from_name(PlayerName::YOU);
    let mut foe_deck = PlayerDeck::from_name(PlayerName::FOE);
    deal_cards(&mut main_deck, &mut user_deck, &mut foe_deck);
    loop {
        print_books(&user_deck, &foe_deck);
        if main_deck.is_empty() && user_deck.get_unbooked_rank_counts().is_empty() {
            println!("{} won the game.", get_winner(&user_deck, &foe_deck));
            break ();
        }
        ask(&mut user_deck, &mut foe_deck.cards, &mut main_deck);
        ask(&mut foe_deck, &mut user_deck.cards, &mut main_deck);
    }
}

fn deal_cards(deck_cards: &mut Vec<Card>, user_deck: &mut PlayerDeck, foe_deck: &mut PlayerDeck) {
    let cards_per_player = 7;
    for _ in 0..cards_per_player {
        foe_deck.cards.push(deck_cards.pop().unwrap());
        user_deck.cards.push(deck_cards.pop().unwrap());
    }
}

fn print_books(user_deck: &PlayerDeck, foe_deck: &PlayerDeck) {
    let user_books = user_deck.get_books();
    let foe_books = foe_deck.get_books();
    let books = [&user_books[..], &foe_books[..]].concat();
    if books.is_empty() {
        return;
    }
    let mut books_iter: Vec<(&Rank, &PlayerName)> = books.into_iter().collect();
    books_iter.sort_by_key(|elem| elem.0);
    let dashes = "--------------------------------";
    println!("{}\nBooks:", dashes);
    for book in books_iter {
        println!("{}\t{}", book.0, book.1);
    }
    println!(
        "User books: [{}], Foe books: [{}]",
        user_books.len(),
        foe_books.len()
    );
    println!("{}", dashes);
}

fn get_winner<'a>(deck_1: &'a PlayerDeck, deck_2: &'a PlayerDeck) -> &'a PlayerName {
    return [deck_1.get_books(), deck_2.get_books()]
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .last()
        .unwrap()
        .1;
}

fn get_user_rank_choice(user_deck: &mut PlayerDeck) -> Option<Rank> {
    let unbooked_ranks = user_deck.get_unbooked_rank_counts();
    if unbooked_ranks.is_empty() {
        let mut stdout = io::stdout();
        write!(
            stdout,
            "You have no open ranks to ask for, press enter to continue..."
        )
        .unwrap();
        stdout.flush().unwrap();
        let _ = io::stdin().read(&mut [0u8]).unwrap();
        return None;
    }
    let mut option = String::new();
    let option_pos: usize;
    loop {
        option.clear();
        io::stdout().flush().unwrap();
        println!("What card are you asking for?");
        for (pos, rank) in unbooked_ranks.iter().enumerate() {
            println!("{}.\t{}\t({}/4)", pos + 1, rank.0, rank.1);
        }
        match io::stdin().read_line(&mut option) {
            Ok(_) => {}
            Err(error) => println!("Error getting input: {}", error),
        }
        match &option.trim().parse::<usize>() {
            Ok(n) => {
                if *n >= 1 && *n <= unbooked_ranks.len() {
                    option_pos = *n;
                    break;
                }
                println!("That is not a valid choice.")
            }
            Err(_error) => {
                println!("Failed to parse your input, try again!");
            }
        }
    }
    return Some(unbooked_ranks[option_pos - 1].0);
}

fn get_foe_rank_choice(foe_deck: &mut PlayerDeck) -> Option<Rank> {
    if foe_deck.cards.is_empty() {
        return None;
    }
    let unbooked_ranks = foe_deck.get_unbooked_rank_counts();
    if unbooked_ranks.is_empty() {
        return None;
    }
    return Some(unbooked_ranks.choose(&mut thread_rng()).unwrap().0);
}

fn ask(asking_deck: &mut PlayerDeck, vulnerable_cards: &mut Vec<Card>, deck_cards: &mut Vec<Card>) {
    loop {
        let asked_rank = match asking_deck.name {
            PlayerName::YOU => get_user_rank_choice(asking_deck),
            PlayerName::FOE => get_foe_rank_choice(asking_deck),
        };
        let rank_val: Rank;
        match asked_rank {
            Some(r) => {
                rank_val = r;
            }
            None => {
                println!("{} had nothing to ask for.", asking_deck.name);
                go_fish(&mut asking_deck.cards, deck_cards);
                return;
            }
        }
        println!("{} asked for {}.", &asking_deck.name, &rank_val);
        let found_cards: Vec<Card> = vulnerable_cards
            .iter()
            .filter(|card| card.rank == rank_val)
            .cloned()
            .collect();
        if found_cards.len() > 0 {
            println!("{} took {} card(s).", asking_deck.name, found_cards.len());
            for card in found_cards {
                let index = vulnerable_cards
                    .iter()
                    .position(|vul_card| *vul_card == card)
                    .unwrap();
                vulnerable_cards.swap_remove(index);
                asking_deck.cards.push(card);
            }
        } else {
            go_fish(&mut asking_deck.cards, deck_cards);
            break;
        }
    }
}

fn go_fish(asking: &mut Vec<Card>, deck_cards: &mut Vec<Card>) {
    println!("Go fish!");
    if let Some(card) = deck_cards.pop() {
        asking.push(card);
    }
}
