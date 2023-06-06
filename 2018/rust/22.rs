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
    let mut risk: i64 = 0;
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

fn part2(
    depth: i64,
    tx: i64,
    ty: i64,
    gi: &mut HashMap<(i64, i64), i64>,
    index: &mut HashMap<(i64, i64), i64>,
    dist: &mut HashMap<(i64, i64), i64>,
    q: &mut HashSet<(i64, i64)>,
    already: &mut HashSet<(i64, i64)>,
    gear: i64,
    minutes: i64,
) -> i64 {
    let mut gear = gear;
    let mut minutes = minutes;

    while !q.is_empty() {
        let u = {
            let mut best = i64::MAX;
            let mut found = None;

            for u in q.clone() {
                let v = dist.get(&u).unwrap();

                if *v < best {
                    best = *v;
                    found = Some(u);
                }
            }

            found.unwrap()
        };

        q.remove(&u);

        for m in [
            (u.0, u.1 + 1),
            (u.0 + 1, u.1),
            (u.0 - 1, u.1),
            (u.0, u.1 - 1),
        ]
        .into_iter()
        .filter(|(x, y)| *x > -1 && *y > -1 && *x < tx + 20 && *y < ty + 20)
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<(i64, i64)>>()
        {
            let v = if q.contains(&m) {
                m
            } else if !index.contains_key(&m) {
                index.insert(m, minutes);
                minutes += 1;
                dist.insert(m, i64::MAX);
                q.insert(m);
                m
            } else {
                continue;
            };

            let alt = dist.get(&u).unwrap() + 1;

            if alt < *dist.get(&v).unwrap() {
                dist.insert(v.clone(), alt);
            }
        }
    }

    println!("{:?}", dist[&(tx, ty)]);
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
    println!(
        "Part 2: {}",
        part2(
            depth,
            x,
            y,
            &mut geologic_index,
            &mut HashMap::from([((0, 0), 0)]),
            &mut HashMap::from([((0, 0), 0)]),
            &mut HashSet::from([(0, 0)]),
            0,
            1
        )
    );
}
