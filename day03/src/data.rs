pub type AnswerDtype = u32;

pub fn import_data(data: &str) -> Vec<&str> {
    data.lines().collect()
}

pub const TEST_DATA_1: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
pub const TEST_ANSWER_1: AnswerDtype = 4361;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 467835;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
