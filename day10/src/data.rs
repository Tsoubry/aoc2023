pub type AnswerDtype = i64;

use crate::grid::Grid;

pub fn import_data<const X: usize, const Y: usize>(data: &str) -> Grid<X, Y> {
    Grid::parse_data(data)
}

pub const TEST_DATA_1: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
pub const TEST_ANSWER_1: AnswerDtype = 8;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data: Grid<5, 5> = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
