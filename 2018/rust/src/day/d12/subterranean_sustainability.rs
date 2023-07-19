use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use std::collections::{HashMap, HashSet};

fn solve(state: &HashSet<i64>, muster: &HashMap<String, String>, steps: u32) -> i64 {
    let mut state = state.clone();
    let mut min: i64 = -1;
    let mut max: i64 = 110;

    for _ in 0..steps {
        let mut next = HashSet::new();
        let mut elements = String::new();

        for j in min..min + 5 {
            elements.push(if state.contains(&j) { '#' } else { '.' });
        }

        for i in min + 2..max {
            if muster.contains_key(&elements) && &muster[&elements] == "#" {
                next.insert(i);
            }

            elements.remove(0);
            elements.push(if state.contains(&(i + 3)) { '#' } else { '.' });
        }

        min -= 2;
        max += 2;
        state = next;
    }

    state.iter().fold(0, |c, i| c + *i)
}

pub fn run(input: Input) -> (Output, Output) {
    let (state, muster): (HashSet<i64>, HashMap<String, String>) = input.unwrap();
    let two_hundred: i64 = solve(&state, &muster, 200);

    (
        Output::Ni64(solve(&state, &muster, 20)),
        Output::Ni64((5000000 - 2) * (solve(&state, &muster, 300) - two_hundred) + two_hundred),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut state: HashSet<i64> = HashSet::new();
    let mut muster: HashMap<String, String> = HashMap::new();

    if let Some((initial, str_muster)) = std::fs::read_to_string("../input/12")?.split_once("\n\n")
    {
        if let Some((_, str_state)) = initial.split_once(": ") {
            let chs: Vec<char> = str_state.chars().collect();

            for i in 0..chs.len() {
                if chs[i] == '#' {
                    state.insert(i as i64);
                }
            }
        } else {
            return Err(InputError::InvalidInput);
        }

        for line in str_muster.lines() {
            if let Some((lhs, rhs)) = line.split_once(" => ") {
                muster.insert(lhs.to_string(), rhs.to_string());
            } else {
                return Err(InputError::InvalidInput);
            }
        }
    } else {
        return Err(InputError::InvalidInput);
    }

    Ok(Input::D12((state, muster)))
}
