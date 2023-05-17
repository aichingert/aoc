// Advent of Code 2018, day 22
// (c) aichingert

use std::collections::HashMap;

fn geologic_index(gi: &mut HashMap<(i64, i64), i64>, x: i64, y: i64, depth: i64) -> i64 {
    if gi.contains_key(&(x, y)) {
        return gi[&(x, y)];
    }

    let geoi = match (x, y) {
        (0, _) => y * 48271,
        (_, 0) => x * 16807,
        (_, _) => {
            let mut a = (geologic_index(gi, x - 1, y, depth) + depth) % 20183;
            let mut b = (geologic_index(gi, x, y - 1, depth) + depth) % 20183;

            a * b
        }
    };
    gi.insert((x, y), geoi);

    gi[&(x, y)]
}

fn part1(depth: i64, x: i64, y: i64, gi: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut risk = 0;
    for i in 0..=y {
        for j in 0..=x {
            let erosion_level = (geologic_index(gi, j, i, depth) + depth) % 20183;
            let cave_type = erosion_level % 3;
            risk += cave_type;

            match cave_type {
                0 => print!("."),
                1 => print!("="),
                2 => print!("|"),
                _ => (),
            };
        }
        println!("");
    }

    risk
}

fn main() {
    let inp = std::fs::read_to_string("../input/22").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let depth: i64 = inp[0][7..].parse().unwrap();
    let (x, y): (&str, &str) = inp[1].split_once(',').unwrap();
    let (x, y): (i64, i64) = (x[8..].parse().unwrap(), y.parse().unwrap());
    let mut geologic_index: HashMap<(i64, i64), i64> = HashMap::from([((0, 0), 0), ((x, y), 0)]);

    println!("Part 1: {}", part1(depth, x, y, &mut geologic_index));
}
