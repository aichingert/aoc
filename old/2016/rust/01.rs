// Advent of Code 2016, day 1
// (c) aichingert

use std::collections::HashSet;

fn part1(inp: &Vec<&str>) -> i32 {
    let (mut x, mut y, mut d): (i32, i32 ,i32) = (0,0,0);

    for i in 0..inp.len() {
        let n: i32 = inp[i][1..].parse::<i32>().unwrap();
        d = update(d, &inp[i][0..1] == "R");

        match d {
            0 => x += n,
            90 => y += n,
            180 => x -= n,
            270 => y -= n,
            _ => ()
        };
    }

    x.abs() + y .abs()
}

fn contains(s: &mut HashSet<(i32, i32)>, p: &mut (i32, i32), n: i32, r: i32, c: i32) -> Option<(i32, i32)> {
    for i in 0..n {
        if s.contains(&(p.0 + (i * r), p.1 + (i * c))) {
            return Some((p.0 + (i * r), p.1 + (i * c)));
        } else {
            s.insert((p.0 + (i * r), p.1 + (i * c)));
        }
    }

    p.0 += n * r;
    p.1 += n * c;

    None
}

fn part2(inp: &Vec<&str>) -> i32 {
    let (mut p, mut d, mut i): ((i32, i32), i32, usize) = ((0,0),0,0);
    let mut set: HashSet<(i32, i32)> = HashSet::new();

    loop {
        d = update(d, &inp[i][0..1] == "R");
        let n: i32 = inp[i][1..].parse::<i32>().unwrap();

        if let Some((x, y)) = match d {
            0 => contains(&mut set, &mut p, n, 0, -1),
            90 => contains(&mut set, &mut p, n, 1, 0),
            180 => contains(&mut set, &mut p, n, 0, 1),
            270 => contains(&mut set, &mut p, n, -1, 0),
            _ => Some((0, 0))
        } {
            return x.abs() + y.abs()
        }

        i += 1;
        if i == inp.len() { i = 0; }
    }
}

fn update(d: i32, p: bool) -> i32 {
    if p {
        (d + 90) % 360
    } else {
        ((d - 90) + 360) % 360
    }
}

fn main() {
    let inp: String = std::fs::read_to_string("../input/01").unwrap();
    let inp: Vec<&str> = inp.trim().split(", ").collect();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
