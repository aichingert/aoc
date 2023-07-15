use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use std::collections::HashSet;

fn part_one(input: &Vec<i32>) -> i32 {
    input.iter().sum::<i32>()
}

fn part_two(input: &Vec<i32>) -> i32 {
    let (mut sum, mut pt) = (input[0], 0usize);
    let mut set: HashSet<i32> = HashSet::new();

    while set.insert(sum) {
        pt = if pt + 1 >= input.len() { 0 } else { pt + 1 };
        sum += input[pt];
    }

    sum
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<i32> = input.unwrap();

    (
        Output::Ni32(part_one(&input)),
        Output::Ni32(part_two(&input)),
    )
}

pub fn parse() -> InputResult<Input> {
    let input = std::fs::read_to_string("../input/01")?
        .lines()
        .map(|n| n.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    if input.is_empty() {
        return Err(InputError::InvalidInput);
    }

    Ok(Input::Vi32(input))
}
