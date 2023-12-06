use day06::*;


fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    println!("Answer of part 1 is: {}", answer_part1(input_data));
    println!("Answer of part 2 is: {}", answer_part2(import_data2(include_str!("../input.txt"))));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_answer1() {
        // let input_data = import_data(TEST_DATA_1);
        assert_eq!(TEST_ANSWER_1, answer_part1(gen_test_data()));
    }

    #[test]
    fn test_answer2() {
        // let input_data = import_data(TEST_DATA_2);
        assert_eq!(TEST_ANSWER_2, answer_part2(gen_test_data2()));
    }

    #[test]
    fn playground() {}
}
