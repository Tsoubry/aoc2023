#![no_std]

pub mod data;
pub mod grid;

pub use crate::data::*;
pub use crate::grid::*;

pub fn answer_part1<const X: usize, const Y: usize>(grid: &Grid<X, Y>) -> AnswerDtype {
    let mut total: AnswerDtype = 0;

    for num in grid.number_list {
        if num.is_none() {
            break;
        }

        if let Some((pos, value)) = num {
            let start_x = pos.x_start.saturating_sub(1);
            let end_x = core::cmp::min(pos.x_end + 1, X);

            let start_y = pos.y.saturating_sub(1);
            let end_y = core::cmp::min(pos.y + 2, Y );

            for x in start_x..end_x {
                for y in start_y..end_y {
                    let item = grid.grid[y][x];
                    if item == Item::Gear || item == Item::Symbol {
                        total += value as u32;
                    }
                }
            }
        };
    }

    total
}

// pub fn answer_part2(data: [(u8, u8); 1000]) -> AnswerDtype {

// }
