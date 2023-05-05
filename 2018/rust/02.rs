// Advent of Code 2018, day 2
// (c) aichingert

use std::collections::HashMap;

fn part1(lines: &Vec<Vec<char>>) -> i32 {
    let (mut two, mut three) = (0,0);

    for i in 0..lines.len() {
        let mut counter = HashMap::<char,i32>::new();

        for j in 0..lines[i].len() {
            *counter.entry(lines[i][j]).or_insert(0) += 1;
        }

        if counter.values().any(|&x| x == 2) { two += 1; }
        if counter.values().any(|&x| x == 3) { three += 1; }
    }

    two * three
}

fn part2(lines: &Vec<Vec<char>>) -> String {
    for i in 0..lines.len() {
        let mut diff;

        for j in i+1..lines.len() {
            if lines[j].len() != lines[i].len() { continue; }
            diff = 0;

            for k in 0..lines[i].len() {
                if lines[i][k] != lines[j][k] {
                    diff += 1;
                }
            }

            if diff == 1 {
                let mut res = Vec::<char>::new();

                for k in 0..lines[i].len() {
                    if lines[i][k] == lines[j][k] {
                        res.push(lines[i][k]);
                    }
                }

                return res.iter().collect::<String>();
            }
        }
    }

    "".to_string()
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
