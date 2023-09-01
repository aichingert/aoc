use crate::day::{d22::Direction, Coordinate, Input, InputResult, Output, Path, Wrapper};
use std::collections::BTreeMap;

const DIRS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn part_one(map: &BTreeMap<Coordinate, char>, path: &Vec<Path>, start: Coordinate) -> u32 {
    let mut pos = start;
    let mut dir = 0;
    let mut idx = 0;
    let mut n;

    let update_position = |dir: &Direction, next: &Coordinate| -> Option<Coordinate> {
        if let Some((c, ch)) = match dir {
            Direction::North => map.iter().filter(|c| c.0.x == next.x).last(),
            Direction::South => map.iter().filter(|c| c.0.x == next.x).rev().last(),
            Direction::East => map.iter().filter(|c| c.0.y == next.y).rev().last(),
            Direction::West => map.iter().filter(|c| c.0.y == next.y).last(),
        } {
            if *ch == '.' {
                return Some(*c);
            }
        }
        None
    };

    while idx < path.len() {
        (dir, n) = match path[idx] {
            Path::Left(n) => (if dir == 0 { DIRS.len() - 1 } else { dir - 1 }, n),
            Path::Right(n) => ((dir + 1) % DIRS.len(), n),
        };
        let vec = DIRS[dir].as_coordinate();

        for _ in 0..n {
            if let Some(next) = pos.apply(vec) {
                match map.get(&next) {
                    Some(ch) => {
                        if *ch == '#' {
                            break;
                        } else {
                            pos = next;
                        }
                    }
                    None => match update_position(&DIRS[dir], &next) {
                        Some(next) => pos = next,
                        None => break,
                    },
                };
            } else {
                match update_position(&DIRS[dir], &pos) {
                    Some(next) => pos = next,
                    None => break,
                }
            }
        }

        idx += 1;
    }

    let facing = match DIRS[dir] {
        Direction::North => 3,
        Direction::South => 1,
        Direction::East => 0,
        Direction::West => 2,
    };

    1000 * (pos.y as u32 + 1) + 4 * (pos.x as u32 + 1) + facing
}

pub fn run(input: Input) -> (Output, Output) {
    let (path, map): (Vec<Path>, BTreeMap<Coordinate, char>) = input.unwrap();
    let (start, _) = map.first_key_value().unwrap();

    println!("{}", part_one(&map, &path, *start));

    (Output::Nu32(0), Output::Nu32(0))
}

pub fn parse() -> InputResult<Input> {
    let input = std::fs::read_to_string("../input/22")?;
    let (inp_map, inp_path) = match input.split_once("\n\n") {
        Some((map, path)) => (map, path),
        None => return Err(crate::day::InputError::InvalidInput),
    };

    let mut map: BTreeMap<Coordinate, char> = BTreeMap::new();
    let mut path: Vec<Path> = Vec::new();
    let mut cur = String::new();
    let mut last: char = 'R';

    for (y, line) in inp_map.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' | '#' => map.insert(Coordinate::new(y, x), ch),
                ' ' => None,
                _ => return Err(crate::day::InputError::InvalidInput),
            };
        }
    }

    for ch in inp_path.chars() {
        match ch {
            'L' | 'R' => {
                let n = cur.parse::<u32>()?;

                path.push(match last {
                    'L' => Path::Left(n),
                    'R' => Path::Right(n),
                    _ => return Err(crate::day::InputError::InvalidInput),
                });

                cur.clear();
                last = ch;
            }
            _ => cur.push(ch),
        }
    }

    path.push(match last {
        'L' => Path::Left(cur.trim().parse::<u32>()?),
        'R' => Path::Right(cur.trim().parse::<u32>()?),
        _ => return Err(crate::day::InputError::InvalidInput),
    });

    Ok(Input::D22((path, map)))
}
