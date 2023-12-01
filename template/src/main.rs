pub mod data;

use crate::data::*;

// fn answer_part1(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }

// fn answer_part2(data: Vec<Parsed>) -> AnswerDtype {
//     todo!()
// }

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    // println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::{TEST_ANSWER_1, TEST_ANSWER_2, TEST_DATA_1, TEST_DATA_2};

    // #[test]
    // fn test_answer1() {
    //     let input_data = import_data(TEST_DATA_1);
    //     assert_eq!(TEST_ANSWER_1, answer_part1(input_data));
    // }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA_2);
    //     assert_eq!(TEST_ANSWER_2, answer_part2(input_data));
    // }

    #[test]
    fn playground() {}
}
