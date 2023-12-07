use std::collections::HashMap;

pub type AnswerDtype = u64;

#[derive(Clone, Copy, Debug)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand {
    cards: [u8; 5],
    hand_type: HandType,
    bid: AnswerDtype,
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
    let bid = splitted.next().expect("getting bid error").parse::<AnswerDtype>().expect("parse bid error");

    let value_cards = get_value_cards(cards);

    Hand {
        cards: value_cards,
        hand_type: get_hand_type(value_cards),
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
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
