#![feature(lazy_cell)]

pub mod data;

pub use crate::data::*;

pub fn answer_part1(data: Vec<Parsed>) -> i64 {
    data.into_iter().map(|item| item.0 * 10 + item.1).sum()
}

pub fn answer_part2(data: Vec<Parsed2>) -> i64 {
    data.into_iter().map(|item| item.0 * 10 + item.1).sum()
}
