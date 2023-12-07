use std::cmp::Ordering;
use std::collections::HashMap;

pub type AnswerDtype = u64;

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

#[derive(Clone, Copy, Debug)]
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

impl Eq for Hand {}

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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn import_data(data: &str) -> Vec<Hand> {
    data.lines().map(|line| parse(line)).collect()
}

fn get_value_cards(cards: &str) -> [u8; 5] {
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
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        };

        value_cards[idx] = card;
    });

    value_cards
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

pub fn parse(line: &str) -> Hand {
    let mut splitted = line.split_whitespace();

    let cards = splitted.next().expect("getting cards error");
    let bid = splitted
        .next()
        .expect("getting bid error")
        .parse::<AnswerDtype>()
        .expect("parse bid error");

    let value_cards = get_value_cards(cards);

    Hand {
        cards: value_cards,
        hand_type: get_hand_type(value_cards),
        bid: bid,
    }
}

pub fn import_data_2(data: &str) -> Vec<Hand2> {
    data.lines().map(|line| parse2(line)).collect()
}

fn get_value_cards_alternative(cards: &str) -> [u8; 5] {
    let mut value_cards = [0u8; 5];

    cards.chars().enumerate().for_each(|(idx, c)| {
        let card = match c {
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 11,
            'K' => 12,
            'A' => 13,
            _ => unreachable!(),
        };

        value_cards[idx] = card;
    });

    value_cards
}

#[derive(Clone, Copy, Debug)]
pub struct Hand2 {
    pub cards: [u8; 5],
    pub hand_type: HandType,
    pub bid: AnswerDtype,
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand2 {}

impl Ord for Hand2 {
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

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse2(line: &str) -> Hand2 {
    let mut splitted = line.split_whitespace();

    let cards = splitted.next().expect("getting cards error");
    let bid = splitted
        .next()
        .expect("getting bid error")
        .parse::<AnswerDtype>()
        .expect("parse bid error");

    let value_cards = get_value_cards_alternative(cards);

    let mut frequency_map = HashMap::new();

    for &item in &value_cards {
        let count = frequency_map.entry(item).or_insert(0);
        *count += 1;
    }

    let mut freq_map: Vec<_> = frequency_map.into_iter().collect();
    freq_map.sort_by_key(|&(_, count)| count);

    println!("{:?}", freq_map);

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

    Hand2 {
        cards: value_cards,
        hand_type: hand_type,
        bid: bid,
    }
}

pub const TEST_DATA_1: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
pub const TEST_ANSWER_1: AnswerDtype = 6440;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 5905;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        // println!("{:?}", input_data);
    }
}
