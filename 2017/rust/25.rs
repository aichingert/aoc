// Advent of Code 2017, day 25
// (c) aichingert

use std::collections::HashMap;

#[derive(Debug,Copy,Clone)]
struct State {
    write: i32,
    dir: i32,
    next: char,
}

impl State {
    fn new(next: char, dir: i32, write: i32) -> Self {
        State { write, dir, next }
    }
}


fn part1(times: i32, states: HashMap<char, [State;2]>) -> i32 {
    let mut tape = HashMap::<i32,i32>::from([(0,0)]);
    let mut loc = 0i32;
    let mut next = 'A';

    for _ in 0..times {
        let modes = states[&next];
        let cur = tape.entry(loc).or_insert(0);

        next = modes[*cur as usize].next;
        loc += modes[*cur as usize].dir;
        *cur = modes[*cur as usize].write;
    }

    tape.values().map(|i| *i).sum::<i32>()
}

fn part2() -> &'static str {
    "Merry Christmas!"
}

fn parse() -> (i32, HashMap<char, [State;2]>) {
    let inp = std::fs::read_to_string("../input/25").unwrap();
    let inp = inp.lines().skip(1);
    let mut states = HashMap::new();
    let times = inp.clone().take(1).collect::<Vec<&str>>()[0].split(' ').collect::<Vec<&str>>()[5].parse::<i32>().unwrap();
    let lines = inp.skip(2).collect::<Vec<&str>>();

    for i in (0..lines.len()).step_by(10) {
        let name = lines[i].chars().collect::<Vec<char>>()[9];
        let mut opts= Vec::<State>::new();

        for j in (i..i+6).step_by(4) {
            let write = lines[j+2].split(' ').collect::<Vec<&str>>();
            let write = write[8][..write[8].len()-1].parse::<i32>().unwrap();

            let dir = match lines[j+3].split(' ').collect::<Vec<&str>>()[10] {
                "left." => -1,
                "right." => 1,
                _ => panic!("invalid input!"),
            };
            let next = lines[j+4].chars().collect::<Vec<char>>()[26];

            opts.push(State::new(next, dir, write));
        }

        states.insert(name, [opts[0], opts[1]]);
    }

    (times, states)
}

fn main() {
    let (times, states) = parse();

    println!("Part 1: {}", part1(times, states));
    println!("Part 2: {}", part2());
}
