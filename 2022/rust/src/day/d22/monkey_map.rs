use crate::day::{Coordinate, Input, InputResult, Output, Path, Wrapper};
use std::collections::BTreeMap;

enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn get_vec(&self) -> (i32, i32) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }
}

const DIRS: [Dir; 4] = [Dir::North, Dir::East, Dir::South, Dir::West];

fn part_one(map: &BTreeMap<Coordinate, char>, path: &Vec<Path>, start: Coordinate) -> u32 {
    let mut pos = start;
    let mut dir = 0;
    let mut idx = 0;

    while idx < path.len() {
        match path[idx] {
            Path::Left(n) => {
                dir = if dir == 0 { DIRS.len() - 1 } else { dir - 1 };
                let vec = DIRS[dir].get_vec();

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
                            None => match DIRS[dir] {
                                Dir::North => {
                                    let (c, ch) =
                                        map.iter().filter(|c| c.0.x == next.x).last().unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::South => {
                                    let (c, ch) = map
                                        .iter()
                                        .filter(|c| c.0.x == next.x)
                                        .rev()
                                        .last()
                                        .unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::East => {
                                    let (c, ch) = map
                                        .iter()
                                        .filter(|c| c.0.y == next.y)
                                        .rev()
                                        .last()
                                        .unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::West => {
                                    let (c, ch) =
                                        map.iter().filter(|c| c.0.y == next.y).last().unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                            },
                        }
                    } else {
                        match DIRS[dir] {
                            Dir::North => {
                                let (c, ch) = map.iter().filter(|c| c.0.x == pos.x).last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::South => {
                                let (c, ch) =
                                    map.iter().filter(|c| c.0.x == pos.x).rev().last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::East => {
                                let (c, ch) =
                                    map.iter().filter(|c| c.0.y == pos.y).rev().last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::West => {
                                let (c, ch) = map.iter().filter(|c| c.0.y == pos.y).last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                        };
                    }
                }
            }
            Path::Right(n) => {
                dir = (dir + 1) % DIRS.len();
                let vec = DIRS[dir].get_vec();

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
                            None => match DIRS[dir] {
                                Dir::North => {
                                    let (c, ch) =
                                        map.iter().filter(|c| c.0.x == next.x).last().unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::South => {
                                    let (c, ch) = map
                                        .iter()
                                        .filter(|c| c.0.x == next.x)
                                        .rev()
                                        .last()
                                        .unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::East => {
                                    let (c, ch) = map
                                        .iter()
                                        .filter(|c| c.0.y == next.y)
                                        .rev()
                                        .last()
                                        .unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                                Dir::West => {
                                    let (c, ch) =
                                        map.iter().filter(|c| c.0.y == next.y).last().unwrap();
                                    if *ch == '.' {
                                        pos = *c;
                                    }
                                }
                            },
                        }
                    } else {
                        match DIRS[dir] {
                            Dir::North => {
                                let (c, ch) = map.iter().filter(|c| c.0.x == pos.x).last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::South => {
                                let (c, ch) =
                                    map.iter().filter(|c| c.0.x == pos.x).rev().last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::East => {
                                let (c, ch) =
                                    map.iter().filter(|c| c.0.y == pos.y).rev().last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                            Dir::West => {
                                let (c, ch) = map.iter().filter(|c| c.0.y == pos.y).last().unwrap();
                                if *ch == '.' {
                                    pos = *c;
                                }
                            }
                        };
                    }
                }
            }
        }

        idx += 1;
    }

    let facing = match DIRS[dir] {
        Dir::North => 3,
        Dir::South => 1,
        Dir::East => 0,
        Dir::West => 2,
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

    for (y, line) in inp_map.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' | '#' => {
                    map.insert(Coordinate::new(y, x), ch);
                }
                ' ' => {}
                _ => return Err(crate::day::InputError::InvalidInput),
            }
        }
    }

    let mut cur = String::new();
    let mut last: char = 'R';

    for ch in inp_path.chars() {
        match ch {
            'L' | 'R' => {
                println!("{:?}", cur);
                let n = cur.parse::<u32>()?;

                path.push(if last == 'L' {
                    Path::Left(n)
                } else {
                    Path::Right(n)
                });

                cur.clear();
                last = ch;
            }
            _ => cur.push(ch),
        }
    }

    path.push(if last == 'R' {
        Path::Right(cur.trim().parse::<u32>()?)
    } else {
        Path::Left(cur.trim().parse::<u32>()?)
    });

    Ok(Input::D22((path, map)))
}
