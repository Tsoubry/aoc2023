pub type AnswerDtype = i64;

pub type Parsed = Vec<AnswerDtype>;

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Parsed {
    line.split_whitespace().map(|x| x.parse().expect("failed parsing number to i64")).collect()
}

pub const TEST_DATA_1: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
pub const TEST_ANSWER_1: AnswerDtype = 114;

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
