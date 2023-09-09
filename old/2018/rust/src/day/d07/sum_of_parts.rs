use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use std::collections::HashMap;

const WORKERS: usize = 5;

fn part_one(blockers: &HashMap<char, Vec<char>>) -> String {
    let mut ans = Vec::<char>::new();
    let mut blockers = blockers.clone();
    let mut free: Vec<char> = Vec::new();

    while !blockers.is_empty() {
        for key in blockers.keys() {
            if blockers[key].is_empty() {
                free.push(*key);
            }
        }

        for i in 0..free.len() {
            blockers.remove(&free[i]);
        }

        free.sort();
        let cur = free.remove(0);

        for value in blockers.values_mut() {
            if let Some(idx) = value.iter().position(|ch| *ch == cur) {
                value.remove(idx);
            }
        }

        ans.push(cur);
    }

    ans.extend(&free);
    ans.iter().collect::<String>()
}

fn part_two(blockers: &mut HashMap<char, Vec<char>>) -> u32 {
    let mut ans: u32 = 0;
    let mut free: Vec<char> = Vec::new();
    let mut workers: Vec<(char, u8)> = Vec::with_capacity(WORKERS);

    while !blockers.is_empty() {
        for key in blockers.keys() {
            if blockers[key].is_empty()
                && !free.contains(key)
                && workers.iter().find(|(ch, _)| ch == key).is_none()
            {
                free.push(*key);
            }
        }

        ans += 1;

        free.sort();
        while free.len() > 0 && workers.len() < WORKERS {
            let key = free.remove(0);
            workers.push((key, (key as u8 - b'A') + 61));
        }

        let mut i: usize = 0;

        while i < workers.len() {
            workers[i].1 -= 1;

            if workers[i].1 == 0 {
                let (key, _) = workers.remove(i);
                blockers.remove(&key);

                for value in blockers.values_mut() {
                    if let Some(idx) = value.iter().position(|ch| *ch == key) {
                        value.remove(idx);
                    }
                }
            } else {
                i += 1;
            }
        }
    }

    ans
}

pub fn run(input: Input) -> (Output, Output) {
    let mut input: HashMap<char, Vec<char>> = input.unwrap();

    (
        Output::S(part_one(&input)),
        Output::Nu32(part_two(&mut input)),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut input: HashMap<char, Vec<char>> = HashMap::new();

    for line in std::fs::read_to_string("../input/07")?.lines() {
        let elements: Vec<char> = line.chars().collect();

        if elements.len() < 36 {
            return Err(InputError::InvalidInput);
        }

        if let Some(locking) = input.get_mut(&elements[36]) {
            locking.push(elements[5]);
        } else {
            input.insert(elements[36], vec![elements[5]]);
        }

        if input.get(&elements[5]) == None {
            input.insert(elements[5], vec![]);
        }
    }

    Ok(Input::HchVch(input))
}
