use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Pipe {
    V,
    H,
    L,
    J,
    W,
    F,
    G,
    S,
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe::V,
            '-' => Pipe::H,
            'L' => Pipe::L,
            'J' => Pipe::J,
            '7' => Pipe::W,
            'F' => Pipe::F,
            '.' => Pipe::G,
            'S' => Pipe::S,
            _ => panic!("Invalid character for Pipe enum"),
        }
    }
}

impl fmt::Debug for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char_representation = match self {
            Pipe::V => '|',
            Pipe::H => '-',
            Pipe::L => 'L',
            Pipe::J => 'J',
            Pipe::W => '7',
            Pipe::F => 'F',
            Pipe::G => '.',
            Pipe::S => 'S',
        };
        write!(f, "{}", char_representation)
    }
}

#[derive(Debug)]
pub struct Grid<const X: usize, const Y: usize> {
    pub grid: [[Pipe; X]; Y],
}

impl<const X: usize, const Y: usize> Grid<X, Y> {
    pub fn new() -> Self {
        Grid {
            grid: [[Pipe::G; X]; Y],
        }
    }

    pub fn parse_data(data: &str) -> Grid<X, Y> {
        let mut grid = Grid::<X, Y>::new();

        data.lines().enumerate().for_each(|(y, line)| {
            for (x, c) in line.chars().enumerate() {
                grid.grid[y][x] = Pipe::from(c);
            }
        });

        grid
    }
}
