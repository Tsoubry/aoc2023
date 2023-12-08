pub type AnswerDtype = u32;

use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapData {
    pub destination_range_start: AnswerDtype,
    pub source_range: Range<AnswerDtype>,
}

pub type Parsed = (Vec<AnswerDtype>, Vec<Vec<MapData>>);

fn split_into_numbers(input: &str) -> Vec<AnswerDtype> {
    input
        .split_whitespace()
        .map(|s| s.parse::<AnswerDtype>().expect("error parse"))
        .collect()
}

pub fn import_data(data: &str) -> Parsed {
    let mut split_lines = data.split("\n\n").into_iter();

    let seeds_str = split_lines
        .next()
        .expect("seeds")
        .trim_start_matches("seeds: ");

    let seeds = split_into_numbers(seeds_str);

    let maps: Vec<_> = split_lines
        .map(|s| {
            let mut lines = s.lines();
            let _ = lines.next().expect("first line"); // skip first line
            lines.map(|line| parse(line)).collect::<Vec<MapData>>()
        })
        .collect();

    (seeds, maps)
}

pub fn parse(line: &str) -> MapData {
    let split_white = split_into_numbers(line);

    let destination_range_start = *split_white.get(0).expect("get range item");
    let source_range_start = *split_white.get(1).expect("get range item");
    let range_length = *split_white.get(2).expect("get range item");

    MapData {
        destination_range_start: destination_range_start,
        source_range: source_range_start..(source_range_start + range_length),
    }
}

pub const TEST_DATA_1: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;
pub const TEST_ANSWER_1: AnswerDtype = 35;

pub const TEST_DATA_2: &str = TEST_DATA_1;
pub const TEST_ANSWER_2: AnswerDtype = 46;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA_1);
        println!("{:?}", input_data);
    }
}
