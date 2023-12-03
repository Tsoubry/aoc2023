#![no_std]

pub mod data;
pub mod grid;

pub use crate::data::*;
pub use crate::grid::*;

pub fn answer_part1<const X: usize, const Y: usize>(grid: &Grid<X, Y>) -> AnswerDtype {
    let mut total: AnswerDtype = 0;

    for line in grid.grid {}

    // for y in 0..data.len() {
    //     let y_str = &data[y];

    //     for num_match in re_number.find_iter(y_str) {
    //         let number = num_match
    //             .as_str()
    //             .parse::<AnswerDtype>()
    //             .expect("can't parse number");
    //         let start = num_match.start();
    //         let end = num_match.end();

    //         if check_neighbour_char(&data, y, start, end) {
    //             total += number;
    //         }
    //     }
    // }

    total
}

// pub fn answer_part2(data: [(u8, u8); 1000]) -> AnswerDtype {

// }
