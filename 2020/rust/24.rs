// Advent of Code 2020, day 24
// (c) aichingert

use std::collections::{HashMap, HashSet};
const DIRS: [(i32, i32); 6] = [(0, 2), (0, -2), (-1, 1), (-1, -1), (1, 1), (1, -1)];

fn generate_floor() -> HashSet<(i32, i32)> {
    let mut black: HashSet<(i32, i32)> = HashSet::new();
    let mut cur: (i32, i32);

    for line in std::fs::read_to_string("../input/24").unwrap().lines() {
        let line = line.chars().collect::<Vec<char>>();
        let mut idx: usize = 0;
        cur = (0, 0);

        while idx < line.len() {
            if idx + 1 < line.len() {
                idx += 2;
                let vec = go_to_direction(line[idx - 2], line[idx - 1], &mut idx);
                cur = (cur.0 + vec.0, cur.1 + vec.1);
            } else {
                let vec = go_to_direction(line[idx], ' ', &mut idx);
                cur = (cur.0 + vec.0, cur.1 + vec.1);
                break;
            }
        }

        if black.contains(&cur) {
            black.remove(&cur);
        } else {
            black.insert(cur);
        }
    }

    black
}

fn go_to_direction(f: char, s: char, idx: &mut usize) -> (i32, i32) {
    match (f, s) {
        ('s', 'e') => (-1, 1),
        ('s', 'w') => (-1, -1),
        ('n', 'w') => (1, -1),
        ('n', 'e') => (1, 1),
        ('w', _) => {
            *idx -= 1;
            (0, -2)
        }
        ('e', _) => {
            *idx -= 1;
            (0, 2)
        }
        _ => panic!("invalid direction"),
    }
}

fn part_two(mut black: HashSet<(i32, i32)>, n: u32) -> usize {
    for _ in 0..n {
        let mut next: HashSet<(i32, i32)> = HashSet::new();
        let mut white: HashMap<(i32, i32), u8> = HashMap::new();

        for tile in black.iter() {
            let mut blacks: u8 = 0;

            for dir in DIRS {
                let neighbour = (tile.0 + dir.0, tile.1 + dir.1);

                if black.contains(&neighbour) {
                    blacks += 1;
                } else if let Some(count) = white.get_mut(&neighbour) {
                    *count += 1;
                } else {
                    white.insert(neighbour, 1);
                }
            }

            if blacks > 0 && blacks < 3 {
                next.insert(*tile);
            }
        }

        for tile in white.iter() {
            if *tile.1 == 2 {
                next.insert(*tile.0);
            }
        }

        black = next;
    }

    black.len()
}

fn main() {
    let black: HashSet<(i32, i32)> = generate_floor();

    println!("Part 1: {}", black.len());
    println!("Part 2: {}", part_two(black, 100));
}
