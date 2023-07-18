use crate::day::{Input, InputError, InputResult, Output, Wrapper};

const GRID_SIZE: usize = 300;

fn solve(grid: &[[i32; GRID_SIZE]; GRID_SIZE], square: usize) -> ((usize, usize), i32) {
    let (mut max, mut pos): (i32, (usize, usize)) = (0, (0, 0));

    for i in 0..grid.len() - square {
        for j in 0..grid[i].len() - square {
            let sum = sum_square(grid, square, i, j);

            if sum > max {
                max = sum;
                pos = (j, i);
            }
        }
    }

    (pos, max)
}

fn sum_square(grid: &[[i32; GRID_SIZE]; GRID_SIZE], square: usize, i: usize, j: usize) -> i32 {
    let mut sum: i32 = 0;

    for k in i..i + square {
        for l in j..j + square {
            sum += grid[k][l];
        }
    }

    sum
}

fn part_two(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, usize) {
    let (mut max, mut pos, mut square): (i32, (usize, usize), usize) = (0, (0, 0), 0);
    for i in 5..GRID_SIZE {
        let (loc, sum) = solve(grid, i);

        if max < sum {
            max = sum;
            pos = loc;
            square = i;
        }
    }

    (pos.0, pos.1, square)
}

fn create_grid(serial_number: i32) -> [[i32; GRID_SIZE]; GRID_SIZE] {
    let mut grid: [[i32; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let rack_id: i32 = (x + 10) as i32;
            grid[y][x] = (((rack_id * y as i32) + serial_number) * rack_id / 100 % 10) - 5;
        }
    }

    grid
}

pub fn run(input: Input) -> (Output, Output) {
    let input: i32 = input.unwrap();
    let grid = create_grid(input);

    let part_one: (usize, usize) = solve(&grid, 3).0;
    let part_two: (usize, usize, usize) = part_two(&grid);

    (
        Output::S(format!("{},{}", part_one.0, part_one.1)),
        Output::S(format!("{},{},{}", part_two.0, part_two.1, part_two.2)),
    )
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::Ni32(
        std::fs::read_to_string("../input/11")?.trim().parse()?,
    ))
}
