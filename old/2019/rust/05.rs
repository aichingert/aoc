// Advent of Code 2019, day 5
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::Computer;

fn part1(opcodes: &Vec<i32>) -> i32 {
    let mut computer = Computer::new(opcodes.clone(),vec![1]);
    computer.run();
    computer.output[computer.output.len()-1]
}

fn part2(opcodes: &Vec<i32>) -> i32 {
    let mut computer = Computer::new(opcodes.clone(),vec![5]);
    computer.run();
    computer.output[computer.output.len()-1]
}

fn main() {
    let opcodes = std::fs::read_to_string("../input/05").unwrap().trim().split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("Part 1: {}", part1(&opcodes));
    println!("Part 2: {}", part2(&opcodes));
}
