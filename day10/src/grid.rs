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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
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

    pub fn find_start(&self) -> (usize, usize) {
        for y in 0..Y {
            for x in 0..X {
                if self.grid[y][x] == Pipe::S {
                    return (y, x);
                }
            }
        }

        return (0, 0); // shouldn't be possible
    }

    pub fn find_up(&self, y: usize, x: usize) -> Option<(usize, usize, Direction)> {
        let new_y = std::cmp::min(y + 1, Y);
        if [Pipe::F, Pipe::V, Pipe::W].contains(&self.grid[new_y][x]) {
            Some((new_y, x, Direction::Up))
        } else {
            None
        }
    }

    pub fn find_down(&self, y: usize, x: usize) -> Option<(usize, usize, Direction)> {
        if let Some(new_y) = y.checked_sub(1) {
            if [Pipe::J, Pipe::V, Pipe::L].contains(&self.grid[new_y][x]) {
                Some((new_y, x, Direction::Down))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn find_right(&self, y: usize, x: usize) -> Option<(usize, usize, Direction)> {
        let new_x = std::cmp::min(x + 1, X);
        if [Pipe::F, Pipe::H, Pipe::J].contains(&self.grid[y][new_x]) {
            Some((y, new_x, Direction::Right))
        } else {
            None
        }
    }

    pub fn find_left(&self, y: usize, x: usize) -> Option<(usize, usize, Direction)> {
        if let Some(new_x) = x.checked_sub(1) {
            if [Pipe::W, Pipe::H, Pipe::L].contains(&self.grid[y][new_x]) {
                Some((y, new_x, Direction::Left))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn find_pipes(&self, y: usize, x: usize) -> Vec<(usize, usize, Direction)> {
        let mut pipes = vec![];

        // Up
        if let Some(pipe) = self.find_up(y, x) {
            pipes.push(pipe);
        }

        // Down
        if let Some(pipe) = self.find_down(y, x) {
            pipes.push(pipe);
        }

        // Left
        if let Some(pipe) = self.find_left(y, x) {
            pipes.push(pipe);
        }

        // Right
        if let Some(pipe) = self.find_right(y, x) {
            pipes.push(pipe);
        }

        pipes
    }

    pub fn next_pipe(
        &self,
        y: usize,
        x: usize,
        from_dir: Direction,
    ) -> Option<(usize, usize, Pipe, Direction)> {

        let current_pipe = self.grid[y][x];

        match current_pipe {
            Pipe::V => {
                if Direction::Up == from_dir {
                    if let Some(pipe) = self.find_up(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Down == from_dir {
                    if let Some(pipe) = self.find_down(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            Pipe::H => {
                if Direction::Left == from_dir {
                    if let Some(pipe) = self.find_left(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Right == from_dir {
                    if let Some(pipe) = self.find_right(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            Pipe::F => {
                if Direction::Left == from_dir {
                    if let Some(pipe) = self.find_down(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Up == from_dir {
                    if let Some(pipe) = self.find_right(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            Pipe::W => {
                if Direction::Right == from_dir {
                    if let Some(pipe) = self.find_down(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Up == from_dir {
                    if let Some(pipe) = self.find_left(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            Pipe::L => {
                if Direction::Left == from_dir {
                    if let Some(pipe) = self.find_up(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Down == from_dir {
                    if let Some(pipe) = self.find_right(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            Pipe::J => {
                if Direction::Right == from_dir {
                    if let Some(pipe) = self.find_up(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }};
                if Direction::Down == from_dir {
                    if let Some(pipe) = self.find_left(y, x) {
                        let new_pipe = self.grid[pipe.0][pipe.1];
                        return Some((pipe.0, pipe.1, new_pipe, pipe.2));
                    }
                }
            },
            _ => { return None },
        }
        None
    }
}
