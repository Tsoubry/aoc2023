pub mod better_solution;
pub mod data;
pub mod static_vec;

pub use crate::data::*;

use std::collections::VecDeque;

fn find_history_value(data: Parsed) -> AnswerDtype {
    let mut rows = vec![data];

    while !rows.last().unwrap().iter().all(|&x| x == 0) {
        let differences = rows
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        rows.push(differences);
    }

    rows.reverse();

    for idx in 0..(rows.len() - 1) {
        let last = rows[idx].last().unwrap().clone();
        let other_last = rows[idx + 1].last().unwrap().clone();
        rows[idx + 1].push(last + other_last);
    }

    *rows.last().unwrap().last().unwrap()
}

fn find_history_value_2(data: Parsed) -> AnswerDtype {
    let mut rows = vec![data];

    while !rows.last().unwrap().iter().all(|&x| x == 0) {
        let differences = rows
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        rows.push(differences);
    }

    let mut rows2: Vec<VecDeque<AnswerDtype>> =
        rows.into_iter().map(|v| v.into_iter().collect()).collect();

    rows2.reverse();

    for idx in 0..(rows2.len() - 1) {
        let first = rows2[idx].front().unwrap().clone();
        let other_first = rows2[idx + 1].front().unwrap().clone();
        rows2[idx + 1].push_front(other_first - first);
    }

    *rows2.last().unwrap().front().unwrap()
}

pub fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
    data.into_iter().map(find_history_value).sum()
}

pub fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
    data.into_iter().map(find_history_value_2).sum()
}
