pub mod data;

pub use crate::data::*;

pub fn answer_part1(mut data: Vec<Hand>) -> AnswerDtype {
    data.sort_by(|a, b| a.cmp(b));

    data.into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as AnswerDtype * hand.bid)
        .sum()
}

pub fn answer_part2(mut data: Vec<Hand2>) -> AnswerDtype {
    data.sort_by(|a, b| a.cmp(b));

    data.into_iter()
        .enumerate()
        // .map(|(i, hand)| {
        //     println!("{:?}", hand);
        //     (i, hand)
        // })
        .map(|(i, hand)| (i + 1) as AnswerDtype * hand.bid)
        .sum()
}
