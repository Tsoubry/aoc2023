use no_std_day01::{answer_part1, answer_part2, parse_data, parse_data_2};

const RAW_DATA: &str = include_str!("../../day01/input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_std() {
    day01::answer_part1(day01::import_data(RAW_DATA));
}


#[divan::bench]
fn part1_no_std() {

    answer_part1(parse_data(RAW_DATA));

}

#[divan::bench]
fn part2_std() {

    day01::answer_part2(day01::import_data_2(RAW_DATA));

}

#[divan::bench]
fn part2_no_std() {

    answer_part2(parse_data_2(RAW_DATA));

}