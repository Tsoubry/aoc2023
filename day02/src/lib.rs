pub mod data;

pub use crate::data::*;

pub fn answer_part1(data: Vec<Game>) -> AnswerDtype {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    data.into_iter()
        .filter_map(|game| {
            if game
                .sets
                .iter()
                .all(|set| set.red <= max_red && set.green <= max_green && set.blue <= max_blue)
            {
                Some(game.number)
            } else {
                None
            }
        })
        .sum()
}

pub fn answer_part2(data: Vec<Game>) -> AnswerDtype {
    data.into_iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for set in game.sets {
                if set.red > red {
                    red = set.red;
                }
                if set.green > green {
                    green = set.green;
                }
                if set.blue > blue {
                    blue = set.blue;
                }
            }

            red * green * blue
        })
        .sum()
}
