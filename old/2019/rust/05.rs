// Advent of Code 2019, day 5
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 1);
    let mut out = 0;

    loop {
        match vm.exec() {
            Status::Output(n) => out = n,
            Status::Exit => break,
            _ => {},
        }
    }

    out
}

fn part_two(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 5);
    let mut out = 0;

    loop {
        match vm.exec() {
            Status::Output(n) => out = n,
            Status::Exit => break,
            _ => {},
        }
    }

    out
}

fn main() {
    let opcodes = std::fs::read_to_string("../input/05")
        .unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();

    println!("Part 1: {}", part_one(&opcodes));
    println!("Part 2: {}", part_two(&opcodes));
}
