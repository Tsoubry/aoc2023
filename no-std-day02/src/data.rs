use core::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};
use strum_macros::EnumString;

pub type AnswerDtype = i64;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumString)]
enum Color {
    blue,
    red,
    green,
}

#[derive(Debug, Clone, Copy)]
pub struct Set {
    pub blue: i64,
    pub red: i64,
    pub green: i64,
}

impl Default for Set {
    fn default() -> Self {
        Self {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub number: i64,
    pub sets: [Set; 10],
}

impl Default for Game {
    fn default() -> Self {
        Self {
            number: 0,
            sets: [Set::default(); 10],
        }
    }
}

pub fn parse_data(data: &str) -> [Game; 100] {
    let mut parsed_data = [Game::default(); 100];

    data.lines()
        .map(|line| parse(line))
        .enumerate()
        .for_each(|(idx, game)| parsed_data[idx] = game);

    parsed_data
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    map_res(
        nom::branch::alt((tag("green"), tag("blue"), tag("red"))),
        Color::from_str,
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn parse_cube(input: &str) -> IResult<&str, (i64, Color)> {
    let (input_, (num, _, color)) = tuple((parse_number, space1, parse_color))(input)?;

    Ok((input_, (num, color)))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let mut set = Set::default();

    let parsed_cubes = input
        .split(", ")
        .map(|cube_str| parse_cube(cube_str).expect("can't parse cube"));

    for (_, (amount, color)) in parsed_cubes {
        match color {
            Color::blue => set.blue = amount,
            Color::red => set.red = amount,
            Color::green => set.green = amount,
        }
    }

    Ok((input, set))
}

fn parse_set_entry(input: &str) -> IResult<&str, [Set; 10]> {
    let mut sets: [Set; 10] = [Default::default(); 10];

    input
        .split("; ")
        .map(|set_str| parse_set(set_str).expect("can't parse sets"))
        .enumerate()
        .for_each(|(idx, (_, set))| {
            sets[idx] = set;
        });

    Ok((input, sets))
}

pub fn parse(line: &str) -> Game {
    let (_, (_, game_number, _, sets)) =
        tuple((tag("Game "), parse_number, tag(": "), parse_set_entry))(line).unwrap();

    Game {
        number: game_number,
        sets: sets,
    }
}

pub const TEST_DATA_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
pub const TEST_ANSWER_1: AnswerDtype = 8;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 2286;
