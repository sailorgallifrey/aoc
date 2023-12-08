use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::fs;
use std::str::Chars;

struct Hand {
    card_counts: HashMap<char, u8>,
    cards: Vec<char>,
    bid: u32,
}
fn main() {
    let file = fs::read_to_string("./data.txt").expect("Couldn't read file.");
    let lines: Vec<&str> = file.lines().collect();

    let hands: Vec<Hand> = get_hands(lines);

    let result: u32 = get_result(hands);

    println!("{:?}", result);
}

fn get_hands(lines: Vec<&str>) -> Vec<Hand> {
    lines.iter().map(|l| get_hand(l)).collect()
}

fn get_hand(line: &str) -> Hand {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let char_counts: HashMap<char, u8> = get_char_counts(parts[0].chars());

    Hand {
        card_counts: char_counts,
        cards: parts[0].chars().collect(),
        bid: parts[1].parse::<u32>().expect("unable to parse bid"),
    }
}

fn get_char_counts(chars: Chars) -> HashMap<char, u8> {
    chars.fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}

fn get_result(mut hands: Vec<Hand>) -> u32 {
    hands.sort_by(|h1, h2| compare_hands(h1, h2));
    hands.iter().enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .reduce(|a, b| a + b)
        .expect("could not calculate result")
}

fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    let s1: u8 = get_score(h1);
    let s2: u8 = get_score(h2);

    if s1 == s2 {
        let tie_break = break_tie(h1, h2);
        return tie_break
    } else if s1 > s2 {
        return Greater;
    } else {
        return Less;
    }
}

fn break_tie(h1: &Hand, h2: &Hand) -> Ordering {
    if h1.cards == h2.cards {
        panic!("shouldn't get here")
    }

    for i in 0..h1.cards.len() {
        if h1.cards[i] != h2.cards[i] {
            if get_rank(h1.cards[i]) > get_rank(h2.cards[i]) {
                return Greater;
            }
            else {
                return Less
            }
        }
    }

    Less
}

fn get_rank(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).expect("failed to parse character as digit") as u8,
    }
}

fn get_score(h: &Hand) -> u8 {
    match h
        .card_counts
        .values()
        .max()
        .expect("could not get max count")
    {
        5 => 10,
        4 => 9,
        3 => {
            if h.card_counts.values().len() == 2 { // Full house
                8
            } else { // three of a kind
                7
            }
        }
        2 => {
            if h.card_counts.values().len() == 3 { // 2 pair
                6
            } else { // one pair
                5
            }
        }
        1 => 4,
        _ => panic!("shouldn't get here")
    }
}
