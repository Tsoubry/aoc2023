use no_std_day03::*;

fn main() {
    let raw_data: &str = include_str!("../input.txt");

    let input_data: Grid<140, 140> = Grid::parse_data(raw_data);

    println!("Answer of part 1 is: {}", answer_part1(&input_data));
    println!("Answer of part 2 is: {}", answer_part2(&input_data));
}

#[cfg(test)]
mod tests {

    use no_std_day03::*;

    #[test]
    fn test_parsing() {
        let input_data: Grid<10, 10> = Grid::parse_data(TEST_DATA_1);
        // println!("{:?}", input_data);
        // println!(
        //     "{:?}",
        //     input_data.number_list.iter().flatten().collect::<Vec<_>>()
        // );
    }

    #[test]
    fn test_answer1() {
        let input_data: Grid<10, 10> = Grid::parse_data(TEST_DATA_1);
        assert_eq!(TEST_ANSWER_1, answer_part1(&input_data));
    }

    #[test]
    fn test_answer2() {
        let input_data: Grid<10, 10> = Grid::parse_data(TEST_DATA_1);
        assert_eq!(TEST_ANSWER_2, answer_part2(&input_data));
    }

    #[test]
    fn playground() {}
}
