use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use std::num::ParseIntError;

fn solve(cal: &Vec<u32>, to: usize) -> Output {
    Output::Nu32(cal[cal.len() - to..].iter().sum::<u32>())
}

pub fn run(cals: Input) -> (Output, Output) {
    let cals: Vec<u32> = cals.unwrap();

    (solve(&cals, 1), solve(&cals, 3))
}

pub fn parse() -> InputResult<Input> {
    let mut inp: Vec<u32> = Vec::new();

    for block in std::fs::read_to_string("../input/01")?
        .trim()
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
    {
        if let Ok(cals) = block
            .split('\n')
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()
        {
            inp.push(cals.iter().sum());
        } else {
            return Err(InputError::InvalidInput);
        }
    }

    if inp.len() < 3 {
        return Err(InputError::InvalidInput);
    }

    inp.sort();

    Ok(Input::Vu32(inp))
}
