extern crate no_std_day01;

use no_std_day01::{answer_part1, answer_part2, parse_data, parse_data_2};

fn main() {
    let raw_data: &str = include_str!("../input.txt");

    println!(
        "Answer of part 1 is: {}",
        answer_part1(parse_data(raw_data))
    );
    println!(
        "Answer of part 2 is: {}",
        answer_part2(parse_data_2(raw_data))
    );
}

#[cfg(test)]
mod tests {

    use no_std_day01::*;

    #[test]
    fn test_parsing() {
        let input_data = parse_data(TEST_DATA);
        println!("{:?}", input_data);
    }

    #[test]
    fn test_answer1() {
        let input_data = parse_data(TEST_DATA);
        assert_eq!(142, answer_part1(input_data));
    }

    #[test]
    fn test_parsing2() {
        let input_data = parse_data_2(TEST_DATA_2);
        println!("{:?}", input_data);
    }

    #[test]
    fn test_answer2() {
        let input_data = parse_data_2(TEST_DATA_2);
        assert_eq!(281, answer_part2(input_data));
    }

    #[test]
    fn playground() {}
}
