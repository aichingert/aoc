// Advent of Code 2018, day 22
// (c) aichingert

use std::collections::{HashMap, HashSet};

fn geologic_index(gi: &mut HashMap<(i64, i64), i64>, x: i64, y: i64, depth: i64) -> i64 {
    if gi.contains_key(&(x, y)) {
        return gi[&(x, y)];
    }

    let geoi = match (x, y) {
        (0, _) => y * 48271,
        (_, 0) => x * 16807,
        (_, _) => {
            let a = (geologic_index(gi, x - 1, y, depth) + depth) % 20183;
            let b = (geologic_index(gi, x, y - 1, depth) + depth) % 20183;

            a * b
        }
    };
    gi.insert((x, y), geoi);

    gi[&(x, y)]
}

fn part1(depth: i64, x: i64, y: i64, gi: &mut HashMap<(i64, i64), i64>) -> i64 {
    (0..=y)
        .map(|i| (0..=x)
             .map(|j| (geologic_index(gi, j, i, depth) + depth) % 20183 % 3)
             .sum::<i64>())
        .sum::<i64>()
}

enum Tool {
    Torch,
    Gear,
    Neither,
}

impl Tool {
    fn adjacent(&self, cur: (i64, i64), gi: &mut HashMap<(i64, i64), i64>) -> Vec<(i64, i64)> {
        let mut adjacent = Vec::new();

        // ROCKY == 0 => Gear | Torch !Neihter
        // WET == 1 => Gear | Neither !Torch
        // NARROW == 2 => Torch | Neither !Gear
 
        for dir in [(1, 0),(-1,0),(0,1),(0,-1)].iter() {
            let x = cur.0 + dir.0;
            let y = cur.1 + dir.1;

            if x < 0 || y < 0 { continue; }

            let region = (geologic_index(gi, x, y, depth) + depth) % 20183 % 3;

            match self {
                Self::Torch => if region == 0 || region == 2 {
                    adjacent.push((x, y));
                }
                _ => (),
            }
        }

        adjacent
    }
}

fn part_two(depth: i64, goal: (i64, i64), gi: &mut HashMap<(i64, i64), i64>) -> u32 {
    let mut tool: Tool = Tool::Torch; 

    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/22").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let depth: i64 = inp[0][7..].parse().unwrap();
    let (x, y): (&str, &str) = inp[1].split_once(',').unwrap();
    let (x, y): (i64, i64) = (x[8..].parse().unwrap(), y.parse().unwrap());
    let mut geologic_index: HashMap<(i64, i64), i64> = HashMap::from([((0, 0), 0), ((x, y), 0)]);

    println!("Part 1: {}", part1(depth, x, y, &mut geologic_index));
    println!("Part 2: {}", 10);
}
