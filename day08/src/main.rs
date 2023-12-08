use day08::*;

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_answer1a() {
        let input_data = import_data(TEST_DATA_1_A);
        assert_eq!(TEST_ANSWER_1_A, answer_part1(input_data));
    }

    #[test]
    fn test_answer1b() {
        let input_data = import_data(TEST_DATA_1_B);
        assert_eq!(TEST_ANSWER_1_B, answer_part1(input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data = import_data(TEST_DATA_2);
        assert_eq!(TEST_ANSWER_2, answer_part2(input_data));
    }

    #[test]
    fn playground() {}
}
