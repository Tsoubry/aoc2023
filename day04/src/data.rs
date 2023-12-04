pub type AnswerDtype = u32;

pub type Parsed = (Vec<AnswerDtype>, Vec<AnswerDtype>);

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Parsed {
    let split_data = line.split(": ");
    let last = split_data.last().expect("last data");

    let mut numbers = last.split(" | ").into_iter();

    let winning_numbers: Vec<AnswerDtype> = numbers
        .next()
        .expect("winning numbers")
        .split_whitespace()
        .map(|x| x.parse().expect("parse winning"))
        .collect();
    let my_numbers = numbers
        .next()
        .expect("my numbers")
        .split_whitespace()
        .map(|x| x.parse().expect("parse winning"))
        .collect();

    (winning_numbers, my_numbers)
}

pub const TEST_DATA_1: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
pub const TEST_ANSWER_1: AnswerDtype = 13;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 30;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
