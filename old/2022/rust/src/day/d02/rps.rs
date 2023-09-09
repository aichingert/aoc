use crate::day::{Input, InputError, InputResult, Output, Wrapper};

fn part_one(n: &Vec<(i32, i32)>) -> Output {
    Output::Ni32(
        n.iter()
            .map(|(lhs, rhs)| rhs + (3 - (2 + lhs - rhs) % 3) % 3 * 3)
            .sum::<i32>(),
    )
}

fn part_two(n: &Vec<(i32, i32)>) -> Output {
    Output::Ni32(
        n.iter()
            .map(|(lhs, rhs)| (lhs + rhs) % 3 + 1 + (rhs - 1) * 3)
            .sum::<i32>(),
    )
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<(i32, i32)> = input.unwrap();

    (part_one(&input), part_two(&input))
}

pub fn parse() -> InputResult<Input> {
    let mut input: Vec<(i32, i32)> = Vec::new();

    for line in std::fs::read_to_string("../input/02")?.trim().lines() {
        if let Some((lhs, rhs)) = line.split_once(' ') {
            input.push((map_value(lhs), map_value(rhs)));
        } else {
            return Err(InputError::InvalidInput);
        }
    }

    Ok(Input::VTi32(input))
}

fn map_value(val: &str) -> i32 {
    match val {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Invalid input!"),
    }
}
