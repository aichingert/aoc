// Advent of Code 2019, day 2
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm: VM = VM::new(opcodes.clone(), 0);

    loop {
        match vm.execute() {
            Status::Exit => break,
            _ => {},
        };
    }

    vm._get_position(0)
}

fn part_two(opcodes: &mut Vec<N>) -> N {
    for noun in 0..100 {
        for verb in 0..100 {
            opcodes[1] = noun;
            opcodes[2] = verb;

            let mut vm: VM = VM::new(opcodes.clone(), 0);

            loop {
                match vm.execute() {
                    Status::Exit => break,
                    _ => {},
                }
            }

            if vm._get_position(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("not found")
}

fn main() {
    let mut opcodes = std::fs::read_to_string("../input/02")
        .unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();
    opcodes[1] = 12;
    opcodes[2] = 2;

    println!("Part 1: {}", part_one(&opcodes));
    println!("Part 2: {}", part_two(&mut opcodes));
}
