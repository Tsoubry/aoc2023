pub type AnswerDtype = u64;

pub type Parsed<'a> = (&'a str, AnswerDtype);

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Parsed {
    let mut splitted = line.split_whitespace();

    let cards = splitted.next().expect("getting cards error");
    let bid = splitted
        .next()
        .expect("getting bid error")
        .parse::<AnswerDtype>()
        .expect("parse bid error");

    (cards, bid)
}

pub const TEST_DATA_1: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
pub const TEST_ANSWER_1: AnswerDtype = 6440;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 5905;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let _input_data = import_data(TEST_DATA_1);
        // println!("{:?}", _input_data);
    }
}
