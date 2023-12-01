pub mod data;

use crate::data::*;

fn answer_part1(data: Vec<Parsed>) -> i64 {

    data
    .into_iter()
    .map(|item| item.0 * 10 + item.1)
    .sum()

}

fn answer_part2(data: Vec<Parsed2>) -> i64 {
    data
    .into_iter()
    .map(|item| item.0 * 10 + item.1)
    .sum()
}

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(import_data_2(include_str!("../input.txt"))));
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::TEST_DATA;

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA);
        assert_eq!(142, answer_part1(input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data_2(TEST_DATA_2);
        assert_eq!(281, answer_part2(input_data));
    }

    #[test]
    fn playground() {
        
    }
}
