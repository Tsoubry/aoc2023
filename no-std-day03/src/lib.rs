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
            let end_y = core::cmp::min(pos.y + 2, Y);

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

fn ranges_overlap<T: PartialOrd>(range1: core::ops::Range<T>, range2: core::ops::Range<T>) -> bool {
    range1.start <= range2.end && range1.end > range2.start
}

pub fn answer_part2<const X: usize, const Y: usize>(grid: &Grid<X, Y>) -> AnswerDtype {
    let mut total: AnswerDtype = 0;

    grid.grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, item)| *item == &Item::Gear)
                .map(move |(x, _)| (x, y))
        })
        .for_each(|(x, y)| {
            let mut neighbours = grid.neighbours(x, y);

            let mut neighbours_numbers =
                unique_elements_slice(&mut neighbours)
                    .iter()
                    .filter_map(|item| match item {
                        Item::Number(value) => Some(value),
                        _ => None,
                    });

            let first_number = *neighbours_numbers.next().unwrap_or(&0) as u32;
            let second_number = *neighbours_numbers.next().unwrap_or(&0) as u32;

            let third = neighbours_numbers.next();
            if third.is_none() {
                total += first_number * second_number;
            }
        });

    total
}

fn unique_elements_slice<T: Eq>(arr: &mut [T]) -> &[T] {
    let mut unique_count = 0;
    let mut unique_end = 0;

    for i in 0..arr.len() {
        let mut is_unique = true;

        for j in 0..i {
            if arr[i] == arr[j] {
                is_unique = false;
                break;
            }
        }

        if is_unique {
            if i != unique_end {
                arr.swap(i, unique_end);
            }
            unique_end += 1;
            unique_count += 1;
        }
    }

    &arr[..unique_count]
}
