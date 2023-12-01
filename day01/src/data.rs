use regex::Regex;

pub type Parsed = (i64, i64);

const NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn import_data(data: &str) -> Vec<Parsed> {
    data.lines().map(|line| parse(line)).collect()
}

pub fn parse(line: &str) -> Parsed {

    let mut first_number: i64 = 0;
    let mut second_number: i64 = 0;

    for i in line.chars() {
        if NUMBERS.contains(&i) {
            first_number = i.to_digit(10).unwrap() as i64;
            break;
        }   
    };

    for i in line.chars().rev() {
        if NUMBERS.contains(&i) {
            second_number = i.to_digit(10).unwrap() as i64;
            break;
        }
        
    };

    (first_number, second_number)

    
}

pub const TEST_DATA: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;


const FULL_NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub type Parsed2 = (i64, i64);

pub fn import_data_2(data: &str) -> Vec<Parsed2> {
    data.lines().map(|line| parse2(line)).collect()
}

fn convert(item: &str) -> i64 {

    let mut map: Vec<_>= FULL_NUMBERS.into_iter().map(|x| x.to_owned()).enumerate().collect();

    map.extend(NUMBERS.into_iter().map(|c| c.to_string()).enumerate());

    let t = map.iter().find(|(_, num)| item == *num).unwrap();

    (t.0 + 1) as i64

}


fn find_all_regex_items(re: Regex, line: &str) -> Vec<String> {

    let mut matches = Vec::<String>::new();
    let mut start = 0;

    while start < line.len() {
        if let Some(matched) = re.find(&line[start..]) {

            matches.push(matched.as_str().to_string());
            // println!("Matched: {}", &line[start + matched.start()..start + matched.end()]);

            start += matched.start() + 1;
        } else {
            break;
        }
    };

    matches

}


pub fn parse2(line: &str) -> Parsed2 {

    let mut all_options: Vec<String> = Vec::new();

    all_options.extend(NUMBERS.into_iter().map(|x| x.to_string()));
    all_options.extend(FULL_NUMBERS.into_iter().map(|x| x.to_string()));

    let pattern = format!("({})", all_options.join("|"));

    let regex = Regex::new(&pattern).unwrap();

    let matches = find_all_regex_items(regex.clone(), line);

    println!("{:?}", &matches);

    (convert(matches.first().expect("no first match")), convert(matches.last().expect("no last match")))
    
}

pub const TEST_DATA_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;


mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input_data = import_data(TEST_DATA);
        println!("{:?}", input_data);
    }


    #[test]
    fn test_parsing_2() {
        let input_data = import_data_2(TEST_DATA_2);
        println!("{:?}", input_data);
    }
}
