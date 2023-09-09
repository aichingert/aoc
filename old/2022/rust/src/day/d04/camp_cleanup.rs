use crate::day::{Input, InputError, InputResult, Loc, Output, Wrapper};

fn part_one(inp: &Vec<(Loc, Loc)>) -> Output {
    Output::Nusize(
        inp.iter()
            .filter(|r| {
                r.0 .0 <= r.1 .0 && r.1 .1 <= r.0 .1 || r.1 .0 <= r.0 .0 && r.0 .1 <= r.1 .1
            })
            .count(),
    )
}

fn part_two(inp: &Vec<(Loc, Loc)>) -> Output {
    Output::Nusize(
        inp.iter()
            .filter(|r| !(r.0 .1 < r.1 .0 || r.0 .0 > r.1 .1))
            .count(),
    )
}

pub fn run(input: Input) -> (Output, Output) {
    let input = input.unwrap();

    (part_one(&input), part_two(&input))
}

pub fn parse() -> InputResult<Input> {
    let mut input: Vec<(Loc, Loc)> = Vec::new();

    for line in std::fs::read_to_string("../input/04")?.lines() {
        let elements: Vec<&str> = line.split('-').collect();

        if elements.len() != 3 {
            return Err(InputError::InvalidInput);
        }

        input.push(if let Some((lhs, rhs)) = elements[1].split_once(',') {
            (
                (elements[0].parse()?, lhs.parse()?),
                (rhs.parse()?, elements[2].parse()?),
            )
        } else {
            return Err(InputError::InvalidInput);
        });
    }

    Ok(Input::VTLoc(input))
}
