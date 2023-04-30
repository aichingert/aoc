// Advent of Code 2018, day 1
// (c) aichingert

fn part1(frq: &Vec<i32>) -> i32 {
    frq.iter().sum::<i32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", part1(&inp));
}
