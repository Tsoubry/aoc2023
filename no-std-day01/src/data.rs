use regex::Regex;

const NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub type Parsed = (u8, u8);

pub fn parse_data(data: &str) -> [(u8, u8); 1000] {
    let mut parsed_data = [(0u8, 0u8); 1000];

    data.lines()
        .map(|line| parse(line))
        .enumerate()
        .for_each(|(idx, item)| parsed_data[idx] = item);

    parsed_data
}

pub fn parse(line: &str) -> Parsed {
    let mut first_number: u8 = 0;
    let mut second_number: u8 = 0;

    for i in line.chars() {
        if NUMBERS.contains(&i) {
            first_number = i.to_digit(10).unwrap() as u8;
            break;
        }
    }

    for i in line.chars().rev() {
        if NUMBERS.contains(&i) {
            second_number = i.to_digit(10).unwrap() as u8;
            break;
        }
    }

    (first_number, second_number)
}

pub const TEST_DATA: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

const FULL_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const STR_NUMBERS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

const PATTERN_LEN: usize = 1 + 9 + 17 + 3 + 3 + 5 + 4 + 4 + 3 + 5 + 5 + 4 + 1;

fn custom_join<'a>(buf: &mut [u8; PATTERN_LEN], iter: impl Iterator<Item = &'a str>) {
    buf[0] = b'(';

    let mut cursor = 1usize;

    for s in iter {
        for c in s.chars() {
            buf[cursor] = c as u8;
            cursor += 1;
        }
        buf[cursor] = b'|';
        cursor += 1;
    }

    buf[PATTERN_LEN - 1] = b')';
}

pub fn parse_data_2(data: &str) -> [(u8, u8); 1000] {
    let mut buf = [0u8; PATTERN_LEN];
    let iter = FULL_NUMBERS.into_iter().chain(STR_NUMBERS.into_iter());
    custom_join(&mut buf, iter);
    let pattern = core::str::from_utf8(&buf).expect("utf8 error");
    let regex = Regex::new(&pattern).unwrap();

    let mut parsed_data = [(0u8, 0u8); 1000];

    data.lines()
        .map(|line| parse2(line, &regex))
        .enumerate()
        .for_each(|(idx, item)| parsed_data[idx] = item);

    parsed_data
}

fn convert(item: &str) -> u8 {
    let mut map = FULL_NUMBERS
        .into_iter()
        .enumerate()
        .chain(STR_NUMBERS.into_iter().enumerate());

    let t = map.find(|(_, num)| item == *num).unwrap();

    (t.0 + 1) as u8
}

fn find_all_regex_items<'a, 'b: 'a>(re: &Regex, line: &'b str, matches: &mut [&'a str; 100]) {
    let mut start = 0;
    let mut current_pos = 0usize;

    while start < line.len() {
        if let Some(matched) = re.find(&line[start..]) {
            matches[current_pos] = matched.as_str();
            current_pos += 1;

            start += matched.start() + 1;
        } else {
            break;
        }
    }
}

pub fn parse2(line: &str, regex: &Regex) -> Parsed {
    let mut matches: [&str; 100] = [""; 100];

    find_all_regex_items(regex, line, &mut matches);

    let non_empty_matches = matches.iter().filter(|s| *s != &"");

    let first_match = non_empty_matches.clone().next().expect("first item");
    let last_match = non_empty_matches.last().expect("last item");

    (convert(first_match), convert(last_match))
}

pub const TEST_DATA_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
