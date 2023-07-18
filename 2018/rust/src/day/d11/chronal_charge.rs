use crate::day::{Input, InputError, InputResult, Output, Wrapper};

const GRID_SIZE: usize = 300;

#[derive(Clone)]
struct Sizes {
    squares: Vec<i32>,
}

impl Sizes {
    fn new() -> Self {
        Self {
            squares: Vec::new(),
        }
    }
}

fn get_from(x: usize, y: usize, serial_number: i32) -> i32 {
    let rack_id: i32 = x as i32 + 10;
    (((rack_id * y as i32) + serial_number) * rack_id / 100 % 10) - 5
}

fn sum_square_bet(
    dp: &Vec<Vec<Sizes>>,
    square: usize,
    i: usize,
    j: usize,
    serial_number: i32,
) -> i32 {
    let mut sum: i32 = if square > 1 {
        dp[i][j].squares[square - 2]
    } else {
        0
    };

    let y2 = i + square - 1;
    let x1 = j + square - 1;
    for k in 0..square {
        let y1 = i + k;
        let x2 = j + k;

        if y1 == y2 && x1 == x2 {
            sum += get_from(y1, x1, serial_number);
        } else {
            sum += get_from(y1, x1, serial_number);
            sum += get_from(y2, x2, serial_number);
        }
    }

    sum
}

fn pre_calculating(serial_number: i32) -> ((usize, usize), (usize, usize, usize)) {
    let mut dp: Vec<Vec<Sizes>> = vec![vec![Sizes::new(); GRID_SIZE]; GRID_SIZE];
    let mut pos: (usize, usize) = (0, 0);
    let mut p2: (usize, usize, usize) = (0, 0, 0);
    let mut m1 = 0;
    let mut a = 0;

    // 1 2 3
    // 4 5 6
    // 7 8 9

    // 1 2
    // 4 5

    // 2 3
    // 5 6

    for square in 1..14 {
        for i in 0..GRID_SIZE - square {
            for j in 0..GRID_SIZE - square {
                let value = sum_square_bet(&dp, square, i, j, serial_number);
                dp[i][j].squares.push(value);

                if value > a {
                    a = value;
                    p2 = (i, j, square);
                }
            }
        }
    }

    for i in 0..GRID_SIZE - 3 {
        for j in 0..GRID_SIZE - 3 {
            if dp[i][j].squares[2] > m1 {
                m1 = dp[i][j].squares[2];
                pos = (i, j);
            }
        }
    }

    (pos, p2)
}

pub fn run(input: Input) -> (Output, Output) {
    let input: i32 = input.unwrap();

    let pos = pre_calculating(input);
    println!("{:?}", pos);
    let part_one = pos.0;
    let part_two = pos.1;

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
