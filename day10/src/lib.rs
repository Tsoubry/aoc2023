pub mod data;
pub mod grid;

pub use crate::data::*;
pub use grid::{Grid, Pipe};

pub fn answer_part1<const X: usize, const Y: usize>(grid: Grid<X, Y>) -> AnswerDtype {
    let (start_y, start_x) = grid.find_start();

    let pipelines = grid.find_pipes(start_y, start_x);

    for (mut y, mut x, mut dir) in pipelines {
        let mut steps = 0;

        loop {
            steps += 1;

            println!("steps: {}", steps);

            if let Some(next_pipe) = grid.next_pipe(y, x, dir) {
                y = next_pipe.0;
                x = next_pipe.1;
                dir = next_pipe.3;
                if next_pipe.2 == Pipe::S {
                    return steps / 2;
                }
            } else {
                println!("break");
                break;
            }
        }
    }

    0
}

pub fn answer_part2<const X: usize, const Y: usize>(data: Grid<X, Y>) -> AnswerDtype {
    todo!()
}
