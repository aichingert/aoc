use crate::day::{Input, Output, InputResult, Wrapper};
use std::collections::{HashMap, HashSet};

pub struct Valve {
    flow_rate: u32,
    connects: Vec<String>,
}

const PART_ONE: u32 = 30;
const PART_TWO: u32 = 26;

fn part_one(
    valves: &HashMap<String, Valve>,
    cached: &mut HashSet<(String, u32, u32)>,
    opened_valves: &mut Vec<String>,
    current_valve: &str,
    minute: u32,
    released: u32,
) -> u32 {
    if minute == PART_ONE || cached.contains(&(current_valve.to_string(), minute, released)) {
        return released;
    }
    cached.insert((current_valve.to_string(), minute, released));

    let valve = valves.get(current_valve).unwrap();
    let current_str = current_valve.to_string();
    let mut max_released: u32 = 0;

    match opened_valves.contains(&current_str) {
        true => {
            for nv in valve.connects.iter() {
                max_released = max_released
                    .max(part_one(valves, cached, opened_valves, nv, minute + 1, released));
            }

            max_released
        },
        false => {
            opened_valves.push(current_str);
            let new_released =released + valve.flow_rate * (PART_ONE - minute - 1);
            max_released = part_one(
                valves, cached, opened_valves, current_valve, minute + 1, new_released
            );
            opened_valves.pop();

            for nv in valve.connects.iter() {
                max_released = max_released
                    .max(part_one(valves, cached, opened_valves, nv, minute + 1, released));
            }
            max_released
        },
    }
}

fn part_two(
    valves: &HashMap<String, Valve>,
    cached: &mut HashSet<(String, String, u32, u32)>,
    opened_valves: &mut Vec<String>,
    me: &str,
    elephant: &str,
    minute: u32,
    released: u32,
) -> u32 {
    if minute == PART_TWO 
        || cached.contains(&(me.to_string(), elephant.to_string(), minute, released))
        || cached.contains(&(elephant.to_string(), me.to_string(), minute, released)) {
        return released;
    }
    //println!("{:?} {me} {elephant} {released}", opened_valves);
    cached.insert((me.to_string(), elephant.to_string(), minute, released));
    cached.insert((elephant.to_string(), me.to_string(), minute, released));

    let my_valve = valves.get(me).unwrap();
    let el_valve = valves.get(elephant).unwrap();

    let my_str = me.to_string();
    let el_str = elephant.to_string();
    let mut max_released: u32 = 0;

    let is_left_open = opened_valves.contains(&my_str);

    match (is_left_open, opened_valves.contains(&el_str)) {
        (true, true) => {
            for nxt_el_valve in el_valve.connects.iter() {
                for nxt_my_valve in my_valve.connects.iter() {
                    max_released = max_released.max(
                        part_two(valves, cached, opened_valves, nxt_my_valve, nxt_el_valve, minute + 1, released));
                }
            }

            max_released
        }
        (true, false) | (false, true) => {
            let (c_str, c_valve, o_valve) = if is_left_open {
                (el_str, el_valve, my_valve)
            } else {
                (my_str, my_valve, el_valve)
            };

            opened_valves.push(c_str);
            let new_released = released + c_valve.flow_rate * (PART_TWO - minute - 1);

            for nxt_o_valve in o_valve.connects.iter() {
                max_released = if is_left_open {
                    max_released.max(part_two(valves, cached, opened_valves, nxt_o_valve, elephant, minute + 1, new_released))
                } else {
                    max_released.max(part_two(valves, cached, opened_valves, me, nxt_o_valve, minute + 1, new_released))
                };
            }
            opened_valves.pop();

            for nxt_c_valve in c_valve.connects.iter() {
                for nxt_o_valve in o_valve.connects.iter() {
                    max_released = max_released
                        .max(part_two(valves, cached, opened_valves, nxt_o_valve, nxt_c_valve, minute + 1, released))
                }
            }

            max_released
        }
        (false, false) => {
            opened_valves.push(el_str);
            let new_released = released + el_valve.flow_rate * (PART_TWO - minute - 1);

            for nxt_my_valve in my_valve.connects.iter() {
                max_released = max_released.max(
                    part_two(valves, cached, opened_valves, nxt_my_valve, elephant, minute + 1, new_released));
            }
            opened_valves.pop();
            opened_valves.push(my_str);
            let new_released = released + my_valve.flow_rate * (PART_TWO - minute - 1);

            for nxt_el_valve in el_valve.connects.iter() {
                max_released = max_released.max(
                    part_two(valves, cached, opened_valves, me, nxt_el_valve, minute + 1, new_released));
            }
            opened_valves.pop();


            for nxt_el_valve in el_valve.connects.iter() {
                for nxt_my_valve in my_valve.connects.iter() {
                    max_released = max_released.max(
                        part_two(valves, cached, opened_valves, nxt_my_valve, nxt_el_valve, minute + 1, released));
                }
            }

            max_released
        }
    }
}

pub fn run(input: Input) -> (Output, Output) {
    let valves: HashMap<String, Valve> = input.unwrap();

    let part_one = part_one(&valves, &mut HashSet::new(), &mut Vec::new(), "AA", 0, 0);
    let part_two = part_two(&valves, &mut HashSet::new(), &mut Vec::new(), "AA", "AA", 0, 0);

    (Output::Nu32(part_one), Output::Nu32(part_two))
}

pub fn parse() -> InputResult<Input> {
    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in std::fs::read_to_string("../input/16")?.trim().lines() {
        let sep = line.split(' ').collect::<Vec<_>>();

        let ident = sep[1].to_string();
        let flow_rate = sep[4][5..sep[4].len() - 1].parse::<u32>()?;

        let mut valve: Valve = Valve {
            flow_rate,
            connects: Vec::new(),
        };

        for i in 9..sep.len() - 1 {
            valve.connects.push(sep[i][..sep[i].len() - 1].to_string());
        }
        valve.connects.push(sep[sep.len() - 1].to_string());
        valves.insert(ident, valve);
    }

    Ok(Input::D16(valves))
}
