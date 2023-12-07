pub mod data;

pub use crate::data::*;

pub fn answer_part1(mut data: Vec<Hand>) -> AnswerDtype {
    
    data.sort_by(|a, b| a.cmp(b));

    data
        .into_iter()
        .enumerate()
        .rev()
        .map(|(i, hand)| (i + 1) as AnswerDtype * hand.bid)
        .sum()

}

pub fn answer_part2(data: Vec<Hand>) -> AnswerDtype {
    todo!()
}
