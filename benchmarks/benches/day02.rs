use no_std_day02::{answer_part1, answer_part2, parse_data};

const RAW_DATA: &str = include_str!("../../day02/input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_std() {
    day02::answer_part1(day02::import_data(RAW_DATA));
}

#[divan::bench]
fn part1_no_std() {
    answer_part1(parse_data(RAW_DATA));
}

#[divan::bench]
fn part2_std() {
    day02::answer_part2(day02::import_data(RAW_DATA));
}

#[divan::bench]
fn part2_no_std() {
    answer_part2(parse_data(RAW_DATA));
}
