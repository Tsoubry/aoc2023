use regex::Regex;

const ARRAY_NUMBER_SIZE: usize = 3000; // should be more than large enough

const CHARACTERS: [char; 9] = ['#', '$', '%', '&', '+', '-', '/', '=', '@'];

#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub x_start: usize,
    pub x_end: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Symbol,
    Gear,
    Number(u16),
    Nothing,
}

#[derive(Debug)]
pub struct Grid<const X: usize, const Y: usize> {
    pub grid: [[Item; X]; Y],
    pub number_list: [Option<(Pos, u16)>; ARRAY_NUMBER_SIZE],
}

impl<const X: usize, const Y: usize> Grid<X, Y> {
    pub fn new() -> Self {
        Grid {
            grid: [[Item::Nothing; X]; Y],
            number_list: [None; ARRAY_NUMBER_SIZE],
        }
    }

    pub fn parse_data(data: &str) -> Grid<X, Y> {
        let mut grid = Grid::<X, Y>::new();

        let re_number = Regex::new(r"\d+").unwrap();

        data.lines().enumerate().for_each(|(idx, item)| {
            grid.parse(idx, item, &re_number);
        });

        grid
    }

    fn parse(&mut self, y: usize, line: &str, re: &Regex) {
        let mut array = [Item::Nothing; X];

        for number_str in re.find_iter(line) {
            let number_parsed = number_str
                .as_str()
                .parse::<u16>()
                .expect("parse number from regex");
            let number = Item::Number(number_parsed);
            let start = number_str.start();
            let end = number_str.end();

            for pos in start..end {
                array[pos] = number;
            }

            for item in self.number_list.iter_mut() {
                if item.is_none() {
                    *item = Some((
                        Pos {
                            x_start: start,
                            x_end: end,
                            y: y,
                        },
                        number_parsed,
                    ));
                    break; // Stop after the first empty item is filled
                }
            }
        }

        line.chars()
            .map(convert_item)
            .enumerate()
            .for_each(|(idx, item)| {
                if let Some(item) = item {
                    array[idx] = item;
                }
            });

        self.grid[y] = array;
    }

    fn assign_item(grid: &[[Item; X]; Y], x: Option<usize>, y: Option<usize>) -> Item {
        if x == None || y == None {
            Item::Nothing
        } else {
            grid.get(y.unwrap())
                .copied()
                .map(|row| row.get(x.unwrap()).copied())
                .flatten()
                .unwrap_or(Item::Nothing)
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> [Item; 8] {
        let bounded_x = x.checked_sub(1);
        let bounded_y = y.checked_sub(1);

        [
            Self::assign_item(&self.grid, bounded_x, bounded_y),
            Self::assign_item(&self.grid, Some(x), bounded_y),
            Self::assign_item(&self.grid, Some(x + 1), bounded_y),
            Self::assign_item(&self.grid, Some(x + 1), Some(y)),
            Self::assign_item(&self.grid, Some(x + 1), Some(y + 1)),
            Self::assign_item(&self.grid, Some(x), Some(y + 1)),
            Self::assign_item(&self.grid, bounded_x, Some(y + 1)),
            Self::assign_item(&self.grid, bounded_x, Some(y)),
        ]
    }
}

fn convert_item(c: char) -> Option<Item> {
    match c {
        '.' => Some(Item::Nothing),
        '*' => Some(Item::Gear),
        c if CHARACTERS.contains(&c) => Some(Item::Symbol),
        _ => None,
    }
}
