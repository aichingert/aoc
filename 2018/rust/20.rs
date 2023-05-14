// Advent of Code 2018, day 20
// (c) aichingert

use std::collections::{HashMap, HashSet};

type Map = HashMap<(i32, i32), HashSet<(i32, i32)>>;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_char(ch: char) -> Self {
        match ch {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            _ => panic!("nope"),
        }
    }

    fn update(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

struct RegularMap {
    map: Map,
}

impl RegularMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn create_map(&mut self, regex: &Vec<char>, s: (i32, i32)) {
        let mut loc: usize = 0;
        let mut cur: (i32, i32) = s;
        self.map.insert(cur, HashSet::new());

        while loc < regex.len() {
            match regex[loc] {
                'N' | 'S' | 'E' | 'W' => {
                    let dir = Direction::from_char(regex[loc]).update();
                    self.map
                        .get_mut(&cur)
                        .map(|val| val.insert((cur.0 + dir.0, cur.1 + dir.1)));
                    cur = (cur.0 + dir.0, cur.1 + dir.1);
                }
                '(' => {
                    loc = self.execute_regex(loc, cur, regex);
                }
                ')' => (),
                s => panic!("invalid token: {}", s),
            }

            loc += 1;
        }
    }

    fn execute_regex(&mut self, sloc: usize, pos: (i32, i32), regex: &Vec<char>) -> usize {
        let mut done = false;
        let mut brackets: u32 = 0;
        let mut splits = Vec::<usize>::new();
        let mut loc = sloc;

        while !done {
            match regex[loc] {
                '(' => brackets += 1,
                ')' => brackets -= 1,
                '|' => {
                    if brackets == 1 {
                        splits.push(loc);
                    }
                }
                _ => (),
            }
            loc += 1;

            done = brackets == 0;
        }

        //println!("{:?}", &regex[1 + sloc..splits[0]]);
        //println!("{:?}", &regex[1 + splits[0]..loc]);

        self.create_map(&regex[1 + sloc..splits[0]].to_vec(), pos);
        self.create_map(&regex[1 + splits[0]..loc].to_vec(), pos);

        loc
    }
}

fn parse() -> RegularMap {
    let inp = std::fs::read_to_string("../input/20")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut regular_map: RegularMap = RegularMap::new();

    regular_map.create_map(&inp[1..inp.len() - 1].to_vec(), (0, 0));
    for 
    println!("{:?}", regular_map.map);

    regular_map
}

fn main() {
    let m = parse();
}
