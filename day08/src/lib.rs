pub mod data;

pub use std::collections::HashMap;

pub use crate::data::*;

pub fn answer_part1(data: (Vec<Direction>, Vec<Node>)) -> AnswerDtype {
    let (instructions, nodes) = data;

    let map: HashMap<_, _> = nodes
        .into_iter()
        .map(|node| (node.from.clone(), node))
        .collect();

    let mut counter = 0;
    let mut index = 0;

    let mut cursor = "AAA".to_string();
    let end = "ZZZ".to_string();

    loop {
        let current_direction = &instructions[index];

        index = (index + 1) % instructions.len();
        counter += 1;

        println!("Cursor: {}", &cursor);

        let new_node = map.get(&cursor).expect("Error getting from map");

        cursor = match current_direction {
            Direction::L => new_node.left.clone(),
            Direction::R => new_node.right.clone(),
        };

        if &cursor == &end {
            break;
        }
    }

    counter
}

pub fn answer_part2(data: (Vec<Direction>, Vec<Node>)) -> AnswerDtype {
    todo!()
}
