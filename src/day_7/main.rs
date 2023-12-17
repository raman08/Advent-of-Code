use aoc2023::{get_file_path, read_file};
use itertools::Itertools;
use std::{collections::HashMap, str::Lines};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

#[derive(Debug, Clone)]
struct Hand {
    card: String,
    bid: i32,
    ctype: CardType,
}

fn find_card_type(hand: String) -> CardType {
    let cards: Vec<_> = hand.chars().collect();

    let counts = cards.iter().fold(Vec::new(), |mut acc, &card| {
        if let Some(entry) = acc.iter_mut().find(|(c, _)| *c == card) {
            entry.1 += 1;
        } else {
            acc.push((card, 1));
        }
        acc
    });

    let mut sorted_counts = counts.clone();
    sorted_counts.sort_by(|(_, a), (_, b)| b.cmp(a));

    match sorted_counts.as_slice() {
        [(_, 5)] => CardType::FiveOfAKind,
        [(_, 4), (_, 1)] => CardType::FourOfAKind,
        [(_, 3), (_, 2)] => CardType::FullHouse,
        [(_, 3), (_, 1), (_, 1)] => CardType::ThreeOfAKind,
        [(_, 2), (_, 2), (_, 1)] => CardType::TwoPair,
        [(_, 2), (_, 1), (_, 1), (_, 1)] => CardType::OnePair,
        _ => CardType::HighCard,
    }
}

fn find_card_type2(hand: String) -> CardType {
    let mut counts = [0; 12];
    for card in hand.chars() {
        match card {
            '2' => counts[0] += 1,
            '3' => counts[1] += 1,
            '4' => counts[2] += 1,
            '5' => counts[3] += 1,
            '6' => counts[4] += 1,
            '7' => counts[5] += 1,
            '8' => counts[6] += 1,
            '9' => counts[7] += 1,
            'T' => counts[8] += 1,
            'Q' => counts[9] += 1,
            'K' => counts[10] += 1,
            'A' => counts[11] += 1,
            _ => {}
        }
    }

    let jokers = hand.matches('J').count();
    let ranks = counts.iter().sorted().rev().cloned().collect::<Vec<_>>();

    if jokers == 5 || ranks[0] + jokers >= 5 {
        return CardType::FiveOfAKind;
    } else if ranks[0] + jokers >= 4 {
        return CardType::FourOfAKind;
    } else if ranks[0] + ranks[1] + jokers >= 5 {
        return CardType::FullHouse;
    } else if ranks[0] + jokers >= 3 {
        return CardType::ThreeOfAKind;
    } else if ranks[0] + ranks[1] + jokers >= 4 {
        return CardType::TwoPair;
    } else if ranks[0] + jokers >= 2 {
        return CardType::OnePair;
    } else {
        return CardType::HighCard;
    }
}

fn part_1(lines: Lines) -> i32 {
    let cards_with_bid: Vec<_> = lines
        .map(|line| {
            let mut tmp = line.split_whitespace();
            let card = tmp.next().unwrap_or_default();
            let bid = tmp.next().unwrap_or_default().parse::<i32>().unwrap();
            let card_type = find_card_type(card.to_string());

            Hand {
                card: card.to_string(),
                bid,
                ctype: card_type,
            }
        })
        .collect();

    let mut cards_sorted = cards_with_bid.clone();

    let card_strengths: HashMap<char, i32> = [
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]
    .iter()
    .cloned()
    .collect();

    cards_sorted.sort_by(|a, b| {
        if a.ctype == b.ctype {
            let cards_a: Vec<i32> = a.card.chars().map(|c| card_strengths[&c]).collect();
            let cards_b: Vec<i32> = b.card.chars().map(|c| card_strengths[&c]).collect();

            for (i, a) in cards_a.iter().enumerate() {
                if cards_b[i] == *a {
                    continue;
                } else {
                    return cards_b[i].cmp(&a);
                }
            }

            std::cmp::Ordering::Equal
        } else {
            a.ctype.cmp(&b.ctype)
        }
    });

    cards_sorted.reverse();

    cards_sorted
        .iter()
        .enumerate()
        .fold(0, |mut acc, (i, hand)| {
            acc += (i as i32 + 1) * hand.bid;
            acc
        })
    // 0
}

fn part_2(lines: Lines) -> i32 {
    let cards_with_bid: Vec<_> = lines
        .map(|line| {
            let mut tmp = line.split_whitespace();
            let card = tmp.next().unwrap_or_default();
            let bid = tmp.next().unwrap_or_default().parse::<i32>().unwrap();
            let card_type = find_card_type2(card.to_string());

            Hand {
                card: card.to_string(),
                bid,
                ctype: card_type,
            }
        })
        .collect();

    let mut cards_sorted = cards_with_bid.clone();

    let card_strengths: HashMap<char, i32> = [
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]
    .iter()
    .cloned()
    .collect();

    cards_sorted.sort_by(|a, b| {
        if a.ctype == b.ctype {
            let cards_a: Vec<i32> = a.card.chars().map(|c| card_strengths[&c]).collect();
            let cards_b: Vec<i32> = b.card.chars().map(|c| card_strengths[&c]).collect();

            for (i, a) in cards_a.iter().enumerate() {
                if cards_b[i] == *a {
                    continue;
                } else {
                    return cards_b[i].cmp(&a);
                }
            }

            std::cmp::Ordering::Equal
        } else {
            a.ctype.cmp(&b.ctype)
        }
    });

    cards_sorted.reverse();

    cards_sorted
        .iter()
        .enumerate()
        .fold(0, |mut acc, (i, hand)| {
            acc += (i as i32 + 1) * hand.bid;
            acc
        })
    // 0
}

fn main() {
    let is_sample = false;
    let path = get_file_path(7, is_sample);
    let contents = read_file(path);

    println!("Part 1: {}", part_1(contents.lines()));
    println!("Part 2: {}", part_2(contents.lines()));
}
