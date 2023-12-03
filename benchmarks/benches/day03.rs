use no_std_day03::{answer_part1, answer_part2, Grid};


const RAW_DATA: &str = include_str!("../../day03/input.txt");

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_std() {
    day03::answer_part1(day03::import_data(RAW_DATA));
}

#[divan::bench]
fn part1_no_std() {

    let input_data: Grid<140, 140> = Grid::parse_data(RAW_DATA);
    answer_part1(&input_data);
}

#[divan::bench]
fn part2_std() {
    day03::answer_part2(day03::import_data(RAW_DATA));
}

#[divan::bench]
fn part2_no_std() {

    let input_data: Grid<140, 140> = Grid::parse_data(RAW_DATA);
    answer_part2(&input_data);
}
