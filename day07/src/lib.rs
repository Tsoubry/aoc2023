pub mod data;

pub use crate::data::*;
use std::cmp::Ordering;
use std::collections::HashMap;

fn get_value_cards(cards: &str, with_joker: bool) -> [u8; 5] {
    let mut value_cards = [0u8; 5];

    cards.chars().enumerate().for_each(|(idx, c)| {
        let card = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => {
                if with_joker {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        };

        value_cards[idx] = card;
    });

    value_cards
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

fn get_hand_type(cards: [u8; 5]) -> HandType {
    let mut frequency_map = HashMap::new();

    for &item in &cards {
        let count = frequency_map.entry(item).or_insert(0);
        *count += 1;
    }

    let max_count = frequency_map.values().cloned().max().unwrap_or(0);
    let amount_of_keys = frequency_map.keys().len();
    let is_full_house = max_count == 3 && amount_of_keys == 2;
    let is_2_pairs = max_count == 2 && amount_of_keys == 3;

    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if is_full_house {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if is_2_pairs {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialOrd)]
pub struct Hand {
    pub cards: [u8; 5],
    pub hand_type: HandType,
    pub bid: AnswerDtype,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare by hand_type
        let hand_type_order = self.hand_type.cmp(&other.hand_type);
        if hand_type_order != Ordering::Equal {
            return hand_type_order;
        } else {
            let zipped = self.cards.iter().zip(other.cards.iter());

            for (this_card, other_card) in zipped {
                let card_order = this_card.cmp(other_card);
                if card_order == Ordering::Equal {
                    continue;
                } else {
                    return this_card.cmp(other_card);
                }
            }

            return Ordering::Equal;
        }
    }
}

pub fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
    let mut card_hands: Vec<Hand> = data
        .into_iter()
        .map(|(cards_str, bid)| {
            let value_cards = get_value_cards(cards_str, false);
            Hand {
                cards: value_cards,
                hand_type: get_hand_type(value_cards),
                bid,
            }
        })
        .collect();

    card_hands.sort_by(|a, b| a.cmp(b));

    card_hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as AnswerDtype * hand.bid)
        .sum()
}

pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
    let mut card_hands: Vec<Hand> = data
        .into_iter()
        .map(|(cards_str, bid)| {
            let value_cards = get_value_cards(cards_str, true);

            let mut frequency_map = HashMap::new();

            for &item in &value_cards {
                let count = frequency_map.entry(item).or_insert(0);
                *count += 1;
            }

            let mut freq_map: Vec<_> = frequency_map.into_iter().collect();
            freq_map.sort_by_key(|&(_, count)| count);

            let replace_val = freq_map
                .iter()
                .filter(|&x| x.0 != 1)
                .map(|x| x.0)
                .last()
                .unwrap_or(1);

            let mut cards_mod = value_cards;

            for idx in 0..5 {
                if cards_mod[idx] == 1 {
                    cards_mod[idx] = replace_val;
                }
            }

            let hand_type = get_hand_type(cards_mod);

            Hand {
                cards: value_cards,
                hand_type: hand_type,
                bid,
            }
        })
        .collect();

    card_hands.sort_by(|a, b| a.cmp(b));

    card_hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as AnswerDtype * hand.bid)
        .sum()
}
