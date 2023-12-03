use day03::*;

fn main() {
    let input_data = import_data(include_str!("../input.txt"));

    // input_data.iter().for_each(|s| println!("{}", s));

    println!("Answer of part 1 is: {}", answer_part1(input_data.clone()));
    // println!("Answer of part 2 is: {}", answer_part2(input_data));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_answer1() {
        let input_data = import_data(TEST_DATA_1);
        assert_eq!(TEST_ANSWER_1, answer_part1(input_data));
    }

    // #[test]
    // fn test_answer2() {
    //     let input_data = import_data(TEST_DATA_2);
    //     assert_eq!(TEST_ANSWER_2, answer_part2(input_data));
    // }

    #[test]
    fn playground() {
        let input_data = import_data(include_str!("../input.txt"));

        let first = input_data.first().unwrap();

        // println!("len: {:?}", first.len());

        // let mut unique_chars: Vec<char> = input_data
        // .iter()
        // .flat_map(|s| s.chars())
        // .collect();

        // unique_chars.sort();
        // unique_chars.dedup();

        // println!("Unique characters: {:?}", unique_chars);
        // use regex::Regex;

        // let re_number = Regex::new(r"\d+").unwrap();

        // for num_match in re_number.find_iter(first) {
        //     let number = num_match.as_str().parse::<AnswerDtype>().expect("can't parse number");
        //     let start = num_match.start();
        //     let end = num_match.end();
        //     let len = num_match.len();

        //     println!("number: {}, start: {}, end: {}, len: {}", number, start, end, len);

        // }
    }
}
