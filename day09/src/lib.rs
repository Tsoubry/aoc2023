pub mod data;

use std::ops::Deref;

pub use crate::data::*;

fn fill(data: Parsed) {

}

fn find_history_value(data: Parsed) -> AnswerDtype {

    let mut rows = vec![data];

    while !rows.last().unwrap().iter().all(|&x| x == 0) {
        let differences = rows.last().unwrap().windows(2).map(|w| w[1] - w[0]).collect();
        rows.push(differences);
    };

    rows.reverse();

    for idx in 0..(rows.len() - 1) {
        let last = rows[idx].last().unwrap().clone();
        let other_last = rows[idx + 1].last().unwrap().clone();
        rows[idx + 1].push(last + other_last);
    }

    *rows.last().unwrap().last().unwrap()

}

pub fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
    data
        .into_iter()
        .map(find_history_value)
        .sum()
}

pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
    todo!()
}
