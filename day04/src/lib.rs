pub mod data;

use std::collections::HashMap;

pub use crate::data::*;

fn calculate_score(winning_numbers: Vec<AnswerDtype>, my_numbers: Vec<AnswerDtype>) -> AnswerDtype {
    let mut my_score = 0;

    for i in winning_numbers {
        if my_numbers.contains(&i) {
            if my_score == 0 {
                my_score = 1;
            } else {
                my_score *= 2;
            }
        }
    }

    my_score
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Card {
    number: AnswerDtype,
    data: (Vec<AnswerDtype>, Vec<AnswerDtype>),
}

pub fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
    data.iter()
        .map(|x| calculate_score(x.0.clone(), x.1.clone()))
        .sum()
}

fn calculate_cards_won(
    winning_numbers: Vec<AnswerDtype>,
    my_numbers: Vec<AnswerDtype>,
) -> AnswerDtype {
    let mut overlap: AnswerDtype = 0;

    for num in winning_numbers {
        if my_numbers.contains(&num) {
            overlap += 1;
        }
    }

    overlap
}

pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
    let all_cards: Vec<_> = data
        .into_iter()
        .enumerate()
        .map(|(idx, x)| Card {
            number: (idx + 1) as AnswerDtype,
            data: x,
        })
        .collect();

    let max_card_number = all_cards.iter().map(|x| x.number).max().unwrap();

    let mut total_cards_won: HashMap<AnswerDtype, AnswerDtype> = HashMap::new();

    for card in all_cards.clone() {
        let cards_won = calculate_cards_won(card.data.0, card.data.1);

        *total_cards_won.entry(card.number).or_insert(0) += 1;

        for card_won in 1..=cards_won {
            let card_number_to_find = card.number + card_won;

            let card_copies = total_cards_won.get(&card.number).unwrap_or(&0);

            if card_number_to_find <= max_card_number {
                println!("{}", card_copies);

                *total_cards_won.entry(card_number_to_find).or_insert(0) += *card_copies;
            }
        }
    }

    total_cards_won.iter().map(|(_, amount)| amount).sum()
}
