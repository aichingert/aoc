// Advent of Code 2019, day 5
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes.clone(), 1);
    let mut out = 0;

    loop {
        match vm.execute() {
            Status::Output(n) => out = n,
            Status::Exit => break,
            _ => {},
        }
    }

    out
}

/*
fn part2(opcodes: &Vec<i32>) -> i32 {
    let mut computer = Computer::new(opcodes.clone(),vec![5]);
    computer.run();
    computer.output[computer.output.len()-1]
}
*/

fn main() {
    let opcodes = std::fs::read_to_string("../input/05")
        .unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();

    println!("Part 1: {}", part_one(&opcodes));
    //println!("Part 2: {}", part2(&opcodes));
}
