// Advent of Code 2022, day 14
// (c) aichingert

use std::collections::HashSet;

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Debug)]
enum State {
    Stopped,
    Continue,
    Infinite,
}

fn solve(cave: &HashSet<Pos>, bottom: i32) -> u32 {
    let mut resting: u32 = 0;
    let mut sand: HashSet<Pos> = HashSet::new();

    'outer: loop {
        let (mut x, mut y) = (500, 0);
        let mut state: State = State::Continue;

        while state == State::Continue {
            state = match drop_sand(cave, &mut sand, bottom, &mut x, &mut y) {
                State::Stopped => {
                    if x == 500 && y == 0 {
                        return resting + 1;
                    }

                    resting += 1;
                    sand.insert((x, y));
                    State::Stopped
                }
                State::Continue => State::Continue,
                State::Infinite => break 'outer,
            };
        }
    }

    resting
}

fn drop_sand(
    cave: &HashSet<Pos>,
    sand: &mut HashSet<Pos>,
    bottom: i32,
    x: &mut i32,
    y: &mut i32,
) -> State {
    if try_down(cave, &sand, *x, y, bottom) {
        if try_left(cave, sand, x, *y) || try_right(cave, sand, x, *y) {
            return State::Continue;
        }
    } else {
        return State::Infinite;
    }

    State::Stopped
}

fn try_down(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: i32, y: &mut i32, bottom: i32) -> bool {
    while *y < bottom && !cave.contains(&(x, *y + 1)) && !sand.contains(&(x, *y + 1)) {
        *y += 1;
    }

    *y < bottom
}

fn try_left(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: &mut i32, y: i32) -> bool {
    if cave.contains(&(*x - 1, y + 1)) || sand.contains(&(*x - 1, y + 1)) {
        return false;
    }

    *x -= 1;
    true
}

fn try_right(cave: &HashSet<Pos>, sand: &HashSet<Pos>, x: &mut i32, y: i32) -> bool {
    if cave.contains(&(*x + 1, y + 1)) || sand.contains(&(*x + 1, y + 1)) {
        return false;
    }

    *x += 1;
    true
}

fn parse() -> (HashSet<Pos>, i32) {
    let inp = std::fs::read_to_string("../input/14").unwrap();
    let mut cave = HashSet::new();
    let mut bottom: i32 = 0;

    for line in inp.lines() {
        let values: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|s| {
                let xy = s.split_once(',').unwrap();
                (xy.0.parse().unwrap(), xy.1.parse().unwrap())
            })
            .collect();

        for i in 0..values.len() - 1 {
            if values[i].0 == values[i + 1].0 {
                let from = values[i].1.min(values[i + 1].1);
                let to = values[i].1.max(values[i + 1].1);

                for y in from..=to {
                    cave.insert((values[i].0, y));
                }

                bottom = bottom.max(to);
            } else {
                let from = values[i].0.min(values[i + 1].0);
                let to = values[i].0.max(values[i + 1].0);

                for x in from..=to {
                    cave.insert((x, values[i].1));
                }
            }
        }
    }

    (cave, bottom)
}

fn main() {
    let (mut cave, bottom) = parse();

    println!("Part 1: {}", solve(&cave, bottom));

    for x in -1000..1000 {
        cave.insert((x, bottom + 2));
    }
    println!("Part 2: {}", solve(&cave, bottom + 2));
}
