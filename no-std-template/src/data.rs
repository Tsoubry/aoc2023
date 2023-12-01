pub type AnswerDtype = i64;

pub type Parsed = ();

pub fn parse_data(data: &str) -> [Parsed; 1000] {
    let mut parsed_data = [(); 1000];

    data.lines()
        .map(|line| parse(line))
        .enumerate()
        .for_each(|(idx, item)| parsed_data[idx] = item);

    parsed_data
}

pub fn parse(line: &str) -> Parsed {
    todo!()
}

pub const TEST_DATA_1: &str = r#""#;
pub const TEST_ANSWER_1: AnswerDtype = 0;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;
