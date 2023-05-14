// Advent of Code 2018, day 20
// (c) aichingert

use std::collections::{HashMap, HashSet};

type Map = HashMap<(i32, i32), HashSet<(i32, i32)>>;

fn part1(map: &Map) -> i32 {
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();

    0
}

fn calc_dists(map: &Map, dists: &mut HashMap<(i32, i32), i32>, s: (i32, i32), dist: i32) {
    if dists.contains_key(&s) {
        return;
    }

    dists.insert(s, dist);

    let mut cur = s;
}

fn parse() -> Map {
    let inp = std::fs::read_to_string("../input/20").unwrap();
    let inp = inp[1..inp.len() - 1].chars().collect::<Vec<char>>();
    let mut map: Map = HashMap::new();

    create_map(&inp, 0, (0, 0), &mut map);

    map
}

fn create_map(regex: &Vec<char>, pos: usize, sloc: (i32, i32), map: &mut Map) {
    let mut loc = sloc;
    let mut pos = pos;
    if !map.contains_key(&loc) {
        map.insert(loc, HashSet::new());
    }

    while pos < regex.len() {
        match regex[pos] {
            'N' => {
                map.get_mut(&loc).map(|val| val.insert((loc.0, loc.1 - 1)));
                loc.1 -= 1;
            }
            'S' => {
                map.get_mut(&loc).map(|val| val.insert((loc.0, loc.1 + 1)));
                loc.1 += 1;
            }
            'E' => {
                map.get_mut(&loc).map(|val| val.insert((loc.0 + 1, loc.1)));
                loc.0 += 1;
            }
            'W' => {
                map.get_mut(&loc).map(|val| val.insert((loc.0 - 1, loc.1)));
                loc.0 -= 1;
            }
            '(' => {
                pos += 1;
                create_map(regex, pos, loc, map);

                let mut brackets: i32 = 1;
                while brackets > 0 {
                    if regex[pos] == '(' {
                        brackets += 1;
                    } else if regex[pos] == ')' {
                        brackets -= 1;
                    }

                    pos += 1;
                }
            }
            '|' => {
                create_map(regex, pos + 1, sloc, map);

                let mut done = false;
                let mut brackets = 1;
                while !done {
                    pos += 1;

                    if regex[pos] == '(' {
                        brackets += 1;
                    } else if regex[pos] == ')' {
                        brackets -= 1;
                    }

                    done = (regex[pos] == '|' && brackets == 1) || brackets == 0;
                }
            }
            _ => (),
        }

        if !map.contains_key(&loc) {
            map.insert(loc, HashSet::new());
        }
        pos += 1;
    }
}

fn path_exists(s: (i32, i32), g: (i32, i32), map: &Map, already: &mut HashSet<(i32, i32)>) -> bool {
    if s == g {
        return true;
    }

    if already.contains(&s) {
        return false;
    }

    let mut cur = s;
    while already.insert(cur) {
        let cons = &map[&cur];

        if cons.len() == 0 {
            return false;
        }
        if cons.len() == 1 {
            cur = *cons.iter().clone().take(1).next().unwrap();

            if cur == g {
                return true;
            }
        } else {
            let mut iter = cons.iter().clone();
            while let Some(p) = iter.next() {
                if path_exists(*p, g, map, already) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let m = parse();
    println!("{}", path_exists((0, 0), (3, -3), &m, &mut HashSet::new()));
}
