// Advent of Code 2017, day 22
// (c) aichingert

use std::collections::HashMap;

#[derive(Clone,Copy)]
enum State {
    //Clean,
    Weakened,
    Infected,
    Flagged,
}

fn parse() -> (HashMap<(i32,i32), State>, (i32,i32)) {
    let inp = std::fs::read_to_string("../input/22").unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut map = HashMap::new();
    let loc = (inp.len() as i32 /2,inp[0].len() as i32 /2);

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] == '#' {
                map.insert((i as i32,j as i32), State::Infected);
            }
        }
    }


    (map, loc)
}

fn solve(start: (i32,i32), m: &HashMap<(i32,i32), State>, steps: u32, part1: bool) -> u32 {
    let mut infected = 0;
    let mut loc = start;
    let mut map = m.clone();
    let mut deg = 90i32;

    for _ in 0..steps {
        if part1 {
            if map.contains_key(&loc) {
                deg -= 90;
                map.remove(&loc);
            } else {
                deg += 90;
                map.insert(loc, State::Infected);
                infected += 1;
            }
        } else {
            if map.contains_key(&loc) {
                match map[&loc] {
                    State::Weakened => {
                        infected += 1;
                        map.insert(loc, State::Infected);
                    },
                    State::Infected => {
                        map.insert(loc, State::Flagged);
                        deg -= 90;
                    },
                    State::Flagged  => {
                        map.remove(&loc);
                        deg = match deg {
                            0  => 180,
                            90 => 270,
                            180=> 0,
                            270=> 90,
                            _ => panic!("invalid degree")
                        };
                        
                    },
                }
            } else {
                deg += 90;
                map.insert(loc, State::Weakened);
            }
        }

        if deg < 0 { deg = 270; }
        deg %= 360;

        match deg {
            0   => loc.1 += 1,
            90  => loc.0 -= 1,
            180 => loc.1 -= 1,
            270 => loc.0 += 1,
            _ => panic!("invalid degree")
        }
    }

    infected
}

fn main() {
    let (map, loc) = parse();

    println!("Part 1: {}", solve(loc, &map, 10000, true));
    println!("Part 2: {}", solve(loc, &map, 10000000, false));
}
