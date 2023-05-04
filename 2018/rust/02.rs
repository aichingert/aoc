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
        let mut pw = false;
        let mut ph = false;

        for val in counter.values() {
            if *val == 2 {
                pw = true;
            }

            if *val == 3 {
                ph = true;
            }
        }

        if pw { two += 1; }
        if ph { three += 1; }

    }

    two * three
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", part1(&inp));
}
