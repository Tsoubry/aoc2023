use regex::Regex;

pub mod data;

pub use crate::data::*;

struct Coordinate {
    line: usize,
    pos: usize,
}

const CHARACTERS: [char; 10] = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];

fn check_neighbour_char(data: &Vec<&str>, y: usize, start: usize, end: usize) -> bool {
    let start_x = start.saturating_sub(1);
    let end_x = end + 1;

    let start_y = y.saturating_sub(1);
    let end_y = core::cmp::min(y + 2, data.len());

    for line in &data[start_y..end_y] {
        let end_bound = core::cmp::min(end_x, line.len());

        for x in start_x..end_bound {
            let char_to_check = line.chars().nth(x).unwrap();

            if CHARACTERS.contains(&char_to_check) {
                return true;
            }
        }
    }

    false
}

pub fn answer_part1(data: Vec<&str>) -> AnswerDtype {
    let mut total: AnswerDtype = 0;

    let re_number = Regex::new(r"\d+").unwrap();

    for y in 0..data.len() {
        let y_str = &data[y];

        for num_match in re_number.find_iter(y_str) {
            let number = num_match
                .as_str()
                .parse::<AnswerDtype>()
                .expect("can't parse number");
            let start = num_match.start();
            let end = num_match.end();

            if check_neighbour_char(&data, y, start, end) {
                total += number;
            }
        }
    }

    total
}

fn ranges_overlap<T: PartialOrd>(range1: core::ops::Range<T>, range2: core::ops::Range<T>) -> bool {
    range1.start <= range2.end && range1.end > range2.start
}

fn find_number_gear(data: &Vec<&str>, y: usize, pos: usize) -> Option<AnswerDtype> {
    let re_number = Regex::new(r"\d+").unwrap();

    let mut parts_found: Vec<AnswerDtype> = vec![];

    let start_x = pos.saturating_sub(1);
    let end_x = pos + 1;

    let start_y = y.saturating_sub(1);
    let end_y = core::cmp::min(y + 2, data.len());

    for line in &data[start_y..end_y] {
        for num_match in re_number.find_iter(line) {
            if ranges_overlap(num_match.range(), start_x..end_x) {
                parts_found.push(
                    num_match
                        .as_str()
                        .parse::<AnswerDtype>()
                        .expect("can't parse number"),
                );
            }
        }
    }

    parts_found.dedup();

    if parts_found.len() == 2 {
        return Some(parts_found.iter().product());
    } else {
        None
    }
}

pub fn answer_part2(data: Vec<&str>) -> AnswerDtype {
    let mut total: AnswerDtype = 0;

    let re_gear = Regex::new(r"\*").unwrap();

    for y in 0..data.len() {
        let y_str = &data[y];

        for gear in re_gear.find_iter(y_str) {
            let x_pos = gear.start();

            if let Some(number) = find_number_gear(&data, y, x_pos) {
                total += number;
            }
        }
    }

    total
}
