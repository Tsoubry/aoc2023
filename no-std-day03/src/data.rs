use regex::Regex;

pub type AnswerDtype = i64;

pub type Parsed = ();

use crate::grid::{Grid, Item};

const CHARACTERS: [char; 9] = ['#', '$', '%', '&', '+', '-', '/', '=', '@'];

pub fn parse_data<const X: usize, const Y: usize>(data: &str) -> Grid<X, Y> {
    let mut grid = Grid::<X, Y>::new();

    let re_number = Regex::new(r"\d+").unwrap();

    data.lines()
        .map(|line| parse(line, &re_number))
        .enumerate()
        .for_each(|(idx, item)| {
            grid.grid[idx] = item;
        });

    grid
}

fn convert_item(c: char) -> Option<Item> {
    match c {
        '.' => Some(Item::Nothing),
        '*' => Some(Item::Gear),
        c if CHARACTERS.contains(&c) => Some(Item::Symbol),
        _ => None,
    }
}

fn parse<const X: usize>(line: &str, re: &Regex) -> [Item; X] {
    let mut array = [Item::Nothing; X];

    for number_str in re.find_iter(line) {
        let number = Item::Number(
            number_str
                .as_str()
                .parse::<u16>()
                .expect("parse number from regex"),
        );
        let start = number_str.start();
        let end = number_str.end();

        for pos in start..=end {
            array[pos] = number;
        }
    }

    line.chars()
        .map(convert_item)
        .enumerate()
        .for_each(|(idx, item)| {
            if let Some(item) = item {
                array[idx] = item;
            }
        });

    array
}

pub const TEST_DATA_1: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
pub const TEST_ANSWER_1: AnswerDtype = 4361;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 467835;
