use std::collections::VecDeque;

use crate::day::{Input, InputError, InputResult, Output, Wrapper};

fn part_one(players: usize, max: usize) -> u32 {
    let mut vec_deq: VecDeque<u32> = VecDeque::with_capacity(max);
    let mut scores: Vec<u32> = vec![0; players];
    let mut pt: usize = 0;
    vec_deq.push_front(0);

    for cur in 1..=max {
        if cur % 23 == 0 {
            for _ in 0..7 {
                if pt > 0 {
                    pt -= 1;
                } else {
                    pt = vec_deq.len() - 1;
                }
            }

            let mut player = cur % players;

            player = if player == 0 { players - 1 } else { player - 1 };

            scores[player] += cur as u32 + vec_deq.remove(pt).unwrap();
        } else {
            pt = if pt + 1 < vec_deq.len() { pt + 1 } else { 0 };
            pt += 1;
            vec_deq.insert(pt, cur as u32);
        }
    }

    if let Some(max) = scores.iter().max() {
        *max
    } else {
        0
    }
}

fn part_two(players: usize, max: usize) -> u32 {
    part_one(players, max * 100)
}

pub fn run(input: Input) -> (Output, Output) {
    let values: Vec<usize> = input.unwrap();
    let (players, max): (usize, usize) = (values[0], values[1]);

    (
        Output::Nu32(part_one(players, max)),
        Output::Nu32(part_two(players, max)),
    )
}

pub fn parse() -> InputResult<Input> {
    let input: String = std::fs::read_to_string("../input/09")?;
    let input: Vec<&str> = input.trim().split(' ').collect();

    if input.len() < 7 {
        return Err(InputError::InvalidInput);
    }

    Ok(Input::Vusize(vec![input[0].parse()?, input[6].parse()?]))
}
