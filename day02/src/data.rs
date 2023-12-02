// use nom::{
//     character::complete::{char, digit1, multispace0, multispace1},
// };

use strum_macros::EnumString;
use std::str::FromStr;


pub type AnswerDtype = i64;


#[derive(Debug, PartialEq, EnumString)]
enum Cube {
    blue,
    red,
    green,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub blue: i64,
    pub red: i64,
    pub green: i64,
}

impl Default for Set {
    fn default() -> Self {
        Self { blue: 0, red: 0, green: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub number: i64,
    pub sets: Vec<Set>
}


pub fn import_data(data: &str) -> Vec<Game> {
    data.lines().map(|line| parse(line)).collect()
}

fn parse_number(input: &str) -> Option<i64>{
    input.parse::<i64>().ok()
}

fn cube_finder(input: &str) -> (Cube, i64) {
    let mut cube_split = input.split(" ");
    let cb_num = cube_split.next().expect("cube number");
    let amount = parse_number(cb_num).expect("cube digit");
    let color = cube_split.next().expect("cube color").parse::<Cube>().expect("parsed cube color");
    (color, amount)
}


fn set_parser(set_str: &str) -> Set {

    let mut set = Set::default();

    set_str
        .split(", ")
        .map(|cube_str| cube_finder(cube_str))
        .for_each(|cube| {
            match cube {
                (Cube::blue, amount) => { set.blue = amount },
                (Cube::red, amount) => { set.red = amount },
                (Cube::green, amount) => { set.green = amount },
            }
        });

    set
}

pub fn parse(line: &str) -> Game {
    
    let mut game_split = line.split(": ");
    let game_str = game_split.next().expect("game split first");
    
    let game_str_split = game_str.split(" ");

    let game_number = parse_number(game_str_split.last().expect("game number last")).expect("game number parsed");

    let set_split = game_split.next().expect("set string").split("; ");

    let sets: Vec<_> = set_split
        .into_iter()
        .map(|set_str| set_parser(set_str))
        .collect();


    Game { number: game_number, sets: sets }

}

pub const TEST_DATA_1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
pub const TEST_ANSWER_1: AnswerDtype = 8;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
