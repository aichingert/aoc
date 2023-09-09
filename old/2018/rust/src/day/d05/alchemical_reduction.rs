use crate::day::{Input, InputResult, Output, Wrapper};
use std::collections::VecDeque;

const OFFSET: u8 = b'a' - b'A';

fn reaction(polymers: &mut VecDeque<u8>) -> usize {
    let mut idx: usize = 0;

    while idx + 1 < polymers.len() {
        if polymers[idx] - OFFSET == polymers[idx + 1]
            || polymers[idx + 1] - OFFSET == polymers[idx]
        {
            polymers.remove(idx);
            polymers.remove(idx);

            if idx > 0 {
                idx -= 1;
            }
        } else {
            idx += 1;
        }
    }

    polymers.len()
}

fn part_two(polymers: VecDeque<u8>) -> usize {
    let mut min: usize = usize::MAX;

    for letter in b'A'..=b'Z' {
        let mut poly_clone: VecDeque<u8> = VecDeque::with_capacity(polymers.len());

        for i in 0..polymers.len() {
            if !(polymers[i] == letter || (polymers[i] - OFFSET) == letter) {
                poly_clone.push_back(polymers[i]);
            }
        }

        let result: usize = reaction(&mut poly_clone);
        min = min.min(result);
    }

    min
}

pub fn run(input: Input) -> (Output, Output) {
    let input: VecDeque<u8> = input.unwrap();

    (
        Output::Nusize(reaction(&mut input.clone())),
        Output::Nusize(part_two(input)),
    )
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::VDu8(
        std::fs::read_to_string("../input/05")?
            .trim()
            .chars()
            .map(|ch| ch as u8)
            .collect(),
    ))
}
