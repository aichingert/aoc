use crate::day::{Input, Output, InputResult, Wrapper};
use std::collections::{HashMap, HashSet};

pub struct Valve {
    flow_rate: u32,
    connects: Vec<String>,
}

const LIMIT: u32 = 30;

fn part_one(valves: &HashMap<String, Valve>) -> u32 {
    rec_find(valves, &Vec::new(), &mut HashSet::new(), "AA", 0, 0)
}

fn rec_find(valves: &HashMap<String, Valve>, 
    open: &Vec<String>, cache: &mut HashSet<(String, u32, u32)>,
    cur: &str, min: u32, rel: u32) -> u32 {
    if min == 30 || cache.contains(&(cur.to_string(), min, rel)) {
        //println!("reached {:?}", open);
        return rel;
    }
    cache.insert((cur.to_string(), min, rel));

    let mut next_open = open.clone();
    let valve = valves.get(cur).unwrap();
    let string = cur.to_string();

    match next_open.contains(&string) {
        true => {
            let mut max_rel = 0;

            for nv in valve.connects.iter() {
                max_rel = max_rel.max(rec_find(valves, &next_open, cache, nv, min + 1, rel));
            }

            max_rel
        },
        false => {
            next_open.push(string);
            let mut cur_max = rec_find(valves, &next_open, cache, cur, min + 1, rel + valve.flow_rate * (LIMIT - min - 1));
            next_open.pop();
            for nv in valve.connects.iter() {
                cur_max = cur_max.max(rec_find(valves, &next_open, cache, nv, min + 1, rel));
            }
            cur_max
        },
    }
}

pub fn run(input: Input) -> (Output, Output) {
    let valves: HashMap<String, Valve> = input.unwrap();

    (Output::Nu32(part_one(&valves)), Output::Nu32(0))
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
