use crate::day::{Input, InputError, InputResult, Output, Wrapper};

#[derive(Clone, Copy)]
struct Marble {
    idx: usize,
    next: usize,
    prev: usize,
}

impl Marble {
    fn new(idx: usize, next: usize, prev: usize) -> Self {
        Self { idx, next, prev }
    }
}

fn part_one(players: usize, max: usize) -> usize {
    let mut scores: Vec<usize> = vec![0; players];
    let mut buf: Vec<Marble> = vec![Marble::new(0, 0, 0); max - max / 23 + 1];

    let mut pt: usize = 0;
    let mut ne: usize = 1;

    for cur in 1..=max {
        if cur % 23 == 0 {
            let mut rem_idx: usize = pt;
            for _ in 0..7 {
                rem_idx = buf[rem_idx].prev;
            }

            let next: usize = buf[rem_idx].next;
            let prev: usize = buf[rem_idx].prev;

            let mut player = cur % players;
            player = if player == 0 { players - 1 } else { player - 1 };

            scores[player] += cur + buf[rem_idx].idx;

            buf[prev].next = next;
            buf[next].prev = prev;
            pt = next;
        } else {
            let new_marble_idx: usize = ne;

            let next_marble_idx: usize = buf[buf[pt].next].next;
            let prev_marble_idx = buf[next_marble_idx].prev;

            buf[next_marble_idx].prev = new_marble_idx;
            buf[prev_marble_idx].next = new_marble_idx;

            pt = new_marble_idx;
            let new_marble: Marble = Marble::new(cur, next_marble_idx, prev_marble_idx);
            buf[ne] = new_marble;
            ne += 1;
        }
    }

    if let Some(max) = scores.iter().max() {
        *max
    } else {
        0
    }
}

fn part_two(players: usize, max: usize) -> usize {
    part_one(players, max * 100)
}

pub fn run(input: Input) -> (Output, Output) {
    let values: Vec<usize> = input.unwrap();
    let (players, max): (usize, usize) = (values[0], values[1]);

    (
        Output::Nusize(part_one(players, max)),
        Output::Nusize(part_two(players, max)),
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
