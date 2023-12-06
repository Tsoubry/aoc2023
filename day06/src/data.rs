use derive_new::new;

pub type AnswerDtype = u64;

#[derive(new, Clone, Copy, Debug)]
pub struct Race {
    pub time: AnswerDtype,
    pub distance: AnswerDtype,
}

pub fn import_data(data: &str) -> Vec<Race> {
    // data.lines().map(|line| parse(line)).collect()

    vec![
        Race::new(60, 475),
        Race::new(94, 2138),
        Race::new(78, 1015),
        Race::new(82, 1650),
    ]
}

pub fn gen_test_data() -> Vec<Race>{
    vec![
    Race::new(7, 9),
    Race::new(15, 40),
    Race::new(30, 200),
]
}


pub const TEST_ANSWER_1: AnswerDtype = 288;

pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        // let input_data = import_data(TEST_DATA_1);
        // println!("{:?}", input_data);
    }
}
