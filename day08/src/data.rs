pub type AnswerDtype = i64;

#[derive(Debug, Clone)]
pub struct Node {
    pub from: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone)]
pub enum Direction {
    L,
    R,
}

pub fn import_data(data: &str) -> (Vec<Direction>, Vec<Node>) {
    let mut lines = data.lines();

    let instructions: Vec<_> = lines
        .next()
        .expect("first line error")
        .chars()
        .map(|c| match c {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => panic!("invalid direction"),
        })
        .collect();
    _ = lines.next().expect("skip line");

    let nodes = lines.map(|line| parse(line)).collect();

    (instructions, nodes)
}

pub fn parse(line: &str) -> Node {
    let mut parts = line.split(" = ");
    let from = parts.next().expect("get from part error").to_string();
    let mut to = parts.next().expect("get to part error").split(", ");

    let left = to.next().expect("get left part error")[1..].to_string();
    let right = to.next().expect("get right part error")[0..3].to_string();

    Node { from, left, right }
}

pub const TEST_DATA_1_A: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
pub const TEST_ANSWER_1_A: AnswerDtype = 2;

pub const TEST_DATA_1_B: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
pub const TEST_ANSWER_1_B: AnswerDtype = 6;

// pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 0;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1_A);
        println!("{:?}", input_data);

        let input_data2 = import_data(TEST_DATA_1_B);
        println!("{:?}", input_data2);
    }
}
