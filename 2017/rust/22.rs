// Advent of Code 2017, day 22
// (c) aichingert

use std::collections::HashMap;

#[derive(Clone,Copy)]
enum State {
    Infected,
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

fn part1(start: (i32,i32), m: &HashMap<(i32,i32), State>) -> u32 {
    let mut infected = 0;
    let mut loc = start;
    let mut map = m.clone();
    let mut deg = 90i32;

    for _ in 0..10000 {
        if map.contains_key(&loc) {
            deg -= 90;
            map.remove(&loc);
        } else {
            deg += 90;
            map.insert(loc, State::Infected);
            infected += 1;
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
    let part1 = part1(loc, &map);

    println!("Part 1: {}", part1);
    //println!("Part 2: {}", part2);
}
