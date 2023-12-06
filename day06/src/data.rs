use derive_new::new;

pub type AnswerDtype = u64;

#[derive(new, Clone, Copy, Debug)]
pub struct Race {
    pub time: AnswerDtype,
    pub distance: AnswerDtype,
}

fn split_whitespace(s: &str) -> Vec<AnswerDtype> {
    s.trim_start_matches("Time:")
        .trim_start_matches("Distance:")
        .trim_start()
        .split_whitespace()
        .map(|s| s.parse().expect("parse into number"))
        .collect()
}

pub fn import_data(data: &str) -> Vec<Race> {
    let mut lines = data.lines();
    let time_line = split_whitespace(lines.next().expect("time line"));
    let distance_line = split_whitespace(lines.next().expect("distance line"));

    time_line
        .iter()
        .zip(distance_line.iter())
        .map(|(&t, &d)| Race::new(t, d))
        .collect()
}

fn make_number(s: &str) -> AnswerDtype {
    s.trim_start_matches("Time:")
        .trim_start_matches("Distance:")
        .replace(" ", "")
        .parse()
        .expect("parse into number")
}

pub fn import_data_2(data: &str) -> Vec<Race> {
    let mut lines = data.lines();
    let time = make_number(lines.next().expect("time line"));
    let distance = make_number(lines.next().expect("distance line"));

    vec![Race::new(time, distance)]
}

pub const TEST_DATA_1: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;
pub const TEST_ANSWER_1: AnswerDtype = 288;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 71503;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
