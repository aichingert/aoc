use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use std::collections::HashMap;

fn part_one(guards: &HashMap<u16, Vec<[u16; 60]>>) -> u16 {
    let mut p1: (&u16, u16) = (&0, 0);

    for key in guards.keys() {
        let sum = guards[key]
            .iter()
            .map(|arr| arr.iter().sum::<u16>())
            .sum::<u16>();

        if p1.1 < sum {
            p1 = (key, sum);
        }
    }

    let time_table = &guards[p1.0];
    let (mut minute, mut max): (u16, u16) = (0, 0);
    let mut combined: [u16; 60] = [0; 60];

    for i in 0..time_table.len() {
        for j in 0..time_table[i].len() {
            combined[j] += time_table[i][j];
        }
    }

    for i in 0..combined.len() {
        if combined[i] > max {
            minute = i as u16;
            max = combined[i];
        }
    }

    *p1.0 * minute
}

fn part_two(guards: &HashMap<u16, Vec<[u16; 60]>>) -> u16 {
    let (mut id, mut minute, mut max): (u16, u16, u16) = (0, 0, 0);

    for key in guards.keys() {
        let time_table = &guards[key];
        let mut asleep = [0u16; 60];

        for i in 0..time_table.len() {
            for j in 0..time_table[i].len() {
                if time_table[i][j] != 0 {
                    asleep[j] += 1;

                    if asleep[j] > max {
                        id = *key;
                        max = asleep[j];
                        minute = j as u16;
                    }
                }
            }
        }
    }

    id * minute
}

pub fn run(input: Input) -> (Output, Output) {
    let input: HashMap<u16, Vec<[u16; 60]>> = input.unwrap();

    (
        Output::Nu16(part_one(&input)),
        Output::Nu16(part_two(&input)),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut guards: HashMap<u16, Vec<[u16; 60]>> = HashMap::new();
    let mut sorted: Vec<((u32, u32, u8, u8), String)> = Vec::new();

    for line in std::fs::read_to_string("../input/04")?.lines() {
        let elements: Vec<&str> = line.split(' ').collect();

        let (hour, minute) = if let Some((hour, minute)) = elements[1].split_once(':') {
            (hour.parse()?, minute[..minute.len() - 1].parse()?)
        } else {
            return Err(InputError::InvalidInput);
        };

        let date: Result<Vec<u32>, _> = elements[0][1..]
            .split('-')
            .map(|n| n.parse::<u32>())
            .collect();
        let date = date?;
        sorted.push(((date[1], date[2], hour, minute), elements[3].to_string()));
    }

    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    let mut last_id: u16 = 0;
    let mut asleep: (bool, (u8, u8)) = (false, (0, 0));

    for i in 0..sorted.len() {
        match sorted[i].1.as_str() {
            "asleep" => asleep = (true, (sorted[i].0 .2, sorted[i].0 .3)),
            "up" => {
                if asleep.0 && last_id != 0 {
                    let mut times = [0; 60];
                    if let Some(time_table) = guards.get_mut(&last_id) {
                        for i in asleep.1 .1..sorted[i].0 .3 {
                            times[i as usize] += 1;
                        }
                        time_table.push(times);
                    } else {
                        for i in asleep.1 .1..sorted[i].0 .3 {
                            times[i as usize] += 1;
                        }
                        guards.insert(last_id, vec![times]);
                    }
                }
            }
            _ => {
                last_id = sorted[i].1[1..].parse()?;
            }
        }
    }

    Ok(Input::D04(guards))
}
