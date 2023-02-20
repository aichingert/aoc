// Advent of Code 2019, day 5
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::Computer;

fn part1(opcodes: &Vec<i32>) {
    let mut computer = Computer::new(opcodes.clone(),1);
    computer.run();
}

fn part2() {}

fn main() {
    let opcodes = std::fs::read_to_string("../input/05").unwrap().trim().split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    part1(&opcodes);
    //println!("Part 2: {}", part2());
}
