#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(&opcodes, 1);
    let mut out = 0;

    loop {
        match vm.execute() {
            Status::Output(n) => out = n,
            Status::Exit      => break,
            _ => {},
        }
    }

    out
}

fn part_two(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(&opcodes, 2);
    let mut out = 0;

    loop {
        match vm.execute() {
            Status::Output(n) => out = n,
            Status::Exit      => break,
            _ => {},
        }
    }

    out
}

fn main() {
    let opcodes = std::fs::read_to_string("../input/09").unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();

    println!("Part one: {}", part_one(&opcodes));
    println!("Part two: {}", part_two(&opcodes));
}
