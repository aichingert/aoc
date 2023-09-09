use crate::day::{Input, InputResult, Output, Wrapper};

const GRID_SIZE: usize = 300;

fn get_from(x: usize, y: usize, serial_number: i32) -> i32 {
    let rack_id: i32 = x as i32 + 10;
    (((rack_id * y as i32) + serial_number) * rack_id / 100 % 10) - 5
}

fn square_sum(
    dp: &Vec<Vec<Vec<i32>>>,
    square: usize,
    i: usize,
    j: usize,
    serial_number: i32,
) -> i32 {
    let mut sum: i32 = if square > 1 {
        dp[i][j][square - 2]
    } else {
        return get_from(i, j, serial_number);
    };

    //  1  2  3  4  5  6  7
    //  8  9 10 11 12 13 14
    // 15 16 17 18 19 20 21
    // 22 23 24 25 26 27 28
    // 29 30 31 32 33 34 35
    // 36 37 38 39 40 41 42
    // 43 44 45 46 47 48 49

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
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![Default::default(); GRID_SIZE]; GRID_SIZE];
    let mut part_two: (usize, usize, usize) = (0, 0, 0);
    let mut total_power = 0;

    for square in 1..15 {
        for i in 0..GRID_SIZE - square {
            for j in 0..GRID_SIZE - square {
                let value = square_sum(&dp, square, i, j, serial_number);
                dp[i][j].push(value);

                if value > total_power {
                    total_power = value;
                    part_two = (i, j, square);
                }
            }
        }
    }

    let mut part_one: (usize, usize) = (0, 0);
    total_power = 0;

    for i in 0..GRID_SIZE - 3 {
        for j in 0..GRID_SIZE - 3 {
            if dp[i][j][2] > total_power {
                total_power = dp[i][j][2];
                part_one = (i, j);
            }
        }
    }

    (part_one, part_two)
}

pub fn run(input: Input) -> (Output, Output) {
    let input: i32 = input.unwrap();

    let (part_one, part_two) = pre_calculating(input);

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
