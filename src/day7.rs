use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(ch: char) -> Self {
        match ch {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let (hand, bid) = s.trim().split_once(' ').unwrap();

        let mut cards = vec![];
        for ch in hand.chars() {
            cards.push(Card::from(ch));
        }
        assert_eq!(cards.len(), 5);

        Hand {
            cards: cards.try_into().unwrap(),
            bid: bid.parse().unwrap()
        }
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in self.cards.iter() {
            *map.entry(*card).or_default() += 1;
        }
        match map.values().max().unwrap_or(&0) {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => {
                if map.values().any(|&v| v == 2) {
                    return HandType::FullHouse;
                }
                HandType::ThreeOfKind
            },
            2 => {
                if map.values().filter(|&&v| v == 2).count() == 2 {
                    return HandType::TwoPair;
                }
                HandType::OnePair
            },
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_type() == other.get_type() {
            self.cards.cmp(&other.cards)
        } else {
            self.get_type().cmp(&other.get_type())
        }
    }
}

#[allow(dead_code)]
fn day7a() {
    // let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
    let mut input = String::new();
    let _ = File::open("inputs/7.txt")
        .unwrap()
        .read_to_string(&mut input);

    let mut hands: Vec<Hand> = vec![];
    for line in input.trim().split('\n') {
        hands.push(Hand::from(line));
    }
    hands.sort();

    let mut product = 0;
    for (idx, hand) in hands.iter().enumerate() {
        product += hand.bid * (idx as u64 + 1)
    }

    println!("{product:?}");
}

fn main() {}
