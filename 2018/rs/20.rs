// Advent of Code 2018, day 20
// (c) aichingert

use std::collections::{HashMap, HashSet};

type Map = HashMap<Loc, HashSet<Loc>>;
type Loc = (i32, i32);

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

    fn update(&self) -> Loc {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

struct RegexDecoder {
    raw: Vec<char>,
    routes: Vec<String>,
}

impl RegexDecoder {
    fn new(raw: Vec<char>) -> Self {
        Self {
            raw,
            routes: Vec::new(),
        }
    }

    fn build(&mut self) {
        let mut cur: String = String::new();
        let mut index: usize = 0;

        while index < self.raw.len() {
            match self.raw[index] {
                'N' | 'E' | 'W' | 'S' => cur.push(self.raw[index]),
                '(' => {
                    let mut paths = evaluate_expresion(&self.raw, &mut index);
                }
                _ => (),
            };

            index += 1;
        }
    }
}

fn evaluate_expresion(raw: &Vec<char>, index: &mut usize) -> Vec<char> {
    let mut branches = Vec::<usize>::with_capacity(4);
    let mut brackets = 0u32;
    let start_index: usize = *index;

    loop {
        match self.raw[*index] {
            '(' => brackets += 1,
            ')' => brackets -= 1,
            '|' => {
                if brackets == 1 {
                    branches.push(*index);
                }
            }
            _ => (),
        };

        *index += 1;

        if brackets == 0 {
            break;
        }
    }

    let len = branches.len();
    let mut slices = Vec::<Vec<char>>::with_capacity(len + 1);

    slices.push(self.raw[start_index + 1..branches[0]].to_vec());
    for i in 1..len - 1 {
        slices.push(self.raw[branches[i] + 1..branches[i + 1]].to_vec());
    }
    slices.push(self.raw[branches[len - 1] + 1..*index - 1].to_vec());

    for s in &slices {
        println!("{:?}", s);
    }

    slices
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

    fn create_map(&mut self, regex: &Vec<char>, s: Loc) -> Loc {
        let locs: Vec<Loc>;
        let mut index: usize = 0;
        let mut cur: Loc = s;
        if !self.map.contains_key(&cur) {
            self.map.insert(cur, HashSet::new());
        }

        while index < regex.len() {
            match regex[index] {
                'N' | 'S' | 'E' | 'W' => {
                    let dir = Direction::from_char(regex[index]).update();
                    self.map
                        .get_mut(&cur)
                        .map(|val| val.insert((cur.0 + dir.0, cur.1 + dir.1)));
                    cur = (cur.0 + dir.0, cur.1 + dir.1);
                }
                '(' => {
                    (index, locs) = self.execute_regex(index, cur, regex);
                    for points in locs {
                        self.create_map(&regex[index..].to_vec(), points);
                    }

                    return cur;
                }
                ')' => (),
                s => panic!("invalid token: {}", s),
            }

            if !self.map.contains_key(&cur) {
                self.map.insert(cur, HashSet::new());
            }

            index += 1;
        }

        cur
    }

    fn execute_regex(&mut self, sidx: usize, pos: Loc, regex: &Vec<char>) -> (usize, Vec<Loc>) {
        let mut done = false;
        let mut brackets: u32 = 0;
        let mut splits = Vec::<usize>::new();
        let mut idx = sidx;

        while !done {
            match regex[idx] {
                '(' => brackets += 1,
                ')' => brackets -= 1,
                '|' => {
                    if brackets == 1 {
                        splits.push(idx);
                    }
                }
                _ => (),
            }
            idx += 1;

            done = brackets == 0;
        }

        let len = splits.len();
        let mut end_locations = Vec::<Loc>::with_capacity(len + 1);

        end_locations.push(self.create_map(&regex[1 + sidx..splits[0]].to_vec(), pos));
        for i in 0..splits.len() - 1 {
            end_locations.push(self.create_map(&regex[1 + splits[i]..splits[i + 1]].to_vec(), pos));
        }
        end_locations.push(self.create_map(&regex[1 + splits[len - 1]..idx].to_vec(), pos));

        (idx, end_locations)
    }
}

fn parse() -> RegularMap {
    let inp = std::fs::read_to_string("../input/20")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut regular_map: RegularMap = RegularMap::new();

    let mut rd = RegexDecoder::new(inp);
    rd.build();
    //regular_map.create_map(&inp[1..inp.len() - 1].to_vec(), (0, 0));

    regular_map
}

fn main() {
    let m = parse();
}
