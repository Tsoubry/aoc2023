#![no_std]

pub mod data;

pub use crate::data::{parse_data, parse_data_2, TEST_DATA, TEST_DATA_2};

pub fn answer_part1(data: [(u8, u8); 1000]) -> i64 {
    data.into_iter()
        .map(|item| item.0 as i64 * 10 + item.1 as i64)
        .sum()
}

pub fn answer_part2(data: [(u8, u8); 1000]) -> i64 {
    data.into_iter()
        .map(|item| item.0 as i64 * 10 + item.1 as i64)
        .sum()
}
