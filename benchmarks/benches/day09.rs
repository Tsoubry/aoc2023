const RAW_DATA: &str = include_str!("../../day09/input.txt");

use day09::better_solution::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_std() {
    day09::answer_part1(day09::import_data(RAW_DATA));
}

#[divan::bench]
fn part1_no_std() {
    part1(RAW_DATA);
}

#[divan::bench]
fn part2_std() {
    day09::answer_part2(day09::import_data(RAW_DATA));
}

#[divan::bench]
fn part2_no_std() {
    part2(RAW_DATA);
}
