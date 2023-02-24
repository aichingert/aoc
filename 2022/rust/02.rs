// Advent of Code 2022, day 2
// (c) aichingert

fn part1() {}

fn part2() {}

fn parse() -> Vec<(RPS, RPS)> {
    let mut out = Vec::<(RPS,RPS)>::new();

    for l in std::fs::read_to_string("../input/01").unwrap().lines() {
        let val = l.split(' ').collect::<Vec<&str
    }
}

fn main() {

    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
